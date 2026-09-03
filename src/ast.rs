// src/ast.rs

#[derive(Debug, Clone, PartialEq)]
pub enum StaticType {
    Integer,
    Boolean,
    Table,
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
    Identifier(String),
    NewTable,
    TableIndex {
        table: Box<Expr>, // E.g., `data` in `data[i]`
        index: Box<Expr>, // E.g., `i` in `data[i]`
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
        table: String,
        index: Expr,
        expr: Expr,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
}
