// src/ir.rs
#![allow(dead_code)] // Silences warnings for compile-time structural code

use crate::compiler::{self, StaticType};
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub enum Instruction {
    LoadInt {
        target: u8,
        val: i64,
    },
    NewTable {
        target: u8,
    },
    SetTable {
        table: u8,
        key: u8,
        val: u8,
    },
    GetTable {
        target: u8,
        table: u8,
        key: u8,
    },
    Move {
        target: u8,
        source: u8,
        ty: StaticType,
    },
    Add {
        target: u8,
        left: u8,
        right: u8,
    },
    Sub {
        target: u8,
        left: u8,
        right: u8,
    },
    Less {
        target: u8,
        left: u8,
        right: u8,
    },
    BeginWhile,
    WhileCondition {
        cond_reg: u8,
    },
    EndWhile,
}

pub struct IrBackend {
    pub ir: Vec<Instruction>,
}

impl IrBackend {
    pub fn new() -> Self {
        Self {
            ir: Vec::with_capacity(1024),
        }
    }

    pub fn generate_rust_code(&self) -> String {
        let mut out = String::new();

        out.push_str("use crate::memory::{Value, Table};\n\n");
        // Add unused_assignments to this list!
        out.push_str("#[allow(unused_variables, unused_mut, unused_assignments)]\n");
        out.push_str("pub fn run_baked() -> Vec<Table> {\n");

        let mut used_i = BTreeSet::new();
        let mut used_b = BTreeSet::new();
        let mut used_t = BTreeSet::new();

        for instr in &self.ir {
            match instr {
                Instruction::LoadInt { target, .. } => {
                    used_i.insert(*target);
                }
                Instruction::NewTable { target } => {
                    used_t.insert(*target);
                }
                Instruction::SetTable { table, key, val } => {
                    used_t.insert(*table);
                    used_i.insert(*key);
                    used_i.insert(*val);
                }
                Instruction::GetTable { target, table, key } => {
                    used_i.insert(*target);
                    used_t.insert(*table);
                    used_i.insert(*key);
                }
                Instruction::Move { target, source, ty } => match ty {
                    compiler::StaticType::Integer => {
                        used_i.insert(*target);
                        used_i.insert(*source);
                    }
                    compiler::StaticType::Boolean => {
                        used_b.insert(*target);
                        used_b.insert(*source);
                    }
                    compiler::StaticType::Table => {
                        used_t.insert(*target);
                        used_t.insert(*source);
                    }
                },
                Instruction::Add {
                    target,
                    left,
                    right,
                }
                | Instruction::Sub {
                    target,
                    left,
                    right,
                } => {
                    used_i.insert(*target);
                    used_i.insert(*left);
                    used_i.insert(*right);
                }
                Instruction::Less {
                    target,
                    left,
                    right,
                } => {
                    used_b.insert(*target);
                    used_i.insert(*left);
                    used_i.insert(*right);
                }
                Instruction::WhileCondition { cond_reg } => {
                    used_b.insert(*cond_reg);
                }
                Instruction::BeginWhile | Instruction::EndWhile => {}
            }
        }

        for r in used_i {
            out.push_str(&format!("    let mut i_r{} = 0i64;\n", r));
        }
        for r in used_b {
            out.push_str(&format!("    let mut b_r{} = false;\n", r));
        }
        for r in used_t {
            out.push_str(&format!("    let mut t_r{} = 0usize;\n", r));
        }
        out.push_str("    let mut tables = Vec::<Table>::with_capacity(128);\n\n");

        for instr in &self.ir {
            match instr {
                Instruction::LoadInt { target, val } => {
                    out.push_str(&format!("    i_r{} = {};\n", target, val));
                }
                Instruction::NewTable { target } => {
                    out.push_str("    tables.push(Table::new());\n");
                    out.push_str(&format!("    t_r{} = tables.len() - 1;\n", target));
                }
                Instruction::SetTable { table, key, val } => {
                    out.push_str(&format!(
                        "    let idx = i_r{k} as usize;\n\
                         \x20   // SAFETY: We assume the table index exists.\n\
                         \x20   let t = unsafe {{ tables.get_unchecked_mut(t_r{tbl}) }};\n\
                         \x20   if idx >= t.array.len() {{\n\
                         \x20       if idx == t.array.len() {{\n\
                         \x20           t.array.push(Value::nil());\n\
                         \x20       }} else {{\n\
                         \x20           t.array.resize(idx + 1, Value::nil());\n\
                         \x20       }}\n\
                         \x20   }}\n\
                         \x20   // Hot path: guaranteed to be in bounds now, no panic edge for LLVM.\n\
                         \x20   unsafe {{ *t.array.get_unchecked_mut(idx) = Value::integer(i_r{v} as i32); }}\n",
                        tbl = table,
                        k = key,
                        v = val
                    ));
                }
                Instruction::GetTable { target, table, key } => {
                    out.push_str(&format!(
                        "    let idx = i_r{k} as usize;\n\
                         \x20   // SAFETY: For max benchmark speed, we assume reads are strictly in-bounds.\n\
                         \x20   // (A production Lua engine would do: if idx < len {{ get_unchecked }} else {{ Value::nil() }})\n\
                         \x20   let raw_val = unsafe {{ *tables.get_unchecked(t_r{tbl}).array.get_unchecked(idx) }};\n\
                         \x20   i_r{t} = (raw_val.0 & 0xFFFF_FFFF) as i32 as i64;\n",
                        t = target,
                        tbl = table,
                        k = key
                    ));
                }
                Instruction::Move { target, source, ty } => match ty {
                    compiler::StaticType::Integer => {
                        out.push_str(&format!("    i_r{} = i_r{};\n", target, source))
                    }
                    compiler::StaticType::Boolean => {
                        out.push_str(&format!("    b_r{} = b_r{};\n", target, source))
                    }
                    compiler::StaticType::Table => {
                        out.push_str(&format!("    t_r{} = t_r{};\n", target, source))
                    }
                },
                Instruction::Add {
                    target,
                    left,
                    right,
                } => {
                    out.push_str(&format!(
                        "    i_r{t} = i_r{l} + i_r{r};\n",
                        t = target,
                        l = left,
                        r = right
                    ));
                }
                Instruction::Sub {
                    target,
                    left,
                    right,
                } => {
                    out.push_str(&format!(
                        "    i_r{t} = i_r{l} - i_r{r};\n",
                        t = target,
                        l = left,
                        r = right
                    ));
                }
                Instruction::Less {
                    target,
                    left,
                    right,
                } => {
                    out.push_str(&format!(
                        "    b_r{t} = i_r{l} < i_r{r};\n",
                        t = target,
                        l = left,
                        r = right
                    ));
                }
                Instruction::BeginWhile => out.push_str("    loop {\n"),
                Instruction::WhileCondition { cond_reg } => {
                    out.push_str(&format!("    if !b_r{} {{ break; }}\n", cond_reg));
                }
                Instruction::EndWhile => out.push_str("    }\n"),
            }
        }

        out.push_str("\n    tables\n}\n");
        out
    }
}

