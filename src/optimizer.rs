// src/optimizer.rs
use std::collections::{HashMap, HashSet, BTreeSet};
use crate::ir::{IrProgram, Instruction, Terminator, BasicBlock, BlockId, RegId};
use crate::ast::StaticType;

// Local constant folder over def_map (LoadInt / int-Move / Add / Sub over
// constants). propagate_constants runs AFTER optimize() in the pipeline, so
// passes that need proof-of-value inside optimize() fold for themselves.
// Phis return None — a loop-carried value has no compile-time value.
fn const_eval(
    blocks: &[BasicBlock],
    def_map: &HashMap<RegId, (BlockId, usize)>,
    r: RegId,
) -> Option<i64> {
    fn go(
        blocks: &[BasicBlock],
        def_map: &HashMap<RegId, (BlockId, usize)>,
        r: RegId,
        depth: usize,
    ) -> Option<i64> {
        if depth > 64 { return None; }
        let &(b, i) = def_map.get(&r)?;
        match &blocks[b].instrs[i] {
            Instruction::LoadInt { val, .. } => Some(*val),
            Instruction::Move { source, .. } => go(blocks, def_map, *source, depth + 1),
            Instruction::Add { left, right, .. } => Some(go(blocks, def_map, *left, depth + 1)?
                .wrapping_add(go(blocks, def_map, *right, depth + 1)?)),
            Instruction::Sub { left, right, .. } => Some(go(blocks, def_map, *left, depth + 1)?
                .wrapping_sub(go(blocks, def_map, *right, depth + 1)?)),
            Instruction::Mul { left, right, .. } => Some(go(blocks, def_map, *left, depth + 1)?
                .wrapping_mul(go(blocks, def_map, *right, depth + 1)?)),
            Instruction::Neg { source, .. } =>
                go(blocks, def_map, *source, depth + 1)?.checked_neg(),
            // Lua floor semantics: quotient rounds toward negative infinity,
            // remainder takes the divisor's sign. checked_div/rem return None
            // on /0 and on i64::MIN/-1 overflow — those stay runtime panics
            // instead of becoming const traps; the sign adjustment only
            // fires when a remainder exists and disagrees with the divisor.
            Instruction::IntDiv { left, right, .. } => {
                let (l, r) = (go(blocks, def_map, *left, depth + 1)?,
                    go(blocks, def_map, *right, depth + 1)?);
                let q = l.checked_div(r)?;
                let m = l.checked_rem(r)?;
                if m != 0 && ((m < 0) != (r < 0)) { q.checked_sub(1) } else { Some(q) }
            }
            Instruction::Mod { left, right, .. } => {
                let (l, r) = (go(blocks, def_map, *left, depth + 1)?,
                    go(blocks, def_map, *right, depth + 1)?);
                let m = l.checked_rem(r)?;
                if m != 0 && ((m < 0) != (r < 0)) { m.checked_add(r) } else { Some(m) }
            }
            _ => None,
        }
    }
    go(blocks, def_map, r, 0)
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

// DEF_MAP INDEX-STABILITY INVARIANT. def_map maps vreg -> (block, index)
// and is built ONCE, before any mutation — every later pass in this
// function must keep those pairs valid:
//   * PASS 3's upgrades rewrite instructions IN PLACE (index preserved);
//   * the tier-4 mints and S5's EC/Hoist pairs only APPEND to pre-header
//     blocks (existing indices preserved — the appends deliberately
//     happen before PASS 3 could run again on the same header);
//   * the orphan-feeder cleanup RETAINS (removes) instructions, which
//     DOES shift indices — but only for blocks inside the region, only
//     after this header's analysis is complete, and only for feeders
//     whose sole use was just rewritten away, so their def_map entries
//     can never be queried again (the LATER headers' analyses look up
//     live operands only). If a future pass ever deletes or inserts
//     mid-block, def_map must be rebuilt (or switched to instruction
//     identity) in the same commit.
pub fn optimize(program: &mut IrProgram) {
    let mut def_map: HashMap<RegId, (BlockId, usize)> = HashMap::new();
    for block in &program.blocks {
        for (i, instr) in block.instrs.iter().enumerate() {
            match instr {
                Instruction::LoadInt { target, .. } | Instruction::LoadFloat { target, .. }
                | Instruction::LoadBool { target, .. } | Instruction::LoadString { target, .. }
                | Instruction::NewTable { target, .. } |
                Instruction::GetTable { target, .. } | Instruction::Move { target, .. } |
                Instruction::Add { target, .. } | Instruction::Sub { target, .. } |
                Instruction::Less { target, .. } | Instruction::Phi { target, .. } |
                Instruction::Mul { target, .. } | Instruction::Div { target, .. } |
                Instruction::IntDiv { target, .. } | Instruction::Mod { target, .. } |
                Instruction::Neg { target, .. } |
                Instruction::Leq { target, .. } | Instruction::Geq { target, .. } |
                Instruction::Eq { target, .. } | Instruction::Not { target, .. } |
                Instruction::Concat { target, .. } |
                Instruction::GetTableFast { target, .. } => {
                    def_map.insert(*target, (block.id, i));
                }
                _ => {}
            }
        }
    }

    // TIER 2: fresh vreg ids for EC-limit materialization, minted from
    // above the entire existing namespace so a minted vreg can never
    // alias an original one. allocate_registers() re-scans the program.
    let mut next_vreg: RegId = program.blocks.iter()
        .flat_map(|b| b.instrs.iter())
        .flat_map(|i| {
            let mut regs = i.use_regs();
            if let Some(d) = i.def_reg() { regs.push(d); }
            regs
        })
        .chain(program.blocks.iter().filter_map(|b| match &b.terminator {
            Some(Terminator::Branch { cond, .. }) => Some(*cond),
            _ => None,
        }))
        .max()
        .unwrap_or(0)
        + 1;

    let get_table_root = |blocks: &[BasicBlock], table_reg: RegId| -> Option<RegId> {
        let mut curr = table_reg;
        let mut seen = HashSet::new();
        loop {
            if !seen.insert(curr) { return None; }
            if let Some(&(b, i)) = def_map.get(&curr) {
                match &blocks[b].instrs[i] {
                    Instruction::NewTable { target, .. } => return Some(*target),
                    Instruction::Move { source, ty: StaticType::Table(_) | StaticType::UnknownTable(_), .. } =>
                    {
                        curr = *source;
                        continue;
                    }
                    _ => return None,
                }
            } else {
                return None;
            }
        }
    };

    let key_offset = |blocks: &[BasicBlock], key_reg: RegId, idx_reg: RegId| -> Option<i64> {
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
                    else {
                        let c = const_of(blocks, *left)?;
                        off = off.wrapping_add(c);
                        curr = *right;
                    }
                }
                Instruction::Sub { left, right, .. } => {
                    off = off.wrapping_sub(const_of(blocks, *right)?);
                    curr = *left;
                }
                _ => return None,
            }
        }
    };

    // TIER 4-13 (fast row resolution) — REACH alias closure, computed
    // ONCE, program-wide. REACH(R) = the roots whose handles may ever
    // sit in R's slots. Handles enter a table's slots ONLY through
    // stores (NewTable mints fresh handles, GetTable copies them out),
    // so two rules close the set: a root-traceable val lands directly,
    // a val READ from another table's slots copies that table's whole
    // reach. A scalar store through a child of R (`t[i][j] = v`) can
    // resize exactly the storage of the Tables REACH(R) names — never
    // R's own array — so regions containing such a store may still
    // hoist R, but must not hoist (nor EC under) anything in REACH(R).
    // Unresolvable (phi) vals poison the dest: unknown slots abort.
    let handle_origin = |blocks: &[BasicBlock], mut r: RegId| -> Option<Result<RegId, RegId>> {
        let mut seen = HashSet::new();
        loop {
            if !seen.insert(r) { return None; }
            let &(b, i) = def_map.get(&r)?;
            match &blocks[b].instrs[i] {
                Instruction::NewTable { .. } => return Some(Ok(r)),
                Instruction::Move { source, ty: StaticType::Table(_) | StaticType::UnknownTable(_), .. } =>
                {
                    r = *source;
                }
                // names whatever Table the parent root's slots hold
                Instruction::GetTable { table, .. } | Instruction::GetTableFast { table, .. } =>
                    return get_table_root(blocks, *table).map(Err),
                _ => return None,
            }
        }
    };
    let mut reach: HashMap<RegId, HashSet<RegId>> = HashMap::new();
    let mut reach_tainted: HashSet<RegId> = HashSet::new();
    loop {
        let mut changed = false;
        for b in &program.blocks {
            for ins in &b.instrs {
                let (Instruction::SetTable { table, val, ty, .. }
                | Instruction::SetTableFast { table, val, ty, .. }) = ins else { continue };
                if !matches!(ty, StaticType::Table(_) | StaticType::UnknownTable(_)) { continue; }
                let Some(dest) = get_table_root(&program.blocks, *table) else { continue };
                match handle_origin(&program.blocks, *val) {
                    Some(Ok(r)) => { changed |= reach.entry(dest).or_default().insert(r); }
                    Some(Err(src)) => {
                        let src_set: Vec<RegId> = reach.get(&src)
                            .map(|s| s.iter().copied().collect())
                            .unwrap_or_default();
                        let d = reach.entry(dest).or_default();
                        for r in src_set {
                            changed |= d.insert(r);
                        }
                    }
                    None => { changed |= reach_tainted.insert(dest); }
                }
            }
        }
        if !changed { break; }
    }

    // The parent root of a child-store's table operand: follow table
    // Moves, then ONE GetTable/GetTableFast hop whose own table
    // operand is root-traceable. Deeper chains and phi-carried tables
    // stay behind the abort firewall (firewall_abort_all, tier4_03).
    let child_parent_root = |blocks: &[BasicBlock], table_reg: RegId| -> Option<RegId> {
        let mut curr = table_reg;
        let mut seen = HashSet::new();
        loop {
            if !seen.insert(curr) { return None; }
            let &(b, i) = def_map.get(&curr)?;
            match &blocks[b].instrs[i] {
                Instruction::Move { source, ty: StaticType::Table(_) | StaticType::UnknownTable(_), .. } =>
                {
                    curr = *source;
                }
                Instruction::GetTable { table, .. } | Instruction::GetTableFast { table, .. } =>
                    return get_table_root(blocks, *table),
                _ => return None,
            }
        }
    };

    let num_blocks = program.blocks.len();
    for header_id in 0..num_blocks {
        let terminator = program.blocks[header_id].terminator.clone();

        if let Some(Terminator::Branch { cond, true_block: body_id, .. }) = terminator
            && let Some(&(cond_block, cond_idx)) = def_map.get(&cond) {
            let is_less = if let Instruction::Less { left, right, .. } = &program.blocks[cond_block].instrs[cond_idx] {
                Some((*left, *right))
            } else { None };

            if let Some((idx_reg, limit_reg)) = is_less {
                let (limit_def_block, _) = def_map.get(&limit_reg).unwrap_or(&(0, 0));
                let limit_is_invariant = *limit_def_block < header_id;

                if limit_is_invariant {
                    let mut clobbered_roots = HashSet::new();
                    // TIER 4: roots receiving ANY store in the region
                    // (any key). Stricter than clobbered_roots: an
                    // affine store cannot invalidate r's own pointer
                    // (EC covers it) but CAN rebind the constant slot
                    // a child handle is read from (pinned: tier4_03).
                    let mut region_stored_roots = HashSet::new();
                    // TIER 4-13: parent roots of non-affine SCALAR
                    // child-stores in this region (see PASS 1).
                    let mut child_store_roots = HashSet::new();
                    let mut hoists = BTreeSet::new();
                    let mut upgrades: Vec<(BlockId, usize, Instruction)> = Vec::new();

                    let mut region = loop_region(&program.blocks, header_id, body_id);
                    region.sort_unstable();

                    let mut root_max_off: HashMap<RegId, i64> = HashMap::new();
                    let mut global_max_off: i64 = 0;
                    let mut abort_all = false;

                    // PASS 1: Read-Only, REGION-WIDE. (Tier 2 poison check)
                    'poison: for &blk in &region {
                        for instr in &program.blocks[blk].instrs {
                            match instr {
                                Instruction::SetTable { table, key, ty, .. } => {
                                    if let Some(root) = get_table_root(&program.blocks, *table) {
                                        region_stored_roots.insert(root);
                                    }
                                    match key_offset(&program.blocks, *key, idx_reg) {
                                        None => match get_table_root(&program.blocks, *table) {
                                            Some(root) => { clobbered_roots.insert(root); }
                                            None => {
                                                // TIER 4-13: a non-affine SCALAR store
                                                // through a one-hop child of root R
                                                // (`t[i][j] = v`, the matrix store seen
                                                // from the OUTER loop) mutates only
                                                // storage of the Tables R's slots name
                                                // — never R's own array. It need not
                                                // abort the region: PASS 2 and tier-4
                                                // below decline exactly the roots in
                                                // REACH(R) instead. Table-VALUED stores
                                                // and deeper/unresolvable operands keep
                                                // the abort — stores are how aliases
                                                // are born (firewall_abort_all, and the
                                                // affine cousin pads ECs globally).
                                                let scalar_val = !matches!(
                                                    ty,
                                                    StaticType::Table(_) | StaticType::UnknownTable(_)
                                                );
                                                if scalar_val {
                                                    match child_parent_root(&program.blocks, *table) {
                                                        Some(r) => { child_store_roots.insert(r); }
                                                        None => { abort_all = true; break 'poison; }
                                                    }
                                                } else {
                                                    abort_all = true; break 'poison;
                                                }
                                            }
                                        },
                                        Some(off) if off < 0 => {} // Cannot resize
                                        Some(off) => match get_table_root(&program.blocks, *table) {
                                            Some(root) => {
                                                root_max_off.entry(root)
                                                    .and_modify(|m| *m = (*m).max(off))
                                                    .or_insert(off);
                                            }
                                            None => { global_max_off = global_max_off.max(off); }
                                        },
                                    }
                                }
                                Instruction::GetTable { table, key, .. } => {
                                    if let Some(off) = key_offset(&program.blocks, *key, idx_reg)
                                        && off >= 0
                                        && let Some(root) = get_table_root(&program.blocks, *table)
                                    {
                                        root_max_off.entry(root)
                                            .and_modify(|m| *m = (*m).max(off))
                                            .or_insert(off);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    if abort_all { continue; }

                    // TIER 4-13 hazard set: everything a child-store
                    // in THIS region may resize. Hoisting (and EC'ing
                    // under a hoist) any of these roots here would be
                    // unsound — the store runs mid-region, past any
                    // pre-header sizing. A tainted parent (phi-typed
                    // vals stored into it somewhere) has unknown
                    // slots: keep the firewall and decline the region.
                    let mut hazard_roots: HashSet<RegId> = HashSet::new();
                    let mut hazard_tainted = false;
                    for &r in &child_store_roots {
                        if reach_tainted.contains(&r) { hazard_tainted = true; break; }
                        if let Some(s) = reach.get(&r) {
                            hazard_roots.extend(s.iter().copied());
                        }
                    }
                    if hazard_tainted { continue; }

                    // PASS 2: Read-Only. Determine upgrades. (Tier 2 + Tier 2b S2 check)
                    for &blk in &region {
                        for (i, instr) in program.blocks[blk].instrs.iter().enumerate() {
                            match instr {
                                Instruction::SetTable { table, key, val, ty } => {
                                    if let Some(root) = get_table_root(&program.blocks, *table) {
                                        // TIER 2b: ROOT-based S2 Dominance.
                                        let (root_def_b, _) = def_map.get(&root).unwrap_or(&(0,0));
                                        if *root_def_b >= header_id { continue; }

                                        if key_offset(&program.blocks, *key, idx_reg).is_some_and(|off| off >= 0)
                                            && !clobbered_roots.contains(&root)
                                            // tier4_13: a child-store in this region may
                                            // resize this root's storage mid-loop
                                            && !hazard_roots.contains(&root)
                                        {
                                            upgrades.push((blk, i, Instruction::SetTableFast { table: root, key: *key, val: *val, ty: ty.clone() }));
                                            hoists.insert(root);
                                        }
                                    }
                                }
                                Instruction::GetTable { target, table, key, ty } => {
                                    if let Some(root) = get_table_root(&program.blocks, *table) {
                                        // TIER 2b: ROOT-based S2 Dominance.
                                        let (root_def_b, _) = def_map.get(&root).unwrap_or(&(0,0));
                                        if *root_def_b >= header_id { continue; }

                                        if key_offset(&program.blocks, *key, idx_reg).is_some_and(|off| off >= 0)
                                            && !clobbered_roots.contains(&root)
                                            && !hazard_roots.contains(&root)
                                        {
                                            upgrades.push((blk, i, Instruction::GetTableFast { target: *target, table: root, key: *key, ty: ty.clone() }));
                                            hoists.insert(root);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    // TIER 4: single-hop child roots through
                    // GetTable-with-constant-key — the nested fast
                    // path (`t[0][i] = v`). A nested op's table
                    // operand is defined by a GetTable (the
                    // feeder); when every eligibility gate passes,
                    // the child handle is loop-invariant:
                    // materialize it once in the pre-header (a
                    // minted dyn GetTable), let the normal S5
                    // machinery EC + hoist it, and rewrite the
                    // nested op against the child. Gates, rationales
                    // and decline pins are annotated end to end in
                    // tests/examples/nested_05_dyn_loop.lua.
                    //
                    // Deliberately narrow (first landing): the
                    // limit must const-fold positive AND the
                    // induction var's entry value must fold
                    // below it — the mint's EC/Hoist nil-panic
                    // is faithful only for a loop that iterates
                    // (tier4_04/05 lim side, tier4_08 entry
                    // side, tier4_12 the unprovable decline);
                    // hop keys are non-negative consts or
                    // registers invariant for the region (an
                    // enclosing phi — the matrix path,
                    // tier4_10/11); orphaned feeders are
                    // deleted only when singly-used, by THIS
                    // pass (a global const-key purity rule
                    // would erase unused reads — nested_06
                    // pins their survival).
                    let mut orphan_feeders: HashSet<RegId> = HashSet::new();
                    // Proven-positive COMPUTED limits: fold the
                    // limit's def chain locally instead of demanding
                    // a LoadInt def (tier4_07). Unprovable limits
                    // still decline (tier4_12).
                    let tier4_lim = const_eval(&program.blocks, &def_map, limit_reg);
                    // >=1-TRIP PROOF, alongside lim > 0: the mint's
                    // EC/Hoist nil-panic in the pre-header is
                    // faithful only if the loop actually iterates,
                    // and a positive limit alone does not guarantee
                    // that — the induction var can START above it
                    // (zero trips, nil child: dyn runs silent, the
                    // mint would panic — tier4_08). The entry value
                    // is the phi's pre-header argument, or the reg
                    // itself when no phi exists (then it is constant
                    // across iterations, so it either trips forever
                    // or never).
                    let tier4_entry = match def_map.get(&idx_reg) {
                        Some(&(b, i)) => match &program.blocks[b].instrs[i] {
                            Instruction::Phi { args, .. } => args.iter()
                                .find(|&&(pb, _)| pb < header_id)
                                .and_then(|&(_, r)| const_eval(&program.blocks, &def_map, r)),
                            _ => const_eval(&program.blocks, &def_map, idx_reg),
                        },
                        None => None,
                    };
                    // tier4_13: with a child-store in this region, a
                    // minted/hoisted child (whose Table is a REACH
                    // member of its parent) could be resized by that
                    // same store mid-region. Blunt decline — inert
                    // for the matrix shapes, where the child-store
                    // is only non-affine at the OUTER level and the
                    // minting pass is the inner one (hazard empty).
                    if matches!((tier4_lim, tier4_entry), (Some(lim), Some(e)) if lim > 0 && e < lim)
                        && hazard_roots.is_empty()
                    {
                        let region_set: HashSet<BlockId> = region.iter().copied().collect();
                        // dedup key: (parent, const-key OR invariant key-reg)
                        let mut child_regs: HashMap<(RegId, Option<i64>, Option<RegId>), RegId> = HashMap::new();
                        // (handle, const?, key operand, parent, ty, fast) — key
                        // operand is a minted LoadInt target for consts, the
                        // original register for invariant keys
                        let mut mints: Vec<(RegId, Option<i64>, RegId, RegId, StaticType, bool)> = Vec::new();
                        for &blk in &region {
                            for (i, instr) in program.blocks[blk].instrs.iter().enumerate() {
                                let (table_op, own_key) = match instr {
                                    Instruction::SetTable { table, key, .. }
                                    | Instruction::GetTable { table, key, .. } => (*table, *key),
                                    _ => continue,
                                };
                                // flat-eligible ops traced to roots in PASS 2 — not ours
                                if get_table_root(&program.blocks, table_op).is_some() { continue; }
                                let Some(&(fb, fi)) = def_map.get(&table_op) else { continue };
                                let Some(off) = key_offset(&program.blocks, own_key, idx_reg) else { continue };
                                if off < 0 { continue; }

                                // The replacement handle the op will target.
                                let h: RegId = if !region_set.contains(&fb) {
                                    // MATRIX PATH (tier4_10/11): the feeder
                                    // lives in THIS loop's pre-header — the
                                    // child varies per ENCLOSING trip
                                    // (`t[i][j]`). No mint: the feeder IS the
                                    // materialization, re-resolved every
                                    // enclosing trip; EC + Hoist land right
                                    // after it in the same block, so the
                                    // hoist re-arms per enclosing trip too.
                                    // Soundness: the feeder's register is
                                    // SSA-stable, and the slot it resolved
                                    // goes stale only via a store through
                                    // the parent root inside THIS region —
                                    // the region_stored_roots guard below.
                                    // The feeder must sit in the pre-header
                                    // block ITSELF: an intervening loop
                                    // between it and this header could
                                    // resize the child or rebind the slot
                                    // after the hoist.
                                    let mut pre: Option<BlockId> = None;
                                    for b in 0..header_id {
                                        if let Some(Terminator::Jump(tgt)) = &program.blocks[b].terminator
                                            && *tgt == header_id { pre = Some(b); break; }
                                    }
                                    if pre != Some(fb) { continue; }
                                    let root = match &program.blocks[fb].instrs[fi] {
                                        Instruction::GetTable { table, .. } =>
                                            get_table_root(&program.blocks, *table),
                                        Instruction::GetTableFast { table, .. } => Some(*table),
                                        _ => None,
                                    };
                                    let Some(root) = root else { continue };
                                    if region_stored_roots.contains(&root) { continue; }
                                    let (root_def_b, _) = def_map.get(&root).unwrap_or(&(0, 0));
                                    if *root_def_b >= header_id { continue; }
                                    table_op
                                } else {
                                    // IN-REGION feeder: walk the chain
                                    // upward (multi-hop, tier4_09). Every
                                    // hop must be an in-region GetTable
                                    // whose key is a non-negative CONST
                                    // or a register INVARIANT for this
                                    // region — defined outside the
                                    // region AND outside this header
                                    // (an enclosing phi: the matrix
                                    // case t[i][j], tier4_10/11). The
                                    // chain is re-materialized in the
                                    // pre-header, root down, deduped
                                    // per (parent, key) so sibling ops
                                    // share prefixes. Only the LEAF is
                                    // EC'd and hoisted — intermediate
                                    // handles are resolution steps. An
                                    // invariant key re-reads its
                                    // register in the pre-header, where
                                    // the coalesced slot holds the
                                    // entry value: the child re-resolves
                                    // once per ENCLOSING trip.
                                    // chain entries carry the hop's kind:
                                    // a Fast feeder rides a root pointer
                                    // an earlier pass hoisted (tier4_13),
                                    // and the mint re-materializes it in
                                    // kind — a FAST row resolution.
                                    let mut chain: Vec<(RegId, Option<i64>, RegId, StaticType, bool)> = Vec::new();
                                    let mut cur = table_op;
                                    while chain.len() < 8 {
                                        let Some(&(cb, ci)) = def_map.get(&cur) else { break };
                                        if !region_set.contains(&cb) { break; }
                                        let (table, key, ty, fast) = match &program.blocks[cb].instrs[ci] {
                                            Instruction::GetTable { table, key, ty, .. } => (table, key, ty, false),
                                            Instruction::GetTableFast { table, key, ty, .. } => (table, key, ty, true),
                                            _ => break,
                                        };
                                        let ck = match const_eval(&program.blocks, &def_map, *key) {
                                            Some(c) if c >= 0 => Some(c),
                                            Some(_) => break,
                                            None => match def_map.get(key) {
                                                // invariant: def outside region and header
                                                Some(&(kb, _)) if !region_set.contains(&kb) && kb != header_id => None,
                                                _ => break,
                                            },
                                        };
                                        chain.push((cur, ck, *key, ty.clone(), fast));
                                        cur = *table;
                                    }
                                    if chain.is_empty() { continue; }
                                    let Some(root) = get_table_root(&program.blocks, cur) else { continue };
                                    if region_stored_roots.contains(&root) { continue; }
                                    let (root_def_b, _) = def_map.get(&root).unwrap_or(&(0, 0));
                                    if *root_def_b >= header_id { continue; }
                                    let mut parent = root;
                                    for &(feeder, ck, kreg, ref fty, fast) in chain.iter().rev() {
                                        let dk = (parent, ck, if ck.is_some() { None } else { Some(kreg) });
                                        let hh = if let Some(&hh) = child_regs.get(&dk) {
                                            hh
                                        } else {
                                            // mint k BEFORE h: the const-hop vreg
                                            // order is frozen in the single-hop locks
                                            let key_mint = ck.map(|c| {
                                                let k = next_vreg; next_vreg += 1;
                                                (k, c)
                                            });
                                            let hh = next_vreg; next_vreg += 1;
                                            match key_mint {
                                                Some((k, c)) => mints.push((hh, Some(c), k, parent, fty.clone(), fast)),
                                                None => mints.push((hh, None, kreg, parent, fty.clone(), fast)),
                                            }
                                            child_regs.insert(dk, hh);
                                            hh
                                        };
                                        parent = hh;
                                        orphan_feeders.insert(feeder);
                                    }
                                    parent
                                };
                                let rewritten = match instr {
                                    Instruction::SetTable { key, val, ty, .. } =>
                                        Instruction::SetTableFast { table: h, key: *key, val: *val, ty: ty.clone() },
                                    Instruction::GetTable { target, key, ty, .. } =>
                                        Instruction::GetTableFast { target: *target, table: h, key: *key, ty: ty.clone() },
                                    _ => unreachable!(),
                                };
                                upgrades.push((blk, i, rewritten));
                                root_max_off.entry(h)
                                    .and_modify(|m| *m = (*m).max(off))
                                    .or_insert(off);
                                hoists.insert(h);
                            }
                        }
                        if !mints.is_empty() {
                            // pre-header placement, before PASS 3 finds
                            // it again for S5: [LoadInt c', GetTable h]
                            // must precede the EC/Hoist S5 appends.
                            let mut pre = 0;
                            for b in 0..header_id {
                                if let Some(Terminator::Jump(tgt)) = &program.blocks[b].terminator
                                    && *tgt == header_id { pre = b; break; }
                            }
                            for (h, ck, key_op, parent, ty, fast) in mints {
                                if let Some(c) = ck {
                                    program.blocks[pre].instrs.push(
                                        Instruction::LoadInt { target: key_op, val: c }
                                    );
                                }
                                // in kind: a Fast hop's mint rides the root
                                // pointer its pass already hoisted + EC'd
                                // (same table, same key, same bound) — the
                                // tier4_13 fast row resolution `row = *p_t.add(i)`
                                if fast {
                                    program.blocks[pre].instrs.push(
                                        Instruction::GetTableFast { target: h, table: parent, key: key_op, ty }
                                    );
                                } else {
                                    program.blocks[pre].instrs.push(
                                        Instruction::GetTable { target: h, table: parent, key: key_op, ty }
                                    );
                                }
                            }
                        }
                        // singly-used in-region feeders die; feeders
                        // with other uses stay (still correct, just
                        // not free)
                        let mut uses: HashMap<RegId, usize> = HashMap::new();
                        for b in &program.blocks {
                            for ins in &b.instrs {
                                for u in ins.use_regs() {
                                    *uses.entry(u).or_insert(0) += 1;
                                }
                            }
                        }
                        orphan_feeders.retain(|f| uses.get(f).copied().unwrap_or(0) <= 1);
                    }

                    // PASS 3: Mutate!
                    for (blk, i, new_instr) in upgrades {
                        program.blocks[blk].instrs[i] = new_instr;
                    }
                    // tier-4 cleanup: the rewrites above dropped the
                    // feeders' last use — remove the dead reads (dyn
                    // and fast: the mint re-does the identical read,
                    // panics included, in a dominating position)
                    if !orphan_feeders.is_empty() {
                        for &blk in &region {
                            program.blocks[blk].instrs.retain(|ins| {
                                !matches!(ins,
                                    Instruction::GetTable { target, .. } | Instruction::GetTableFast { target, .. }
                                    if orphan_feeders.contains(target))
                            });
                        }
                    }

                    let mut pre_header_id = 0;
                    for b in 0..header_id {
                        if let Some(Terminator::Jump(tgt)) = &program.blocks[b].terminator
                            && *tgt == header_id
                        {
                            pre_header_id = b;
                            break;
                        }
                    }

                    // S5: EC before HR in Pre-Header (Tier 2 Sizing)
                    let limit_lit: Option<i64> = def_map.get(&limit_reg).and_then(|&(b, i)| {
                        if let Instruction::LoadInt { val, .. } = &program.blocks[b].instrs[i] {
                            Some(*val)
                        } else {
                            None
                        }
                    });
                    for table in hoists {
                        let m = root_max_off.get(&table).copied().unwrap_or(0).max(global_max_off);
                        let ec_limit = if m <= 0 {
                            limit_reg
                        } else if let Some(v) = limit_lit {
                            let r = next_vreg; next_vreg += 1;
                            program.blocks[pre_header_id].instrs.push(
                                Instruction::LoadInt { target: r, val: v.wrapping_add(m) }
                            );
                            r
                        } else {
                            let c = next_vreg; next_vreg += 1;
                            program.blocks[pre_header_id].instrs.push(
                                Instruction::LoadInt { target: c, val: m }
                            );
                            let a = next_vreg; next_vreg += 1;
                            program.blocks[pre_header_id].instrs.push(
                                Instruction::Add { target: a, left: limit_reg, right: c }
                            );
                            a
                        };
                        program.blocks[pre_header_id].instrs.push(
                            Instruction::EnsureCapacity { table, limit: ec_limit }
                        );
                        program.blocks[pre_header_id].instrs.push(
                            Instruction::HoistRawPtr { table }
                        );
                    }
                }
            }
        }
    }
}
