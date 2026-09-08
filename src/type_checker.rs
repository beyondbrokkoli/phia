// src/type_checker.rs
use std::collections::HashMap;
use crate::ast::{Expr, Stmt, StaticType, BinOp};

pub struct TypeChecker {
    // A stack of scopes. The last element is the current innermost scope.
    scopes: Vec<HashMap<String, StaticType>>,
    // Table identity -> resolved element type. UnknownTable placeholder until
    // the first store or read locks it; resolution is WRITE-ONCE per table.
    table_types: HashMap<usize, StaticType>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            table_types: HashMap::new(),
        }
    }

    pub fn get_type_map(&self) -> &HashMap<usize, StaticType> {
        &self.table_types
    }

    fn begin_scope(&mut self) { self.scopes.push(HashMap::new()); }
    fn end_scope(&mut self) { self.scopes.pop().expect("Cannot pop global scope"); }

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

    // Cosmetic cache refresh only — the truth lives in table_types.
    fn update_var_type(&mut self, name: &str, ty: StaticType) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(t) = scope.get_mut(name) {
                *t = ty;
                return;
            }
        }
    }

    // Map an unresolved table reference through its identity. A placeholder
    // entry resolves to itself (still unresolved); anything else is a copy.
    fn resolve(&self, ty: &StaticType) -> StaticType {
        match ty {
            StaticType::UnknownTable(id) =>
                self.table_types.get(id).cloned().unwrap_or_else(|| ty.clone()),
            other => other.clone(),
        }
    }

    // Write-once resolution: an unresolved (or never-seen) table may lock to
    // anything exactly once; re-locking to a DIFFERENT type is the aliasing
    // divergence class and is a hard error.
    fn resolve_table(&mut self, id: usize, ty: StaticType) {
        match self.table_types.get(&id) {
            None | Some(StaticType::UnknownTable(_)) => { self.table_types.insert(id, ty); }
            Some(existing) if *existing != ty =>
                panic!("Type Error: table element type conflict ({:?} vs {:?})", existing, ty),
            Some(_) => {}
        }
    }

    pub fn check_program(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.check_stmt(stmt);
        }
        // Tables that were never stored into or read: only ever eligible for
        // integer semantics, so default them. After this pass table_types
        // holds only resolved types — the lowerer can unwrap safely.
        for ty in self.table_types.values_mut() {
            if matches!(ty, StaticType::UnknownTable(_)) {
                *ty = StaticType::Table(Box::new(StaticType::Integer));
            }
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::LocalDecl { name, expr } => {
                let expr_type = self.check_expr(expr);
                self.declare_var(name.clone(), expr_type);
            }
            Stmt::Assignment { name, expr } => {
                let expected_type = self.resolve(&self.get_var_type(name));
                let actual_type = self.check_expr(expr);
                let actual_type = self.resolve(&actual_type);

                let is_table = |t: &StaticType|
                    matches!(t, StaticType::Table(_) | StaticType::UnknownTable(_));
                if is_table(&expected_type) && is_table(&actual_type) {
                    // Tables are the only reference type, and assignment
                    // REBINDS: the variable now tracks the assigned table
                    // (by id, unresolved included — it resolves later through
                    // table_types). An abandoned unresolved table normalizes
                    // to Integer at end of program. Element types may differ
                    // across a rebind: reads through the variable resolve
                    // through the NEW id, so they cannot diverge from the
                    // runtime object. Kind mixing (table <-> scalar) below
                    // is the only assignment that can be ill-typed.
                    self.update_var_type(name, actual_type);
                } else if expected_type != actual_type {
                    panic!(
                        "Type Error (Rule #2 Violation): Cannot assign {:?} to variable '{}' of type {:?}",
                        actual_type, name, expected_type
                    );
                }
            }
            Stmt::TableAssign { table, index, expr } => {
                let table_type = self.check_expr(table);
                let table_type = self.resolve(&table_type);
                let index_type = self.check_expr(index);
                if index_type != StaticType::Integer {
                    panic!("Type Error: Table index must be an Integer");
                }
                let expr_type = self.check_expr(expr);
                let expr_type = self.resolve(&expr_type);

                match table_type {
                    StaticType::UnknownTable(id) => {
                        // First store wins — and the stored value must itself
                        // be resolved (the `a[0] = {}` restriction).
                        let inner = match expr_type {
                            StaticType::UnknownTable(_) =>
                                panic!("Type Error: Cannot store unresolved table. Annotate by storing first."),
                            resolved => resolved,
                        };
                        self.resolve_table(id, StaticType::Table(Box::new(inner.clone())));
                        if let Expr::Identifier(name) = table {
                            self.update_var_type(name, StaticType::Table(Box::new(inner)));
                        }
                    }
                    StaticType::Table(inner) => {
                        match expr_type {
                            StaticType::UnknownTable(_) =>
                                panic!("Type Error: Cannot store unresolved table into table of {:?}. Annotate by storing first.", inner),
                            ref t if *t != *inner =>
                                panic!("Type Error: table element type mismatch"),
                            _ => {}
                        }
                    }
                    _ => panic!("Type Error: target is not a table"),
                }
            }
            Stmt::While { condition, body } => {
                let cond_type = self.check_expr(condition);
                if cond_type != StaticType::Boolean {
                    panic!("Type Error: 'while' condition must be a Boolean");
                }
                self.begin_scope();
                for s in body { self.check_stmt(s); }
                self.end_scope();
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> StaticType {
        match expr {
            Expr::Integer(_) => StaticType::Integer,
            Expr::NewTable(id) => {
                self.table_types.entry(*id)
                    .or_insert(StaticType::UnknownTable(*id));
                StaticType::UnknownTable(*id)
            }
            Expr::Identifier(name) => self.get_var_type(name),
            Expr::TableIndex { table, index } => {
                let table_type = self.check_expr(table);
                let table_type = self.resolve(&table_type);
                if self.check_expr(index) != StaticType::Integer {
                    panic!("Type Error: Table index must be an Integer");
                }
                match table_type {
                    StaticType::UnknownTable(id) => {
                        // Read before write locks the element type to Integer.
                        let locked = StaticType::Table(Box::new(StaticType::Integer));
                        self.resolve_table(id, locked.clone());
                        if let Expr::Identifier(name) = &**table {
                            self.update_var_type(name, locked);
                        }
                        StaticType::Integer
                    }
                    StaticType::Table(inner) => *inner,
                    _ => panic!("Type Error: Attempted to index a non-table"),
                }
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
