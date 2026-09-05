// src/lowerer.rs
use std::collections::{HashMap, HashSet};
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
    loop_depth: usize, // <--- ADDED
}

impl IrLowerer {
    pub fn new() -> Self {
        let entry_block = BasicBlock::new(0, 0);
        Self {
            blocks: vec![entry_block], current_block: 0, free_reg: 0,
            scopes: vec![HashMap::new()], loop_depth: 0,
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

    pub fn lower_program(mut self, stmts: &[Stmt]) -> IrProgram {
        for stmt in stmts {
            self.lower_stmt(stmt);
        }
        if self.blocks[self.current_block].terminator.is_none() {
            self.terminate(Terminator::Halt);
        }
        IrProgram { blocks: self.blocks }
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
                let table_local = self.read_var(table);
                let (index_reg, _) = self.lower_expr(index, None);
                let (val_reg, _) = self.lower_expr(expr, None);

                self.emit(Instruction::SetTable {
                    table: table_local.reg,
                    key: index_reg,
                    val: val_reg,
                });
            }
            Stmt::While { condition, body } => {
                let pre_header = self.current_block;

                self.loop_depth += 1;
                let header_block = self.new_block();
                let body_block = self.new_block();

                self.loop_depth -= 1; // Exit block belongs to the outer scope
                let exit_block = self.new_block();
                self.loop_depth += 1; // Restore for body generation

                let mutated_vars = find_mutated_vars(body);
                let mut phis = Vec::new();

                self.terminate(Terminator::Jump(header_block));
                self.current_block = header_block;

                // FIX: Pass the type into the Phi node
                for var in mutated_vars {
                    if self.has_var(&var) {
                        let pre_loop_local = self.read_var(&var);
                        let phi_reg = self.next_reg();

                        self.emit(Instruction::Phi {
                            target: phi_reg,
                            ty: pre_loop_local.ty.clone(), // <-- ADDED
                            args: vec![(pre_header, pre_loop_local.reg)],
                        });
                        self.update_var(&var, phi_reg);
                        phis.push((var, phi_reg));
                    }
                }

                let (cond_reg, _) = self.lower_expr(condition, None);
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

                for (var, phi_reg) in phis {
                    let back_edge_local = self.read_var(&var);
                    for instr in &mut self.blocks[header_block].instrs {
                        if let Instruction::Phi { target, args, .. } = instr {
                            if *target == phi_reg {
                                args.push((end_of_body, back_edge_local.reg));
                                break;
                            }
                        }
                    }
                }
                self.current_block = exit_block;

                // FIX: Restore the loop depth for the outer scope!
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
            Expr::NewTable => {
                self.emit(Instruction::NewTable { target: reg });
                (reg, StaticType::Table)
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
                let (t_reg, _) = self.lower_expr(table, None);
                let (i_reg, _) = self.lower_expr(index, None);
                self.emit(Instruction::GetTable { target: reg, table: t_reg, key: i_reg });
                (reg, StaticType::Integer)
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
fn find_mutated_vars(stmts: &[Stmt]) -> HashSet<String> {
    let mut mutated = HashSet::new();
    for stmt in stmts {
        match stmt {
            Stmt::Assignment { name, .. } => { mutated.insert(name.clone()); }
            Stmt::While { body, .. } => { mutated.extend(find_mutated_vars(body)); }
            _ => {}
        }
    }
    mutated
}
