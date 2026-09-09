// src/lowerer.rs
use std::collections::{HashMap, BTreeSet};
use crate::ast::{Expr, Stmt, BinOp, UnOp, StaticType};
use crate::ir::{Instruction, BasicBlock, Terminator, BlockId, RegId, IrProgram};

#[derive(Clone)]
struct Local {
    reg: RegId,
    ty: StaticType,
}

pub struct IrLowerer {
    pub blocks: Vec<BasicBlock>,
    current_block: BlockId,
    free_reg: RegId,
    scopes: Vec<HashMap<String, Local>>,
    loop_depth: usize,
    type_map: HashMap<usize, StaticType>, // table id -> resolved element type (from the checker)
}

impl IrLowerer {
    pub fn new() -> Self {
        let entry_block = BasicBlock::new(0, 0);
        Self {
            blocks: vec![entry_block], current_block: 0, free_reg: 0,
            scopes: vec![HashMap::new()], loop_depth: 0,
            type_map: HashMap::new(),
        }
    }

    fn new_block(&mut self) -> BlockId {
        let id = self.blocks.len();
        self.blocks.push(BasicBlock::new(id, self.loop_depth));
        id
    }

    fn next_reg(&mut self) -> RegId {
        let r = self.free_reg;
        self.free_reg += 1;
        r
    }

    fn emit(&mut self, instr: Instruction) {
        self.blocks[self.current_block].instrs.push(instr);
    }

    fn terminate(&mut self, term: Terminator) {
        self.blocks[self.current_block].terminator = Some(term);
    }

    // --- Variable Tracking ---

    fn declare_var(&mut self, name: String, reg: RegId, ty: StaticType) {
        self.scopes.last_mut().unwrap().insert(name, Local { reg, ty });
    }

