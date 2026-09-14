// src/register_alloc.rs
use crate::ir::{IrProgram, Instruction, Terminator, BasicBlock, RegId};
use std::collections::{HashMap, HashSet};
use crate::ast::StaticType;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Pool { Int, Float, Bool, String, Table, TableFloat, TableString, TableBool }

fn pool_of(t: &StaticType) -> Pool {
    match t {
        StaticType::Integer => Pool::Int,
        StaticType::Float => Pool::Float,
        StaticType::Boolean => Pool::Bool,
        StaticType::String => Pool::String,
        // A table whose ELEMENTS are floats is its own pool: its pointer
        // pair is *mut f64/farray, and a physical id must never serve
        // both it and a handle-array table — the per-id pointer decl and
        // EC/HR field choice would be ambiguous (found by the Float
        // Gauntlet: fa_tide's slot reused by fd_grid; pinned by
        // gauntlet_float + float_14). String-element and bool-element
        // tables get the same surgery for the same reason: *mut
        // String/sarray, *mut bool/barray.
        StaticType::Table(inner) if matches!(**inner, StaticType::Float) => Pool::TableFloat,
        StaticType::Table(inner) if matches!(**inner, StaticType::String) => Pool::TableString,
        StaticType::Table(inner) if matches!(**inner, StaticType::Boolean) => Pool::TableBool,
        StaticType::Table(_) | StaticType::UnknownTable(_) => Pool::Table,
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
                if let Some(d) = i.def_reg() { live.remove(&d); }
                for u in i.use_regs() { live.insert(u); }
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
            if let Some(d) = instr.def_reg() { live.remove(&d); }
            for u in instr.use_regs() { live.insert(u); }
            for &r in live.iter() { touch(&mut iv, r, base + i); }
            if let Some(d) = instr.def_reg() { touch(&mut iv, d, base + i); }
        }
        base += n + 1; // one slot for the terminator
    }
    iv
}

pub struct AllocInfo {
    pub n_int: usize, pub n_bool: usize, pub n_float: usize, pub n_str: usize,
    pub n_table: usize, pub n_ftable: usize, pub n_tstr: usize, pub n_btable: usize,
    pub phys_base: RegId, pub float_base: RegId, pub ftable_base: RegId,
    pub tstr_base: RegId, pub btable_base: RegId
}

