// src/backend.rs
use crate::ir::{IrProgram, Instruction, Terminator, BasicBlock, BlockId, RegId};
use std::collections::{HashMap, HashSet};
use crate::ast::StaticType;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Pool { Int, Bool, Table }

fn pool_of(t: &StaticType) -> Pool {
    match t { StaticType::Integer => Pool::Int, StaticType::Boolean => Pool::Bool, StaticType::Table => Pool::Table }
}

fn def_reg(i: &Instruction) -> Option<RegId> {
    match i {
        Instruction::LoadInt { target, .. } | Instruction::NewTable { target }
        | Instruction::GetTable { target, .. } | Instruction::GetTableFast { target, .. }
        | Instruction::Move { target, .. } | Instruction::Add { target, .. }
        | Instruction::Sub { target, .. } | Instruction::Less { target, .. }
        | Instruction::Phi { target, .. } => Some(*target),
        _ => None,
    }
}

fn def_type(i: &Instruction) -> Option<StaticType> {
    match i {
        Instruction::LoadInt { .. } | Instruction::Add { .. } | Instruction::Sub { .. }
        | Instruction::GetTable { .. } | Instruction::GetTableFast { .. } => Some(StaticType::Integer),
        Instruction::Less { .. } => Some(StaticType::Boolean),
        Instruction::NewTable { .. } => Some(StaticType::Table),
        Instruction::Move { ty, .. } | Instruction::Phi { ty, .. } => Some(ty.clone()),
        _ => None,
    }
}

fn use_regs(i: &Instruction) -> Vec<RegId> {
    match i {
        Instruction::LoadInt { .. } | Instruction::NewTable { .. } => vec![],
        Instruction::Move { source, .. } => vec![*source],
        Instruction::Add { left, right, .. } | Instruction::Sub { left, right, .. }
        | Instruction::Less { left, right, .. } => vec![*left, *right],
        Instruction::SetTable { table, key, val } | Instruction::SetTableFast { table, key, val } => vec![*table, *key, *val],
        Instruction::GetTable { table, key, .. } | Instruction::GetTableFast { table, key, .. } => vec![*table, *key],
        Instruction::EnsureCapacity { table, limit } => vec![*table, *limit],
        Instruction::HoistRawPtr { table } => vec![*table],
        Instruction::Phi { args, .. } => args.iter().map(|&(_, r)| r).collect(),
    }
}

fn remap_instr<F: Fn(RegId) -> RegId>(i: &mut Instruction, f: &F) {
    let g = |r: &mut RegId| *r = f(*r);
    match i {
        Instruction::LoadInt { target, .. } | Instruction::NewTable { target } => g(target),
        Instruction::SetTable { table, key, val } | Instruction::SetTableFast { table, key, val } => { g(table); g(key); g(val); }
        Instruction::GetTable { target, table, key } | Instruction::GetTableFast { target, table, key } => { g(target); g(table); g(key); }
        Instruction::Move { target, source, .. } => { g(target); g(source); }
        Instruction::Add { target, left, right } | Instruction::Sub { target, left, right }
        | Instruction::Less { target, left, right } => { g(target); g(left); g(right); }
        Instruction::Phi { target, args, .. } => { g(target); for (_, r) in args.iter_mut() { g(r); } }
        Instruction::EnsureCapacity { table, limit } => { g(table); g(limit); }
        Instruction::HoistRawPtr { table } => g(table),
    }
}

fn touch(iv: &mut HashMap<RegId, (usize, usize)>, r: RegId, p: usize) {
    let e = iv.entry(r).or_insert((p, p));
    if p < e.0 { e.0 = p; }
    if p > e.1 { e.1 = p; }
}

fn compute_liveness(blocks: &[BasicBlock]) -> (Vec<HashSet<RegId>>, Vec<HashSet<RegId>>) {
    let n = blocks.len();
    let mut live_in  = vec![HashSet::<RegId>::new(); n];
    let mut live_out = vec![HashSet::<RegId>::new(); n];
    let mut changed = true;
    while changed {
        changed = false;
        for b in (0..n).rev() {
            let mut out = HashSet::new();
            match &blocks[b].terminator {
                Some(Terminator::Jump(t)) => out.extend(live_in[*t].iter().copied()),
                Some(Terminator::Branch { true_block, false_block, .. }) => {
                    out.extend(live_in[*true_block].iter().copied());
                    out.extend(live_in[*false_block].iter().copied());
                }
                _ => {}
            }
            let mut live = out.clone();
            if let Some(Terminator::Branch { cond, .. }) = &blocks[b].terminator { live.insert(*cond); }
            for i in blocks[b].instrs.iter().rev() {
                if let Some(d) = def_reg(i) { live.remove(&d); }
                for u in use_regs(i) { live.insert(u); }
            }
            if live != live_in[b] || out != live_out[b] {
                live_in[b] = live;
                live_out[b] = out;
                changed = true;
            }
        }
    }
    (live_in, live_out)
}