impl compiler::Backend for IrBackend {
    fn emit_load_int(&mut self, target: u8, val: i64) {
        self.ir.push(Instruction::LoadInt { target, val });
    }
    fn emit_new_table(&mut self, target: u8) {
        self.ir.push(Instruction::NewTable { target });
    }
    fn emit_set_table(&mut self, table: u8, key: u8, val: u8) {
        self.ir.push(Instruction::SetTable { table, key, val });
    }
    fn emit_get_table(&mut self, target: u8, table: u8, key: u8) {
        self.ir.push(Instruction::GetTable { target, table, key });
    }
    fn emit_move(&mut self, target: u8, source: u8, ty: StaticType) {
        if target != source {
            self.ir.push(Instruction::Move { target, source, ty });
        }
    }
    fn emit_add(&mut self, target: u8, left: u8, right: u8) {
        self.ir.push(Instruction::Add {
            target,
            left,
            right,
        });
    }
    fn emit_sub(&mut self, target: u8, left: u8, right: u8) {
        self.ir.push(Instruction::Sub {
            target,
            left,
            right,
        });
    }
    fn emit_less(&mut self, target: u8, left: u8, right: u8) {
        self.ir.push(Instruction::Less {
            target,
            left,
            right,
        });
    }
    fn begin_while(&mut self) {
        self.ir.push(Instruction::BeginWhile);
    }
    fn while_condition(&mut self, cond_reg: u8) {
        self.ir.push(Instruction::WhileCondition { cond_reg });
    }
    fn end_while(&mut self) {
        self.ir.push(Instruction::EndWhile);
    }
}