pub fn allocate_registers(program: &mut IrProgram, consts_i: &HashMap<RegId, i64>, consts_b: &HashMap<RegId, bool>) -> AllocInfo {
    let blocks = &program.blocks;

    // Const vregs never get a physical slot: all their uses render as
    // literals. Excluded so a vreg id can never be confused with a
    // physical id at codegen time.
    let skip: HashSet<RegId> = consts_i.keys().copied()
        .chain(consts_b.keys().copied()).collect();

    // 1. types
    // The arithmetic ops are untyped in the IR: their targets inherit
    // the operand pool (Integer or Float) — pass 2 fixpoints that
    // through chains (`y = x + 1` needs x's pool first).
    let mut ty: HashMap<RegId, Pool> = HashMap::new();
    for b in blocks {
        for i in &b.instrs {
            let (Some(d), Some(t)) = (i.def_reg(), i.def_type()) else { continue };
            if skip.contains(&d) { continue; }
            if matches!(i, Instruction::Add { .. } | Instruction::Sub { .. }
                | Instruction::Mul { .. } | Instruction::Div { .. }
                | Instruction::IntDiv { .. } | Instruction::Mod { .. }
                | Instruction::Neg { .. }) { continue; }
            let p = pool_of(&t);
            match ty.insert(d, p) {
                Some(old) if old != p => panic!("reg {d} has conflicting types"),
                _ => {}
            }
        }
    }
    loop {
        let mut changed = false;
        for b in blocks {
            for i in &b.instrs {
                let (target, src_pools) = match i {
                    Instruction::Add { target, left, right, .. }
                    | Instruction::Sub { target, left, right, .. }
                    | Instruction::Mul { target, left, right, .. }
                    | Instruction::Div { target, left, right, .. }
                    | Instruction::IntDiv { target, left, right, .. }
                    | Instruction::Mod { target, left, right, .. } =>
                        (*target, [ty.get(left).copied(), ty.get(right).copied()]),
                    Instruction::Neg { target, source, .. } =>
                        (*target, [ty.get(source).copied(), None]),
                    _ => continue,
                };
                if skip.contains(&target) || ty.contains_key(&target) { continue; }
                if let Some(p) = src_pools.into_iter().flatten().next() {
                    ty.insert(target, p);
                    changed = true;
                }
            }
        }
        if !changed { break; }
    }
    for b in blocks {
        for i in &b.instrs {
            let ops: Vec<(RegId, StaticType)> = match i {
                Instruction::SetTable { table, key, val, ty } | Instruction::SetTableFast { table, key, val, ty } =>
                    vec![(*table, StaticType::Table(Box::new(StaticType::Integer))), (*key, StaticType::Integer), (*val, ty.clone())],
                Instruction::GetTable { table, key, .. } | Instruction::GetTableFast { table, key, .. } =>
                    vec![(*table, StaticType::Table(Box::new(StaticType::Integer))), (*key, StaticType::Integer)],
                Instruction::Add { left, right, .. } | Instruction::Sub { left, right, .. }
                | Instruction::Mul { left, right, .. } | Instruction::Div { left, right, .. }
                | Instruction::IntDiv { left, right, .. } | Instruction::Mod { left, right, .. }
                | Instruction::Less { left, right, .. }
                | Instruction::Leq { left, right, .. } | Instruction::Geq { left, right, .. } =>
                    vec![(*left, StaticType::Integer), (*right, StaticType::Integer)],
                // Eq's operands are polymorphic: the instruction's ty is
                // the only reliable pool source (Int and Bool physicals
                // share one id range, so nothing else disambiguates).
                Instruction::Eq { left, right, ty, .. } =>
                    vec![(*left, ty.clone()), (*right, ty.clone())],
                // Concat is monomorphic String — no ambiguity to carry.
                Instruction::Concat { left, right, .. } =>
                    vec![(*left, StaticType::String), (*right, StaticType::String)],
                Instruction::Neg { source, .. } => vec![(*source, StaticType::Integer)],
                Instruction::Not { source, .. } => vec![(*source, StaticType::Boolean)],
                Instruction::Move { source, ty: t, .. } => vec![(*source, t.clone())],
                Instruction::EnsureCapacity { table, limit } =>
                    vec![(*table, StaticType::Table(Box::new(StaticType::Integer))), (*limit, StaticType::Integer)],
                Instruction::HoistRawPtr { table } =>
                    vec![(*table, StaticType::Table(Box::new(StaticType::Integer)))],
                // The probe's operand list IS its kind carrier. Without
                // this arm the operands never enter the ty map, never
                // get physical ids, and emission renders undeclared
                // registers — the polymorphic-operand rule, enforced.
                Instruction::DebugProbe { operands, .. } =>
                    operands.iter().map(|(r, t)| (*r, t.clone())).collect(),
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
            if let Some(d) = i.def_reg() { if d > max_reg { max_reg = d; } }
            for u in i.use_regs() { if u > max_reg { max_reg = u; } }
        }
        if let Some(Terminator::Branch { cond, .. }) = &b.terminator {
            if *cond > max_reg { max_reg = *cond; }
        }
    }
    // Mint from above the ENTIRE id space the emission-side const maps
    // can ever be queried with: surviving IR vregs AND consts keys.
    // propagate_constants runs before simplify's DCE, so a folded def
    // can be deleted from the IR while its consts_i/consts_b entry
    // lives on — that id appears in no instruction (invisible to the
    // scan above) yet the maps still answer for it. A physical minted
    // onto such a stale key makes emit_instr's const early-out silently
    // swallow the instruction (a GetTable vanishes wholesale) and
    // iop_str render the dead constant at every use. Found by the
    // differential fuzzer, seed 43: `local v12 = v8` DCE'd, its folded
    // consts_i key numerically equaled the physical minted for a later
    // GetTable target, and the probe printed the stale 0 instead of 26.
    let max_const = consts_i.keys().copied()
        .chain(consts_b.keys().copied())
        .max()
        .unwrap_or(0);
    let base = max_reg.max(max_const) + 1;

    let mut vregs: Vec<RegId> = ty.keys().copied().collect();

    // Int/Bool/Table/String scalars deliberately SHARE the physical id
    // space (i_r12, b_r12, t_r12, s_r12 coexist — the prefix
    // disambiguates; that layout is what every integer lock freezes).
    // Float scalars mint from a disjoint range (emission asks "is reg
    // N float?" from N alone), float-ELEMENT tables from a second
    // disjoint range on top, string-ELEMENT tables from a third,
    // bool-ELEMENT tables from a fourth: the per-id pointer decls and
    // EC/HR field choices (*mut f64/farray, *mut String/sarray, *mut
    // bool/barray) must never serve the wrong table kind.
    // Pure-integer programs mint none of these: integer ids, integer
    // bytes, byte-for-byte.
    let pool_count = |p: Pool| ty.values().filter(|&&q| q == p).count();
    let max_other = pool_count(Pool::Int)
        .max(pool_count(Pool::Bool))
        .max(pool_count(Pool::Table))
        .max(pool_count(Pool::String));
    let float_base = base + max_other as RegId;
    let ftable_base = float_base + pool_count(Pool::Float) as RegId;
    let tstr_base = ftable_base + pool_count(Pool::TableFloat) as RegId;
    let btable_base = tstr_base + pool_count(Pool::TableString) as RegId;

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
            let n = *c; *c += 1;
            match p {
                Pool::Float => float_base + n as RegId,
                Pool::TableFloat => ftable_base + n as RegId,
                Pool::TableString => tstr_base + n as RegId,
                Pool::TableBool => btable_base + n as RegId,
                _ => base + n as RegId,
            }
        });
        act.push((phys, end));
        map.insert(r, phys);
    }

    // 4. rewrite references (const vregs stay identity: codegen looks
    //    them up in the const maps and never emits them)
    for b in &mut program.blocks {
        for i in &mut b.instrs { i.remap_instr(&|r| *map.get(&r).unwrap_or(&r)); }
        if let Some(Terminator::Branch { cond, .. }) = &mut b.terminator {
            if let Some(&p) = map.get(&*cond) { *cond = p; }
        }
    }

    // 5. self-copies are no-ops
    for b in &mut program.blocks {
        b.instrs.retain(|i|
            !matches!(i, Instruction::Move { target, source, .. } if target == source));
    }

    AllocInfo {
        n_int: *count.entry(Pool::Int).or_insert(0),
        n_bool: *count.entry(Pool::Bool).or_insert(0),
        n_float: *count.entry(Pool::Float).or_insert(0),
        n_str: *count.entry(Pool::String).or_insert(0),
        n_table: *count.entry(Pool::Table).or_insert(0),
        n_ftable: *count.entry(Pool::TableFloat).or_insert(0),
        n_tstr: *count.entry(Pool::TableString).or_insert(0),
        n_btable: *count.entry(Pool::TableBool).or_insert(0),
        phys_base: base,
        float_base,
        ftable_base,
        tstr_base,
        btable_base,
    }
}


