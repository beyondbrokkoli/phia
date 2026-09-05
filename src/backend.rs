// src/backend.rs
use crate::ir::{IrProgram, Instruction, Terminator, BasicBlock, BlockId, RegId};
use std::collections::{HashMap, HashSet};

pub struct IrBackend {
    pub program: IrProgram,
}

impl IrBackend {
    pub fn new(program: IrProgram) -> Self {
        Self { program }
    }

    pub fn optimize(&mut self) {
        let mut def_map: HashMap<RegId, (BlockId, usize)> = HashMap::new();
        for block in &self.program.blocks {
            for (i, instr) in block.instrs.iter().enumerate() {
                match instr {
                    Instruction::LoadInt { target, .. } | Instruction::NewTable { target } |
                    Instruction::GetTable { target, .. } | Instruction::Move { target, .. } |
                    Instruction::Add { target, .. } | Instruction::Sub { target, .. } |
                    Instruction::Less { target, .. } | Instruction::Phi { target, .. } |
                    Instruction::GetTableFast { target, .. } => {
                        def_map.insert(*target, (block.id, i));
                    }
                    _ => {}
                }
            }
        }

        // Helper: Trace a table back to its original NewTable allocation.
        // Uses `blocks` argument explicitly so `self` is NOT locked.
        let get_table_root = |blocks: &[BasicBlock], table_reg: RegId| -> Option<RegId> {
            let mut curr = table_reg;
            let mut seen = HashSet::new();
            loop {
                if !seen.insert(curr) { return None; } // cycle
                if let Some(&(b, i)) = def_map.get(&curr) {
                    match &blocks[b].instrs[i] {
                        Instruction::NewTable { target } => return Some(target.clone()),
                        Instruction::Move { source, ty, .. } if *ty == crate::ast::StaticType::Table => {
                            curr = source.clone();
                            continue;
                        }
                        _ => return None, // Phi or ambiguous
                    }
                } else {
                    return None;
                }
            }
        };

        // Helper: Check if a key traces directly back to idx_reg via only Moves.
        // Uses `blocks` argument explicitly so `self` is NOT locked.
        let is_safe_key = |blocks: &[BasicBlock], key_reg: RegId, idx_reg: RegId| -> bool {
            let mut curr = key_reg;
            loop {
                if curr == idx_reg { return true; }
                if let Some(&(b, i)) = def_map.get(&curr) {
                    match &blocks[b].instrs[i] {
                        Instruction::Move { source, ty, .. } if *ty == crate::ast::StaticType::Integer => {
                            curr = source.clone();
                            continue;
                        }
                        _ => return false,
                    }
                } else {
                    return false;
                }
            }
        };

        let num_blocks = self.program.blocks.len();
        for header_id in 0..num_blocks {
            let terminator = self.program.blocks[header_id].terminator.clone();

            if let Some(Terminator::Branch { cond, true_block: body_id, .. }) = terminator {
                if let Some(&(cond_block, cond_idx)) = def_map.get(&cond) {

                    let is_less = if let Instruction::Less { left, right, .. } = &self.program.blocks[cond_block].instrs[cond_idx] {
                        Some((left.clone(), right.clone()))
                    } else { None };

                    if let Some((idx_reg, limit_reg)) = is_less {
                        let (limit_def_block, _) = def_map.get(&limit_reg).unwrap_or(&(0, 0));
                        let limit_is_invariant = *limit_def_block < header_id;

                        if limit_is_invariant {
                            let mut clobbered_roots = HashSet::new();
                            let mut hoists = HashSet::new();
                            let mut upgrades = Vec::new();

                            // PASS 1: Read-Only. Find Poisoned Chalices!
                            let mut abort_all = false;
                            for instr in &self.program.blocks[body_id].instrs {
                                if let Instruction::SetTable { table, key, .. } = instr {
                                    // NO DOMINANCE CHECK HERE! All dynamic writes matter.
                                    if !is_safe_key(&self.program.blocks, *key, idx_reg) {
                                        if let Some(root) = get_table_root(&self.program.blocks, *table) {
                                            clobbered_roots.insert(root);
                                        } else {
                                            // If we can't trace the root of an unsafe write (e.g. it's a Phi),
                                            // it might be reallocating ONE OF OUR HOISTED TABLES!
                                            // We must abort hoisting for this entire loop.
                                            abort_all = true;
                                            break;
                                        }
                                    }
                                }
                            }

                            if abort_all { continue; } // Safety valve: bail out!

                            // PASS 2: Read-Only. Determine which instructions to upgrade.
                            for (i, instr) in self.program.blocks[body_id].instrs.iter().enumerate() {
                                match instr {
                                    Instruction::SetTable { table, key, val } => {
                                        // S2 DOMINANCE CHECK: Only hoist pointers defined BEFORE the loop.
                                        let (def_b, _) = def_map.get(table).unwrap_or(&(0,0));
                                        if *def_b >= header_id { continue; }

                                        if let Some(root) = get_table_root(&self.program.blocks, *table) {
                                            if is_safe_key(&self.program.blocks, *key, idx_reg) && !clobbered_roots.contains(&root) {
                                                upgrades.push((i, Instruction::SetTableFast { table: *table, key: *key, val: *val }));
                                                hoists.insert(*table);
                                            }
                                        }
                                    }
                                    Instruction::GetTable { target, table, key } => {
                                        // S2 DOMINANCE CHECK: Only hoist pointers defined BEFORE the loop.
                                        let (def_b, _) = def_map.get(table).unwrap_or(&(0,0));
                                        if *def_b >= header_id { continue; }

                                        if let Some(root) = get_table_root(&self.program.blocks, *table) {
                                            if is_safe_key(&self.program.blocks, *key, idx_reg) && !clobbered_roots.contains(&root) {
                                                upgrades.push((i, Instruction::GetTableFast { target: *target, table: *table, key: *key }));
                                                hoists.insert(*table);
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }

                            // PASS 3: Mutate!
                            for (i, new_instr) in upgrades {
                                self.program.blocks[body_id].instrs[i] = new_instr;
                            }

                            // FIX: Structurally locate the true pre-header block
                            let mut pre_header_id = 0;
                            for b in 0..header_id {
                                if let Some(Terminator::Jump(tgt)) = &self.program.blocks[b].terminator {
                                    if *tgt == header_id {
                                        pre_header_id = b;
                                        break;
                                    }
                                }
                            }

                            // S5: EC before HR in Pre-Header
                            for table in hoists {
                                self.program.blocks[pre_header_id].instrs.push(Instruction::EnsureCapacity { table, limit: limit_reg });
                                self.program.blocks[pre_header_id].instrs.push(Instruction::HoistRawPtr { table });
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn resolve_phis(&mut self) {
        let mut injections = Vec::new();

        // 1. Gather all the incoming edges for every Phi node
        for block in &self.program.blocks {
            for instr in &block.instrs {
                // FIX: Extract `ty`
                if let Instruction::Phi { target, ty, args } = instr {
                    for (pred_id, src_reg) in args {
                        injections.push((*pred_id, *target, *src_reg, ty.clone()));
                    }
                }
            }
        }

        // 2. Inject a Move instruction at the end of the predecessor blocks
        for (pred_id, target, src, ty) in injections {
            self.program.blocks[pred_id].instrs.push(Instruction::Move {
                target,
                source: src,
                ty, // FIX: Use the extracted type instead of hardcoding Integer
            });
        }

        // 3. Delete the Phi nodes
        for block in &mut self.program.blocks {
            block.instrs.retain(|i| !matches!(i, Instruction::Phi { .. }));
        }
    }

    pub fn generate_rust_code(&self) -> String {
        let mut out = String::new();
        out.push_str("// target/release/build/phia-*/out/baked_native.rs\n\n");
        out.push_str("use crate::memory::Table;\n\n");
        out.push_str("#[allow(unused_variables, unused_mut, unused_assignments)]\n");
        out.push_str("pub fn run_baked() -> Vec<Box<Table>> {\n");

        let mut max_reg = 0;
        for block in &self.program.blocks {
            for instr in &block.instrs {
                match instr {
                    Instruction::LoadInt { target, .. } | Instruction::NewTable { target } |
                    Instruction::GetTable { target, .. } | Instruction::Move { target, .. } |
                    Instruction::Add { target, .. } | Instruction::Sub { target, .. } |
                    Instruction::Less { target, .. } | Instruction::GetTableFast { target, .. } => {
                        if *target > max_reg { max_reg = *target; }
                    }
                    _ => {}
                }
            }
        }

        for r in 0..=max_reg {
            out.push_str(&format!("    let mut i_r{} = 0i64;\n", r));
            out.push_str(&format!("    let mut b_r{} = false;\n", r));
            out.push_str(&format!("    let mut t_r{}: *mut Table = std::ptr::null_mut();\n", r));
            out.push_str(&format!("    let mut p_r{}: *mut i64 = std::ptr::null_mut();\n", r));
            out.push_str(&format!("    let mut len_r{} = 0usize;\n", r));
        }
        out.push_str("    let mut tables = Vec::<Box<Table>>::with_capacity(128);\n\n");

        out.push_str("    let mut current_block = 0;\n");
        out.push_str("    'cfg: loop {\n");
        out.push_str("        match current_block {\n");

        for block in &self.program.blocks {
            out.push_str(&format!("            {} => {{\n", block.id));

            for instr in &block.instrs {
                match instr {
                    Instruction::LoadInt { target, val } => out.push_str(&format!("                i_r{} = {};\n", target, val)),
                    Instruction::NewTable { target } => {
                        out.push_str("                let mut new_table = Box::new(Table::new());\n");
                        out.push_str(&format!("                t_r{} = &mut *new_table as *mut Table;\n", target));
                        out.push_str("                tables.push(new_table);\n");
                    }
                    Instruction::Move { target, source, ty } => match ty {
                        crate::ast::StaticType::Integer => out.push_str(&format!("                i_r{} = i_r{};\n", target, source)),
                        crate::ast::StaticType::Boolean => out.push_str(&format!("                b_r{} = b_r{};\n", target, source)),
                        crate::ast::StaticType::Table => out.push_str(&format!("                t_r{} = t_r{};\n", target, source)),
                    },
                    Instruction::Add { target, left, right } => out.push_str(&format!("                i_r{} = i_r{} + i_r{};\n", target, left, right)),
                    Instruction::Sub { target, left, right } => out.push_str(&format!("                i_r{} = i_r{} - i_r{};\n", target, left, right)),
                    Instruction::Less { target, left, right } => out.push_str(&format!("                b_r{} = i_r{} < i_r{};\n", target, left, right)),

                    Instruction::EnsureCapacity { table, limit } => out.push_str(&format!(
                        "                let lim = i_r{limit};\n\
                                         if lim > 0 {{\n\
                                             let t = unsafe {{ &mut *t_r{table} }};\n\
                                             if (lim as usize) > t.array.len() {{\n\
                                                 t.array.resize(lim as usize, 0);\n\
                                             }}\n\
                                         }}\n"
                    )),
                    Instruction::HoistRawPtr { table } => out.push_str(&format!(
                        "                len_r{table} = unsafe {{ (*t_r{table}).array.len() }};\n\
                                         p_r{table} = unsafe {{ (*t_r{table}).array.as_mut_ptr() }};\n"
                    )),

                    Instruction::SetTable { table, key, val } => out.push_str(&format!(
                        "                let k = i_r{key};\n\
                                         if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                                         let idx = k as usize;\n\
                                         let t = unsafe {{ &mut *t_r{table} }};\n\
                                         if idx >= t.array.len() {{ t.array.resize(idx + 1, 0); }}\n\
                                         unsafe {{ *t.array.get_unchecked_mut(idx) = i_r{val}; }}\n"
                    )),
                    Instruction::GetTable { target, table, key } => out.push_str(&format!(
                        "                let k = i_r{key};\n\
                                         if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                                         let idx = k as usize;\n\
                                         let t = unsafe {{ &*t_r{table} }};\n\
                                         i_r{target} = if idx < t.array.len() {{ unsafe {{ *t.array.get_unchecked(idx) }} }} else {{ 0 }};\n"
                    )),
                    Instruction::SetTableFast { table, key, val } => out.push_str(&format!(
                        "                let k = i_r{key};\n\
                                         if k < 0 {{ panic!(\"Runtime Error: Negative index in fast path\"); }}\n\
                                         if (k as usize) < len_r{table} {{\n\
                                             unsafe {{ *p_r{table}.add(k as usize) = i_r{val}; }}\n\
                                         }} else {{\n\
                                             panic!(\"optimizer invariant violated: fast-path bounds check failed\");\n\
                                         }}\n"
                    )),
                    Instruction::GetTableFast { target, table, key } => out.push_str(&format!(
                        "                let k = i_r{key};\n\
                                         if k < 0 {{ panic!(\"Runtime Error: Negative index in fast path\"); }}\n\
                                         if (k as usize) < len_r{table} {{\n\
                                             i_r{target} = unsafe {{ *p_r{table}.add(k as usize) }};\n\
                                         }} else {{\n\
                                             panic!(\"optimizer invariant violated: fast-path bounds check failed\");\n\
                                         }}\n"
                    )),
                    Instruction::Phi { .. } => {}
                }
            }

            match &block.terminator {
                Some(Terminator::Jump(target)) => out.push_str(&format!("                current_block = {};\n", target)),
                Some(Terminator::Branch { cond, true_block, false_block }) => {
                    out.push_str(&format!("                current_block = if b_r{} {{ {} }} else {{ {} }};\n", cond, true_block, false_block));
                }
                Some(Terminator::Halt) | None => out.push_str("                break 'cfg;\n"),
            }
            out.push_str("            }\n");
        }

        out.push_str("            _ => unreachable!(),\n        }\n    }\n    tables\n}\n");
        out
    }
}
