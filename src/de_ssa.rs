// src/de_ssa.rs
use std::collections::{HashMap};
use crate::ir::{IrProgram, Instruction, Terminator, BlockId, RegId, CONST_REG_BASE};
use crate::ast::StaticType;

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

// One gathered phi, pre-classified for resolve_phis. `shape` is Some((pre,
// back)) when the args are exactly one edge from before the header and one
// back edge (pre first) — the loop-header shape step 2 coalesces; the rest
// takes the classic one-Move-per-edge lowering.
struct GatheredPhi {
    target: RegId,
    ty: StaticType,
    args: Vec<(BlockId, RegId)>,
    shape: Option<((BlockId, RegId), (BlockId, RegId))>,
}

pub fn resolve_phis(program: &mut IrProgram) {
    // 0. def counts (a Phi counts as one def of its target)
    let mut defs: HashMap<RegId, usize> = HashMap::new();
    for b in &program.blocks {
        for i in &b.instrs {
            if let Some(d) = i.def_reg() { *defs.entry(d).or_insert(0) += 1; }
        }
    }

    // 1. gather phis; recognize the loop-header shape: one pred before
    //    the header (preheader), one after (back edge)
    let mut phis: Vec<GatheredPhi> = Vec::new();
    for b in &program.blocks {
        for i in &b.instrs {
            if let Instruction::Phi { target, ty, args } = i {
                let shape = if args.len() == 2 && (args[0].0 < b.id) != (args[1].0 < b.id) {
                    Some(if args[0].0 < b.id { (args[0], args[1]) } else { (args[1], args[0]) })
                } else { None };
                phis.push(GatheredPhi { target: *target, ty: ty.clone(), args: args.clone(), shape });
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

    for phi in &phis {
        let mut done = false;
        if let Some((pre, back)) = phi.shape {
            let b_res = resolve_via(&rename, back.1);
            if single_def(&defs, back.1) && b_res != phi.target {
                rename.insert(phi.target, b_res);
                injects.push((pre.0, back.1, pre.1, phi.ty.clone()));
                done = true;
            }
        }
        if !done {
            // plain phi: one Move per incoming edge (the classic lowering)
            for (pred, src) in &phi.args {
                injects.push((*pred, phi.target, *src, phi.ty.clone()));
            }
        }
    }

    // 3. inject raw (renames are applied afterwards, so chained phis —
    //    same variable phi'd at nested loop levels — compose correctly)
    for (blk, tgt, src, ty) in injects {
        program.blocks[blk].instrs
            .push(Instruction::Move { target: tgt, source: src, ty });
    }
    for b in &mut program.blocks {
        b.instrs.retain(|i| !matches!(i, Instruction::Phi { .. }));
    }
    if !rename.is_empty() {
        for b in &mut program.blocks {
            for i in &mut b.instrs {
                i.remap_instr(&|r| resolve_via(&rename, r));
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
    for b in &mut program.blocks {
        b.instrs.retain(|i|
            !matches!(i, Instruction::Move { target, source, .. } if target == source));
    }
}


pub fn propagate_constants(program: &mut IrProgram) -> (HashMap<RegId, i64>, HashMap<RegId, bool>) {
    let mut defs: HashMap<RegId, usize> = HashMap::new();
    for b in &program.blocks {
        for i in &b.instrs {
            if let Some(d) = i.def_reg() { *defs.entry(d).or_insert(0) += 1; }
        }
    }

    let mut ci: HashMap<RegId, i64> = HashMap::new();
    let mut cb: HashMap<RegId, bool> = HashMap::new();

    // Fixpoint: entries are only ever added (single-def regs are
    // immutable), and block-id order matches dominance order here, so
    // this converges in ~2 sweeps.
    loop {
        let before = ci.len() + cb.len();
        for b in &program.blocks {
            for i in &b.instrs {
                match i {
                    Instruction::LoadInt { target, val }
                        if single_def(&defs, *target) => { ci.insert(*target, *val); }
                    Instruction::LoadBool { target, val }
                        if single_def(&defs, *target) => { cb.insert(*target, *val); }
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
                    Instruction::Leq { target, left, right }
                        if single_def(&defs, *target) => {
                            if let (Some(&l), Some(&r)) = (ci.get(left), ci.get(right)) {
                                cb.insert(*target, l <= r);
                            }
                        }
                    Instruction::Geq { target, left, right }
                        if single_def(&defs, *target) => {
                            if let (Some(&l), Some(&r)) = (ci.get(left), ci.get(right)) {
                                cb.insert(*target, l >= r);
                            }
                        }
                    // Only INTEGER equality folds. Bool Eqs deliberately
                    // stay unfolded (emission renders both-const bools as
                    // literals anyway) — a byte-frozen choice.
                    Instruction::Eq { target, left, right, ty: StaticType::Integer }
                        if single_def(&defs, *target) => {
                            if let (Some(&l), Some(&r)) = (ci.get(left), ci.get(right)) {
                                cb.insert(*target, l == r);
                            }
                        }
                    Instruction::Not { target, source }
                        if single_def(&defs, *target) => {
                            if let Some(&v) = cb.get(source) {
                                cb.insert(*target, !v);
                            }
                        }
                    Instruction::Mul { target, left, right }
                        if single_def(&defs, *target) => {
                            if let (Some(&l), Some(&r)) = (ci.get(left), ci.get(right)) {
                                ci.insert(*target, l.wrapping_mul(r));
                            }
                        }
                    Instruction::Neg { target, source }
                        if single_def(&defs, *target) => {
                            if let Some(&v) = ci.get(source) {
                                ci.insert(*target, v.wrapping_neg());
                            }
                        }
                    // Lua floor semantics, matching the runtime templates;
                    // checked_div/rem: /0 and MIN/-1 stay unfolded
                    Instruction::IntDiv { target, left, right }
                        if single_def(&defs, *target) => {
                            if let (Some(&l), Some(&r)) = (ci.get(left), ci.get(right))
                                && let (Some(q), Some(m)) = (l.checked_div(r), l.checked_rem(r))
                            {
                                let v = if m != 0 && ((m < 0) != (r < 0)) { q - 1 } else { q };
                                ci.insert(*target, v);
                            }
                        }
                    Instruction::Mod { target, left, right }
                        if single_def(&defs, *target) => {
                            if let (Some(&l), Some(&r)) = (ci.get(left), ci.get(right))
                                && let Some(m) = l.checked_rem(r)
                            {
                                let v = if m != 0 && ((m < 0) != (r < 0)) { m + r } else { m };
                                ci.insert(*target, v);
                            }
                        }
                    Instruction::Move { target, source, ty }
                        if single_def(&defs, *target) => match ty {
                            StaticType::Integer =>
                                { if let Some(&v) = ci.get(source) { ci.insert(*target, v); } }
                            StaticType::Boolean =>
                                { if let Some(&v) = cb.get(source) { cb.insert(*target, v); } }
                            StaticType::Float => {}
                            // string constant folding is not
                            // implemented either — float precedent
                            StaticType::String => {}
                            StaticType::Table(_) | StaticType::UnknownTable(_) => {}
                        },
                    _ => {}
                }
            }
        }
        if ci.len() + cb.len() == before { break; }
    }

    // REMINT — every folded id leaves the vreg namespace for the reserved
    // const range (CONST_REG_BASE up), renumbering the maps' keys and every
    // reference to them in the IR. The fold itself ran on original vreg ids
    // (byte-identical discovery fixpoint); this pass only relocates the
    // results, making const ids and vreg/physical ids disjoint ranges by
    // construction. k-th SORTED folded id -> CONST_REG_BASE + k: HashMap
    // iteration is random per process, and mint assignment must be
    // deterministic (the reg_alloc `r`-tiebreak precedent).
    let mut folded: Vec<RegId> = ci.keys().copied().chain(cb.keys().copied()).collect();
    folded.sort_unstable();
    let mint: HashMap<RegId, RegId> = folded.iter().enumerate()
        .map(|(k, &old)| (old, CONST_REG_BASE + k as RegId))
        .collect();
    for b in &mut program.blocks {
        for i in &mut b.instrs { i.remap_instr(&|r| *mint.get(&r).unwrap_or(&r)); }
        // Branch conditions ride folded consts (`while true do` lowers the
        // condition to the folded vreg): an unrenamed terminator misses the
        // rekeyed maps and emits b_r<dead-id> — the exact hazard class
        // resolve_phis documents for its own renames.
        if let Some(Terminator::Branch { cond, .. }) = &mut b.terminator {
            if let Some(&m) = mint.get(cond) { *cond = m; }
        }
    }
    ci = ci.into_iter().map(|(k, v)| (mint[&k], v)).collect();
    cb = cb.into_iter().map(|(k, v)| (mint[&k], v)).collect();
    (ci, cb)
}

/// Copy propagation + DCE. Deliberately conservative: a Move is erased
/// only when BOTH target and source are single-def vregs. Phi targets
/// are multi-def after resolve_phis, so loop-carried copies are never
/// touched and the parallel-copy/swap hazard cannot appear.
pub fn simplify(program: &mut IrProgram) {
    let mut defs: HashMap<RegId, usize> = HashMap::new();
    for b in &program.blocks {
        for i in &b.instrs { if let Some(d) = i.def_reg() { *defs.entry(d).or_insert(0) += 1; } }
    }

    let mut rename: HashMap<RegId, RegId> = HashMap::new();
    for b in &program.blocks {
        for i in &b.instrs {
            if let Instruction::Move { target, source, .. } = i
                && target != source
                && defs.get(target) == Some(&1)
                && defs.get(source) == Some(&1)
            { rename.insert(*target, *source); }
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
        for b in &mut program.blocks {
            for i in &mut b.instrs { i.remap_instr(&resolve); }
            if let Some(Terminator::Branch { cond, .. }) = &mut b.terminator {
                *cond = resolve(*cond);
            }
        }
    }

    // DCE sweeps to a FIXPOINT: a removed instruction was its operands'
    // last use, so each sweep can expose one more layer of dead defs. Int
    // and bool chains mostly ride the const maps instead (a folded def
    // costs nothing whether or not it lingers in the IR), but floats and
    // strings have no const map — a dead Concat's LoadString operands die
    // only on the next sweep, and the chain head on the sweep after that.
    loop {
        let mut uses: HashMap<RegId, usize> = HashMap::new();
        for b in &program.blocks {
            for i in &b.instrs { for u in i.use_regs() { *uses.entry(u).or_insert(0) += 1; } }
            if let Some(Terminator::Branch { cond, .. }) = &b.terminator { *uses.entry(*cond).or_insert(0) += 1; }
        }

        let mut removed = false;
        for b in &mut program.blocks {
            b.instrs.retain(|i| {
                if matches!(i, Instruction::Move { target, source, .. } if target == source) {
                    removed = true;
                    return false;
                }
                // dead PURE defs only: NewTable allocates output, GetTable can
                // panic on negative keys — neither is ever "dead code" here.
                // LoadFloat/LoadString/Concat join the pure set despite their
                // heap traffic: their only hazard is OOM, a hazard class the
                // dead Div already establishes (a dead `x = a / b` loses its
                // /0 panic the same way).
                let dead = i.def_reg().map(|d| uses.get(&d).copied().unwrap_or(0) == 0).unwrap_or(false);
                let pure = matches!(i,
                    Instruction::LoadInt { .. } | Instruction::LoadBool { .. }
                    | Instruction::LoadFloat { .. } | Instruction::LoadString { .. }
                    | Instruction::Concat { .. } | Instruction::Move { .. }
                    | Instruction::Add { .. } | Instruction::Sub { .. } | Instruction::Less { .. }
                    | Instruction::Mul { .. } | Instruction::Div { .. }
                    | Instruction::IntDiv { .. } | Instruction::Mod { .. }
                    | Instruction::Neg { .. }
                    | Instruction::Leq { .. } | Instruction::Geq { .. }
                    | Instruction::Eq { .. } | Instruction::Not { .. });
                let drop = dead && pure;
                if drop { removed = true; }
                !drop
            });
        }
        if !removed { break; }
    }
}
