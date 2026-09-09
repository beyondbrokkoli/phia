// src/type_checker.rs
use std::collections::HashMap;
use crate::ast::{Expr, Stmt, StaticType, BinOp, UnOp};

// Checker-internal type algebra. The lowerer never sees these: at the end
// of check_program every table literal's type is resolved to a
// `StaticType` (unbound variables default to Integer), so table_types
// holds only resolved container types.
//
// LAZY UNIFICATION: reads return the element type as a constraint-sharing
// TERM (a bare Var when unconstrained); stores and value uses accumulate
// unification constraints on those variables; the end of the program
// resolves everything. Aliases share variables by construction — the old
// write-once-resolution-by-identity rule IS variable identity. Element
// variables bind only Integer or Table (the codegenable kinds).
//
// Consequence map, pinned by sentinels: statement order is not part of
// validity (nested_06/07; nested_02's build-error -> nil-panic flip);
// `a[0] = {}` legal, resolves jointly (nested_08/15); aliases constrain
// the same variable (nested_04/10); Boolean rejected up front
// (nested_14); occurs check (nested_13); arithmetic USE, not the read,
// fixes the type (float_13).
#[derive(Debug, Clone, PartialEq)]
enum Ty {
    Integer,
    Float,
    Boolean,
    Table(Box<Ty>),
    Var(usize),
}

