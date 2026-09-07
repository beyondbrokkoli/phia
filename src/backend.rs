// src/backend.rs
use crate::ir::{IrProgram, Instruction, Terminator, BasicBlock, BlockId, RegId};
use std::collections::{HashMap, HashSet, BTreeSet};
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

fn resolve_via(map: &HashMap<RegId, RegId>, mut r: RegId) -> RegId {
    let mut guard = 0usize;
    while let Some(&next) = map.get(&r) {
        r = next;
        guard += 1;
        if guard > 1_000_000 { panic!("phi coalesce: rename cycle"); }
    }
    r
}

fn single_def(defs: &HashMap<RegId, usize>, r: RegId) -> bool {
    defs.get(&r).copied() == Some(1)
}

fn loop_region(blocks: &[BasicBlock], header: BlockId, body: BlockId) -> Vec<BlockId> {
    // Every block an iteration can execute: reachable from the body
    // without passing back through the header. Headers themselves are
    // excluded (they hold only phis + condition reads — never a SetTable).
    let mut region = Vec::new();
    let mut seen = HashSet::new();
    let mut stack = vec![body];
    while let Some(b) = stack.pop() {
        if b == header || !seen.insert(b) { continue; }
        region.push(b);
        match &blocks[b].terminator {
            Some(Terminator::Jump(t)) => stack.push(*t),
            Some(Terminator::Branch { true_block, false_block, .. }) => {
                stack.push(*true_block);
                stack.push(*false_block);
            }
            _ => {}
        }
    }
    region
}

fn indent(d: usize) -> String { "    ".repeat(d) }

pub struct IrBackend {
    pub program: IrProgram,
    n_int: usize, n_bool: usize, n_table: usize,
    phys_base: RegId, did_alloc: bool,
    consts_i: HashMap<RegId, i64>,
    consts_b: HashMap<RegId, bool>,
}

impl IrBackend {
    pub fn new(program: IrProgram) -> Self {
        Self {
            program,
            n_int: 0, n_bool: 0, n_table: 0,
            phys_base: 0, did_alloc: false,
            consts_i: HashMap::new(), consts_b: HashMap::new(),
        }
    }

    fn iop_str(&self, r: RegId) -> String {
        self.consts_i.get(&r).map(|v| v.to_string())
            .unwrap_or_else(|| format!("i_r{}", r))
    }
    fn bop_str(&self, r: RegId) -> String {
        self.consts_b.get(&r)
            .map(|v| if *v { "true" } else { "false" }.to_string())
            .unwrap_or_else(|| format!("b_r{}", r))
    }

    fn reg_uses(&self, r: RegId) -> usize {
        let mut n = 0;
        for b in &self.program.blocks {
            for i in &b.instrs { for u in use_regs(i) { if u == r { n += 1; } } }
            if let Some(Terminator::Branch { cond, .. }) = &b.terminator {
                if *cond == r { n += 1; }
            }
        }
        n
    }