fn live_intervals(blocks: &[BasicBlock], live_out: &[HashSet<RegId>]) -> HashMap<RegId, (usize, usize)> {
    let mut iv = HashMap::new();
    let mut base = 0usize;
    for b in 0..blocks.len() {
        let n = blocks[b].instrs.len();
        let mut live = live_out[b].clone();
        if let Some(Terminator::Branch { cond, .. }) = &blocks[b].terminator { live.insert(*cond); }
        // Values live at block END keep their interval wrapped around loop
        // back edges. This is the load-bearing line: without it, loop-carried
        // values get intervals that stop at their last textual use, and the
        // allocator happily hands their slot to a temp inside the loop.
        for &r in live.iter() { touch(&mut iv, r, base + n); }
        for i in (0..n).rev() {
            let instr = &blocks[b].instrs[i];
            if let Some(d) = def_reg(instr) { live.remove(&d); }
            for u in use_regs(instr) { live.insert(u); }
            for &r in live.iter() { touch(&mut iv, r, base + i); }
            if let Some(d) = def_reg(instr) { touch(&mut iv, d, base + i); }
        }
        base += n + 1; // one slot for the terminator
    }
    iv
}

pub struct IrBackend {
    pub program: IrProgram,
    n_int: usize, n_bool: usize, n_table: usize, did_alloc: bool,
}

impl IrBackend {
    pub fn new(program: IrProgram) -> Self {
        Self { program, n_int: 0, n_bool: 0, n_table: 0, did_alloc: false }
    }

    /// Copy propagation + DCE. Deliberately conservative: a Move is erased
    /// only when BOTH target and source are single-def vregs. Phi targets
    /// are multi-def after resolve_phis, so loop-carried copies are never
    /// touched and the parallel-copy/swap hazard cannot appear.
    pub fn simplify(&mut self) {
        let mut defs: HashMap<RegId, usize> = HashMap::new();
        for b in &self.program.blocks {
            for i in &b.instrs { if let Some(d) = def_reg(i) { *defs.entry(d).or_insert(0) += 1; } }
        }

        let mut rename: HashMap<RegId, RegId> = HashMap::new();
        for b in &self.program.blocks {
            for i in &b.instrs {
                if let Instruction::Move { target, source, .. } = i {
                    if target != source
                        && defs.get(target) == Some(&1)
                        && defs.get(source) == Some(&1)
                    { rename.insert(*target, *source); }
                }
            }
        }

        if !rename.is_empty() {
            let resolve = |mut r: RegId| -> RegId {
                let mut guard = 0usize;
                while let Some(&next) = rename.get(&r) {
                    r = next; guard += 1;
                    if guard > 100_000 { panic!("copy-prop: rename cycle"); }
                }
                r
            };
            for b in &mut self.program.blocks {
                for i in &mut b.instrs { remap_instr(i, &resolve); }
            }
        }

        let mut uses: HashMap<RegId, usize> = HashMap::new();
        for b in &self.program.blocks {
            for i in &b.instrs { for u in use_regs(i) { *uses.entry(u).or_insert(0) += 1; } }
            if let Some(Terminator::Branch { cond, .. }) = &b.terminator { *uses.entry(*cond).or_insert(0) += 1; }
        }

        for b in &mut self.program.blocks {
            b.instrs.retain(|i| {
                if matches!(i, Instruction::Move { target, source, .. } if target == source) { return false; }
                // dead PURE defs only: NewTable allocates output, GetTable can
                // panic on negative keys — neither is ever "dead code" here.
                let dead = def_reg(i).map(|d| uses.get(&d).copied().unwrap_or(0) == 0).unwrap_or(false);
                let pure = matches!(i,
                    Instruction::LoadInt { .. } | Instruction::Move { .. }
                    | Instruction::Add { .. } | Instruction::Sub { .. } | Instruction::Less { .. });
                !(dead && pure)
            });
        }
    }

