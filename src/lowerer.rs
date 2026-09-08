// src/lowerer.rs
use std::collections::{HashMap, BTreeSet};
use crate::ast::{Expr, Stmt, BinOp, StaticType};
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
    /// pre-materialized literal-bound registers (`bound_left` / `bound_right`)
    /// in for the direct `Expr::Integer` operands that produced them.
    ///
    /// The splice is self-checking: a bound register is used only when the
    /// operand it replaces is actually an integer literal; any other operand
    /// lowers right here, exactly as `lower_expr` would. Operand order
    /// (left, then right) and the target-register-first numbering of
    /// `lower_expr`'s `BinaryOp` path are preserved.
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
                // The backend's bounds-check hoisting only fires when the loop
                // limit is defined BEFORE the header (`limit_def_block <
                // header_id`). A literal bound (`while i < 10`) would otherwise get
                // its `LoadInt` emitted inside the header. So, while
                // `current_block` is still the pre-header, materialize any *direct*
                // integer operand of a `LessThan` condition into a register NOW,
                // and splice that register into the condition when it is lowered
                // inside the header (see `lower_while_condition`).
                //
                // `LoadInt` is pure and cannot trap, so hoisting it across the
                // pre-header/header edge is always sound. The AST is never touched:
                // the "modified condition" is just two `Option<RegId>`s.
                let mut bound_left: Option<RegId> = None;
                let mut bound_right: Option<RegId> = None;
                if let Expr::BinaryOp { op: BinOp::LessThan, left, right } = condition {
                    // Left first, then right: preserves operand order.
                    bound_left = self.materialize_bound(left);
                    bound_right = self.materialize_bound(right);
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
                let (l_reg, _) = self.lower_expr(left, None);
                let (r_reg, _) = self.lower_expr(right, None);
                match op {
                    BinOp::Add => self.emit(Instruction::Add { target: reg, left: l_reg, right: r_reg }),
                    BinOp::Sub => self.emit(Instruction::Sub { target: reg, left: l_reg, right: r_reg }),
                    BinOp::LessThan => self.emit(Instruction::Less { target: reg, left: l_reg, right: r_reg }),
                }
                let ty = match op {
                    BinOp::LessThan => StaticType::Boolean,
                    _ => StaticType::Integer,
                };
                (reg, ty)
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
            _ => {}
        }
    }
    mutated
}
