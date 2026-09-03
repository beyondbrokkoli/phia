// src/parser.rs
use std::iter::Peekable;
use crate::lexer::Token;
use crate::ast::{Expr, Stmt, BinOp};

pub struct Parser<'a> {
    tokens: Peekable<std::vec::IntoIter<Token<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    // Helper to consume a specific token or panic
    fn expect(&mut self, expected: Token<'a>) {
        let next = self.tokens.next();
        if next != Some(expected.clone()) {
            panic!("Syntax Error: Expected {:?}, got {:?}", expected, next);
        }
    }

    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.tokens.peek().is_some() {
            stmts.push(self.parse_stmt());
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.tokens.peek().cloned() {
            Some(Token::Local) => {
                self.tokens.next(); // consume 'local'
                let name = match self.tokens.next() {
                    Some(Token::Identifier(n)) => n.to_string(),
                    _ => panic!("Syntax Error: Expected variable name after 'local'"),
                };
                self.expect(Token::Assign);
                let expr = self.parse_expr();
                Stmt::LocalDecl { name, expr }
            }
            Some(Token::While) => {
                self.tokens.next(); // consume 'while'
                let condition = self.parse_expr();
                self.expect(Token::Do);

                let mut body = Vec::new();
                while self.tokens.peek() != Some(&Token::End) {
                    body.push(self.parse_stmt());
                }
                self.expect(Token::End);

                Stmt::While { condition, body }
            }
            Some(Token::Identifier(name)) => {
                self.tokens.next(); // consume identifier
                let name = name.to_string();

                match self.tokens.peek() {
                    Some(Token::Assign) => {
                        self.tokens.next(); // consume '='
                        let expr = self.parse_expr();
                        Stmt::Assignment { name, expr }
                    }
                    Some(Token::LeftBracket) => {
                        self.tokens.next(); // consume '['
                        let index = self.parse_expr();
                        self.expect(Token::RightBracket);
                        self.expect(Token::Assign);
                        let expr = self.parse_expr();
                        Stmt::TableAssign { table: name, index, expr }
                    }
                    _ => panic!("Syntax Error: Expected '=' or '[' after identifier"),
                }
            }
            _ => panic!("Syntax Error: Unexpected statement starting with {:?}", self.tokens.peek()),
        }
    }

    // --- Expression Parsing (Recursive Descent with Precedence) ---

    pub fn parse_expr(&mut self) -> Expr {
        self.parse_comparison()
    }

    // Lowest precedence: <
    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_term();

        while let Some(Token::LessThan) = self.tokens.peek() {
            self.tokens.next(); // consume '<'
            let right = self.parse_term();
            left = Expr::BinaryOp {
                op: BinOp::LessThan,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    // Next precedence: +, -
    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_primary();

        while let Some(Token::Plus) | Some(Token::Minus) = self.tokens.peek() {
            let op = match self.tokens.next().unwrap() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_primary();
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    // Highest precedence: literals, identifiers, table creation, and table indexing
    fn parse_primary(&mut self) -> Expr {
        let mut expr = match self.tokens.next() {
            Some(Token::Integer(val)) => Expr::Integer(val),
            Some(Token::Identifier(name)) => Expr::Identifier(name.to_string()),
            Some(Token::LeftBrace) => {
                self.expect(Token::RightBrace);
                Expr::NewTable
            }
            _ => panic!("Syntax Error: Expected expression"),
        };

        // Handle postfix table indexing (e.g., `data[i]`)
        while let Some(Token::LeftBracket) = self.tokens.peek() {
            self.tokens.next(); // consume '['
            let index = self.parse_expr();
            self.expect(Token::RightBracket);

            expr = Expr::TableIndex {
                table: Box::new(expr),
                index: Box::new(index),
            };
        }

        expr
    }
}