    fn is_loop_header(&self, h: BlockId) -> bool {
        // A loop header has a back edge: a LATER block jumping to it.
        self.program.blocks[h + 1..].iter().any(|p| {
            matches!(&p.terminator, Some(Terminator::Jump(t)) if *t == h)
        })
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
                if let Some(Terminator::Branch { cond, .. }) = &mut b.terminator {
                    *cond = resolve(*cond);
                }
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

    pub fn propagate_constants(&mut self) {
        let mut defs: HashMap<RegId, usize> = HashMap::new();
        for b in &self.program.blocks {
            for i in &b.instrs {
                if let Some(d) = def_reg(i) { *defs.entry(d).or_insert(0) += 1; }
            }
        }

        let mut ci: HashMap<RegId, i64> = HashMap::new();
        let mut cb: HashMap<RegId, bool> = HashMap::new();

        // Fixpoint: entries are only ever added (single-def regs are
        // immutable), and block-id order matches dominance order here, so
        // this converges in ~2 sweeps.
        loop {
            let before = ci.len() + cb.len();
            for b in &self.program.blocks {
                for i in &b.instrs {
                    match i {
                        Instruction::LoadInt { target, val }
                            if single_def(&defs, *target) => { ci.insert(*target, *val); }
                        Instruction::Add { target, left, right }
                            if single_def(&defs, *target) => {
                            if let (Some(&l), Some(&r)) = (ci.get(left), ci.get(right)) {
                                ci.insert(*target, l.wrapping_add(r));
                            }
                        }
                        Instruction::Sub { target, left, right }
                            if single_def(&defs, *target) => {
                            if let (Some(&l), Some(&r)) = (ci.get(left), ci.get(right)) {
                                ci.insert(*target, l.wrapping_sub(r));
                            }
                        }
                        Instruction::Less { target, left, right }
                            if single_def(&defs, *target) => {
                            if let (Some(&l), Some(&r)) = (ci.get(left), ci.get(right)) {
                                cb.insert(*target, l < r);
                            }
                        }
                        Instruction::Move { target, source, ty }
                            if single_def(&defs, *target) => match ty {
                                StaticType::Integer =>
                                    { if let Some(&v) = ci.get(source) { ci.insert(*target, v); } }
                                StaticType::Boolean =>
                                    { if let Some(&v) = cb.get(source) { cb.insert(*target, v); } }
                                StaticType::Table => {}
                            },
                        _ => {}
                    }
                }
            }
            if ci.len() + cb.len() == before { break; }
        }
        self.consts_i = ci;
        self.consts_b = cb;
    }

    pub fn allocate_registers(&mut self) {
        let blocks = &self.program.blocks;

        // Const vregs never get a physical slot: all their uses render as
        // literals. Excluded so a vreg id can never be confused with a
        // physical id at codegen time.
        let skip: HashSet<RegId> = self.consts_i.keys().copied()
            .chain(self.consts_b.keys().copied()).collect();

        // 1. types
        let mut ty: HashMap<RegId, Pool> = HashMap::new();
        for b in blocks {
            for i in &b.instrs {
                if let (Some(d), Some(t)) = (def_reg(i), def_type(i)) {
                    if skip.contains(&d) { continue; }
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
                for (r, t) in ops {
                    if skip.contains(&r) { continue; }
                    ty.entry(r).or_insert(pool_of(&t));
                }
            }
            if let Some(Terminator::Branch { cond, .. }) = &b.terminator {
                if !skip.contains(cond) { ty.entry(*cond).or_insert(Pool::Bool); }
            }
        }

        // 2. liveness + intervals (unchanged)
        let (_, live_out) = compute_liveness(blocks);
        let iv = live_intervals(blocks, &live_out);

        // 3. mint physical ids from ABOVE the whole vreg namespace —
        //    a physical id must never equal a vreg id (const or not).
        let mut max_reg: RegId = 0;
        for b in blocks {
            for i in &b.instrs {
                if let Some(d) = def_reg(i) { if d > max_reg { max_reg = d; } }
                for u in use_regs(i) { if u > max_reg { max_reg = u; } }
            }
            if let Some(Terminator::Branch { cond, .. }) = &b.terminator {
                if *cond > max_reg { max_reg = *cond; }
            }
        }
        let base = max_reg + 1;

        let mut vregs: Vec<RegId> = ty.keys().copied().collect();

        // LOAD-BEARING: the `r` tiebreak makes this a total order. Without it,
        // equal-interval regs fall back to HashMap iteration order (random per
        // process) and allocation becomes non-deterministic.
        vregs.sort_by_key(|&r| (iv.get(&r).copied().unwrap_or((0, 0)), r));

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
                let n = *c; *c += 1; base + n as RegId
            });
            act.push((phys, end));
            map.insert(r, phys);
        }

        // 4. rewrite references (const vregs stay identity: codegen looks
        //    them up in the const maps and never emits them)
        for b in &mut self.program.blocks {
            for i in &mut b.instrs { remap_instr(i, &|r| *map.get(&r).unwrap_or(&r)); }
            if let Some(Terminator::Branch { cond, .. }) = &mut b.terminator {
                if let Some(&p) = map.get(&*cond) { *cond = p; }
            }
        }

        // 5. self-copies are no-ops
        for b in &mut self.program.blocks {
            b.instrs.retain(|i|
                !matches!(i, Instruction::Move { target, source, .. } if target == source));
        }

        self.n_int   = *count.entry(Pool::Int).or_insert(0);
        self.n_bool  = *count.entry(Pool::Bool).or_insert(0);
        self.n_table = *count.entry(Pool::Table).or_insert(0);
        self.phys_base = base;
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

