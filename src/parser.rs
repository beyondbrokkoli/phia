// src/parser.rs
use std::iter::Peekable;
use crate::lexer::Token;
use crate::ast::{Expr, Stmt, BinOp, UnOp};

pub struct Parser<'a> {
    tokens: Peekable<std::vec::IntoIter<Token<'a>>>,
    table_counter: usize, // mints 1-based ids for `{}` literals; 0 is reserved for the null handle
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
            table_counter: 0,
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
            Some(Token::If) => {
                self.tokens.next(); // consume 'if'
                let condition = self.parse_expr();
                self.expect(Token::Then);

                let mut then_body = Vec::new();
                while self.peek_not_block_end() {
                    then_body.push(self.parse_stmt());
                }

                // elseif chains desugar to nested Ifs in the else arm
                let mut else_body = Vec::new();
                match self.tokens.peek().cloned() {
                    Some(Token::ElseIf) => {
                        else_body.push(self.parse_elseif_chain());
                    }
                    Some(Token::Else) => {
                        self.tokens.next(); // consume 'else'
                        while self.peek_not_block_end() {
                            else_body.push(self.parse_stmt());
                        }
                        self.expect(Token::End);
                    }
                    Some(Token::End) => {
                        self.tokens.next(); // consume 'end'
                    }
                    _ => panic!("Syntax Error: Expected 'else', 'elseif' or 'end' after if body"),
                }

                Stmt::If { condition, then_body, else_body }
            }
            Some(Token::Print) => {
                // Lua call syntax, statement position only. print("tag", e, ...)
                // is NOT a function call — nothing downstream of the parser
                // ever sees one; the parens are pure surface, consumed HERE
                // and never entering parse_expr (the expression paren-gate
                // is untouched). A bare `print "tag" e` spelling is refused:
                // it would not parse as Lua.
                self.tokens.next(); // consume 'print'
                self.expect(Token::LeftParen);
                // Parse the first argument through the expression parser so
                // concatenation chains and parenthesised expressions are handled
                // correctly.  A bare string literal becomes the probe tag;
                // anything else is added to the expression list with an empty
                // tag (avoids the confusing "Expected RightParen, got Concat"
                // panic).
                let first_expr = self.parse_expr();
                let mut tag = String::new();
                let mut exprs: Vec<Expr> = if let Expr::String(s) = first_expr {
                    tag = s;
                    Vec::new()
                } else {
                    vec![first_expr]
                };
                while matches!(self.tokens.peek(), Some(Token::Comma)) {
                    self.tokens.next(); // consume ','
                    exprs.push(self.parse_expr());
                }
                self.expect(Token::RightParen);
                Stmt::Probe { tag, exprs }
            }
            Some(Token::Identifier(_)) => {
                let lhs = self.parse_expr();
                self.expect(Token::Assign);
                let rhs = self.parse_expr();
                match lhs {
                    Expr::Identifier(name) => Stmt::Assignment { name, expr: rhs },
                    Expr::TableIndex { table, index } =>
                        Stmt::TableAssign { table: *table, index: *index, expr: rhs },
                    _ => panic!("Syntax Error: Invalid assignment target"),
                }
            }
            _ => panic!("Syntax Error: Unexpected statement starting with {:?}", self.tokens.peek()),
        }
    }

    // A body ends at 'end', 'else', or 'elseif' — none of which can start a statement
    fn peek_not_block_end(&mut self) -> bool {
        !matches!(
            self.tokens.peek(),
            Some(Token::End) | Some(Token::Else) | Some(Token::ElseIf) | None
        )
    }

    // Parse `elseif cond then body [elseif...|else...|end]` as a nested If
    fn parse_elseif_chain(&mut self) -> Stmt {
        self.expect(Token::ElseIf);
        let condition = self.parse_expr();
        self.expect(Token::Then);

        let mut then_body = Vec::new();
        while self.peek_not_block_end() {
            then_body.push(self.parse_stmt());
        }

        let mut else_body = Vec::new();
        match self.tokens.peek().cloned() {
            Some(Token::ElseIf) => {
                else_body.push(self.parse_elseif_chain());
            }
            Some(Token::Else) => {
                self.tokens.next(); // consume 'else'
                while self.peek_not_block_end() {
                    else_body.push(self.parse_stmt());
                }
                self.expect(Token::End);
            }
            Some(Token::End) => {
                self.tokens.next(); // consume 'end'
            }
            _ => panic!("Syntax Error: Expected 'else', 'elseif' or 'end' after elseif body"),
        }

        Stmt::If { condition, then_body, else_body }
    }

    // --- Expression Parsing (Recursive Descent with Precedence) ---

    pub fn parse_expr(&mut self) -> Expr {
        self.parse_comparison()
    }

    // Lowest precedence: comparisons (all non-chaining, like Lua)
    fn parse_comparison(&mut self) -> Expr {
        let left = self.parse_concat();

        let op = match self.tokens.peek() {
            Some(Token::LessThan) => BinOp::LessThan,
            Some(Token::LessEq) => BinOp::LessEq,
            Some(Token::GreaterThan) => BinOp::GreaterThan,
            Some(Token::GreaterEq) => BinOp::GreaterEq,
            Some(Token::Equal) => BinOp::Equal,
            Some(Token::NotEqual) => BinOp::NotEqual,
            _ => return left,
        };
        self.tokens.next(); // consume the operator
        let right = self.parse_concat();
        Expr::BinaryOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    // Next precedence: .. (concat). Lua places it between comparisons and
    // + -, and right-associates it; chains here fold LEFT. Concat is
    // associative — (a..b)..c and a..(b..c) are the same string — so the
    // associativity divergence is unobservable (pinned by string tests).
    fn parse_concat(&mut self) -> Expr {
        let mut left = self.parse_term();

        while let Some(Token::Concat) = self.tokens.peek() {
            self.tokens.next(); // consume '..'
            let right = self.parse_term();
            left = Expr::BinaryOp {
                op: BinOp::Concat,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    // Next precedence: +, -
    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();

        while let Some(Token::Plus) | Some(Token::Minus) = self.tokens.peek() {
            let op = match self.tokens.next().unwrap() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_factor();
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    // Next precedence: *, /, //, %
    fn parse_factor(&mut self) -> Expr {
        let mut left = self.parse_unary();

        while let Some(Token::Star) | Some(Token::Slash) | Some(Token::DoubleSlash)
        | Some(Token::Percent) = self.tokens.peek()
        {
            let op = match self.tokens.next().unwrap() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                Token::DoubleSlash => BinOp::IntDiv,
                Token::Percent => BinOp::Mod,
                _ => unreachable!(),
            };
            let right = self.parse_unary();
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    // Unary: -x, not x
    fn parse_unary(&mut self) -> Expr {
        match self.tokens.peek().cloned() {
            Some(Token::Minus) => {
                self.tokens.next();
                let expr = self.parse_unary();
                Expr::UnaryOp { op: UnOp::Neg, expr: Box::new(expr) }
            }
            Some(Token::Not) => {
                self.tokens.next();
                let expr = self.parse_unary();
                Expr::UnaryOp { op: UnOp::Not, expr: Box::new(expr) }
            }
            _ => self.parse_primary(),
        }
    }

    // Highest precedence: literals, identifiers, table creation, and table indexing
    fn parse_primary(&mut self) -> Expr {
        let mut expr = match self.tokens.next() {
            Some(Token::Integer(val)) => Expr::Integer(val),
            Some(Token::Float(val)) => Expr::Float(val),
            Some(Token::True) => Expr::Boolean(true),
            Some(Token::False) => Expr::Boolean(false),
            Some(Token::String(s)) => Expr::String(s.trim_matches('"').to_string()),
            Some(Token::Identifier(name)) => Expr::Identifier(name.to_string()),
            Some(Token::LeftBrace) => {
                self.expect(Token::RightBrace);
                self.table_counter += 1;
                Expr::NewTable(self.table_counter)
            }
            Some(Token::LeftParen) => {
                let inner = self.parse_expr();
                self.expect(Token::RightParen);
                inner
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