pub struct TypeChecker {
    // A stack of scopes. The last element is the current innermost scope.
    scopes: Vec<HashMap<String, Ty>>,
    // Unification variable -> bound term. Absent means unbound; unbound
    // variables default to Integer when the final map is built. The occurs
    // check in `bind` keeps this graph acyclic, so deref always terminates.
    subst: HashMap<usize, Ty>,
    // Fresh variable minting. Literal #id's element type IS Var(id), so the
    // parser's table-id space and the fresh-variable space share one
    // namespace; fresh mints stay above every parser id seen so far.
    next_var: usize,
    // Every table literal id minted by the parser, in first-seen order. The
    // final type map is built over exactly these.
    table_ids: Vec<usize>,
    // Parser table id -> fully resolved container type. Filled once, at the
    // end of check_program — the lowerer's only window into this module.
    table_types: HashMap<usize, StaticType>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            subst: HashMap::new(),
            next_var: 0,
            table_ids: Vec::new(),
            table_types: HashMap::new(),
        }
    }

    pub fn get_type_map(&self) -> &HashMap<usize, StaticType> {
        &self.table_types
    }

    fn begin_scope(&mut self) { self.scopes.push(HashMap::new()); }
    fn end_scope(&mut self) { self.scopes.pop().expect("Cannot pop global scope"); }

    fn declare_var(&mut self, name: String, ty: Ty) {
        let current_scope = self.scopes.last_mut().unwrap();
        if current_scope.contains_key(&name) {
            panic!("Variable '{}' already declared in this scope", name);
        }
        current_scope.insert(name, ty);
    }

    // Variable types are live constraint terms now, not a cosmetic cache:
    // every later read of the variable re-shares these variables.
    fn get_var_type(&self, name: &str) -> Ty {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return ty.clone();
            }
        }
        panic!("Undeclared variable: '{}'", name);
    }

    fn update_var_type(&mut self, name: &str, ty: Ty) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(t) = scope.get_mut(name) {
                *t = ty;
                return;
            }
        }
    }

    // --- unification -------------------------------------------------------

    fn fresh_var(&mut self) -> Ty {
        let v = self.next_var;
        self.next_var += 1;
        Ty::Var(v)
    }

    // Register literal #id: its element variable is Var(id), and fresh mints
    // must stay above the whole parser id space.
    fn mint_table_var(&mut self, id: usize) {
        if id >= self.next_var { self.next_var = id + 1; }
        if !self.table_ids.contains(&id) { self.table_ids.push(id); }
    }

    // Chase variable bindings to the term's current head. An unbound
    // variable derefs to itself.
    fn deref(&self, ty: &Ty) -> Ty {
        let mut cur = ty.clone();
        loop {
            match cur {
                Ty::Var(v) => match self.subst.get(&v) {
                    Some(t) => cur = t.clone(),
                    None => return Ty::Var(v),
                },
                other => return other,
            }
        }
    }

    // Bind variable v to term. v must currently be unbound (callers only
    // bind deref'd vars). Two rejections live here because they apply to
    // EVERY binding site: Boolean (not a codegenable element kind —
    // nested_14) and occurs (an infinite element type — nested_13).
    fn bind(&mut self, v: usize, term: &Ty) {
        if matches!(term, Ty::Boolean) {
            panic!("Type Error: table elements cannot be Boolean");
        }
        if self.occurs(v, term) {
            panic!("Type Error: recursive table type (a table cannot contain itself)");
        }
        self.subst.insert(v, term.clone());
    }

    fn occurs(&self, v: usize, term: &Ty) -> bool {
        match self.deref(term) {
            Ty::Var(w) => w == v,
            Ty::Table(inner) => self.occurs(v, &inner),
            _ => false,
        }
    }

    fn unify(&mut self, a: &Ty, b: &Ty) {
        let a = self.deref(a);
        let b = self.deref(b);
        if a == b { return; }
        match (&a, &b) {
            (Ty::Integer, Ty::Integer) | (Ty::Boolean, Ty::Boolean) => {}
            (Ty::Table(x), Ty::Table(y)) => {
                let (x, y) = ((**x).clone(), (**y).clone());
                self.unify(&x, &y);
            }
            (Ty::Var(v), _) => { let v = *v; self.bind(v, &b); }
            (_, Ty::Var(v)) => { let v = *v; self.bind(v, &a); }
            _ => panic!("Type Error: table element type conflict ({} vs {})",
                        self.display(&a), self.display(&b)),
        }
    }

    // User-facing rendering for error messages: resolved shapes only, an
    // unconstrained element prints as "?" (it genuinely is not known yet).
    // Never prints variable numbers — output stays a pure function of the
    // source program.
    fn display(&self, ty: &Ty) -> String {
        match self.deref(ty) {
            Ty::Integer => "Integer".to_string(),
            Ty::Float => "Float".to_string(),
            Ty::Boolean => "Boolean".to_string(),
            Ty::Var(_) => "?".to_string(),
            Ty::Table(inner) => format!("Table({})", self.display(&inner)),
        }
    }

    // Deep-resolve a Ty into the lowerer's StaticType. Unbound variables
    // default to Integer: an element nobody ever constrained can only ever
    // be read as 0, which is integer semantics.
    fn resolve_static(&self, ty: &Ty) -> StaticType {
        match self.deref(ty) {
            Ty::Integer => StaticType::Integer,
            Ty::Float => StaticType::Float,
            // bind() rejects Boolean bindings, so this is a checker bug,
            // not a program error — fail loudly instead of mis-lowering.
            Ty::Boolean => panic!("checker bug: Boolean leaked into a table type"),
            Ty::Var(_) => StaticType::Integer,
            Ty::Table(inner) => StaticType::Table(Box::new(self.resolve_static(&inner))),
        }
    }

    // --- program ------------------------------------------------------------

    pub fn check_program(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.check_stmt(stmt);
        }
        // End of program: materialize every table literal's container type
        // from the accumulated constraints. table_types holds only resolved
        // StaticTypes afterwards — the lowerer unwraps safely.
        for id in self.table_ids.clone() {
            let resolved = self.resolve_static(&Ty::Table(Box::new(Ty::Var(id))));
            self.table_types.insert(id, resolved);
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::LocalDecl { name, expr } => {
                let expr_type = self.check_expr(expr);
                self.declare_var(name.clone(), expr_type);
            }
            Stmt::Assignment { name, expr } => {
                let expected_type = self.deref(&self.get_var_type(name));
                let actual_type = self.check_expr(expr);
                let actual_type = self.deref(&actual_type);

                match (&expected_type, &actual_type) {
                    // Tables are the only reference type, and assignment
                    // REBINDS: the variable adopts the assigned value's
                    // type. Element types may differ across a rebind — the
                    // old type is dropped, never unified. When either side
                    // is still deferred (Var), adopting is the early rebind:
                    // from here on the variable tracks the very same
                    // constraint variables, so every later use constrains
                    // the runtime object it will actually hold.
                    (Ty::Table(_), Ty::Table(_))
                    | (Ty::Table(_), Ty::Var(_))
                    | (Ty::Var(_), _) => {
                        self.update_var_type(name, actual_type.clone());
                    }
                    // A concrete scalar variable is monomorphic: assigning
                    // a deferred value constrains that value's type to the
                    // variable's (an Integer variable really does hold an
                    // Integer, so the table it came from has Integer
                    // elements).
                    (_, Ty::Var(_)) => {
                        let (e, a) = (expected_type.clone(), actual_type.clone());
                        self.unify(&e, &a);
                    }
                    (Ty::Integer, Ty::Integer)
                    | (Ty::Float, Ty::Float)
                    | (Ty::Boolean, Ty::Boolean) => {}
                    _ => panic!(
                        "Type Error (Rule #2 Violation): Cannot assign {} to variable '{}' of type {}",
                        self.display(&actual_type), name, self.display(&expected_type)
                    ),
                }
            }
            Stmt::TableAssign { table, index, expr } => {
                let table_type = self.check_expr(table);
                let table_type = self.deref(&table_type);
                let index_type = self.check_expr(index);
                let index_type = self.deref(&index_type);
                match index_type {
                    Ty::Integer => {}
                    Ty::Var(v) => self.bind(v, &Ty::Integer),
                    _ => panic!("Type Error: Table index must be an Integer"),
                }
                let expr_type = self.check_expr(expr);
                let expr_type = self.deref(&expr_type);

                match table_type {
                    // Store into a known table: the element type must unify
                    // with the stored value's type. Nothing eager — if the
                    // element is still a free variable this simply binds it.
                    Ty::Table(inner) => { self.unify(&inner, &expr_type); }
                    // Store through a value of unknown table-ness (e.g.
                    // `a[0][i] = 1` before a's element type is decided):
                    // the target IS a table from now on, with a fresh
                    // element variable that this store gets to constrain.
                    Ty::Var(v) => {
                        let elem = self.fresh_var();
                        self.bind(v, &Ty::Table(Box::new(elem.clone())));
                        self.unify(&elem, &expr_type);
                    }
                    _ => panic!("Type Error: target is not a table"),
                }
            }
            Stmt::Probe { exprs, .. } => {
                // Observation only: operands must be well-typed expressions
                // (reads constrain element vars exactly like any other use),
                // but the probe itself types nothing and returns nothing.
                for e in exprs { self.check_expr(e); }
            }
            Stmt::While { condition, body } => {
                self.check_cond(condition, "while");
                self.begin_scope();
                for s in body { self.check_stmt(s); }
                self.end_scope();
            }
            Stmt::If { condition, then_body, else_body } => {
                self.check_cond(condition, "if");
                self.begin_scope();
                for s in then_body { self.check_stmt(s); }
                self.end_scope();
                self.begin_scope();
                for s in else_body { self.check_stmt(s); }
                self.end_scope();
            }
        }
    }

    fn check_cond(&mut self, condition: &Expr, kw: &str) {
        let cond_type = self.check_expr(condition);
        let cond_type = self.deref(&cond_type);
        match cond_type {
            Ty::Boolean => {}
            Ty::Var(v) => {
                // Only a table-element read can still be deferred
                // here, and Boolean is not a legal element type.
                self.bind(v, &Ty::Boolean); // always rejects
            }
            _ => panic!("Type Error: '{}' condition must be a Boolean", kw),
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> Ty {
        match expr {
            Expr::Integer(_) => Ty::Integer,
            Expr::Float(_) => Ty::Float,
            Expr::Boolean(_) => Ty::Boolean,
            Expr::NewTable(id) => {
                self.mint_table_var(*id);
                Ty::Table(Box::new(Ty::Var(*id)))
            }
            Expr::Identifier(name) => self.get_var_type(name),
            Expr::TableIndex { table, index } => {
                let table_type = self.check_expr(table);
                let table_type = self.deref(&table_type);
                let index_type = self.check_expr(index);
                let index_type = self.deref(&index_type);
                match index_type {
                    Ty::Integer => {}
                    Ty::Var(v) => self.bind(v, &Ty::Integer),
                    _ => panic!("Type Error: Table index must be an Integer"),
                }
                match table_type {
                    // The read's type IS the element type term — the very
                    // same variables the table's stores constrain. Reading
                    // locks nothing; using the result does.
                    Ty::Table(inner) => *inner,
                    // Reading through a value of unknown table-ness (a
                    // value that came out of another table): it is a table
                    // from now on, with a fresh element variable.
                    Ty::Var(v) => {
                        let elem = self.fresh_var();
                        self.bind(v, &Ty::Table(Box::new(elem.clone())));
                        elem
                    }
                    _ => panic!("Type Error: Attempted to index a non-table"),
                }
            }
            Expr::BinaryOp { op, left, right } => {
                let left_type = self.check_expr(left);
                let left_type = self.deref(&left_type);
                let right_type = self.check_expr(right);
                let right_type = self.deref(&right_type);
                // Arithmetic is strictly same-type numeric, no coercion:
                // a mixed pair is a build error, a deliberate pinned
                // divergence from Lua (float_03). A deferred operand takes
                // its type from the concrete sibling — THIS, not the read,
                // is what fixes a table's element type (float_13).
                let operand_ty = match (&left_type, &right_type) {
                    (Ty::Integer, Ty::Integer) => Ty::Integer,
                    (Ty::Float, Ty::Float) => Ty::Float,
                    (Ty::Integer, Ty::Float) | (Ty::Float, Ty::Integer) =>
                        panic!("Type Error: Binary operations do not support mixed Integer and Float"),
                    (Ty::Var(_), Ty::Integer | Ty::Float) => {
                        let t = right_type.clone();
                        if let Ty::Var(v) = left_type { self.bind(v, &t); }
                        t
                    }
                    (Ty::Integer | Ty::Float, Ty::Var(_)) => {
                        let t = left_type.clone();
                        if let Ty::Var(v) = right_type { self.bind(v, &t); }
                        t
                    }
                    (Ty::Var(_), Ty::Var(_)) => {
                        let (l, r) = (left_type.clone(), right_type.clone());
                        self.unify(&l, &r);
                        l
                    }
                    // == / ~= on two Booleans: the only non-numeric binary op
                    (Ty::Boolean, Ty::Boolean) if matches!(op, BinOp::Equal | BinOp::NotEqual) =>
                        Ty::Boolean,
                    _ => panic!("Type Error: Binary operations currently only support Integers"),
                };
                match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::IntDiv
                    | BinOp::Mod => operand_ty,
                    BinOp::LessThan | BinOp::GreaterThan | BinOp::LessEq | BinOp::GreaterEq
                        if matches!(operand_ty, Ty::Integer | Ty::Float) => Ty::Boolean,
                    // == and ~= additionally accept two Booleans
                    BinOp::Equal | BinOp::NotEqual
                        if matches!(operand_ty, Ty::Integer | Ty::Float | Ty::Boolean) =>
                        Ty::Boolean,
                    // comparisons on non-numeric operands (e.g. tables)
                    _ => panic!("Type Error: comparisons require numeric operands"),
                }
            }
            Expr::UnaryOp { op, expr } => {
                let operand = self.check_expr(expr);
                let operand = self.deref(&operand);
                match op {
                    UnOp::Neg => match operand {
                        Ty::Integer | Ty::Float => operand,
                        Ty::Var(v) => {
                            // negation is a numeric USE: it fixes a deferred
                            // operand to Integer (the arithmetic-USE rule)
                            self.bind(v, &Ty::Integer);
                            Ty::Integer
                        }
                        _ => panic!("Type Error: unary '-' requires a numeric operand"),
                    },
                    UnOp::Not => match operand {
                        Ty::Boolean => Ty::Boolean,
                        Ty::Var(v) => {
                            self.bind(v, &Ty::Boolean); // always rejects
                            Ty::Boolean
                        }
                        _ => panic!("Type Error: 'not' requires a Boolean operand"),
                    },
                }
            }
        }
    }
}
