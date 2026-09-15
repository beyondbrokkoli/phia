// src/register_alloc.rs
use crate::ir::{IrProgram, Instruction, Terminator, BasicBlock, RegId, CONST_REG_BASE, is_const_reg};
use std::collections::{HashMap, HashSet};
use crate::ast::StaticType;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Pool { Int, Float, Bool, String, Table, TableFloat, TableString, TableBool }

/// StaticType -> Pool. THE pool oracle — every "which storage side?"
/// question routes through here (or through the disjoint-range
/// predicates that mirror these pools in backend.rs).
///
/// Adding an element kind (the bool patch is the reference, 2026-09-15):
/// ```text
///  1. type_checker.rs : resolve_static arm; check_cond/Not Var arms if the
///     kind is Boolean-like; BinaryOp Var arms are OP-GUARDED (== / ~= only)
///     so arith on the deferred kind still falls through to the rejections.
///  2. memory.rs       : storage side + is_* flag + constructor.
///  3. reg_alloc.rs    : pool_of arm; disjoint base range; AllocInfo field;
///     mint arm; decl-range comment.
///  4. backend.rs      : is_*table_reg predicate; pool_prefixed; decl
///     ptr_ty; NewTable (handle AND pointer mode); SetTable (both modes);
///     GetTable (handle tuple arm + pointer branch); SetTableFast;
///     GetTableFast; EnsureCapacity (fld, zero); HoistRawPtr (fld);
///     DebugProbe (fld token).
///  5. main.rs         : dump branch (order: is_string -> is_float -> is_bool
///     -> int; flags mutually exclusive by NewTable).
///  6. Corpus          : positive dyn / fast / handle-mode / deferred-bind
///     tests + negative element-conflict test; list in tests/listing.lua
///     (single source — boss hygiene and lockdown both read it; a NEW
///     test needs listing.lua + a relock for its lock).
///  7. Docs            : subset.lua AND its README copy must stay in sync
///     (they are two copies of one file, by design).
/// ```
/// A missed backend arm is almost always loud (generated Rust fails to
/// compile, or a fast-path bounds panic) — the quiet corner is a wrong
/// `fld` that still compiles; check EC/HR agreement first.
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
    pub int_base: RegId, pub bool_base: RegId, pub table_base: RegId,
    pub str_base: RegId, pub float_base: RegId, pub ftable_base: RegId,
    pub tstr_base: RegId, pub btable_base: RegId
}

