// src/compiler.rs
use std::iter::Peekable;
use crate::lexer::Token;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StaticType {
    Integer,
    Table,
    Boolean,
}

pub trait Backend {
    fn emit_load_int(&mut self, target: u8, val: i64);
    fn emit_new_table(&mut self, target: u8);
    fn emit_set_table(&mut self, table: u8, key: u8, val: u8);
    fn emit_get_table(&mut self, target: u8, table: u8, key: u8);
    fn emit_move(&mut self, target: u8, source: u8, ty: StaticType);
    fn emit_add(&mut self, target: u8, left: u8, right: u8);
    fn emit_sub(&mut self, target: u8, left: u8, right: u8);
    fn emit_less(&mut self, target: u8, left: u8, right: u8);
    fn begin_while(&mut self);
    fn while_condition(&mut self, cond_reg: u8);
    fn end_while(&mut self);
}

pub struct Local<'a> {
    pub name: &'a str,
    pub depth: usize,
    pub reg: u8,
}

pub struct Compiler<'a, B: Backend> {
    tokens: Peekable<std::vec::IntoIter<Token<'a>>>,
    pub backend: B,
    pub locals: Vec<Local<'a>>,
    pub scope_depth: usize,
    pub free_reg: u16,
    pub reg_types: [StaticType; 256],
}

impl<'a, B: Backend> Compiler<'a, B> {
    pub fn new(tokens: Vec<Token<'a>>, backend: B) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
            backend,
            locals: Vec::new(),
            scope_depth: 0,
            free_reg: 0,
            reg_types: [StaticType::Integer; 256],
        }
    }

    fn next_reg(&mut self) -> u8 {
        let r = self.free_reg;
        self.free_reg += 1;
        r as u8
    }

    // NEW: Automatically resets free_reg to just above the highest active local variable.
    // This safely reclaims all temporary registers used during expression evaluation.
    fn reset_temps(&mut self) {
        self.free_reg = self.locals.last().map(|l| l.reg as u16 + 1).unwrap_or(0);
    }

    fn begin_scope(&mut self) { self.scope_depth += 1; }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        while let Some(local) = self.locals.last() {
            if local.depth > self.scope_depth {
                self.locals.pop();
            } else { break; }
        }
        self.reset_temps(); // Free registers belonging to out-of-scope locals
    }

    pub fn compile_stmt(&mut self) {
        match self.tokens.peek() {
            Some(Token::Local) => {
                self.tokens.next();
                let name = match self.tokens.next() {
                    Some(Token::Identifier(n)) => n,
                    _ => panic!("Expected variable name"),
                };
                self.tokens.next(); // Consume '='
                let target_reg = self.next_reg();
                self.compile_expr(target_reg);
                self.locals.push(Local { name, depth: self.scope_depth, reg: target_reg });
                self.reset_temps();
            }
            Some(Token::Identifier(_)) => {
                let name = match self.tokens.next() {
                    Some(Token::Identifier(n)) => n,
                    _ => unreachable!(),
                };
                let var_idx = self.locals.iter().rposition(|l| l.name == name).unwrap();
                let var_reg = self.locals[var_idx].reg;

                match self.tokens.peek() {
                    Some(Token::Assign) => {
                        self.tokens.next(); // '='
                        self.compile_expr(var_reg);
                    }
                    Some(Token::LeftBracket) => {
                        self.tokens.next(); // '['
                        let key_reg = self.next_reg();
                        self.compile_expr(key_reg);
                        self.tokens.next(); // ']'
                        self.tokens.next(); // '='
                        let val_reg = self.next_reg();
                        self.compile_expr(val_reg);
                        self.backend.emit_set_table(var_reg, key_reg, val_reg);
                    }
                    _ => panic!("Expected '=' or '['"),
                }
                self.reset_temps();
            }
            Some(Token::While) => {
                self.tokens.next();
                self.backend.begin_while();
                let cond_reg = self.next_reg();
                self.compile_expr(cond_reg);
                self.tokens.next(); // 'do'
                self.backend.while_condition(cond_reg);

                self.reset_temps(); // Immediately free cond_reg and condition temps

                self.begin_scope();
                while let Some(token) = self.tokens.peek() {
                    if token == &Token::End {
                        self.tokens.next();
                        self.end_scope();
                        self.backend.end_while();
                        break;
                    }
                    self.compile_stmt();
                }
            }
            _ => panic!("Unexpected statement"),
        }
    }

    pub fn compile_expr(&mut self, target_reg: u8) {
        // We pass Some(target_reg) so if it's a simple literal, it writes directly to target.
        // If it's an identifier, it ignores target_reg and returns its canonical register.
        let mut lhs_reg = self.compile_simple_expr(Some(target_reg));
        let mut has_operator = false;

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Plus | Token::Minus | Token::LessThan => {
                    has_operator = true;
                    let op = if token == &Token::Plus { 0 } else if token == &Token::Minus { 1 } else { 2 };
                    self.tokens.next();

                    let rhs_reg = self.compile_simple_expr(None);

                    if op == 0 { self.backend.emit_add(target_reg, lhs_reg, rhs_reg); }
                    else if op == 1 { self.backend.emit_sub(target_reg, lhs_reg, rhs_reg); }
                    else {
                        self.backend.emit_less(target_reg, lhs_reg, rhs_reg);
                        self.reg_types[target_reg as usize] = StaticType::Boolean;
                    }
                    lhs_reg = target_reg;
                }
                _ => break,
            }
        }

        // Only emit a move if there were no operations AND the simple_expr didn't already
        // write directly into our target_reg.
        if !has_operator && lhs_reg != target_reg {
            self.backend.emit_move(target_reg, lhs_reg, self.reg_types[lhs_reg as usize]);
            self.reg_types[target_reg as usize] = self.reg_types[lhs_reg as usize];
        }
    }

    pub fn compile_simple_expr(&mut self, target_opt: Option<u8>) -> u8 {
        let mut out_reg;
        match self.tokens.next() {
            Some(Token::Identifier(name)) => {
                let var_idx = self.locals.iter().rposition(|l| l.name == name).unwrap();
                out_reg = self.locals[var_idx].reg;
            }
            Some(Token::Integer(i)) => {
                out_reg = target_opt.unwrap_or_else(|| self.next_reg());
                self.backend.emit_load_int(out_reg, i);
                self.reg_types[out_reg as usize] = StaticType::Integer;
            }
            Some(Token::LeftBrace) => {
                self.tokens.next(); // '}'
                out_reg = target_opt.unwrap_or_else(|| self.next_reg());
                self.backend.emit_new_table(out_reg);
                self.reg_types[out_reg as usize] = StaticType::Table;
            }
            _ => panic!("Expected identifier, integer, or '{{'"),
        }

        if let Some(Token::LeftBracket) = self.tokens.peek() {
            self.tokens.next(); // '['
            let key_reg = self.next_reg();
            self.compile_expr(key_reg);
            self.tokens.next(); // ']'

            let final_out = target_opt.unwrap_or_else(|| self.next_reg());
            self.backend.emit_get_table(final_out, out_reg, key_reg);
            self.reg_types[final_out as usize] = StaticType::Integer;
            out_reg = final_out;
        }

        out_reg
    }

    pub fn is_done(&mut self) -> bool {
        self.tokens.peek().is_none()
    }
}
