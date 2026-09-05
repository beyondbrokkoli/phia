// src/backend.rs
use crate::ir::Instruction;
use crate::ast::StaticType;
use std::collections::BTreeSet;

pub struct IrBackend {
    pub ir: Vec<Instruction>,
}

impl IrBackend {
    pub fn new() -> Self {
        Self { ir: Vec::with_capacity(1024) }
    }

    pub fn optimize(&mut self) {
        // GLOBAL TABLE ALIAS TRACKING (Fixes Bugs #4, #11, #12)
        let mut table_roots = std::collections::HashMap::new();
        let mut ambiguous_roots = std::collections::BTreeSet::new();

        for instr in &self.ir {
            if let Instruction::Move { target, source, ty: StaticType::Table } = instr {
                if table_roots.contains_key(target) {
                    ambiguous_roots.insert(*target);
                } else {
                    table_roots.insert(*target, *source);
                }
            }
        }

        // Cycle-safe and ambiguity-aware root resolution.
        // Returns `None` if a cycle is detected or if the register is ambiguously reassigned.
        let get_table_root = |mut reg: u32| -> Option<u32> {
            let mut seen = std::collections::BTreeSet::new();
            loop {
                if ambiguous_roots.contains(&reg) { return None; }
                if !seen.insert(reg) { return None; } // Cycle detected

                if let Some(&parent) = table_roots.get(&reg) {
                    reg = parent;
                } else {
                    return Some(reg); // Found the root!
                }
            }
        };

        let mut i = 0;
        while i < self.ir.len() {
            if let Instruction::BeginWhile = self.ir[i] {
                let mut limit_reg = None;
                let mut idx_reg = None;
                let mut header_depth = 1;

                for j in i + 1..self.ir.len() {
                    match self.ir[j] {
                        Instruction::BeginWhile => header_depth += 1,
                        Instruction::EndWhile => {
                            header_depth -= 1;
                            if header_depth == 0 { break; }
                        }
                        Instruction::Less { target, left, right } if header_depth == 1 => {
                            if let Some(Instruction::WhileCondition { cond_reg }) = self.ir.get(j + 1) {
                                if *cond_reg == target {
                                    idx_reg = Some(left);
                                    limit_reg = Some(right);
                                }
                            }
                        }
                        Instruction::WhileCondition { .. } if header_depth == 1 => {
                            // The header for THIS loop ends here.
                            // If we haven't found our `Less` by now, it doesn't exist.
                            break;
                        }
                        _ => {}
                    }
                }

                if let (Some(idx), Some(limit)) = (idx_reg, limit_reg) {
                    // --- BUG #1 & BUG #3 FIX: Limit Invariance Scan ---
                    let mut limit_is_invariant = true;
                    let mut loop_depth = 0;
                    for k in i..self.ir.len() {
                        match self.ir[k] {
                            Instruction::BeginWhile => loop_depth += 1,
                            Instruction::EndWhile => {
                                loop_depth -= 1;
                                if loop_depth == 0 { break; }
                            }
                            Instruction::LoadInt { target, .. } | Instruction::Move { target, .. } |
                            Instruction::Add { target, .. } | Instruction::Sub { target, .. } |
                            Instruction::Less { target, .. } | Instruction::GetTable { target, .. } |
                            Instruction::GetTableFast { target, .. } | Instruction::NewTable { target } => {
                                if target == limit {
                                    limit_is_invariant = false;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }

                    // Only proceed if the limit is safe
                    if limit_is_invariant {
                        // 0. Prepass: Compute def_sets for nested loops (Bug #15)
                        let mut def_sets = std::collections::HashMap::new();
                        let mut loop_stack = Vec::new();
                        for p in i..self.ir.len() {
                            match self.ir[p] {
                                Instruction::BeginWhile => {
                                    loop_stack.push(p);
                                    def_sets.insert(p, std::collections::BTreeSet::new());
                                }
                                Instruction::EndWhile => {
                                    loop_stack.pop().unwrap();
                                    if loop_stack.is_empty() { break; } // exited the candidate loop
                                }
                                Instruction::Move { target, .. } |
                                Instruction::LoadInt { target, .. } |
                                Instruction::Add { target, .. } |
                                Instruction::Sub { target, .. } |
                                Instruction::Less { target, .. } |
                                Instruction::GetTable { target, .. } |
                                Instruction::GetTableFast { target, .. } |
                                Instruction::NewTable { target } => {
                                    // Record the mutation in all open loops EXCEPT the candidate itself (i)
                                    for &lp in &loop_stack {
                                        if lp != i {
                                            def_sets.get_mut(&lp).unwrap().insert(target);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }

                        // 1. Define the Scope-Aware Tracker
                        struct AliasTracker<'a> {
                            active: Vec<u32>,
                            scopes: Vec<(Vec<u32>, std::collections::BTreeSet<u32>)>,
                            def_sets: &'a std::collections::HashMap<usize, std::collections::BTreeSet<u32>>,
                        }

                        impl<'a> AliasTracker<'a> {
                            fn new(idx_reg: u32, def_sets: &'a std::collections::HashMap<usize, std::collections::BTreeSet<u32>>) -> Self {
                                Self { active: vec![idx_reg], scopes: Vec::new(), def_sets }
                            }

                            fn contains(&self, reg: &u32) -> bool {
                                self.active.contains(reg)
                            }

                            fn update(&mut self, instr: &Instruction, pos: usize) {
                                match instr {
                                    Instruction::BeginWhile => {
                                        // Snapshot entry state and prepare a new kill zone
                                        self.scopes.push((self.active.clone(), std::collections::BTreeSet::new()));
                                        // BUG #15: Kill aliases modified anywhere inside this nested region
                                        if let Some(defs) = self.def_sets.get(&pos) {
                                            self.active.retain(|r| !defs.contains(r));
                                        }
                                    }
                                    Instruction::EndWhile => {
                                        // Restore: entry_set - (everything defined anywhere inside this scope)
                                        if let Some((entry, killed)) = self.scopes.pop() {
                                            self.active = entry.into_iter().filter(|r| !killed.contains(r)).collect();
                                            // Bubble the kills up to the parent scope
                                            if let Some(parent) = self.scopes.last_mut() {
                                                parent.1.extend(killed);
                                            }
                                        }
                                    }
                                    Instruction::Move { target, source, .. } => {
                                        if self.active.contains(source) {
                                            if !self.active.contains(target) { self.active.push(*target); }
                                        } else {
                                            self.active.retain(|r| r != target);
                                        }
                                        if let Some(scope) = self.scopes.last_mut() { scope.1.insert(*target); }
                                    }
                                    Instruction::LoadInt { target, .. } |
                                    Instruction::Add { target, .. } |
                                    Instruction::Sub { target, .. } |
                                    Instruction::Less { target, .. } |
                                    Instruction::GetTable { target, .. } |
                                    Instruction::GetTableFast { target, .. } |
                                    Instruction::NewTable { target } => {
                                        self.active.retain(|r| r != target);
                                        if let Some(scope) = self.scopes.last_mut() { scope.1.insert(*target); }
                                    }
                                    _ => {}
                                }
                            }
                        }

                        // PASS 1: The Safety Scan
                        let mut clobbered_tables = std::collections::BTreeSet::new();
                        let mut tracker1 = AliasTracker::new(idx, &def_sets);
                        let mut scan_depth = 1;
                        let mut abort_optimization = false;

                        let mut clobber_root = |reg: u32, clobbers: &mut std::collections::BTreeSet<u32>| {
                            if let Some(r) = get_table_root(reg) {
                                clobbers.insert(r);
                            } else {
                                abort_optimization = true;
                            }
                        };

                        for k in (i + 1)..self.ir.len() {
                            let instr = self.ir[k].clone();
                            let is_alias = match instr {
                                Instruction::SetTable { key, .. } | Instruction::GetTable { key, .. } => tracker1.contains(&key),
                                _ => false,
                            };

                            match instr {
                                Instruction::BeginWhile => scan_depth += 1,
                                Instruction::EndWhile => {
                                    scan_depth -= 1;
                                    if scan_depth == 0 { break; }
                                }
                                Instruction::NewTable { target } => clobber_root(target, &mut clobbered_tables),
                                Instruction::Move { target, source, ref ty } => {
                                    if *ty == StaticType::Table {
                                        clobber_root(target, &mut clobbered_tables);
                                        clobber_root(source, &mut clobbered_tables);
                                    }
                                }
                                Instruction::SetTable { table, .. } | Instruction::GetTable { target: _, table, .. } => {
                                    if !is_alias { clobber_root(table, &mut clobbered_tables); }
                                }
                                _ => {}
                            }

                            tracker1.update(&instr, k);
                        }

                        if abort_optimization {
                            i += 1;
                            continue;
                        }

                        // PASS 2: The Rewrite Pass
                        let mut hoists = Vec::new();
                        let mut tracker2 = AliasTracker::new(idx, &def_sets);
                        let mut j = i + 1;
                        let mut j_depth = 1;

                        while j < self.ir.len() {
                            let instr = self.ir[j].clone();
                            let is_alias = match instr {
                                Instruction::SetTable { key, .. } | Instruction::GetTable { key, .. } => tracker2.contains(&key),
                                _ => false,
                            };

                            match instr {
                                Instruction::BeginWhile => j_depth += 1,
                                Instruction::EndWhile => {
                                    j_depth -= 1;
                                    if j_depth == 0 { break; }
                                }
                                Instruction::SetTable { table, key, val } => {
                                    if is_alias {
                                        if let Some(r) = get_table_root(table) {
                                            if !clobbered_tables.contains(&r) {
                                                self.ir[j] = Instruction::SetTableFast { table, key, val };
                                                if !hoists.contains(&table) { hoists.push(table); }
                                            }
                                        }
                                    }
                                }
                                Instruction::GetTable { target, table, key } => {
                                    if is_alias {
                                        if let Some(r) = get_table_root(table) {
                                            if !clobbered_tables.contains(&r) {
                                                self.ir[j] = Instruction::GetTableFast { target, table, key };
                                                if !hoists.contains(&table) { hoists.push(table); }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }

                            tracker2.update(&instr, j);
                            j += 1;
                        }

                        // Hoist capacity AND raw pointer extraction BEFORE the loop
                        for table in hoists {
                            self.ir.insert(i, Instruction::EnsureCapacity { table, limit });
                            i += 1;
                            self.ir.insert(i, Instruction::HoistRawPtr { table });
                            i += 1;
                        }
                    }
                }
            }
            i += 1;
        }
    }

    pub fn generate_rust_code(&self) -> String {
        let mut out = String::new();

        out.push_str("// target/release/build/phia-*/out/baked_native.rs\n\n");
        out.push_str("use crate::memory::Table;\n\n");
        out.push_str("#[allow(unused_variables, unused_mut, unused_assignments)]\n");
        out.push_str("pub fn run_baked() -> Vec<Box<Table>> {\n");

        let mut used_i = BTreeSet::new();
        let mut used_b = BTreeSet::new();
        let mut used_t = BTreeSet::new();

        for instr in &self.ir {
            match instr {
                Instruction::LoadInt { target, .. } => { used_i.insert(*target); }
                Instruction::NewTable { target } => { used_t.insert(*target); }
                Instruction::SetTable { table, key, val } => {
                    used_t.insert(*table); used_i.insert(*key); used_i.insert(*val);
                }
                Instruction::GetTable { target, table, key } => {
                    used_i.insert(*target); used_t.insert(*table); used_i.insert(*key);
                }
                Instruction::Move { target, source, ty } => match ty {
                    StaticType::Integer => { used_i.insert(*target); used_i.insert(*source); }
                    StaticType::Boolean => { used_b.insert(*target); used_b.insert(*source); }
                    StaticType::Table => { used_t.insert(*target); used_t.insert(*source); }
                },
                Instruction::Add { target, left, right } | Instruction::Sub { target, left, right } => {
                    used_i.insert(*target); used_i.insert(*left); used_i.insert(*right);
                }
                Instruction::Less { target, left, right } => {
                    used_b.insert(*target); used_i.insert(*left); used_i.insert(*right);
                }
                Instruction::WhileCondition { cond_reg } => { used_b.insert(*cond_reg); }
                Instruction::BeginWhile | Instruction::EndWhile => {}
                Instruction::EnsureCapacity { table, limit } => {
                    used_t.insert(*table); used_i.insert(*limit);
                }
                Instruction::HoistRawPtr { table } => {
                    used_t.insert(*table);
                }
                Instruction::SetTableFast { table, key, val } => {
                    used_t.insert(*table); used_i.insert(*key); used_i.insert(*val);
                }
                Instruction::GetTableFast { target, table, key } => {
                    used_t.insert(*table); used_i.insert(*target); used_i.insert(*key);
                }
            }
        }

        for r in used_i { out.push_str(&format!("let mut i_r{} = 0i64;\n", r)); }
        for r in used_b { out.push_str(&format!("let mut b_r{} = false;\n", r)); }
        for r in used_t {
            out.push_str(&format!("let mut t_r{}: *mut Table = std::ptr::null_mut();\n", r));
            // Give every table register a companion raw pointer!
            out.push_str(&format!("let mut p_r{}: *mut i64 = std::ptr::null_mut();\n", r));
        }

        out.push_str("let mut tables = Vec::<Box<Table>>::with_capacity(128);\n\n");

        for instr in &self.ir {
            match instr {
                Instruction::LoadInt { target, val } => {
                    out.push_str(&format!("i_r{} = {};\n", target, val));
                }
                Instruction::NewTable { target } => {
                    out.push_str("let mut new_table = Box::new(Table::new());\n");
                    out.push_str(&format!("t_r{} = &mut *new_table as *mut Table;\n", target));
                    out.push_str("tables.push(new_table);\n");
                }

                Instruction::Move { target, source, ty } => match ty {
                    StaticType::Integer => out.push_str(&format!("i_r{} = i_r{};\n", target, source)),
                    StaticType::Boolean => out.push_str(&format!("b_r{} = b_r{};\n", target, source)),
                    StaticType::Table => out.push_str(&format!("t_r{} = t_r{};\n", target, source)),
                },
                Instruction::Add { target, left, right } => {
                    out.push_str(&format!("i_r{t} = i_r{l} + i_r{r};\n", t = target, l = left, r = right));
                }
                Instruction::Sub { target, left, right } => {
                    out.push_str(&format!("i_r{t} = i_r{l} - i_r{r};\n", t = target, l = left, r = right));
                }
                Instruction::Less { target, left, right } => {
                    out.push_str(&format!("b_r{t} = i_r{l} < i_r{r};\n", t = target, l = left, r = right));
                }
                Instruction::BeginWhile => out.push_str("loop {\n"),
                Instruction::WhileCondition { cond_reg } => {
                    out.push_str(&format!("if !b_r{} {{ break; }}\n", cond_reg));
                }
                Instruction::EndWhile => out.push_str("}\n"),

                // 1. Guard the Dynamic Write Path
                Instruction::SetTable { table, key, val } => {
                    out.push_str(&format!(
                        "let k = i_r{k};\n\
                         if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         let idx = k as usize;\n\
                         let t = unsafe {{ &mut *t_r{tbl} }};\n\
                         if idx >= t.array.len() {{\n\
                             if idx == t.array.len() {{\n\
                                 t.array.push(0);\n\
                             }} else {{\n\
                                 t.array.resize(idx + 1, 0);\n\
                             }}\n\
                         }}\n\
                         unsafe {{ *t.array.get_unchecked_mut(idx) = i_r{v}; }}\n",
                        tbl = table, k = key, v = val
                    ));
                }

                // 2. Guard the Dynamic Read Path
                Instruction::GetTable { target, table, key } => {
                    out.push_str(&format!(
                        "let k = i_r{k};\n\
                         if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         let idx = k as usize;\n\
                         let t = unsafe {{ &*t_r{tbl} }};\n\
                         i_r{t} = if idx < t.array.len() {{\n\
                             unsafe {{ *t.array.get_unchecked(idx) }}\n\
                         }} else {{\n\
                             0\n\
                         }};\n",
                        t = target, tbl = table, k = key
                    ));
                }

                // 3. Guard the Fast Paths (Upgrade the Interim Fence)
                Instruction::SetTableFast { table, key, val } => {
                    out.push_str(&format!(
                        "let k = i_r{key};\n\
                         if k < 0 {{ panic!(\"Runtime Error: Negative index in fast path\"); }}\n\
                         if (k as usize) < len_r{table} {{\n\
                             unsafe {{ *p_r{table}.add(k as usize) = i_r{val}; }}\n\
                         }} else {{\n\
                             panic!(\"optimizer invariant violated: fast-path bounds check failed\");\n\
                         }}\n",
                        table = table, key = key, val = val
                    ));
                }

                Instruction::GetTableFast { target, table, key } => {
                    out.push_str(&format!(
                        "let k = i_r{key};\n\
                         if k < 0 {{ panic!(\"Runtime Error: Negative index in fast path\"); }}\n\
                         i_r{target} = if (k as usize) < len_r{table} {{\n\
                             unsafe {{ *p_r{table}.add(k as usize) }}\n\
                         }} else {{\n\
                             panic!(\"optimizer invariant violated: fast-path bounds check failed\");\n\
                         }};\n",
                        target = target, table = table, key = key
                    ));
                }

                // 3. EnsureCapacity (Protect against negative loop limit)
                Instruction::EnsureCapacity { table, limit } => {
                    out.push_str(&format!(
                        "let lim = i_r{limit};\n\
                         if lim > 0 {{\n\
                             let cap_limit = lim as usize;\n\
                             let t = unsafe {{ &mut *t_r{table} }};\n\
                             if cap_limit > t.array.len() {{\n\
                                 t.array.resize(cap_limit, 0);\n\
                             }}\n\
                         }}\n",
                        table = table, limit = limit
                    ));
                }

                // 4. Hoist the Length Alongside the Pointer
                Instruction::HoistRawPtr { table } => {
                    out.push_str(&format!(
                        "let len_r{table} = unsafe {{ (*t_r{table}).array.len() }};\n\
                         p_r{table} = unsafe {{ (*t_r{table}).array.as_mut_ptr() }};\n",
                        table = table
                    ));
                }
            }
        }

        out.push_str("\n    tables\n}\n");
        out
    }
}
