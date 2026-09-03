// src/type_checker.rs
use std::collections::HashMap;
use crate::ast::{Expr, Stmt, StaticType, BinOp};

pub struct TypeChecker {
    // A stack of scopes. The last element is the current innermost scope.
    scopes: Vec<HashMap<String, StaticType>>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // Start with global/main scope
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop().expect("Cannot pop global scope");
    }

    fn declare_var(&mut self, name: String, ty: StaticType) {
        let current_scope = self.scopes.last_mut().unwrap();
        if current_scope.contains_key(&name) {
            panic!("Variable '{}' already declared in this scope", name);
        }
        current_scope.insert(name, ty);
    }

    fn get_var_type(&self, name: &str) -> StaticType {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return ty.clone();
            }
        }
        panic!("Undeclared variable: '{}'", name);
    }

    pub fn check_program(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.check_stmt(stmt);
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::LocalDecl { name, expr } => {
                let expr_type = self.check_expr(expr);
                self.declare_var(name.clone(), expr_type);
            }
            Stmt::Assignment { name, expr } => {
                let expected_type = self.get_var_type(name);
                let actual_type = self.check_expr(expr);
                if expected_type != actual_type {
                    panic!(
                        "Type Error (Rule #2 Violation): Cannot assign {:?} to variable '{}' of type {:?}",
                        actual_type, name, expected_type
                    );
                }
            }
            Stmt::TableAssign { table, index, expr } => {
                let table_type = self.get_var_type(table);
                if table_type != StaticType::Table {
                    panic!("Type Error: '{}' is not a table", table);
                }

                let index_type = self.check_expr(index);
                if index_type != StaticType::Integer {
                    panic!("Type Error: Table index must be an Integer");
                }

                let expr_type = self.check_expr(expr);
                // Currently, Phia tables only store Integers
                if expr_type != StaticType::Integer {
                    panic!("Type Error: Tables currently only support Integer values");
                }
            }
            Stmt::While { condition, body } => {
                let cond_type = self.check_expr(condition);
                if cond_type != StaticType::Boolean {
                    panic!("Type Error: 'while' condition must be a Boolean");
                }

                self.begin_scope();
                for s in body {
                    self.check_stmt(s);
                }
                self.end_scope();
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> StaticType {
        match expr {
            Expr::Integer(_) => StaticType::Integer,
            Expr::NewTable => StaticType::Table,
            Expr::Identifier(name) => self.get_var_type(name),
            Expr::TableIndex { table, index } => {
                let table_type = self.check_expr(table);
                if table_type != StaticType::Table {
                    panic!("Type Error: Attempted to index a non-table");
                }

                let index_type = self.check_expr(index);
                if index_type != StaticType::Integer {
                    panic!("Type Error: Table index must be an Integer");
                }

                // Currently, indexing a table always yields an Integer
                StaticType::Integer
            }
            Expr::BinaryOp { op, left, right } => {
                let left_type = self.check_expr(left);
                let right_type = self.check_expr(right);

                if left_type != StaticType::Integer || right_type != StaticType::Integer {
                    panic!("Type Error: Binary operations currently only support Integers");
                }

                match op {
                    BinOp::Add | BinOp::Sub => StaticType::Integer,
                    BinOp::LessThan => StaticType::Boolean,
                }
            }
        }
    }
}
