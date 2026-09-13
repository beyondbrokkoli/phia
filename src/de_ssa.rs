// src/de_ssa.rs
use std::collections::{HashMap};
use crate::ir::{IrProgram, Instruction, Terminator, BlockId, RegId};
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
    let mut phis: Vec<(RegId, StaticType, Vec<(BlockId, RegId)>,
                        Option<((BlockId, RegId), (BlockId, RegId))>)> = Vec::new();
    for b in &program.blocks {
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
                            if let (Some(&l), Some(&r)) = (ci.get(left), ci.get(right)) {
                                if let (Some(q), Some(m)) = (l.checked_div(r), l.checked_rem(r)) {
                                    let v = if m != 0 && ((m < 0) != (r < 0)) { q - 1 } else { q };
                                    ci.insert(*target, v);
                                }
                            }
                        }
                    Instruction::Mod { target, left, right }
                        if single_def(&defs, *target) => {
                            if let (Some(&l), Some(&r)) = (ci.get(left), ci.get(right)) {
                                if let Some(m) = l.checked_rem(r) {
                                    let v = if m != 0 && ((m < 0) != (r < 0)) { m + r } else { m };
                                    ci.insert(*target, v);
                                }
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
    (ci, cb)
}
