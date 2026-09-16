// src/register_alloc.rs
use crate::ir::{IrProgram, Instruction, Terminator, BasicBlock, RegId, ConstVal, CONST_REG_BASE, is_const_reg};
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
///  4. backend.rs      : the table-side ORACLE — table_fld /
///     table_fld_zero / table_ptr_ty answer every storage-side question
///     (decl ptr, NewTable ctor, Set/Get dyn + fast, EC, HR, probe
///     token); operand_kind kinds scalar operands (Eq renderer, probe
///     fields, stored-value rendering). No site re-derives the side on
///     its own, and every instruction that still carries a ty runs
///     assert_fld / the operand-kind assert against the pool — the
///     emission-side pool==ty tripwires.
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
    consts: &HashMap<RegId, ConstVal>,
) -> AllocInfo {
    let blocks = &program.blocks;

    // The skip set is exactly layer-B membership (Invariant 2, ir.rs):
    // const ids never enter a liveness interval, never receive a
    // physical slot — every use renders as a literal — so no const id
    // can be confused with a physical at emission. All four scalar
    // kinds ride the one map (the variant is the kind); the base scan
    // below is already layer-guarded, so nothing else changes.
    let skip: HashSet<RegId> = consts.keys().copied().collect();

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
        // CONST MAP's variant, not in ty (skip excludes it). A folded
        // operand is invisible to the fixpoint below — and an arith
        // target whose operands are ALL const while its own result
        // stayed runtime (the is_finite guard declining inf/NaN, or a
        // multi-def slot) would otherwise mint NO physical at all and
        // render as an undeclared register (Boundary Defense B2, ir.rs
        // — feat_ops_10: `inf = a / b` after consts_f). Const bools/
        // strings can never be arith operands (the checker rejects
        // them), so the Float/Int variants are a complete answer.
        let const_pool = |r: RegId| match consts.get(&r) {
            Some(ConstVal::Float(_)) => Some(Pool::Float),
            Some(ConstVal::Int(_)) => Some(Pool::Int),
            _ => None,
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

    // 3. mint physical ids from ZERO (not max-vreg+1 — physical
    //    numbering inherits no offset from the vreg namespace).
    //    Layer C does not sit ABOVE layer A; it REPLACES it
    //    (Invariant 1, ir.rs): the remap in step 4 rewrites every def,
    //    use and branch cond through `map`, erasing the vreg namespace
    //    from the IR in the same pass that mints the physicals, so the
    //    numeric overlap of [0, V) and [0, P) is unobservable. The only
    //    cross-layer keys still alive at emission are the CONST maps,
    //    and their keys are reminted layer-B ids, disjoint from every
    //    physical by construction (Boundary Defense B1) — vregs carry
    //    no map of their own that could outlive the namespace. Consts
    //    never get a physical slot: every use renders as a literal
    //    (the skip set is exactly layer-B membership, Invariant 2).

    // ONE GLOBAL TIMELINE of physical ids — Invariant 3 (ir.rs):
    // eight consecutive per-pool ranges minted in fixed order — int,
    // bool, table, string, float, ftable, tstr, btable. Every physical
    // id belongs to exactly one pool and no two physical registers
    // share a number, so `i_r12`/`b_r12` co-numbering cannot arise,
    // pool membership is a pure range check, and nothing downstream
    // needs a carried StaticType to disambiguate a scalar operand.
    // (The superseded layout co-numbered Int/Bool/Table/String from
    // one shared range; Eq's ty, probe operand kinds, and the
    // pretty-while bool-context counter were all type-context
    // workarounds for that sharing — Boundary Defense B3.)
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

    // 4. rewrite references — the Total Rewrite, Invariant 1 (ir.rs).
    //    Const ids keep their identity (emission resolves them through
    //    the const maps and never emits a register); every OTHER id
    //    must have entered a pool, and the closure itself enforces it
    //    — there is no identity fallback. An unmapped vreg would keep
    //    its low id, numerically collide with a minted physical, and
    //    silently read/write a declared local of the wrong pool, so
    //    the closure panics instead. Coverage is exactly remap_instr +
    //    branch conds — the positions that actually get rewritten — so
    //    a future instruction kind cannot slip a position past a
    //    use_regs/def_reg match that has drifted out of sync. A real
    //    assert, not debug_assert: silent wrong code is the one
    //    failure mode this pass forbids.
    let rewrite = |r: RegId| -> RegId {
        if is_const_reg(r) { return r; }
        match map.get(&r) {
            Some(&p) => p,
            None => panic!(
                "reg_alloc: vreg {r} never entered a pool — it would \
                 survive the rewrite and collide with physical ids"),
        }
    };
    for b in &mut program.blocks {
        for i in &mut b.instrs { i.remap_instr(&rewrite); }
        if let Some(Terminator::Branch { cond, .. }) = &mut b.terminator {
            *cond = rewrite(*cond);
        }
    }

    // 5. self-copies are no-ops
    for b in &mut program.blocks {
        b.instrs.retain(|i|
            !matches!(i, Instruction::Move { target, source, .. } if target == source));
    }

    // 6. post-alloc id-space audit (Invariant 4, ir.rs) — emission
    //    leans on the invariants silently, so the FINAL program is
    //    what gets checked; audit_id_space below spells them out.
    let info = AllocInfo {
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
    };
    audit_id_space(program, &info, consts);
    info
}

/// POST-ALLOC ID-SPACE AUDIT — Invariant 4 (ir.rs): invariants 1–3
/// re-verified over the FINAL program in one scan (defs first, then
/// uses and branch conds), real asserts, no mutation. Emission relies
/// on every claim here silently; the audit turns each into a named
/// failure. It runs inside allocate_registers, so every corpus run,
/// every lock regen, and every differential-fuzzer seed audits every
/// compile — fuzzer_01_paying_rent.lua is the designated complex
/// witness: if any generated shape can break the unique-id law, a
/// fuzzer seed is where it surfaces first.
///   * remint purity — every consts-map key is layer-B (>= the
///     reserved base). Boundary Defense B1: a low id in a const map
///     could shadow a minted physical at emission's const early-outs.
///   * membership — every non-const def/use/branch-cond id lies
///     inside a minted pool range; nothing unminted survives the
///     rewrite.
///   * pool purity — each def's target range matches the defining
///     instruction's kind, and one id is never defined into two pools
///     (Boundary Defense B3; guaranteed by the mint's construction,
///     asserted anyway).
///   * bool conds — a Branch cond is bool-pool or const (a non-bool
///     cond would render as an undeclared b_r at emission).
///   * no dangling uses — every non-const use/cond has a def
///     somewhere (the undeclared-register class).
///
/// Slot reuse across DISJOINT intervals is legal and pinned
/// (floatinf_01: sum coalesces onto neg's freed slot) — the invariant
/// is one-id-one-pool, not one-id-one-def.
pub fn audit_id_space(
    program: &IrProgram,
    alloc: &AllocInfo,
    consts: &HashMap<RegId, ConstVal>,
) {
    use std::borrow::Cow;

    for r in consts.keys() {
        assert!(is_const_reg(*r),
            "id-space audit: consts map holds low id {r} — layer B must be \
             reminted above CONST_REG_BASE or emission's const early-outs \
             can shadow a minted physical (the fuzzer_01 class)");
    }

    let end = alloc.btable_base + alloc.n_btable as RegId;
    let ranges: [(RegId, RegId, Pool); 8] = [
        (alloc.int_base, alloc.bool_base, Pool::Int),
        (alloc.bool_base, alloc.table_base, Pool::Bool),
        (alloc.table_base, alloc.str_base, Pool::Table),
        (alloc.str_base, alloc.float_base, Pool::String),
        (alloc.float_base, alloc.ftable_base, Pool::Float),
        (alloc.ftable_base, alloc.tstr_base, Pool::TableFloat),
        (alloc.tstr_base, alloc.btable_base, Pool::TableString),
        (alloc.btable_base, end, Pool::TableBool),
    ];
    let classify = |r: RegId| -> Option<Pool> {
        ranges.iter().find(|&&(lo, hi, _)| r >= lo && r < hi).map(|&(_, _, p)| p)
    };
    let classify_or_panic = |r: RegId| -> Pool {
        classify(r).unwrap_or_else(|| panic!(
            "id-space audit: id {r} is neither const nor inside any minted \
             pool range — an unminted id survived the rewrite"))
    };

    // pass 1: defs (membership, per-instruction pool purity, and the
    // one-id-one-pool accumulation across the whole program)
    let mut def_pool: HashMap<RegId, Pool> = HashMap::new();
    for b in &program.blocks {
        for i in &b.instrs {
            let Some(d) = i.def_reg() else { continue };
            if is_const_reg(d) { continue; }
            let actual = classify_or_panic(d);
            let allowed: Cow<'static, [Pool]> = match i {
                Instruction::LoadInt { .. } => Cow::Borrowed(&[Pool::Int]),
                Instruction::LoadFloat { .. } => Cow::Borrowed(&[Pool::Float]),
                Instruction::LoadBool { .. } => Cow::Borrowed(&[Pool::Bool]),
                Instruction::LoadString { .. } | Instruction::Concat { .. }
                    => Cow::Borrowed(&[Pool::String]),
                Instruction::Less { .. } | Instruction::Leq { .. }
                | Instruction::Geq { .. } | Instruction::Eq { .. }
                | Instruction::Not { .. } => Cow::Borrowed(&[Pool::Bool]),
                // arith is kind-polymorphic: int and float share the
                // operator set, so either scalar range is legal
                Instruction::Add { .. } | Instruction::Sub { .. }
                | Instruction::Mul { .. } | Instruction::Div { .. }
                | Instruction::IntDiv { .. } | Instruction::Mod { .. }
                | Instruction::Neg { .. } => Cow::Borrowed(&[Pool::Int, Pool::Float]),
                // ty is the TARGET's static type on all of these
                // (NewTable carries the full table type, GetTable the
                // element type, Move/Phi the value type)
                Instruction::NewTable { ty, .. } | Instruction::GetTable { ty, .. }
                | Instruction::GetTableFast { ty, .. } | Instruction::Move { ty, .. }
                | Instruction::Phi { ty, .. } => Cow::Owned(vec![pool_of(ty)]),
                _ => Cow::Borrowed(&[]),
            };
            assert!(allowed.contains(&actual),
                "id-space audit: {i:?} defines {d} inside the {actual:?} range — \
                 pool purity broken (one id, one pool)");
            match def_pool.entry(d) {
                std::collections::hash_map::Entry::Occupied(e) => assert!(*e.get() == actual,
                    "id-space audit: id {d} defined in both {:?} and {actual:?} pools — \
                     physical id collision", e.get()),
                std::collections::hash_map::Entry::Vacant(e) => { e.insert(actual); }
            }
        }
    }

    // pass 2: uses and branch conds
    for b in &program.blocks {
        for i in &b.instrs {
            for u in i.use_regs() {
                if is_const_reg(u) { continue; }
                classify_or_panic(u);
                assert!(def_pool.contains_key(&u),
                    "id-space audit: id {u} is used but never defined — \
                     emission would render an undeclared register");
            }
        }
        if let Some(Terminator::Branch { cond, .. }) = &b.terminator && !is_const_reg(*cond) {
            let p = classify_or_panic(*cond);
            assert!(p == Pool::Bool,
                "id-space audit: branch cond {cond} is {p:?}-pool, not bool");
            assert!(def_pool.contains_key(cond),
                "id-space audit: branch cond {cond} is used but never defined — \
                 emission would render an undeclared register");
        }
    }
}