    fn update_var(&mut self, name: &str, reg: RegId) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(local) = scope.get_mut(name) {
                local.reg = reg; // Update to the new SSA version
                return;
            }
        }
        panic!("Lowerer: Undeclared variable");
    }

    fn read_var(&self, name: &str) -> Local {
        for scope in self.scopes.iter().rev() {
            if let Some(local) = scope.get(name) {
                return local.clone();
            }
        }
        panic!("Lowerer: Undeclared variable");
    }

    fn has_var(&self, name: &str) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(name) {
                return true;
            }
        }
        false
    }
    // --- Lowering Logic ---

    pub fn lower_program(mut self, stmts: &[Stmt], type_map: HashMap<usize, StaticType>) -> IrProgram {
        self.type_map = type_map;
        for stmt in stmts {
            self.lower_stmt(stmt);
        }
        if self.blocks[self.current_block].terminator.is_none() {
            self.terminate(Terminator::Halt);
        }
        IrProgram { blocks: self.blocks }
    }

    /// Materialize an integer literal into a register in the CURRENT block.
    /// Call this while `current_block` is still the loop pre-header.
    /// Returns `None` (and emits nothing) for any non-literal operand —
    /// those must be lowered inside the header, where loop-carried phis
    /// are visible.
    fn materialize_bound(&mut self, operand: &Expr) -> Option<RegId> {
        if let Expr::Integer(v) = operand {
            let reg = self.next_reg();
            self.emit(Instruction::LoadInt { target: reg, val: *v });
            Some(reg)
        } else {
            None
        }
    }

    /// Lower a `while` condition inside the header, splicing the
    /// pre-materialized literal-bound registers in for the direct
    /// `Expr::Integer` operands that produced them; any other operand
    /// lowers right here, exactly as `lower_expr` would (the splice is
    /// self-checking by construction). Operand order and target-first
    /// numbering are preserved. Why this exists: opt_literal_bound.lua.
    /// The comparison desugars ride the same splice so `while i <= n` and
    /// `while n > i` keep the tier4 `idx < invariant` gate shape:
    ///   a > b  => Less(b, a)      (operands swap, bounds follow)
    ///   a <= b => Less(a, b+1)    (the +1 is PRE-materialized with the
    ///                              literal bound, outside the header)
    fn lower_while_condition(
        &mut self,
        condition: &Expr,
        bound_left: Option<RegId>,
        bound_right: Option<RegId>,
    ) -> (RegId, StaticType) {
        match condition {
            Expr::BinaryOp { op: BinOp::LessThan, left, right }
                if bound_left.is_some() || bound_right.is_some() =>
            {
                let target = self.next_reg();

                let l_reg = match (&**left, bound_left) {
                    (Expr::Integer(_), Some(reg)) => reg,
                    _ => self.lower_expr(left, None).0,
                };
                let r_reg = match (&**right, bound_right) {
                    (Expr::Integer(_), Some(reg)) => reg,
                    _ => self.lower_expr(right, None).0,
                };

                self.emit(Instruction::Less { target, left: l_reg, right: r_reg });
                (target, StaticType::Boolean)
            }
            // a > b: Less with swapped operands; the Less's left operand is
            // the AST's right (and vice versa), so the splices swap too
            Expr::BinaryOp { op: BinOp::GreaterThan, left, right }
                if bound_left.is_some() || bound_right.is_some() =>
            {
                let target = self.next_reg();

                let l_reg = match (&**right, bound_right) {
                    (Expr::Integer(_), Some(reg)) => reg,
                    _ => self.lower_expr(right, None).0,
                };
                let r_reg = match (&**left, bound_left) {
                    (Expr::Integer(_), Some(reg)) => reg,
                    _ => self.lower_expr(left, None).0,
                };

                self.emit(Instruction::Less { target, left: l_reg, right: r_reg });
                (target, StaticType::Boolean)
            }
            // a <= b: bound_right is ALREADY the pre-header b+1 register
            Expr::BinaryOp { op: BinOp::LessEq, left, .. } if bound_right.is_some() => {
                let target = self.next_reg();

                let l_reg = match (&**left, bound_left) {
                    (Expr::Integer(_), Some(reg)) => reg,
                    _ => self.lower_expr(left, None).0,
                };

                self.emit(Instruction::Less { target, left: l_reg, right: bound_right.unwrap() });
                (target, StaticType::Boolean)
            }
            // No materialized bounds: byte-for-byte the original lowering.
            _ => self.lower_expr(condition, None),
        }
    }

    fn lower_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::LocalDecl { name, expr } => {
                let target_reg = self.next_reg();
                let (_, ty) = self.lower_expr(expr, Some(target_reg));
                self.declare_var(name.clone(), target_reg, ty);
            }
            Stmt::Assignment { name, expr } => {
                let new_reg = self.next_reg();
                self.lower_expr(expr, Some(new_reg));
                self.update_var(name, new_reg);
            }
            Stmt::TableAssign { table, index, expr } => {
                // A bare `name[...]` lvalue reads the variable directly:
                // lowering it as an expression would mint (and abandon) a
                // fresh vreg — burning an id shifts phys_base and every
                // physical register in the program. Only genuine
                // sub-expressions (nested lvalues like `t[0][i]`) lower.
                let t_reg = match table {
                    Expr::Identifier(name) => self.read_var(name).reg,
                    _ => self.lower_expr(table, None).0,
                };
                let (index_reg, _) = self.lower_expr(index, None);
                let (val_reg, val_ty) = self.lower_expr(expr, None);

                self.emit(Instruction::SetTable {
                    table: t_reg,
                    key: index_reg,
                    val: val_reg,
                    ty: val_ty,
                });
            }
            Stmt::While { condition, body } => {
                let pre_header = self.current_block;

                // --- Literal bound materialization --------------------------------
                // The loop gate needs the limit defined BEFORE the header
                // (`limit_def_block < header_id`); a literal bound would
                // otherwise LoadInt inside it. While still in the pre-header,
                // materialize direct integer operands of the LessThan
                // condition NOW and splice them into the header-lowered
                // condition (lower_while_condition). LoadInt is pure — the
                // hoist is always sound; the AST is never touched. Pinned by
                // opt_literal_bound.lua (the fast_sets/hoists flip).
                let mut bound_left: Option<RegId> = None;
                let mut bound_right: Option<RegId> = None;
                if let Expr::BinaryOp { op, left, right } = condition {
                    match op {
                        BinOp::LessThan | BinOp::GreaterThan => {
                            // Left first, then right: preserves operand order.
                            bound_left = self.materialize_bound(left);
                            bound_right = self.materialize_bound(right);
                        }
                        BinOp::LessEq => {
                            // a <= b => a < b+1: materialize the literal
                            // bound AND the +1 here in the pre-header, so
                            // the tier4 gate sees an invariant limit.
                            // The desugar is only exact when b+1 cannot
                            // overflow: at b == i64::MAX the Add would wrap
                            // to MIN and flip the comparison. checked_add
                            // declines there, no bound is materialized, and
                            // the condition lowers to a native Leq in the
                            // header instead (no lock covers the MAX shape).
                            bound_left = self.materialize_bound(left);
                            if let Expr::Integer(v) = &**right {
                                if v.checked_add(1).is_some() {
                                    if let Some(r) = self.materialize_bound(right) {
                                        let one = self.next_reg();
                                        self.emit(Instruction::LoadInt { target: one, val: 1 });
                                        let r2 = self.next_reg();
                                        self.emit(Instruction::Add { target: r2, left: r, right: one });
                                        bound_right = Some(r2);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }

                self.loop_depth += 1;
                let header_block = self.new_block();
                let body_block = self.new_block();

                self.loop_depth -= 1; // Exit block belongs to the outer scope
                let exit_block = self.new_block();
                self.loop_depth += 1; // Restore for body generation

                let mutated_vars = find_mutated_vars(body);

                // CANONICAL PHI ORDER — the rename-invariance fix.
                // find_mutated_vars returns a BTreeSet<String>, so phi
                // minting order was ALPHABETICAL BY NAME. A pure rename
                // can flip that order → the two phi vreg ids swap →
                // header slots swap → interval starts swap → the
                // (start, end, r) total order flips their allocation
                // order → physical registers swap. Semantics were never
                // at risk (allocation is order-correct); codegen simply
                // wasn't canonical: same logic, different spelling.
                let mut phi_order: Vec<(String, Local)> = mutated_vars.into_iter()
                    .filter(|name| self.has_var(name))
                    .map(|name| { let local = self.read_var(&name); (name, local) })
                    .collect();
                phi_order.sort_by(|(name_a, a), (name_b, b)|
                    (a.reg, name_a).cmp(&(b.reg, name_b)));

                let mut phis = Vec::new();

                self.terminate(Terminator::Jump(header_block));
                self.current_block = header_block;

                for (var, pre_loop_local) in phi_order {
                    let phi_reg = self.next_reg();
                    self.emit(Instruction::Phi {
                        target: phi_reg,
                        ty: pre_loop_local.ty.clone(),
                        args: vec![(pre_header, pre_loop_local.reg)],
                    });
                    self.update_var(&var, phi_reg);
                    phis.push((var, phi_reg));
                }

                // The condition still lowers INSIDE the header (phi'd variables are
                // read here), but a materialized literal bound now consumes its
                // pre-header register instead of re-emitting `LoadInt` in the loop.
                let (cond_reg, _) =
                    self.lower_while_condition(condition, bound_left, bound_right);
                self.terminate(Terminator::Branch {
                    cond: cond_reg,
                    true_block: body_block,
                    false_block: exit_block,
                });

                self.current_block = body_block;
                self.scopes.push(HashMap::new());
                for s in body { self.lower_stmt(s); }
                self.scopes.pop();

                let end_of_body = self.current_block;
                self.terminate(Terminator::Jump(header_block));

                for (var, phi_reg) in &phis {
                    let back_edge_local = self.read_var(var);
                    for instr in &mut self.blocks[header_block].instrs {
                        if let Instruction::Phi { target, args, .. } = instr {
                            if *target == *phi_reg {
                                args.push((end_of_body, back_edge_local.reg));
                                break;
                            }
                        }
                    }
                }

                // NEW: post-loop reads resolve to the phi, not the body's
                // last def. The header dominates the exit and the phi merges
                // the pre-loop value, so a zero-iteration loop still yields
                // the pre-loop value instead of a never-written register.
                // (Reads after an assignment but INSIDE the body were
                // already emitted against the body's def — unaffected.)
                for (var, phi_reg) in &phis {
                    self.update_var(var, *phi_reg);
                }

                self.current_block = exit_block;
                self.loop_depth -= 1;
            }
            Stmt::Probe { tag, exprs } => {
                // Operands lower in place, here: identifiers resolve to
                // their current SSA reg (lower_expr mints no Move for a
                // bare read), so the probe observes exactly what the next
                // instruction at this point would read.
                let mut operands = Vec::new();
                for e in exprs {
                    let (r, ty) = self.lower_expr(e, None);
                    operands.push((r, ty));
                }
                self.emit(Instruction::DebugProbe { tag: tag.clone(), operands });
            }
            Stmt::If { condition, then_body, else_body } => {
                // Structured if: cond block branches to then/else arms, both
                // arms jump to a join block. Variables mutated in EITHER arm
                // get a join phi (2 same-side preds -> classic Move lowering
                // in resolve_phis). The scope snapshot isolates the arms:
                // whatever the then-arm renames must not leak into the
                // else-arm's reads (each arm continues from the pre-if state).
                let (cond_reg, _) = self.lower_expr(condition, None);

                let then_block = self.new_block();
                let else_block = self.new_block();
                let join_block = self.new_block();

                self.terminate(Terminator::Branch {
                    cond: cond_reg,
                    true_block: then_block,
                    false_block: else_block,
                });

                let mut mutated = find_mutated_vars(then_body);
                mutated.extend(find_mutated_vars(else_body));
                let mutated: Vec<String> = mutated.into_iter()
                    .filter(|name| self.has_var(name))
                    .collect();

                // canonical order, same policy as the while loop phis
                let mut phi_order: Vec<(String, RegId)> = mutated.into_iter()
                    .map(|name| { let local = self.read_var(&name); (name, local.reg) })
                    .collect();
                phi_order.sort_by(|(name_a, a), (name_b, b)| (a, name_a).cmp(&(b, name_b)));

                let snapshot = self.scopes.clone();

                // then arm
                self.current_block = then_block;
                self.scopes.push(HashMap::new());
                for s in then_body { self.lower_stmt(s); }
                self.scopes.pop();
                let then_end = self.current_block;
                let then_regs: Vec<(String, RegId)> = phi_order.iter()
                    .map(|(name, _)| (name.clone(), self.read_var(name).reg))
                    .collect();
                self.terminate(Terminator::Jump(join_block));

                // else arm continues from the pre-if state
                self.scopes = snapshot;
                self.current_block = else_block;
                self.scopes.push(HashMap::new());
                for s in else_body { self.lower_stmt(s); }
                self.scopes.pop();
                let else_end = self.current_block;
                let else_regs: Vec<(String, RegId)> = phi_order.iter()
                    .map(|(name, _)| (name.clone(), self.read_var(name).reg))
                    .collect();
                self.terminate(Terminator::Jump(join_block));

                // join: one phi per mutated var, merging both arm outcomes
                self.current_block = join_block;
                for (i, (name, _)) in phi_order.iter().enumerate() {
                    let phi_reg = self.next_reg();
                    self.emit(Instruction::Phi {
                        target: phi_reg,
                        ty: self.read_var(name).ty.clone(),
                        args: vec![(then_end, then_regs[i].1), (else_end, else_regs[i].1)],
                    });
                    self.update_var(name, phi_reg);
                }
            }
        }
    }

    fn lower_expr(&mut self, expr: &Expr, target: Option<RegId>) -> (RegId, StaticType) {
        let reg = target.unwrap_or_else(|| self.next_reg());

        match expr {
            Expr::Integer(val) => {
                self.emit(Instruction::LoadInt { target: reg, val: *val });
                (reg, StaticType::Integer)
            }
            Expr::Float(val) => {
                self.emit(Instruction::LoadFloat { target: reg, val: *val });
                (reg, StaticType::Float)
            }
            Expr::Boolean(val) => {
                self.emit(Instruction::LoadBool { target: reg, val: *val });
                (reg, StaticType::Boolean)
            }
            Expr::NewTable(id) => {
                let ty = self.type_map.get(id).cloned()
                    .unwrap_or(StaticType::Table(Box::new(StaticType::Integer)));
                self.emit(Instruction::NewTable { target: reg, ty: ty.clone() });
                (reg, ty)
            }
            Expr::Identifier(name) => {
                let local = self.read_var(name);
                if target.is_some() && reg != local.reg {
                    self.emit(Instruction::Move { target: reg, source: local.reg, ty: local.ty.clone() });
                } else {
                    return (local.reg, local.ty);
                }
                (reg, local.ty)
            }
            Expr::TableIndex { table, index } => {
                let (t_reg, t_ty) = self.lower_expr(table, None);
                let (i_reg, _) = self.lower_expr(index, None);
                let ret_ty = match t_ty {
                    StaticType::Table(inner) => *inner,
                    _ => StaticType::Integer,
                };
                self.emit(Instruction::GetTable { target: reg, table: t_reg, key: i_reg, ty: ret_ty.clone() });
                (reg, ret_ty)
            }
            Expr::BinaryOp { op, left, right } => {
                let (l_reg, l_ty) = self.lower_expr(left, None);
                let (r_reg, r_ty) = self.lower_expr(right, None);
                // Comparison desugars ride the EXISTING Less machinery so the
                // tier4 loop gate (`i < lim` shape) keeps recognizing them:
                //   a > b  => b < a          (operand swap, exact)
                //   a <= b => a < b+1        (exact in i64/f64 integers/floats
                //                             away from overflow; see below)
                //   a >= b => b < a+1
                // The +1 is a fresh LoadInt/Add pair — for `while i <= n` the
                // Add is loop-invariant, so the gate still opens.
                match op {
                    BinOp::Add => self.emit(Instruction::Add { target: reg, left: l_reg, right: r_reg }),
                    BinOp::Sub => self.emit(Instruction::Sub { target: reg, left: l_reg, right: r_reg }),
                    BinOp::Mul => self.emit(Instruction::Mul { target: reg, left: l_reg, right: r_reg }),
                    BinOp::Div => self.emit(Instruction::Div { target: reg, left: l_reg, right: r_reg }),
                    BinOp::IntDiv => self.emit(Instruction::IntDiv { target: reg, left: l_reg, right: r_reg }),
                    BinOp::Mod => self.emit(Instruction::Mod { target: reg, left: l_reg, right: r_reg }),
                    BinOp::LessThan =>
                        self.emit(Instruction::Less { target: reg, left: l_reg, right: r_reg }),
                    BinOp::GreaterThan =>
                        self.emit(Instruction::Less { target: reg, left: r_reg, right: l_reg }),
                    // Native forms: the a < b+1 desugar wraps at i64::MAX and
                    // rounds wrong for floats from 2^53 up (b+1.0 == b), so
                    // plain-expression <= / >= compare directly. Only the
                    // while-literal-bound path keeps the desugar (tier4 gate).
                    BinOp::LessEq =>
                        self.emit(Instruction::Leq { target: reg, left: l_reg, right: r_reg }),
                    BinOp::GreaterEq =>
                        self.emit(Instruction::Geq { target: reg, left: l_reg, right: r_reg }),
                    BinOp::Equal =>
                        self.emit(Instruction::Eq { target: reg, left: l_reg, right: r_reg, ty: l_ty.clone() }),
                    BinOp::NotEqual => {
                        let e = self.next_reg();
                        self.emit(Instruction::Eq { target: e, left: l_reg, right: r_reg, ty: l_ty.clone() });
                        self.emit(Instruction::Not { target: reg, source: e });
                    }
                }
                // The checker guarantees same-type numeric operands, so the
                // static result type follows either operand. The IR Add/Sub
                // are untyped (allocate_registers re-derives their pool),
                // but THIS ty feeds store/decl metadata — an int label on a
                // float add makes the store template pick the wrong storage
                // side and render an f_r register as i_r (found by the Float
                // Gauntlet storing t[i] = t[i] + 0.25; pinned by float_14).
                let ty = match op {
                    BinOp::LessThan | BinOp::GreaterThan | BinOp::LessEq | BinOp::GreaterEq
                    | BinOp::Equal | BinOp::NotEqual => StaticType::Boolean,
                    _ => if matches!(l_ty, StaticType::Float) || matches!(r_ty, StaticType::Float) {
                        StaticType::Float
                    } else {
                        StaticType::Integer
                    },
                };
                (reg, ty)
            }
            Expr::UnaryOp { op, expr } => {
                let (x_reg, x_ty) = self.lower_expr(expr, None);
                match op {
                    UnOp::Neg => {
                        // True negation, not 0 - x: 0.0 - x leaves +0.0's
                        // sign bit unchanged (-0.0 must be negative zero,
                        // and it is checksum-visible). Wraps on i64::MIN
                        // exactly like Lua's integer arithmetic.
                        self.emit(Instruction::Neg { target: reg, source: x_reg });
                        (reg, x_ty)
                    }
                    UnOp::Not => {
                        self.emit(Instruction::Not { target: reg, source: x_reg });
                        (reg, StaticType::Boolean)
                    }
                }
            }
        }
    }
}

// Simple pre-pass to find variables reassigned in a block
fn find_mutated_vars(stmts: &[Stmt]) -> BTreeSet<String> {
    let mut mutated = BTreeSet::new();
    for stmt in stmts {
        match stmt {
            Stmt::Assignment { name, .. } => { mutated.insert(name.clone()); }
            Stmt::While { body, .. } => { mutated.extend(find_mutated_vars(body)); }
            Stmt::If { then_body, else_body, .. } => {
                mutated.extend(find_mutated_vars(then_body));
                mutated.extend(find_mutated_vars(else_body));
            }
            _ => {}
        }
    }
    mutated
}
