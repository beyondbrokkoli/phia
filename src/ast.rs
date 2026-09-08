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
    LessThan,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
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
}