pub fn allocate_registers(
    program: &mut IrProgram,
    consts_i: &HashMap<RegId, i64>,
    consts_b: &HashMap<RegId, bool>,
    consts_f: &HashMap<RegId, f64>,
    consts_s: &HashMap<RegId, String>,
) -> AllocInfo {
    let blocks = &program.blocks;

    // Const vregs never get a physical slot: all their uses render as
    // literals. Excluded so a vreg id can never be confused with a
    // physical id at codegen time. All FOUR scalar kinds — the skip set
    // is exactly layer-B membership (constitution law 2); the base scan
    // below is already layer-guarded, so nothing else changes.
    let skip: HashSet<RegId> = consts_i.keys().copied()
        .chain(consts_b.keys().copied())
        .chain(consts_f.keys().copied())
        .chain(consts_s.keys().copied())
        .collect();

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
        // An arith operand that const-folded carries its pool in the
        // CONST MAP, not in ty (skip excludes it). A folded operand is
        // invisible to the fixpoint below — and an arith target whose
        // operands are ALL const while its own result stayed runtime
        // (the is_finite guard declining inf/NaN, or a multi-def slot)
        // would otherwise mint NO physical at all and render as an
        // undeclared register (feat_ops_10: `inf = a / b` after consts_f).
        // Const bools/strings can never be arith operands (the checker
        // rejects them), so ci/cf membership is a complete answer.
        let const_pool = |r: RegId| {
            if consts_f.contains_key(&r) { Some(Pool::Float) }
            else if consts_i.contains_key(&r) { Some(Pool::Int) }
            else { None }
        };
        for b in blocks {
            for i in &b.instrs {
                let (target, src_pools) = match i {
                    Instruction::Add { target, left, right, .. }
                    | Instruction::Sub { target, left, right, .. }
                    | Instruction::Mul { target, left, right, .. }
                    | Instruction::Div { target, left, right, .. }
                    | Instruction::IntDiv { target, left, right, .. }
                    | Instruction::Mod { target, left, right, .. } =>
                        (*target, [
                            ty.get(left).copied().or_else(|| const_pool(*left)),
                            ty.get(right).copied().or_else(|| const_pool(*right)),
                        ]),
                    Instruction::Neg { target, source, .. } =>
                        (*target, [ty.get(source).copied().or_else(|| const_pool(*source)), None]),
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
                // Eq's operands are polymorphic and the fold (which runs
                // pre-alloc, where ids carry no pool ranges) needs the
                // instruction's ty as its kind source here.
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
                // The probe's operand list IS its kind carrier for this
                // pre-mint typing pass (ids answer pool questions only
                // AFTER the ranges exist). Without this arm the operands
                // never enter the ty map, never get physical ids, and
                // emission renders undeclared registers.
                Instruction::DebugProbe { operands, .. } =>
                    operands.iter().map(|(r, t)| (*r, t.clone())).collect(),
                _ => vec![],
            };
            for (r, t) in ops {
                if skip.contains(&r) { continue; }
                ty.entry(r).or_insert(pool_of(&t));
            }
        }
        if let Some(Terminator::Branch { cond, .. }) = &b.terminator
            && !skip.contains(cond) { ty.entry(*cond).or_insert(Pool::Bool); }
    }

    // 2. liveness + intervals (unchanged)
    let (_, live_out) = compute_liveness(blocks);
    let iv = live_intervals(blocks, &live_out);

    // 3. mint physical ids from ZERO — the scars of the vreg namespace
    //    are gone (the old base was max-vreg+1, so every program's
    //    physicals carried an arbitrary offset). Layer C does not sit
    //    ABOVE layer A; it REPLACES it: the remap in step 4 rewrites
    //    every def, use and branch cond through `map`, erasing the
    //    vreg namespace from the IR in the same pass that mints the
    //    physicals. The numeric overlap of [0, V) and [0, P) is
    //    therefore unobservable — the only cross-layer keys still
    //    alive at emission are the CONST maps, and layer B is the
    //    reserved high range, disjoint from every physical by
    //    construction. (Seed-43/fuzzer_01 is the cautionary tale for
    //    a map outliving its namespace; vregs have no map.)
    //    Consts never get a physical slot: all their uses render as
    //    literals (the skip set is exactly layer-B membership,
    //    constitution law 2).

    // ONE GLOBAL TIMELINE of physical ids: eight consecutive per-pool
    // ranges, minted in fixed order — int, bool, table, string, float,
    // ftable, tstr, btable. Every physical id belongs to exactly one
    // pool and no two physical registers ever share a number (the
    // SSA spirit carried into the physical namespace): `i_r12` and
    // `b_r12` can no longer coexist, so pool membership is a pure
    // range check and nothing downstream needs a carried StaticType
    // to disambiguate a scalar operand. The old layout co-numbered
    // Int/Bool/Table/String from one shared range — every
    // "which pool is this id?" hazard (Eq's ty, probe operand kinds,
    // the pretty-while bool-context counter) traces to that sharing.
    let pool_count = |p: Pool| ty.values().filter(|&&q| q == p).count();
    let int_base: RegId = 0;
    let bool_base = int_base + pool_count(Pool::Int) as RegId;
    let table_base = bool_base + pool_count(Pool::Bool) as RegId;
    let str_base = table_base + pool_count(Pool::Table) as RegId;
    let float_base = str_base + pool_count(Pool::String) as RegId;
    let ftable_base = float_base + pool_count(Pool::Float) as RegId;
    let tstr_base = ftable_base + pool_count(Pool::TableFloat) as RegId;
    let btable_base = tstr_base + pool_count(Pool::TableString) as RegId;
    // layer-C ceiling stays under the const range (tripwire: the two
    // namespaces are disjoint by construction, and a program big
    // enough to reach 2^31 physicals is a build bug, not a workload)
    debug_assert!(btable_base + pool_count(Pool::TableBool) as RegId <= CONST_REG_BASE);

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
            let n = *c; *c += 1;
            match p {
                Pool::Int => int_base + n as RegId,
                Pool::Bool => bool_base + n as RegId,
                Pool::Table => table_base + n as RegId,
                Pool::String => str_base + n as RegId,
                Pool::Float => float_base + n as RegId,
                Pool::TableFloat => ftable_base + n as RegId,
                Pool::TableString => tstr_base + n as RegId,
                Pool::TableBool => btable_base + n as RegId,
            }
        });
        act.push((phys, end));
        map.insert(r, phys);
    }

    // 3.5 REMAP-COMPLETENESS TRIPWIRE. Every non-const id in the
    //     program must be in `map` (or the skip set). An unmapped vreg
    //     rides the identity fallback below and keeps its id — which
    //     since the zero-base mint NUMERICALLY COLLIDES with minted
    //     physicals and would silently read/write a declared local of
    //     the wrong pool. (Pre-zero-base it rendered as a high,
    //     undeclared id and failed the build loudly.) A real assert,
    //     not debug_assert: silent wrong code is the one failure this
    //     pass may never produce. Cost: one scan per compile.
    for b in blocks {
        for i in &b.instrs {
            for r in i.use_regs().into_iter().chain(i.def_reg()) {
                assert!(map.contains_key(&r) || is_const_reg(r),
                    "reg_alloc: vreg {r} never entered a pool — it would \
                     survive the rewrite and collide with physical ids");
            }
        }
        if let Some(Terminator::Branch { cond, .. }) = &b.terminator {
            assert!(map.contains_key(cond) || is_const_reg(*cond),
                "reg_alloc: branch cond {cond} never entered a pool — it would \
                 survive the rewrite and collide with physical ids");
        }
    }

    // 4. rewrite references (const vregs stay identity: codegen looks
    //    them up in the const maps and never emits them)
    for b in &mut program.blocks {
        for i in &mut b.instrs { i.remap_instr(&|r| *map.get(&r).unwrap_or(&r)); }
        if let Some(Terminator::Branch { cond, .. }) = &mut b.terminator
            && let Some(&p) = map.get(&*cond) { *cond = p; }
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
        int_base,
        bool_base,
        table_base,
        str_base,
        float_base,
        ftable_base,
        tstr_base,
        btable_base,
    }
}


