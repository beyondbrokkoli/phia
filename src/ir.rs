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

    EnsureCapacity {
        table: u8,
        limit: u8,
    },
    SetTableFast {
        table: u8,
        key: u8,
        val: u8,
    },
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

    // NEW: The Optimization Pass (with Copy Propagation)
    pub fn optimize(&mut self) {
        let mut i = 0;
        while i < self.ir.len() {
            if let Instruction::BeginWhile = self.ir[i] {
                let mut limit_reg = None;
                let mut idx_reg = None;

                // 1. Lookahead: Scan forward to see if this loop has a clean `<` bounds check
                for j in i + 1..self.ir.len() {
                    match self.ir[j] {
                        Instruction::Less { target, left, right } => {
                            if let Some(Instruction::WhileCondition { cond_reg }) = self.ir.get(j + 1) {
                                if *cond_reg == target {
                                    idx_reg = Some(left);
                                    limit_reg = Some(right);
                                }
                            }
                        }
                        Instruction::EndWhile => break,
                        _ => {}
                    }
                }

                // 2. If we proved the bounds, find SetTable instructions that use this index (or a copy of it)
                if let (Some(idx), Some(limit)) = (idx_reg, limit_reg) {
                    let mut hoists = Vec::new();
                    let mut j = i + 1;

                    // Track the canonical index, plus any temporary registers it gets moved into
                    let mut active_aliases = vec![idx];

                    while j < self.ir.len() {
                        match self.ir[j] {
                            Instruction::Move { target, source, .. } => {
                                // If the loop index gets copied to a temporary, track the temporary!
                                if active_aliases.contains(&source) {
                                    active_aliases.push(target);
                                }
                            }
                            Instruction::SetTable { table, key, val } if active_aliases.contains(&key) => {
                                // Swap to the branchless fast-path instruction
                                self.ir[j] = Instruction::SetTableFast { table, key, val };
                                hoists.push(table);
                            }
                            Instruction::EndWhile => break,
                            _ => {}
                        }
                        j += 1;
                    }

                    // 3. Hoist the capacity check to BEFORE the loop begins
                    for table in hoists {
                        self.ir.insert(i, Instruction::EnsureCapacity { table, limit });
                        i += 1; // Keep `i` pointing at BeginWhile after insertion
                    }
                }
            }
            i += 1;
        }
    }

    pub fn generate_rust_code(&self) -> String {
        let mut out = String::new();

        out.push_str("use crate::memory::{Value, Table};\n\n");
        out.push_str("#[allow(unused_variables, unused_mut, unused_assignments)]\n");
        // 1. Change the return type to Box<Table>
        out.push_str("pub fn run_baked() -> Vec<Box<Table>> {\n");

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

                Instruction::EnsureCapacity { table, limit } => {
                    used_t.insert(*table);
                    used_i.insert(*limit);
                }
                Instruction::SetTableFast { table, key, val } => {
                    used_t.insert(*table);
                    used_i.insert(*key);
                    used_i.insert(*val);
                }
            }
        }

        for r in used_i {
            out.push_str(&format!("    let mut i_r{} = 0i64;\n", r));
        }
        for r in used_b {
            out.push_str(&format!("    let mut b_r{} = false;\n", r));
        }
        for r in used_t {
            // 2. Table registers are now literal CPU pointers!
            out.push_str(&format!("    let mut t_r{}: *mut Table = std::ptr::null_mut();\n", r));
        }
        // 3. The arena holds boxed tables so their heap addresses never change
        out.push_str("    let mut tables = Vec::<Box<Table>>::with_capacity(128);\n\n");

        for instr in &self.ir {
            match instr {
                Instruction::LoadInt { target, val } => {
                    out.push_str(&format!("    i_r{} = {};\n", target, val));
                }
                Instruction::NewTable { target } => {
                    out.push_str("    let mut new_table = Box::new(Table::new());\n");
                    out.push_str(&format!("    t_r{} = &mut *new_table as *mut Table;\n", target));
                    out.push_str("    tables.push(new_table);\n");
                }
                Instruction::SetTable { table, key, val } => {
                    out.push_str(&format!(
                        "    let idx = i_r{k} as usize;\n\
                         \x20   // Direct pointer deref - zero arena lookups!\n\
                         \x20   let t = unsafe {{ &mut *t_r{tbl} }};\n\
                         \x20   if idx >= t.array.len() {{\n\
                         \x20       if idx == t.array.len() {{\n\
                         \x20           t.array.push(Value::nil());\n\
                         \x20       }} else {{\n\
                         \x20           t.array.resize(idx + 1, Value::nil());\n\
                         \x20       }}\n\
                         \x20   }}\n\
                         \x20   unsafe {{ *t.array.get_unchecked_mut(idx) = Value::integer(i_r{v} as i32); }}\n",
                        tbl = table,
                        k = key,
                        v = val
                    ));
                }
                Instruction::GetTable { target, table, key } => {
                    out.push_str(&format!(
                        "    let idx = i_r{k} as usize;\n\
                         \x20   // Explicitly create a safe reference first to satisfy the borrow checker\n\
                         \x20   let t = unsafe {{ &*t_r{tbl} }};\n\
                         \x20   let raw_val = unsafe {{ *t.array.get_unchecked(idx) }};\n\
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

                Instruction::EnsureCapacity { table, limit } => {
                    out.push_str(&format!(
                        "    let cap_limit = i_r{limit} as usize;\n\
                         \x20   let t = unsafe {{ &mut *t_r{table} }};\n\
                         \x20   if cap_limit > t.array.len() {{\n\
                         \x20       t.array.resize(cap_limit, Value::nil());\n\
                         \x20   }}\n",
                        table = table,
                        limit = limit
                    ));
                }
                Instruction::SetTableFast { table, key, val } => {
                    out.push_str(&format!(
                        "    let idx = i_r{key} as usize;\n\
                         \x20   let t = unsafe {{ &mut *t_r{table} }};\n\
                         \x20   // HOT PATH: pure unchecked assignment, capacity proven!\n\
                         \x20   unsafe {{ *t.array.get_unchecked_mut(idx) = Value::integer(i_r{val} as i32); }}\n",
                        table = table,
                        key = key,
                        val = val
                    ));
                }
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
