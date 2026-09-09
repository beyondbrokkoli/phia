// src/ast.rs

#[derive(Debug, Clone, PartialEq)]
pub enum StaticType {
    Integer,
    Float,
    Boolean,
    UnknownTable(usize), // Unique ID for first-store inference
    Table(Box<StaticType>),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    Mod,
    LessThan,
    GreaterThan,
    LessEq,
    GreaterEq,
    Equal,
    NotEqual,
}

#[derive(Debug, Clone)]
pub enum UnOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Identifier(String),
    NewTable(usize),
    TableIndex {
        table: Box<Expr>,
        index: Box<Expr>,
    },
    BinaryOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnOp,
        expr: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    LocalDecl {
        name: String,
        expr: Expr,
    },
    Assignment {
        name: String,
        expr: Expr,
    },
    TableAssign {
        table: Expr,      // Lvalue generalized
        index: Expr,
        expr: Expr,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_body: Vec<Stmt>,
        // elseif chains desugar to nested Ifs in the else arm (parser)
        else_body: Vec<Stmt>,
    },
    // Runtime observation point: print the operands' values under the tag.
    // Effect-only, type-transparent — the checker validates operands but
    // a probe constrains nothing and returns nothing.
    Probe {
        tag: String,
        exprs: Vec<Expr>,
    },
}