        // Follows Integer Moves and Add/Sub-with-a-constant, folding the constant
        // offsets. SSA makes the chain acyclic: every operand is defined strictly
        // earlier, so the loop terminates. LoadInt keys, GetTable results, Phis,
        // and non-const arithmetic all return None. Uses `blocks` explicitly so
        // `self` is NOT locked (same pattern as get_table_root).
        let key_offset = |blocks: &[BasicBlock], key_reg: RegId, idx_reg: RegId| -> Option<i64> {
            // A register is a compile-time constant if it is a single LoadInt.
            // (def_map is all we have at optimize() time — consts_i doesn't
            // exist until propagate_constants, three passes later.)
            let const_of = |blocks: &[BasicBlock], r: RegId| -> Option<i64> {
                match def_map.get(&r) {
                    Some(&(b, i)) => match &blocks[b].instrs[i] {
                        Instruction::LoadInt { val, .. } => Some(*val),
                        _ => None,
                    },
                    None => None,
                }
            };
            let mut curr = key_reg;
            let mut off: i64 = 0;
            loop {
                if curr == idx_reg { return Some(off); }
                let &(b, i) = def_map.get(&curr)?;
                match &blocks[b].instrs[i] {
                    Instruction::Move { source, ty, .. } if *ty == StaticType::Integer => { curr = *source; }
                    Instruction::Add { left, right, .. } => {
                        if let Some(c) = const_of(blocks, *right) { off = off.wrapping_add(c); curr = *left; }
                        else if let Some(c) = const_of(blocks, *left) { off = off.wrapping_add(c); curr = *right; }
                        else { return None; }
                    }
                    Instruction::Sub { left, right, .. } => {
                        off = off.wrapping_sub(const_of(blocks, *right)?);
                        curr = *left;
                    }
                    _ => return None,
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
                            let mut hoists = BTreeSet::new(); // PATCH C: ROOT regs — one EC+HR pair per unique TABLE
                            // PATCH D: upgrades are now (block, index, instr) — they can land in any
                            // block of the loop region, not just the direct body.
                            let mut upgrades: Vec<(BlockId, usize, Instruction)> = Vec::new();

                            // PASS 1: Read-Only, REGION-WIDE. A dynamic SetTable anywhere in the
                            // loop's region (nested loop body, post-nested tail) can resize the
                            // table and invalidate a pre-header HoistRawPtr.
                            let mut region = loop_region(&self.program.blocks, header_id, body_id);
                            region.sort_unstable(); // PATCH D tidiness: region scanned in block-id order

                            let mut abort_all = false;
                            'poison: for &blk in &region {
                                for instr in &self.program.blocks[blk].instrs {
                                    if let Instruction::SetTable { table, key, .. } = instr {
                                        // PATCH B: unsafe == "not provably equal to the induction
                                        // value" — that is BOTH None (untraceable) AND Some(off != 0)
                                        // (provably different). `.is_none()` here would let `i + 300`
                                        // writes escape the poison scan: the single most dangerous
                                        // way to write this line. PASS 1 must stay the exact
                                        // negation of PASS 2's predicate.
                                        if key_offset(&self.program.blocks, *key, idx_reg) != Some(0) {
                                            match get_table_root(&self.program.blocks, *table) {
                                                Some(root) => { clobbered_roots.insert(root); }
                                                None => { abort_all = true; break 'poison; }
                                            }
                                        }
                                    }
                                }
                            }


                            if abort_all { continue; } // Safety valve: bail out!

                            // PASS 2: Read-Only. Determine which instructions to upgrade.
                            // PATCH D: scan the WHOLE loop region (from Patch A's loop_region), not
                            // just the direct body block. Soundness, per access:
                            //   * KEY:   key_offset folding to Some(0) proves the operand holds the
                            //     iteration-ENTRY induction value — the exact register the header's
                            //     Less proved < limit. SSA fixes that value for the entire iteration:
                            //     it cannot change anywhere in the region without minting a new vreg,
                            //     which breaks the trace and auto-declines. The register IS the proof;
                            //     the access's position in the region is irrelevant. (Patch B's +0
                            //     keys are covered: `φi + 0` folds to Some(0) and holds that value.)
                            //   * TABLE: def block < header (unchanged S2 check).
                            //   * POINTER: PASS 1 already poisoned any root with an unsafe-key write
                            //     (key_offset != Some(0)) anywhere in the region — which is why Patch A
                            //     had to land first, and why PASS 1 must NEVER be weakened to .is_none().
                            // Headers are excluded by loop_region: they hold only phis + condition
                            // lowering, never a SetTable, and a condition GetTable's key structurally
                            // cannot trace to idx_reg (idx_reg IS its result — see gauntlet_pD).
                            // Instructions already upgraded to *Fast by an earlier (outer) pass match
                            // neither arm here, so no double-upgrade is possible.
                            for &blk in &region {
                                for (i, instr) in self.program.blocks[blk].instrs.iter().enumerate() {
                                    match instr {
                                        Instruction::SetTable { table, key, val } => {
                                            // S2 DOMINANCE CHECK: Only hoist pointers defined BEFORE the loop.
                                            let (def_b, _) = def_map.get(table).unwrap_or(&(0,0));
                                            if *def_b >= header_id { continue; }

                                            if let Some(root) = get_table_root(&self.program.blocks, *table) {
                                                if key_offset(&self.program.blocks, *key, idx_reg) == Some(0) && !clobbered_roots.contains(&root) {
                                                    // PATCH C: emit the fast op against the ROOT reg and key
                                                    // the hoist set by root. Aliased regs (b = a) share one
                                                    // table, so they share one EC + one HR; every upgraded
                                                    // access points at the root, and the now-unused alias
                                                    // Moves die in simplify().
                                                    upgrades.push((blk, i, Instruction::SetTableFast { table: root, key: *key, val: *val }));
                                                    hoists.insert(root);
                                                }
                                            }
                                        }
                                        Instruction::GetTable { target, table, key } => {
                                            // S2 DOMINANCE CHECK: Only hoist pointers defined BEFORE the loop.
                                            let (def_b, _) = def_map.get(table).unwrap_or(&(0,0));
                                            if *def_b >= header_id { continue; }

                                            if let Some(root) = get_table_root(&self.program.blocks, *table) {
                                                if key_offset(&self.program.blocks, *key, idx_reg) == Some(0) && !clobbered_roots.contains(&root) {
                                                    upgrades.push((blk, i, Instruction::GetTableFast { target: *target, table: root, key: *key }));
                                                    hoists.insert(root);
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }

                            // PASS 3: Mutate!
                            for (blk, i, new_instr) in upgrades {
                                self.program.blocks[blk].instrs[i] = new_instr;
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
        // 0. def counts (a Phi counts as one def of its target)
        let mut defs: HashMap<RegId, usize> = HashMap::new();
        for b in &self.program.blocks {
            for i in &b.instrs {
                if let Some(d) = def_reg(i) { *defs.entry(d).or_insert(0) += 1; }
            }
        }

        // 1. gather phis; recognize the loop-header shape: one pred before
        //    the header (preheader), one after (back edge)
        let mut phis: Vec<(RegId, StaticType, Vec<(BlockId, RegId)>,
                            Option<((BlockId, RegId), (BlockId, RegId))>)> = Vec::new();
        for b in &self.program.blocks {
            for i in &b.instrs {
                if let Instruction::Phi { target, ty, args } = i {
                    let shape = if args.len() == 2 && (args[0].0 < b.id) != (args[1].0 < b.id) {
                        Some(if args[0].0 < b.id { (args[0], args[1]) } else { (args[1], args[0]) })
                    } else { None };
                    phis.push((*target, ty.clone(), args.clone(), shape));
                }
            }
        }

        // 2. coalesce phi target -> back arg (single-def only):
        //    * the back arg's def is the ONLY write to that slot inside the
        //      loop, so at the back edge the slot holds exactly what the phi
        //      would have merged;
        //    * every read of the phi precedes the back arg's def in program
        //      order (the assignment that creates the back arg redirects all
        //      later reads to newer regs), so those reads see the slot's
        //      previous-iteration value — which IS the phi's value;
        //    * one initializing Move in the preheader covers first entry and
        //      zero-iteration paths.
        //    Injected Move targets are always body-defined regs; sources are
        //    always pre-loop regs — disjoint by construction (one def per
        //    reg), so two injected Moves can never alias and the classic
        //    phi-swap problem cannot arise.
        let mut rename: HashMap<RegId, RegId> = HashMap::new();
        let mut injects: Vec<(BlockId, RegId, RegId, StaticType)> = Vec::new();

        for (target, ty, args, shape) in &phis {
            let mut done = false;
            if let Some((pre, back)) = shape {
                let b_res = resolve_via(&rename, back.1);
                if single_def(&defs, back.1) && b_res != *target {
                    rename.insert(*target, b_res);
                    injects.push((pre.0, back.1, pre.1, ty.clone()));
                    done = true;
                }
            }
            if !done {
                // plain phi: one Move per incoming edge (the classic lowering)
                for (pred, src) in args {
                    injects.push((*pred, *target, *src, ty.clone()));
                }
            }
        }

        // 3. inject raw (renames are applied afterwards, so chained phis —
        //    same variable phi'd at nested loop levels — compose correctly)
        for (blk, tgt, src, ty) in injects {
            self.program.blocks[blk].instrs
                .push(Instruction::Move { target: tgt, source: src, ty });
        }
        for b in &mut self.program.blocks {
            b.instrs.retain(|i| !matches!(i, Instruction::Phi { .. }));
        }
        if !rename.is_empty() {
            for b in &mut self.program.blocks {
                for i in &mut b.instrs {
                    remap_instr(i, &|r| resolve_via(&rename, r));
                }
                // A register id can appear in exactly two places: instruction
                // operands and the Branch condition. `while flag do` lowers
                // the condition to the phi itself — no Less in between — so
                // the terminator MUST be renamed too, or it points at a reg
                // that nothing ever writes.
                if let Some(Terminator::Branch { cond, .. }) = &mut b.terminator {
                    *cond = resolve_via(&rename, *cond);
                }
            }
        }
        // chained coalesces can turn injected Moves into self-copies
        for b in &mut self.program.blocks {
            b.instrs.retain(|i|
                !matches!(i, Instruction::Move { target, source, .. } if target == source));
        }
    }

    fn emit_instr(&self, out: &mut String, instr: &Instruction, d: usize) {
        // compile-time-computed defs emit nothing: their uses are literals
        if let Some(t) = def_reg(instr) {
            if self.consts_i.contains_key(&t) || self.consts_b.contains_key(&t) {
                return;
            }
        }
        let ind = indent(d);
        match instr {
            Instruction::LoadInt { target, val } =>
                out.push_str(&format!("{ind}i_r{target} = {val};\n")),
            Instruction::NewTable { target } => out.push_str(&format!(
                "{ind}let mut new_table = Box::new(Table::new());\n\
                 {ind}t_r{target} = &mut *new_table as *mut Table;\n\
                 {ind}tables.push(new_table);\n"
            )),
            Instruction::Move { target, source, ty } => match ty {
                StaticType::Integer => out.push_str(&format!("{ind}i_r{target} = {};\n", self.iop_str(*source))),
                StaticType::Boolean => out.push_str(&format!("{ind}b_r{target} = {};\n", self.bop_str(*source))),
                StaticType::Table => out.push_str(&format!("{ind}t_r{target} = t_r{source};\n")),
            },
            Instruction::Add { target, left, right } =>
                out.push_str(&format!("{ind}i_r{target} = {} + {};\n", self.iop_str(*left), self.iop_str(*right))),
            Instruction::Sub { target, left, right } =>
                out.push_str(&format!("{ind}i_r{target} = {} - {};\n", self.iop_str(*left), self.iop_str(*right))),
            Instruction::Less { target, left, right } =>
                out.push_str(&format!("{ind}b_r{target} = {} < {};\n", self.iop_str(*left), self.iop_str(*right))),

            Instruction::EnsureCapacity { table, limit } => out.push_str(&format!(
                "{ind}let lim = {lim};\n\
                 {ind}if lim > 0 {{\n\
                 {ind}    let t = unsafe {{ &mut *t_r{table} }};\n\
                 {ind}    if (lim as usize) > t.array.len() {{\n\
                 {ind}        t.array.resize(lim as usize, 0);\n\
                 {ind}    }}\n\
                 {ind}}}\n",
                lim = self.iop_str(*limit)
            )),
            Instruction::HoistRawPtr { table } => out.push_str(&format!(
                "{ind}len_r{table} = unsafe {{ (*t_r{table}).array.len() }};\n\
                 {ind}p_r{table} = unsafe {{ (*t_r{table}).array.as_mut_ptr() }};\n"
            )),

            Instruction::SetTable { table, key, val } => out.push_str(&format!(
                "{ind}let k = {key};\n\
                 {ind}if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                 {ind}let idx = k as usize;\n\
                 {ind}let t = unsafe {{ &mut *t_r{table} }};\n\
                 {ind}if idx >= t.array.len() {{ t.array.resize(idx + 1, 0); }}\n\
                 {ind}unsafe {{ *t.array.get_unchecked_mut(idx) = {val}; }}\n",
                key = self.iop_str(*key), val = self.iop_str(*val)
            )),
            Instruction::GetTable { target, table, key } => out.push_str(&format!(
                "{ind}let k = {key};\n\
                 {ind}if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                 {ind}let idx = k as usize;\n\
                 {ind}let t = unsafe {{ &*t_r{table} }};\n\
                 {ind}i_r{target} = if idx < t.array.len() {{ unsafe {{ *t.array.get_unchecked(idx) }} }} else {{ 0 }};\n",
                key = self.iop_str(*key)
            )),
            Instruction::SetTableFast { table, key, val } => out.push_str(&format!(
                "{ind}let k = {key};\n\
                 {ind}if k < 0 {{ panic!(\"Runtime Error: Negative index in fast path\"); }}\n\
                 {ind}if (k as usize) < len_r{table} {{\n\
                 {ind}    unsafe {{ *p_r{table}.add(k as usize) = {val}; }}\n\
                 {ind}}} else {{\n\
                 {ind}    panic!(\"optimizer invariant violated: fast-path bounds check failed\");\n\
                 {ind}}}\n",
                key = self.iop_str(*key), val = self.iop_str(*val)
            )),
            Instruction::GetTableFast { target, table, key } => out.push_str(&format!(
                "{ind}let k = {key};\n\
                 {ind}if k < 0 {{ panic!(\"Runtime Error: Negative index in fast path\"); }}\n\
                 {ind}if (k as usize) < len_r{table} {{\n\
                 {ind}    i_r{target} = unsafe {{ *p_r{table}.add(k as usize) }};\n\
                 {ind}}} else {{\n\
                 {ind}    panic!(\"optimizer invariant violated: fast-path bounds check failed\");\n\
                 {ind}}}\n",
                key = self.iop_str(*key)
            )),
            Instruction::Phi { .. } => {} // deleted by resolve_phis
        }
    }

    /// Emit block `b` and everything that follows it, staying inside the loop
    /// whose header is `hdr` (a back edge to `hdr` closes the loop body).
    fn emit_seq(&self, out: &mut String, b: BlockId, hdr: Option<BlockId>, d: usize, emitted: &mut [bool]) {
        if emitted[b] { panic!("structured codegen: block {b} reached twice — CFG is not a tree"); }
        emitted[b] = true;
        let block = &self.program.blocks[b];
        for i in &block.instrs { self.emit_instr(out, i, d); }
        match &block.terminator {
            None | Some(Terminator::Halt) => {
                // early return == the dispatcher's `break 'cfg`: there is no
                // code after Halt, so jumping to the end is exactly a return
                out.push_str(&format!("{}return tables;\n", indent(d)));
            }
            Some(Terminator::Jump(t)) => {
                if Some(*t) == hdr {
                    // back edge: this loop body is complete
                } else if *t < b {
                    panic!("structured codegen: stray backward jump {b} -> {t}");
                } else if self.is_loop_header(*t) {
                    // forward jump into a loop header = entering a loop
                    let (cond, tb, fb) = match &self.program.blocks[*t].terminator {
                        Some(Terminator::Branch { cond, true_block, false_block }) =>
                            (*cond, *true_block, *false_block),
                        _ => panic!("structured codegen: block {t} has a back edge but no Branch"),
                    };
                    self.emit_loop(out, *t, cond, tb, d, emitted);
                    self.emit_seq(out, fb, hdr, d, emitted);
                } else {
                    self.emit_seq(out, *t, hdr, d, emitted);
                }
            }
            Some(Terminator::Branch { cond, true_block, false_block }) => {
                // unreachable in practice (headers are entered via Jump), kept
                // for totality — same handling as the Jump-into-header case
                self.emit_loop(out, b, *cond, *true_block, d, emitted);
                self.emit_seq(out, *false_block, hdr, d, emitted);
            }
        }
    }

    fn emit_loop(&self, out: &mut String, h: BlockId, cond: RegId, body: BlockId, d: usize, emitted: &mut [bool]) {
        if !self.is_loop_header(h) {
            panic!("structured codegen: Branch in block {h} is not a loop header — `if` is not supported by this codegen");
        }
        if emitted[h] { panic!("structured codegen: header {h} reached twice"); }
        emitted[h] = true;

        let block = &self.program.blocks[h];
        let ind = indent(d);

        // Pretty form: the header holds nothing (identifier condition, e.g.
        // phase I / bug16a) or exactly the Less computing the branch
        // condition with no other readers of its result. The Less folds into
        // the while-condition — still evaluated every iteration.
        let pretty = if block.instrs.is_empty() {
            Some(self.bop_str(cond))
        } else if block.instrs.len() == 1 {
            match &block.instrs[0] {
                Instruction::Less { target, left, right }
                    if *target == cond && self.reg_uses(cond) == 1 =>
                    Some(format!("{} < {}", self.iop_str(*left), self.iop_str(*right))),
                _ => None,
            }
        } else { None };

        if let Some(c) = pretty {
            out.push_str(&format!("{ind}while {c} {{\n"));
            self.emit_seq(out, body, Some(h), d + 1, emitted);
            out.push_str(&format!("{ind}}}\n"));
        } else {
            // General fallback: everything in the header runs every
            // iteration. Never fires on the current corpus — it exists so a
            // surprising CFG degrades to correct-but-ugly, not wrong.
            out.push_str(&format!("{ind}loop {{\n"));
            for i in &block.instrs { self.emit_instr(out, i, d + 1); }
            out.push_str(&format!("{}if {} {{\n", indent(d + 1), self.bop_str(cond)));
            self.emit_seq(out, body, Some(h), d + 2, emitted);
            out.push_str(&format!("{}    }} else {{\n", indent(d + 1)));
            out.push_str(&format!("{}        break;\n", indent(d + 1)));
            out.push_str(&format!("{}    }}\n{ind}}}\n", indent(d + 1)));
        }
    }

    pub fn generate_rust_code(&self) -> String {
        let mut out = String::new();
        out.push_str("// target/release/build/phia-*/out/baked_native.rs\n\n");
        out.push_str("use crate::memory::Table;\n\n");
        out.push_str("#[allow(unused_variables, unused_mut, unused_assignments)]\n");
        out.push_str("pub fn run_baked() -> Vec<Box<Table>> {\n");

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

        let (n_i, n_b, n_t, base) = if self.did_alloc {
            (self.n_int, self.n_bool, self.n_table, self.phys_base as usize)
        } else {
            let mut max: RegId = 0;
            for b in &self.program.blocks {
                for i in &b.instrs { if let Some(dd) = def_reg(i) { if dd > max { max = dd; } } }
            }
            (max as usize + 1, max as usize + 1, max as usize + 1, 0usize)
        };

        for r in base..base + n_i { out.push_str(&format!("    let mut i_r{r} = 0i64;\n")); }
        for r in base..base + n_b { out.push_str(&format!("    let mut b_r{r} = false;\n")); }
        for r in base..base + n_t {
            out.push_str(&format!("    let mut t_r{r}: *mut Table = std::ptr::null_mut();\n"));
            if fast_phys.contains(&(r as RegId)) {
                out.push_str(&format!("    let mut p_r{r}: *mut i64 = std::ptr::null_mut();\n"));
                out.push_str(&format!("    let mut len_r{r} = 0usize;\n"));
            }
        }
        out.push_str("    let mut tables = Vec::<Box<Table>>::with_capacity(128);\n\n");

        let mut emitted = vec![false; self.program.blocks.len()];
        self.emit_seq(&mut out, 0, None, 1, &mut emitted);
        let orphans: Vec<usize> = emitted.iter().enumerate()
            .filter(|(_, e)| !**e).map(|(i, _)| i).collect();
        if !orphans.is_empty() {
            panic!("structured codegen: blocks never reached: {orphans:?}");
        }

        out.push_str("}\n");
        out
    }
}