    /// Linear-scan allocation: vregs with disjoint live intervals share one
    /// physical slot. One pool per type, so i_/b_/t_ variables are independent.
    pub fn allocate_registers(&mut self) {
        let blocks = &self.program.blocks;

        // 1. types: defs first, operand positions as defensive fallback
        let mut ty: HashMap<RegId, Pool> = HashMap::new();
        for b in blocks {
            for i in &b.instrs {
                if let (Some(d), Some(t)) = (def_reg(i), def_type(i)) {
                    let p = pool_of(&t);
                    match ty.insert(d, p) {
                        Some(old) if old != p => panic!("reg {} has conflicting types", d),
                        _ => {}
                    }
                }
            }
        }
        for b in blocks {
            for i in &b.instrs {
                let ops: Vec<(RegId, StaticType)> = match i {
                    Instruction::SetTable { table, key, val } | Instruction::SetTableFast { table, key, val } =>
                        vec![(*table, StaticType::Table), (*key, StaticType::Integer), (*val, StaticType::Integer)],
                    Instruction::GetTable { table, key, .. } | Instruction::GetTableFast { table, key, .. } =>
                        vec![(*table, StaticType::Table), (*key, StaticType::Integer)],
                    Instruction::Add { left, right, .. } | Instruction::Sub { left, right, .. }
                    | Instruction::Less { left, right, .. } =>
                        vec![(*left, StaticType::Integer), (*right, StaticType::Integer)],
                    Instruction::EnsureCapacity { table, limit } =>
                        vec![(*table, StaticType::Table), (*limit, StaticType::Integer)],
                    Instruction::Move { source, ty: t, .. } => vec![(*source, t.clone())],
                    Instruction::HoistRawPtr { table } => vec![(*table, StaticType::Table)],
                    _ => vec![],
                };
                for (r, t) in ops { ty.entry(r).or_insert(pool_of(&t)); }
            }
            if let Some(Terminator::Branch { cond, .. }) = &b.terminator {
                ty.entry(*cond).or_insert(Pool::Bool);
            }
        }

        // 2. liveness + intervals
        let (_, live_out) = compute_liveness(blocks);
        let iv = live_intervals(blocks, &live_out);

        // 3. linear scan per pool
        let mut vregs: Vec<RegId> = ty.keys().copied().collect();
        vregs.sort_by_key(|&r| (iv.get(&r).copied().unwrap_or((0, 0)), r)); // deterministic

        let mut active: HashMap<Pool, Vec<(RegId, usize)>> = HashMap::new();
        let mut free: HashMap<Pool, Vec<RegId>> = HashMap::new();
        let mut count: HashMap<Pool, usize> = HashMap::new();
        let mut map: HashMap<RegId, RegId> = HashMap::new();

        for r in vregs {
            let p = ty[&r];
            let (start, end) = iv.get(&r).copied().unwrap_or((0, 0));
            let act = active.entry(p).or_default();

            let mut keep: Vec<(RegId, usize)> = Vec::new();
            for &(phys, e) in act.iter() {
                if e < start { free.entry(p).or_default().push(phys); }
                else { keep.push((phys, e)); }
            }
            *act = keep;

            let phys = free.entry(p).or_default().pop().unwrap_or_else(|| {
                let c = count.entry(p).or_insert(0);
                let n = *c; *c += 1; n as RegId
            });
            act.push((phys, end));
            map.insert(r, phys);
        }

        // 4. rewrite every reference in place — codegen body needs NO changes
        for b in &mut self.program.blocks {
            for i in &mut b.instrs { remap_instr(i, &|r| *map.get(&r).unwrap_or(&r)); }
            if let Some(Terminator::Branch { cond, .. }) = &mut b.terminator {
                if let Some(&p) = map.get(&*cond) { *cond = p; }
            }
        }

        // 5. moves that became self-copies are no-ops
        for b in &mut self.program.blocks {
            b.instrs.retain(|i| !matches!(i, Instruction::Move { target, source, .. } if target == source));
        }

        self.n_int   = *count.entry(Pool::Int).or_insert(0);
        self.n_bool  = *count.entry(Pool::Bool).or_insert(0);
        self.n_table = *count.entry(Pool::Table).or_insert(0);
        self.did_alloc = true;
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

        // tables touched by the fast path need hoisted ptr/len slots
        let mut fast_phys: HashSet<RegId> = HashSet::new();
        for b in &self.program.blocks {
            for i in &b.instrs {
                match i {
                    Instruction::HoistRawPtr { table }
                    | Instruction::SetTableFast { table, .. }
                    | Instruction::GetTableFast { table, .. } => { fast_phys.insert(*table); }
                    _ => {}
                }
            }
        }

        // pool sizes: from the allocator, or identity fallback if skipped
        let (n_i, n_b, n_t) = if self.did_alloc {
            (self.n_int, self.n_bool, self.n_table)
        } else {
            let mut max: RegId = 0;
            for b in &self.program.blocks {
                for i in &b.instrs { if let Some(d) = def_reg(i) { if d > max { max = d; } } }
            }
            (max as usize + 1, max as usize + 1, max as usize + 1)
        };

        for r in 0..n_i { out.push_str(&format!("    let mut i_r{} = 0i64;\n", r)); }
        for r in 0..n_b { out.push_str(&format!("    let mut b_r{} = false;\n", r)); }
        for r in 0..n_t {
            out.push_str(&format!("    let mut t_r{}: *mut Table = std::ptr::null_mut();\n", r));
            if fast_phys.contains(&(r as RegId)) {
                out.push_str(&format!("    let mut p_r{}: *mut i64 = std::ptr::null_mut();\n", r));
                out.push_str(&format!("    let mut len_r{} = 0usize;\n", r));
            }
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
