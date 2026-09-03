// src/lowerer.rs
use crate::ast::{Expr, Stmt, BinOp, StaticType};
use crate::ir::Instruction;

#[derive(Clone)]
struct Local {
    name: String,
    depth: usize,
    reg: u8,
    ty: StaticType,
}

pub struct IrLowerer {
    pub ir: Vec<Instruction>,
    locals: Vec<Local>,
    scope_depth: usize,
    free_reg: u8,
}

impl IrLowerer {
    pub fn new() -> Self {
        Self {
            ir: Vec::with_capacity(1024),
            locals: Vec::new(),
            scope_depth: 0,
            free_reg: 0,
        }
    }

    fn next_reg(&mut self) -> u8 {
        let r = self.free_reg;
        self.free_reg += 1;
        r
    }

    // Safely reclaims all temporary registers used during expression evaluation.
    fn reset_temps(&mut self) {
        self.free_reg = self.locals.last().map(|l| l.reg + 1).unwrap_or(0);
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        while let Some(local) = self.locals.last() {
            if local.depth > self.scope_depth {
                self.locals.pop();
            } else {
                break;
            }
        }
        self.reset_temps();
    }

    fn find_local(&self, name: &str) -> Local {
        self.locals.iter().rev().find(|l| l.name == name).cloned()
            .expect("Lowerer: Undeclared variable (should be caught by TypeChecker)")
    }

    pub fn lower_program(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.lower_stmt(stmt);
        }
    }

    fn lower_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::LocalDecl { name, expr } => {
                let target_reg = self.next_reg();
                let (_, ty) = self.lower_expr(expr, Some(target_reg));
                self.locals.push(Local {
                    name: name.clone(),
                    depth: self.scope_depth,
                    reg: target_reg,
                    ty
                });
                self.reset_temps();
            }
            Stmt::Assignment { name, expr } => {
                let local = self.find_local(name);
                self.lower_expr(expr, Some(local.reg));
                self.reset_temps();
            }
            Stmt::TableAssign { table, index, expr } => {
                let table_local = self.find_local(table);
                let (index_reg, _) = self.lower_expr(index, None);
                let (val_reg, _) = self.lower_expr(expr, None);

                self.ir.push(Instruction::SetTable {
                    table: table_local.reg,
                    key: index_reg,
                    val: val_reg
                });
                self.reset_temps();
            }
            Stmt::While { condition, body } => {
                self.ir.push(Instruction::BeginWhile);

                let (cond_reg, _) = self.lower_expr(condition, None);
                self.ir.push(Instruction::WhileCondition { cond_reg });

                // Free condition temps immediately before entering loop body
                self.reset_temps();

                self.begin_scope();
                for s in body {
                    self.lower_stmt(s);
                }
                self.end_scope();

                self.ir.push(Instruction::EndWhile);
            }
        }
    }

    // Evaluates an expression, placing the result in `target_reg` if provided.
    // Returns the register containing the final value, and its type.
    fn lower_expr(&mut self, expr: &Expr, target_reg: Option<u8>) -> (u8, StaticType) {
        match expr {
            Expr::Integer(val) => {
                let reg = target_reg.unwrap_or_else(|| self.next_reg());
                self.ir.push(Instruction::LoadInt { target: reg, val: *val });
                (reg, StaticType::Integer)
            }
            Expr::NewTable => {
                let reg = target_reg.unwrap_or_else(|| self.next_reg());
                self.ir.push(Instruction::NewTable { target: reg });
                (reg, StaticType::Table)
            }
            Expr::Identifier(name) => {
                let local = self.find_local(name);
                if let Some(target) = target_reg {
                    if target != local.reg {
                        self.ir.push(Instruction::Move {
                            target,
                            source: local.reg,
                            ty: local.ty.clone()
                        });
                        return (target, local.ty.clone());
                    }
                }
                (local.reg, local.ty.clone())
            }
            Expr::TableIndex { table, index } => {
                let (table_reg, _) = self.lower_expr(table, None);
                let (index_reg, _) = self.lower_expr(index, None);
                let reg = target_reg.unwrap_or_else(|| self.next_reg());

                self.ir.push(Instruction::GetTable {
                    target: reg,
                    table: table_reg,
                    key: index_reg
                });
                (reg, StaticType::Integer)
            }
            Expr::BinaryOp { op, left, right } => {
                let (left_reg, _) = self.lower_expr(left, None);
                let (right_reg, _) = self.lower_expr(right, None);
                let reg = target_reg.unwrap_or_else(|| self.next_reg());

                match op {
                    BinOp::Add => self.ir.push(Instruction::Add { target: reg, left: left_reg, right: right_reg }),
                    BinOp::Sub => self.ir.push(Instruction::Sub { target: reg, left: left_reg, right: right_reg }),
                    BinOp::LessThan => self.ir.push(Instruction::Less { target: reg, left: left_reg, right: right_reg }),
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
