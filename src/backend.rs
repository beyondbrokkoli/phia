// src/backend.rs
use crate::ir::{IrProgram, Instruction, Terminator, BasicBlock, BlockId, RegId};
use std::collections::{HashMap, HashSet, BTreeSet};
use crate::ast::StaticType;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Pool { Int, Float, Bool, Table, TableFloat }

fn pool_of(t: &StaticType) -> Pool {
    match t {
        StaticType::Integer => Pool::Int,
        StaticType::Float => Pool::Float,
        StaticType::Boolean => Pool::Bool,
        // A table whose ELEMENTS are floats is its own pool: its pointer
        // pair is *mut f64/farray, and a physical id must never serve
        // both it and a handle-array table — the per-id pointer decl and
        // EC/HR field choice would be ambiguous (found by the Float
        // Gauntlet: fa_tide's slot reused by fd_grid; pinned by
        // gauntlet_float + float_14).
        StaticType::Table(inner) if matches!(**inner, StaticType::Float) => Pool::TableFloat,
        StaticType::Table(_) | StaticType::UnknownTable(_) => Pool::Table,
    }
}

fn def_reg(i: &Instruction) -> Option<RegId> {
    match i {
        Instruction::LoadInt { target, .. } | Instruction::LoadFloat { target, .. }
        | Instruction::LoadBool { target, .. }
        | Instruction::NewTable { target, .. }
        | Instruction::GetTable { target, .. } | Instruction::GetTableFast { target, .. }
        | Instruction::Move { target, .. } | Instruction::Add { target, .. }
        | Instruction::Sub { target, .. } | Instruction::Less { target, .. }
        | Instruction::Mul { target, .. } | Instruction::Div { target, .. }
        | Instruction::IntDiv { target, .. } | Instruction::Mod { target, .. }
        | Instruction::Neg { target, .. }
        | Instruction::Leq { target, .. } | Instruction::Geq { target, .. }
        | Instruction::Eq { target, .. } | Instruction::Not { target, .. }
        | Instruction::Phi { target, .. } => Some(*target),
        _ => None,
    }
}

fn def_type(i: &Instruction) -> Option<StaticType> {
    match i {
        Instruction::LoadInt { .. } | Instruction::Add { .. } | Instruction::Sub { .. }
        | Instruction::Mul { .. } | Instruction::Div { .. } | Instruction::IntDiv { .. }
        | Instruction::Mod { .. } | Instruction::Neg { .. } => Some(StaticType::Integer),
        Instruction::LoadFloat { .. } => Some(StaticType::Float),
        Instruction::GetTable { ty, .. } | Instruction::GetTableFast { ty, .. } => Some(ty.clone()),
        Instruction::Less { .. } | Instruction::Leq { .. } | Instruction::Geq { .. }
        | Instruction::Eq { .. } | Instruction::Not { .. }
        | Instruction::LoadBool { .. } => Some(StaticType::Boolean),
        Instruction::NewTable { ty, .. } => Some(ty.clone()),
        Instruction::Move { ty, .. } | Instruction::Phi { ty, .. } => Some(ty.clone()),
        _ => None,
    }
}

fn use_regs(i: &Instruction) -> Vec<RegId> {
    match i {
        Instruction::LoadInt { .. } | Instruction::LoadFloat { .. } | Instruction::LoadBool { .. }
        | Instruction::NewTable { .. } => vec![],
        Instruction::Move { source, .. } | Instruction::Neg { source, .. } => vec![*source],
        Instruction::Add { left, right, .. } | Instruction::Sub { left, right, .. }
        | Instruction::Less { left, right, .. } | Instruction::Mul { left, right, .. }
        | Instruction::Div { left, right, .. } | Instruction::IntDiv { left, right, .. }
        | Instruction::Mod { left, right, .. }
        | Instruction::Leq { left, right, .. } | Instruction::Geq { left, right, .. }
        | Instruction::Eq { left, right, .. } =>
            vec![*left, *right],
        Instruction::Not { source, .. } => vec![*source],
        Instruction::SetTable { table, key, val, .. } | Instruction::SetTableFast { table, key, val, .. } => vec![*table, *key, *val],
        Instruction::GetTable { table, key, .. } | Instruction::GetTableFast { table, key, .. } => vec![*table, *key],
        Instruction::EnsureCapacity { table, limit } => vec![*table, *limit],
        Instruction::HoistRawPtr { table } => vec![*table],
        Instruction::DebugProbe { operands, .. } =>
            operands.iter().map(|&(r, _)| r).collect(),
        Instruction::Phi { args, .. } => args.iter().map(|&(_, r)| r).collect(),
    }
}

fn remap_instr<F: Fn(RegId) -> RegId>(i: &mut Instruction, f: &F) {
    let g = |r: &mut RegId| *r = f(*r);
    match i {
        Instruction::LoadInt { target, .. } | Instruction::LoadFloat { target, .. }
        | Instruction::LoadBool { target, .. }
        | Instruction::NewTable { target, .. } => g(target),
        Instruction::SetTable { table, key, val, .. } | Instruction::SetTableFast { table, key, val, .. } => { g(table); g(key); g(val); }
        Instruction::GetTable { target, table, key, .. } | Instruction::GetTableFast { target, table, key, .. } => { g(target); g(table); g(key); }
        Instruction::Move { target, source, .. } => { g(target); g(source); }
        Instruction::Add { target, left, right } | Instruction::Sub { target, left, right }
        | Instruction::Less { target, left, right } | Instruction::Mul { target, left, right }
        | Instruction::Div { target, left, right } | Instruction::IntDiv { target, left, right }
        | Instruction::Mod { target, left, right }
        | Instruction::Leq { target, left, right } | Instruction::Geq { target, left, right }
        | Instruction::Eq { target, left, right, .. } =>
            { g(target); g(left); g(right); }
        Instruction::Neg { target, source } | Instruction::Not { target, source } => { g(target); g(source); }
        Instruction::Phi { target, args, .. } => { g(target); for (_, r) in args.iter_mut() { g(r); } }
        Instruction::EnsureCapacity { table, limit } => { g(table); g(limit); }
        Instruction::HoistRawPtr { table } => g(table),
        Instruction::DebugProbe { operands, .. } =>
            { for (r, _) in operands.iter_mut() { g(r); } }
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

fn indent(d: usize) -> String { "    ".repeat(d) }

// DUAL-TEMPLATE GATE, shared with the build.rs probe-map sidecar (which
// must render the same table tokens the runtime PROBE line prints). A
// Table element type can only be born from a table-typed table op
// (checker unification is its only producer), so this predicate is
// exactly "the program nests tables": nested programs render handle-mode
// templates (nested_*/tier4_* locks), pure-integer programs the frozen
// pointer templates (all others).
pub fn program_uses_handles(blocks: &[BasicBlock]) -> bool {
    blocks.iter().any(|b| b.instrs.iter().any(|i| match i {
        Instruction::SetTable { ty, .. } | Instruction::SetTableFast { ty, .. }
        | Instruction::GetTable { ty, .. } | Instruction::GetTableFast { ty, .. } =>
            matches!(ty, StaticType::Table(_) | StaticType::UnknownTable(_)),
        _ => false,
    }))
}

pub struct IrBackend {
    pub program: IrProgram,
    n_int: usize, n_bool: usize, n_float: usize, n_table: usize, n_ftable: usize,
    phys_base: RegId, float_base: RegId, ftable_base: RegId, did_alloc: bool,
    consts_i: HashMap<RegId, i64>,
    consts_b: HashMap<RegId, bool>,
    // physical reg -> pool, captured by allocate_registers: emission needs
    // it to pick i_r/f_r renderings for the untyped arith instructions.
    // Float physicals live in a disjoint id range, so the bare-id lookup
    // is unambiguous.
    phys_pools: HashMap<RegId, Pool>,
}

impl IrBackend {
    pub fn new(program: IrProgram) -> Self {
        Self {
            program,
            n_int: 0, n_bool: 0, n_float: 0, n_table: 0, n_ftable: 0,
            phys_base: 0, float_base: 0, ftable_base: 0, did_alloc: false,
            consts_i: HashMap::new(), consts_b: HashMap::new(),
            phys_pools: HashMap::new(),
        }
    }

    fn iop_str(&self, r: RegId) -> String {
        self.consts_i.get(&r).map(|v| v.to_string())
            .unwrap_or_else(|| format!("i_r{r}"))
    }
    fn bop_str(&self, r: RegId) -> String {
        self.consts_b.get(&r)
            .map(|v| if *v { "true" } else { "false" }.to_string())
            .unwrap_or_else(|| format!("b_r{r}"))
    }
    // Floats are never compile-time folded (no consts_f), so a float
    // operand is always a physical register.
    fn fop_str(&self, r: RegId) -> String { format!("f_r{r}") }
    fn is_float_reg(&self, r: RegId) -> bool {
        self.phys_pools.get(&r) == Some(&Pool::Float)
    }

    // Storage side of a hoisted/EC'd table physical: float-ELEMENT tables
    // live in their own pool, so the id alone answers farray vs array
    // (and *mut f64 vs *mut i64 in the decl block) unambiguously.
    fn is_ftable_reg(&self, r: RegId) -> bool {
        self.phys_pools.get(&r) == Some(&Pool::TableFloat)
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
        // A loop header has a back edge: a LATER block jumping to it, from
        // inside its own body. The second clause is load-bearing: the
        // lowerer mints if-arm blocks (including nested ifs' joins) AFTER
        // the enclosing join, so a nested if's inner join jumps backward
        // to the outer join — a back edge in id space, but not a loop.
        // For a real header the back-jumper is reachable from the header's
        // own branch successors (the body flows to the back edge); an
        // if-join's jumper sits in a sibling arm nothing downstream
        // reaches. Found by feat_if_02 (nested if, then another if in the
        // same join block): the false-positive header hit emit_loop twice.
        let Some(Terminator::Branch { true_block, false_block, .. }) =
            &self.program.blocks[h].terminator else { return false; };
        let mut body_reach = self.reachable_from(*true_block);
        body_reach.extend(self.reachable_from(*false_block));
        self.program.blocks[h + 1..].iter().enumerate().any(|(i, p)| {
            matches!(&p.terminator, Some(Terminator::Jump(t)) if *t == h)
                && body_reach.contains(&(h + 1 + i))
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
                    Instruction::LoadInt { .. } | Instruction::LoadBool { .. }
                    | Instruction::Move { .. }
                    | Instruction::Add { .. } | Instruction::Sub { .. } | Instruction::Less { .. }
                    | Instruction::Mul { .. } | Instruction::Div { .. }
                    | Instruction::IntDiv { .. } | Instruction::Mod { .. }
                    | Instruction::Neg { .. }
                    | Instruction::Leq { .. } | Instruction::Geq { .. }
                    | Instruction::Eq { .. } | Instruction::Not { .. });
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
                                // float constant folding is not implemented —
                                // float operands always render as registers
                                StaticType::Float => {}
                                StaticType::Table(_) | StaticType::UnknownTable(_) => {}
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
        // The arithmetic ops are untyped in the IR: their targets inherit
        // the operand pool (Integer or Float) — pass 2 fixpoints that
        // through chains (`y = x + 1` needs x's pool first).
        let mut ty: HashMap<RegId, Pool> = HashMap::new();
        for b in blocks {
            for i in &b.instrs {
                let (Some(d), Some(t)) = (def_reg(i), def_type(i)) else { continue };
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
                if let Some(d) = def_reg(i) { if d > max_reg { max_reg = d; } }
                for u in use_regs(i) { if u > max_reg { max_reg = u; } }
            }
            if let Some(Terminator::Branch { cond, .. }) = &b.terminator {
                if *cond > max_reg { max_reg = *cond; }
            }
        }
        let base = max_reg + 1;

        let mut vregs: Vec<RegId> = ty.keys().copied().collect();

        // Int/Bool/Table deliberately SHARE the physical id space (i_r12,
        // b_r12, t_r12 coexist — the prefix disambiguates; that layout is
        // what every integer lock freezes). Float scalars mint from a
        // disjoint range (emission asks "is reg N float?" from N alone),
        // and float-ELEMENT tables mint from a second disjoint range on
        // top: their pointer pair is *mut f64/farray, so a physical id
        // must never serve both a handle table and a float table — the
        // per-id pointer decl and EC/HR field choice would be ambiguous.
        // Pure-integer programs mint none of these: integer ids, integer
        // bytes, byte-for-byte.
        let pool_count = |p: Pool| ty.values().filter(|&&q| q == p).count();
        let max_other = pool_count(Pool::Int)
            .max(pool_count(Pool::Bool))
            .max(pool_count(Pool::Table));
        let float_base = base + max_other as RegId;
        let ftable_base = float_base + pool_count(Pool::Float) as RegId;

        // LOAD-BEARING: the `r` tiebreak makes this a total order. Without it,
        // equal-interval regs fall back to HashMap iteration order (random per
        // process) and allocation becomes non-deterministic.
        vregs.sort_by_key(|&r| (iv.get(&r).copied().unwrap_or((0, 0)), r));

        let mut active: HashMap<Pool, Vec<(RegId, usize)>> = HashMap::new();
        let mut free: HashMap<Pool, Vec<RegId>> = HashMap::new();
        let mut count: HashMap<Pool, usize> = HashMap::new();
        let mut map: HashMap<RegId, RegId> = HashMap::new();
        let mut phys_pools: HashMap<RegId, Pool> = HashMap::new();

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
                    _ => base + n as RegId,
                }
            });
            act.push((phys, end));
            map.insert(r, phys);
            phys_pools.insert(phys, p);
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
        self.n_float = *count.entry(Pool::Float).or_insert(0);
        self.n_table = *count.entry(Pool::Table).or_insert(0);
        self.n_ftable = *count.entry(Pool::TableFloat).or_insert(0);
        self.phys_base = base;
        self.float_base = float_base;
        self.ftable_base = ftable_base;
        self.phys_pools = phys_pools;
        self.did_alloc = true;
    }

    pub fn optimize(&mut self) {
        let mut def_map: HashMap<RegId, (BlockId, usize)> = HashMap::new();
        for block in &self.program.blocks {
            for (i, instr) in block.instrs.iter().enumerate() {
                match instr {
                    Instruction::LoadInt { target, .. } | Instruction::LoadFloat { target, .. }
                    | Instruction::LoadBool { target, .. }
                    | Instruction::NewTable { target, .. } |
                    Instruction::GetTable { target, .. } | Instruction::Move { target, .. } |
                    Instruction::Add { target, .. } | Instruction::Sub { target, .. } |
                    Instruction::Less { target, .. } | Instruction::Phi { target, .. } |
                    Instruction::Mul { target, .. } | Instruction::Div { target, .. } |
                    Instruction::IntDiv { target, .. } | Instruction::Mod { target, .. } |
                    Instruction::Neg { target, .. } |
                    Instruction::Leq { target, .. } | Instruction::Geq { target, .. } |
                    Instruction::Eq { target, .. } | Instruction::Not { target, .. } |
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
        let mut next_vreg: RegId = self.program.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .flat_map(|i| {
                let mut regs = use_regs(i);
                if let Some(d) = def_reg(i) { regs.push(d); }
                regs
            })
            .chain(self.program.blocks.iter().filter_map(|b| match &b.terminator {
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
                        Instruction::Move { source, ty, .. }
                            if matches!(ty, StaticType::Table(_) | StaticType::UnknownTable(_)) =>
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
                    Instruction::Move { source, ty, .. }
                        if matches!(ty, StaticType::Table(_) | StaticType::UnknownTable(_)) =>
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
            for b in &self.program.blocks {
                for ins in &b.instrs {
                    let (Instruction::SetTable { table, val, ty, .. }
                    | Instruction::SetTableFast { table, val, ty, .. }) = ins else { continue };
                    if !matches!(ty, StaticType::Table(_) | StaticType::UnknownTable(_)) { continue; }
                    let Some(dest) = get_table_root(&self.program.blocks, *table) else { continue };
                    match handle_origin(&self.program.blocks, *val) {
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
                    Instruction::Move { source, ty, .. }
                        if matches!(ty, StaticType::Table(_) | StaticType::UnknownTable(_)) =>
                    {
                        curr = *source;
                    }
                    Instruction::GetTable { table, .. } | Instruction::GetTableFast { table, .. } =>
                        return get_table_root(blocks, *table),
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

                            let mut region = loop_region(&self.program.blocks, header_id, body_id);
                            region.sort_unstable();

                            let mut root_max_off: HashMap<RegId, i64> = HashMap::new();
                            let mut global_max_off: i64 = 0;
                            let mut abort_all = false;

                            // PASS 1: Read-Only, REGION-WIDE. (Tier 2 poison check)
                            'poison: for &blk in &region {
                                for instr in &self.program.blocks[blk].instrs {
                                    match instr {
                                        Instruction::SetTable { table, key, ty, .. } => {
                                            if let Some(root) = get_table_root(&self.program.blocks, *table) {
                                                region_stored_roots.insert(root);
                                            }
                                            match key_offset(&self.program.blocks, *key, idx_reg) {
                                                None => match get_table_root(&self.program.blocks, *table) {
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
                                                            match child_parent_root(&self.program.blocks, *table) {
                                                                Some(r) => { child_store_roots.insert(r); }
                                                                None => { abort_all = true; break 'poison; }
                                                            }
                                                        } else {
                                                            abort_all = true; break 'poison;
                                                        }
                                                    }
                                                },
                                                Some(off) if off < 0 => {} // Cannot resize
                                                Some(off) => match get_table_root(&self.program.blocks, *table) {
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
                                            if let Some(off) = key_offset(&self.program.blocks, *key, idx_reg) {
                                                if off >= 0 {
                                                    if let Some(root) = get_table_root(&self.program.blocks, *table) {
                                                        root_max_off.entry(root)
                                                            .and_modify(|m| *m = (*m).max(off))
                                                            .or_insert(off);
                                                    }
                                                }
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
                                for (i, instr) in self.program.blocks[blk].instrs.iter().enumerate() {
                                    match instr {
                                        Instruction::SetTable { table, key, val, ty } => {
                                            if let Some(root) = get_table_root(&self.program.blocks, *table) {
                                                // TIER 2b: ROOT-based S2 Dominance.
                                                let (root_def_b, _) = def_map.get(&root).unwrap_or(&(0,0));
                                                if *root_def_b >= header_id { continue; }

                                                if key_offset(&self.program.blocks, *key, idx_reg).map_or(false, |off| off >= 0)
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
                                            if let Some(root) = get_table_root(&self.program.blocks, *table) {
                                                // TIER 2b: ROOT-based S2 Dominance.
                                                let (root_def_b, _) = def_map.get(&root).unwrap_or(&(0,0));
                                                if *root_def_b >= header_id { continue; }

                                                if key_offset(&self.program.blocks, *key, idx_reg).map_or(false, |off| off >= 0)
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
                            let tier4_lim = const_eval(&self.program.blocks, &def_map, limit_reg);
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
                                Some(&(b, i)) => match &self.program.blocks[b].instrs[i] {
                                    Instruction::Phi { args, .. } => args.iter()
                                        .find(|&&(pb, _)| pb < header_id)
                                        .and_then(|&(_, r)| const_eval(&self.program.blocks, &def_map, r)),
                                    _ => const_eval(&self.program.blocks, &def_map, idx_reg),
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
                                    for (i, instr) in self.program.blocks[blk].instrs.iter().enumerate() {
                                        let (table_op, own_key) = match instr {
                                            Instruction::SetTable { table, key, .. }
                                            | Instruction::GetTable { table, key, .. } => (*table, *key),
                                            _ => continue,
                                        };
                                        // flat-eligible ops traced to roots in PASS 2 — not ours
                                        if get_table_root(&self.program.blocks, table_op).is_some() { continue; }
                                        let Some(&(fb, fi)) = def_map.get(&table_op) else { continue };
                                        let Some(off) = key_offset(&self.program.blocks, own_key, idx_reg) else { continue };
                                        if off < 0 { continue; }

                                        // The replacement handle the op will target.
                                        let h: RegId;
                                        if !region_set.contains(&fb) {
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
                                                if let Some(Terminator::Jump(tgt)) = &self.program.blocks[b].terminator {
                                                    if *tgt == header_id { pre = Some(b); break; }
                                                }
                                            }
                                            if pre != Some(fb) { continue; }
                                            let root = match &self.program.blocks[fb].instrs[fi] {
                                                Instruction::GetTable { table, .. } =>
                                                    get_table_root(&self.program.blocks, *table),
                                                Instruction::GetTableFast { table, .. } => Some(*table),
                                                _ => None,
                                            };
                                            let Some(root) = root else { continue };
                                            if region_stored_roots.contains(&root) { continue; }
                                            let (root_def_b, _) = def_map.get(&root).unwrap_or(&(0, 0));
                                            if *root_def_b >= header_id { continue; }
                                            h = table_op;
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
                                                let (table, key, ty, fast) = match &self.program.blocks[cb].instrs[ci] {
                                                    Instruction::GetTable { table, key, ty, .. } => (table, key, ty, false),
                                                    Instruction::GetTableFast { table, key, ty, .. } => (table, key, ty, true),
                                                    _ => break,
                                                };
                                                let ck = match const_eval(&self.program.blocks, &def_map, *key) {
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
                                            let Some(root) = get_table_root(&self.program.blocks, cur) else { continue };
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
                                            h = parent;
                                        }
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
                                        if let Some(Terminator::Jump(tgt)) = &self.program.blocks[b].terminator {
                                            if *tgt == header_id { pre = b; break; }
                                        }
                                    }
                                    for (h, ck, key_op, parent, ty, fast) in mints {
                                        if let Some(c) = ck {
                                            self.program.blocks[pre].instrs.push(
                                                Instruction::LoadInt { target: key_op, val: c }
                                            );
                                        }
                                        // in kind: a Fast hop's mint rides the root
                                        // pointer its pass already hoisted + EC'd
                                        // (same table, same key, same bound) — the
                                        // tier4_13 fast row resolution `row = *p_t.add(i)`
                                        if fast {
                                            self.program.blocks[pre].instrs.push(
                                                Instruction::GetTableFast { target: h, table: parent, key: key_op, ty }
                                            );
                                        } else {
                                            self.program.blocks[pre].instrs.push(
                                                Instruction::GetTable { target: h, table: parent, key: key_op, ty }
                                            );
                                        }
                                    }
                                }
                                // singly-used in-region feeders die; feeders
                                // with other uses stay (still correct, just
                                // not free)
                                let mut uses: HashMap<RegId, usize> = HashMap::new();
                                for b in &self.program.blocks {
                                    for ins in &b.instrs {
                                        for u in use_regs(ins) {
                                            *uses.entry(u).or_insert(0) += 1;
                                        }
                                    }
                                }
                                orphan_feeders.retain(|f| uses.get(f).copied().unwrap_or(0) <= 1);
                            }

                            // PASS 3: Mutate!
                            for (blk, i, new_instr) in upgrades {
                                self.program.blocks[blk].instrs[i] = new_instr;
                            }
                            // tier-4 cleanup: the rewrites above dropped the
                            // feeders' last use — remove the dead reads (dyn
                            // and fast: the mint re-does the identical read,
                            // panics included, in a dominating position)
                            if !orphan_feeders.is_empty() {
                                for &blk in &region {
                                    self.program.blocks[blk].instrs.retain(|ins| {
                                        !matches!(ins,
                                            Instruction::GetTable { target, .. } | Instruction::GetTableFast { target, .. }
                                            if orphan_feeders.contains(target))
                                    });
                                }
                            }

                            let mut pre_header_id = 0;
                            for b in 0..header_id {
                                if let Some(Terminator::Jump(tgt)) = &self.program.blocks[b].terminator {
                                    if *tgt == header_id {
                                        pre_header_id = b;
                                        break;
                                    }
                                }
                            }

                            // S5: EC before HR in Pre-Header (Tier 2 Sizing)
                            let limit_lit: Option<i64> = def_map.get(&limit_reg).and_then(|&(b, i)| {
                                if let Instruction::LoadInt { val, .. } = &self.program.blocks[b].instrs[i] {
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
                                    self.program.blocks[pre_header_id].instrs.push(
                                        Instruction::LoadInt { target: r, val: v.wrapping_add(m) }
                                    );
                                    r
                                } else {
                                    let c = next_vreg; next_vreg += 1;
                                    self.program.blocks[pre_header_id].instrs.push(
                                        Instruction::LoadInt { target: c, val: m }
                                    );
                                    let a = next_vreg; next_vreg += 1;
                                    self.program.blocks[pre_header_id].instrs.push(
                                        Instruction::Add { target: a, left: limit_reg, right: c }
                                    );
                                    a
                                };
                                self.program.blocks[pre_header_id].instrs.push(
                                    Instruction::EnsureCapacity { table, limit: ec_limit }
                                );
                                self.program.blocks[pre_header_id].instrs.push(
                                    Instruction::HoistRawPtr { table }
                                );
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

    fn emit_table_decl(&self, out: &mut String, r: RegId, uses_handles: bool, fast_phys: &HashSet<RegId>) {
        if uses_handles {
            out.push_str(&format!("    let mut t_r{r} = 0i64;\n"));
        } else {
            out.push_str(&format!("    let mut t_r{r}: *mut Table = std::ptr::null_mut();\n"));
        }
        if fast_phys.contains(&r) {
            let ptr_ty = if self.is_ftable_reg(r) { "*mut f64" } else { "*mut i64" };
            out.push_str(&format!("    let mut p_r{r}: {ptr_ty} = std::ptr::null_mut();\n"));
            out.push_str(&format!("    let mut len_r{r} = 0usize;\n"));
        }
    }

    fn emit_instr(&self, out: &mut String, instr: &Instruction, d: usize, uses_handles: bool) {
        // compile-time-computed defs emit nothing: their uses are literals
        if let Some(t) = def_reg(instr) {
            if self.consts_i.contains_key(&t) || self.consts_b.contains_key(&t) {
                return;
            }
        }
        let ind = indent(d);
        // In handle mode a table-typed operand renders as its t_r handle reg.
        // Checked arena resolution (tables.get/get_mut + nil panic) instead of
        // get_unchecked: even a hypothetical checker bug degrades to a clean
        // "Runtime Error", never UB. Handle mode only — the pointer templates
        // below are frozen, byte-identical to the milestone locks.
        let is_tbl = |ty: &StaticType| matches!(ty, StaticType::Table(_) | StaticType::UnknownTable(_));
        match instr {
            Instruction::LoadInt { target, val } =>
                out.push_str(&format!("{ind}i_r{target} = {val};\n")),
            Instruction::LoadFloat { target, val } =>
                out.push_str(&format!("{ind}f_r{target} = {val:?};\n")),
            Instruction::LoadBool { target, val } =>
                out.push_str(&format!("{ind}b_r{target} = {val};\n")),
            Instruction::NewTable { target, ty } => {
                // A table is monomorphic: its static element type picks the
                // storage side at construction and it never changes.
                let float_tbl = matches!(ty, StaticType::Table(inner) if matches!(**inner, StaticType::Float));
                if uses_handles {
                    // 1-based arena handle; 0 stays reserved for null
                    if float_tbl {
                        out.push_str(&format!(
                            "{ind}tables.push(Box::new(Table::new_float()));\n\
                             {ind}t_r{target} = tables.len() as i64;\n"
                        ));
                    } else {
                        out.push_str(&format!(
                            "{ind}tables.push(Box::new(Table::new()));\n\
                             {ind}t_r{target} = tables.len() as i64;\n"
                        ));
                    }
                } else if float_tbl {
                    out.push_str(&format!(
                        "{ind}let mut new_table = Box::new(Table::new_float());\n\
                         {ind}t_r{target} = &mut *new_table as *mut Table;\n\
                         {ind}tables.push(new_table);\n"
                    ));
                } else {
                    out.push_str(&format!(
                        "{ind}let mut new_table = Box::new(Table::new());\n\
                         {ind}t_r{target} = &mut *new_table as *mut Table;\n\
                         {ind}tables.push(new_table);\n"
                    ));
                }
            }
            Instruction::Move { target, source, ty } => match ty {
                StaticType::Integer => out.push_str(&format!("{ind}i_r{target} = {};\n", self.iop_str(*source))),
                StaticType::Boolean => out.push_str(&format!("{ind}b_r{target} = {};\n", self.bop_str(*source))),
                StaticType::Float => out.push_str(&format!("{ind}f_r{target} = {};\n", self.fop_str(*source))),
                StaticType::Table(_) | StaticType::UnknownTable(_) => out.push_str(&format!("{ind}t_r{target} = t_r{source};\n")),
            },
            Instruction::Add { target, left, right } => {
                if self.is_float_reg(*target) {
                    out.push_str(&format!("{ind}f_r{target} = {} + {};\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!("{ind}i_r{target} = {} + {};\n", self.iop_str(*left), self.iop_str(*right)))
                }
            }
            Instruction::Sub { target, left, right } => {
                if self.is_float_reg(*target) {
                    out.push_str(&format!("{ind}f_r{target} = {} - {};\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!("{ind}i_r{target} = {} - {};\n", self.iop_str(*left), self.iop_str(*right)))
                }
            }
            Instruction::Mul { target, left, right } => {
                if self.is_float_reg(*target) {
                    out.push_str(&format!("{ind}f_r{target} = {} * {};\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!("{ind}i_r{target} = {} * {};\n", self.iop_str(*left), self.iop_str(*right)))
                }
            }
            Instruction::Div { target, left, right } => {
                if self.is_float_reg(*target) {
                    out.push_str(&format!("{ind}f_r{target} = {} / {};\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!("{ind}i_r{target} = {} / {};\n", self.iop_str(*left), self.iop_str(*right)))
                }
            }
            // Lua floor division: `//` rounds toward negative infinity
            // (-7 // 2 == -4), on both the integer and float sides. The
            // integer side is spelled explicitly (trunc quotient, minus one
            // when a remainder exists and disagrees with the divisor's
            // sign) rather than via div_floor — this toolchain predates
            // stabilized int_roundings. (`/` above is the pinned
            // divergence: Lua's `/` always yields a float and strict
            // typing forbids that, so integer `/` is truncating division
            // and float `/` is plain division.) /0 and MIN/-1 still panic
            // inside the leading L / R.
            Instruction::IntDiv { target, left, right } => {
                if self.is_float_reg(*target) {
                    out.push_str(&format!("{ind}f_r{target} = ({} / {}).floor();\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!(
                        "{ind}i_r{target} = {} / {} - i64::from({} % {} != 0 && ({} < 0) != ({} < 0));\n",
                        self.iop_str(*left), self.iop_str(*right),
                        self.iop_str(*left), self.iop_str(*right),
                        self.iop_str(*left), self.iop_str(*right)))
                }
            }
            // Lua modulo: result takes the divisor's sign (-7 % 3 == 2),
            // unlike Rust's truncated remainder — adjust the remainder by
            // the divisor exactly when the two signs disagree.
            Instruction::Mod { target, left, right } => {
                if self.is_float_reg(*target) {
                    out.push_str(&format!("{ind}f_r{target} = {} - ({} / {}).floor() * {};\n",
                        self.fop_str(*left), self.fop_str(*left), self.fop_str(*right), self.fop_str(*right)))
                } else {
                    out.push_str(&format!(
                        "{ind}i_r{target} = {} % {} + i64::from({} % {} != 0 && ({} % {} < 0) != ({} < 0)) * {};\n",
                        self.iop_str(*left), self.iop_str(*right),
                        self.iop_str(*left), self.iop_str(*right),
                        self.iop_str(*left), self.iop_str(*right),
                        self.iop_str(*right), self.iop_str(*right)))
                }
            }
            Instruction::Neg { target, source } => {
                if self.is_float_reg(*target) {
                    out.push_str(&format!("{ind}f_r{target} = -{};\n", self.fop_str(*source)))
                } else {
                    out.push_str(&format!("{ind}i_r{target} = -{};\n", self.iop_str(*source)))
                }
            }
            Instruction::Less { target, left, right } => {
                if self.is_float_reg(*left) {
                    out.push_str(&format!("{ind}b_r{target} = {} < {};\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!("{ind}b_r{target} = {} < {};\n", self.iop_str(*left), self.iop_str(*right)))
                }
            }
            Instruction::Leq { target, left, right } => {
                if self.is_float_reg(*left) {
                    out.push_str(&format!("{ind}b_r{target} = {} <= {};\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!("{ind}b_r{target} = {} <= {};\n", self.iop_str(*left), self.iop_str(*right)))
                }
            }
            Instruction::Geq { target, left, right } => {
                if self.is_float_reg(*left) {
                    out.push_str(&format!("{ind}b_r{target} = {} >= {};\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!("{ind}b_r{target} = {} >= {};\n", self.iop_str(*left), self.iop_str(*right)))
                }
            }
            // The instruction's ty is the single source of truth for the
            // rendering: Int and Bool physicals share one id range, so the
            // pool tables cannot tell an int operand from a bool operand
            // (a const-bool-on-the-left Eq used to render undeclared i_rN
            // registers, and an aliased physical bool rendered the wrong
            // variable entirely).
            Instruction::Eq { target, left, right, ty } => match ty {
                StaticType::Float =>
                    out.push_str(&format!("{ind}b_r{target} = {} == {};\n", self.fop_str(*left), self.fop_str(*right))),
                StaticType::Boolean =>
                    out.push_str(&format!("{ind}b_r{target} = {} == {};\n", self.bop_str(*left), self.bop_str(*right))),
                _ =>
                    out.push_str(&format!("{ind}b_r{target} = {} == {};\n", self.iop_str(*left), self.iop_str(*right))),
            },
            Instruction::Not { target, source } =>
                out.push_str(&format!("{ind}b_r{target} = !{};\n", self.bop_str(*source))),

            // Runtime observation. One line per trip, naming each operand's
            // physical slot: the runtime line names the exact register
            // ir_final_cfg.txt shows, which is what joins a run back to its
            // dump. Determinism contract: values, handles and arena-derived
            // lengths only — never addresses — so PROBE lines stay
            // re-derivable pins. A table operand prints its handle (0 is
            // the one observable nil in the language) plus its materialized
            // length; a nil handle's len renders as MAX, a sentinel no real
            // table can collide with.
            Instruction::DebugProbe { tag, operands } => {
                let mut fmt_parts: Vec<String> = Vec::new();
                let mut args: Vec<String> = Vec::new();
                for &(r, ref t) in operands {
                    match t {
                        StaticType::Integer => {
                            fmt_parts.push(format!("i_r{r}={{}}"));
                            args.push(self.iop_str(r));
                        }
                        StaticType::Float => {
                            fmt_parts.push(format!("f_r{r}={{:?}}"));
                            args.push(self.fop_str(r));
                        }
                        StaticType::Boolean => {
                            fmt_parts.push(format!("b_r{r}={{}}"));
                            args.push(self.bop_str(r));
                        }
                        StaticType::Table(_) | StaticType::UnknownTable(_) => {
                            let fld = if self.is_ftable_reg(r) { "farray" } else { "array" };
                            if uses_handles {
                                fmt_parts.push(format!("t_r{r}={{}} len_r{r}={{}}"));
                                args.push(format!("t_r{r}"));
                                args.push(format!(
                                    "match tables.get((t_r{r} - 1) as usize) \
                                     {{ Some(t) => t.{fld}.len(), None => usize::MAX }}"
                                ));
                            } else {
                                // pointer mode: a table-typed operand is
                                // always NewTable-defined before use (only
                                // nested programs read tables out of tables,
                                // and those render handle mode), so the
                                // deref cannot see null
                                fmt_parts.push(format!("len_r{r}={{}}"));
                                args.push(format!("unsafe {{ (*t_r{r}).{fld}.len() }}"));
                            }
                        }
                    }
                }
                // a brace in the user tag would be a format directive
                let safe_tag = tag.replace('{', "{{").replace('}', "}}");
                out.push_str(&format!(
                    "{ind}println!(\"PROBE {safe_tag}: {}\", {});\n",
                    fmt_parts.join(" "),
                    args.join(", ")
                ));
            }

            Instruction::EnsureCapacity { table, limit } => {
                // Storage side comes from the table's pool: float-element
                // tables resize the farray with 0.0 zeros, handle tables
                // the frozen integer template. Monomorphism guarantees the
                // fast ops riding this EC agree with the pool.
                let (fld, zero) = if self.is_ftable_reg(*table) {
                    ("farray", "0.0")
                } else {
                    ("array", "0")
                };
                if uses_handles {
                    out.push_str(&format!(
                        "{ind}let lim = {lim};\n\
                         {ind}if lim > 0 {{\n\
                         {ind}    if t_r{table} == 0 {{ panic!(\"Runtime Error: table is nil\"); }}\n\
                         {ind}    let t = match tables.get_mut((t_r{table} - 1) as usize) {{ Some(t) => &mut **t, None => panic!(\"Runtime Error: table is nil\") }};\n\
                         {ind}    if (lim as usize) > t.{fld}.len() {{\n\
                         {ind}        t.{fld}.resize(lim as usize, {zero});\n\
                         {ind}    }}\n\
                         {ind}}}\n",
                        lim = self.iop_str(*limit)
                    ));
                } else {
                    out.push_str(&format!(
                        "{ind}let lim = {lim};\n\
                         {ind}if lim > 0 {{\n\
                         {ind}    let t = unsafe {{ &mut *t_r{table} }};\n\
                         {ind}    if (lim as usize) > t.{fld}.len() {{\n\
                         {ind}        t.{fld}.resize(lim as usize, {zero});\n\
                         {ind}    }}\n\
                         {ind}}}\n",
                        lim = self.iop_str(*limit)
                    ));
                }
            }
            Instruction::HoistRawPtr { table } => {
                let fld = if self.is_ftable_reg(*table) { "farray" } else { "array" };
                if uses_handles {
                    out.push_str(&format!(
                        "{ind}if t_r{table} == 0 {{ panic!(\"Runtime Error: table is nil\"); }}\n\
                         {ind}let t = match tables.get_mut((t_r{table} - 1) as usize) {{ Some(t) => &mut **t, None => panic!(\"Runtime Error: table is nil\") }};\n\
                         {ind}len_r{table} = t.{fld}.len();\n\
                         {ind}p_r{table} = t.{fld}.as_mut_ptr();\n"
                    ));
                } else {
                    out.push_str(&format!(
                        "{ind}len_r{table} = unsafe {{ (*t_r{table}).{fld}.len() }};\n\
                         {ind}p_r{table} = unsafe {{ (*t_r{table}).{fld}.as_mut_ptr() }};\n"
                    ));
                }
            }

            Instruction::SetTable { table, key, val, ty } => {
                // Element-kind selects storage side and zero. The integer and
                // table cases render byte-identically to the frozen templates
                // (fld="array", zero="0"); floats take the f64 side.
                if uses_handles {
                    let (fld, zero, val_str) = if is_tbl(ty) {
                        ("array", "0", format!("t_r{val}"))
                    } else if matches!(ty, StaticType::Float) {
                        ("farray", "0.0", format!("f_r{val}"))
                    } else {
                        ("array", "0", self.iop_str(*val))
                    };
                    out.push_str(&format!(
                        "{ind}let k = {key};\n\
                         {ind}if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         {ind}let idx = k as usize;\n\
                         {ind}if t_r{table} == 0 {{ panic!(\"Runtime Error: table is nil\"); }}\n\
                         {ind}let t = match tables.get_mut((t_r{table} - 1) as usize) {{ Some(t) => &mut **t, None => panic!(\"Runtime Error: table is nil\") }};\n\
                         {ind}if idx >= t.{fld}.len() {{ t.{fld}.resize(idx + 1, {zero}); }}\n\
                         {ind}unsafe {{ *t.{fld}.get_unchecked_mut(idx) = {val_str}; }}\n",
                        key = self.iop_str(*key)
                    ));
                } else {
                    let (fld, zero, val_str) = if matches!(ty, StaticType::Float) {
                        ("farray", "0.0", self.fop_str(*val))
                    } else {
                        ("array", "0", self.iop_str(*val))
                    };
                    out.push_str(&format!(
                        "{ind}let k = {key};\n\
                         {ind}if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         {ind}let idx = k as usize;\n\
                         {ind}let t = unsafe {{ &mut *t_r{table} }};\n\
                         {ind}if idx >= t.{fld}.len() {{ t.{fld}.resize(idx + 1, {zero}); }}\n\
                         {ind}unsafe {{ *t.{fld}.get_unchecked_mut(idx) = {val_str}; }}\n",
                        key = self.iop_str(*key)
                    ));
                }
            }
            Instruction::GetTable { target, table, key, ty } => {
                if uses_handles {
                    let (fld, zero, target_str) = if is_tbl(ty) {
                        ("array", "0", format!("t_r{target}"))
                    } else if matches!(ty, StaticType::Float) {
                        ("farray", "0.0", format!("f_r{target}"))
                    } else {
                        ("array", "0", format!("i_r{target}"))
                    };
                    out.push_str(&format!(
                        "{ind}let k = {key};\n\
                         {ind}if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         {ind}let idx = k as usize;\n\
                         {ind}if t_r{table} == 0 {{ panic!(\"Runtime Error: table is nil\"); }}\n\
                         {ind}let t = match tables.get((t_r{table} - 1) as usize) {{ Some(t) => &**t, None => panic!(\"Runtime Error: table is nil\") }};\n\
                         {ind}{target_str} = if idx < t.{fld}.len() {{ unsafe {{ *t.{fld}.get_unchecked(idx) }} }} else {{ {zero} }};\n",
                        key = self.iop_str(*key)
                    ));
                } else if matches!(ty, StaticType::Float) {
                    out.push_str(&format!(
                        "{ind}let k = {key};\n\
                         {ind}if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         {ind}let idx = k as usize;\n\
                         {ind}let t = unsafe {{ &*t_r{table} }};\n\
                         {ind}f_r{target} = if idx < t.farray.len() {{ unsafe {{ *t.farray.get_unchecked(idx) }} }} else {{ 0.0 }};\n",
                        key = self.iop_str(*key)
                    ));
                } else {
                    out.push_str(&format!(
                        "{ind}let k = {key};\n\
                         {ind}if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         {ind}let idx = k as usize;\n\
                         {ind}let t = unsafe {{ &*t_r{table} }};\n\
                         {ind}i_r{target} = if idx < t.array.len() {{ unsafe {{ *t.array.get_unchecked(idx) }} }} else {{ 0 }};\n",
                        key = self.iop_str(*key)
                    ));
                }
            }
            Instruction::SetTableFast { table, key, val, ty } => {
                let val_str = if uses_handles && is_tbl(ty) {
                    format!("t_r{val}")
                } else if matches!(ty, StaticType::Float) {
                    self.fop_str(*val)
                } else {
                    self.iop_str(*val)
                };
                out.push_str(&format!(
                    "{ind}let k = {key};\n\
                     {ind}if k < 0 {{ panic!(\"Runtime Error: Negative index in fast path\"); }}\n\
                     {ind}if (k as usize) < len_r{table} {{\n\
                     {ind}    unsafe {{ *p_r{table}.add(k as usize) = {val_str}; }}\n\
                     {ind}}} else {{\n\
                     {ind}    panic!(\"optimizer invariant violated: fast-path bounds check failed\");\n\
                     {ind}}}\n",
                    key = self.iop_str(*key)
                ));
            }
            Instruction::GetTableFast { target, table, key, ty } => {
                let target_str = if uses_handles && is_tbl(ty) {
                    format!("t_r{target}")
                } else if matches!(ty, StaticType::Float) {
                    format!("f_r{target}")
                } else {
                    format!("i_r{target}")
                };
                out.push_str(&format!(
                    "{ind}let k = {key};\n\
                     {ind}if k < 0 {{ panic!(\"Runtime Error: Negative index in fast path\"); }}\n\
                     {ind}if (k as usize) < len_r{table} {{\n\
                     {ind}    {target_str} = unsafe {{ *p_r{table}.add(k as usize) }};\n\
                     {ind}}} else {{\n\
                     {ind}    panic!(\"optimizer invariant violated: fast-path bounds check failed\");\n\
                     {ind}}}\n",
                    key = self.iop_str(*key)
                ));
            }
            Instruction::Phi { .. } => {} // deleted by resolve_phis
        }
    }

    /// Every block reachable from `b` via any terminator edge.
    fn reachable_from(&self, b: BlockId) -> HashSet<BlockId> {
        let mut seen = HashSet::new();
        let mut stack = vec![b];
        while let Some(x) = stack.pop() {
            if !seen.insert(x) { continue; }
            match &self.program.blocks[x].terminator {
                Some(Terminator::Jump(t)) => stack.push(*t),
                Some(Terminator::Branch { true_block, false_block, .. }) => {
                    stack.push(*true_block);
                    stack.push(*false_block);
                }
                _ => {}
            }
        }
        seen
    }

    /// The block both if-arms converge on (None if they never rejoin, i.e.
    /// both paths halt — not producible by this language, defensive only).
    /// The join is minted immediately after both arms, and every other block
    /// reachable from BOTH arms sits behind the join — so it is exactly the
    /// smallest common block id above both arms.
    fn common_join(&self, tb: BlockId, fb: BlockId) -> Option<BlockId> {
        let rf = self.reachable_from(fb);
        self.reachable_from(tb).into_iter()
            .filter(|b| *b > tb.max(fb) && rf.contains(b))
            .min()
    }

    /// Emit block `b` and everything that follows it, staying inside the loop
    /// whose header is `hdr` (a back edge to `hdr` closes the loop body).
    /// `stop` is the join block of an enclosing structured if: reaching it
    /// ends this arm — the parent emits the join after both arms.
    fn emit_seq(&self, out: &mut String, b: BlockId, hdr: Option<BlockId>, stop: Option<BlockId>, d: usize, emitted: &mut [bool], uses_handle: bool) {
        if Some(b) == stop { return; }
        if emitted[b] { panic!("structured codegen: block {b} reached twice — CFG is not a tree"); }
        emitted[b] = true;
        let block = &self.program.blocks[b];
        for i in &block.instrs { self.emit_instr(out, i, d, uses_handle); }
        match &block.terminator {
            None | Some(Terminator::Halt) => {
                // early return == the dispatcher's `break 'cfg`: there is no
                // code after Halt, so jumping to the end is exactly a return
                out.push_str(&format!("{}return tables;\n", indent(d)));
            }
            Some(Terminator::Jump(t)) => {
                if Some(*t) == hdr {
                    // back edge: this loop body is complete
                } else if Some(*t) == stop {
                    // if-arm reached its join: the parent continues
                } else if *t < b {
                    panic!("structured codegen: stray backward jump {b} -> {t}");
                } else if self.is_loop_header(*t) {
                    // forward jump into a loop header = entering a loop
                    let (cond, tb, fb) = match &self.program.blocks[*t].terminator {
                        Some(Terminator::Branch { cond, true_block, false_block }) =>
                            (*cond, *true_block, *false_block),
                        _ => panic!("structured codegen: block {t} has a back edge but no Branch"),
                    };
                    self.emit_loop(out, *t, cond, tb, d, emitted, uses_handle);
                    self.emit_seq(out, fb, hdr, stop, d, emitted, uses_handle);
                } else {
                    self.emit_seq(out, *t, hdr, stop, d, emitted, uses_handle);
                }
            }
            Some(Terminator::Branch { cond, true_block, false_block }) if self.is_loop_header(b) => {
                // a header reached directly (not via its pre-header Jump):
                // same handling as the Jump-into-header case
                self.emit_loop(out, b, *cond, *true_block, d, emitted, uses_handle);
                self.emit_seq(out, *false_block, hdr, stop, d, emitted, uses_handle);
            }
            Some(Terminator::Branch { cond, true_block, false_block }) => {
                // non-header branch = structured if/else. Both arms converge
                // on the join block, which the parent emits after the arms.
                let join = self.common_join(*true_block, *false_block);
                let ind = indent(d);
                out.push_str(&format!("{ind}if {} {{\n", self.bop_str(*cond)));
                self.emit_seq(out, *true_block, hdr, join, d + 1, emitted, uses_handle);
                // skip an `else` that would be empty: bare else-block with
                // no instructions jumping straight to the join. The block
                // is still CONSUMED — mark it emitted, or the orphan check
                // below fires on the common `if c then flag = true end`
                // inside a loop (the coalesced loop phi turns the else
                // arm's join Move into a removable self-copy, re-emptying
                // the block; found by probe_ops_loop_forms).
                let trivial_else = self.program.blocks[*false_block].instrs.is_empty()
                    && matches!(&self.program.blocks[*false_block].terminator,
                                Some(Terminator::Jump(t)) if Some(*t) == join);
                if !trivial_else {
                    out.push_str(&format!("{ind}}} else {{\n"));
                    self.emit_seq(out, *false_block, hdr, join, d + 1, emitted, uses_handle);
                } else {
                    emitted[*false_block] = true;
                }
                out.push_str(&format!("{ind}}}\n"));
                if let Some(j) = join {
                    self.emit_seq(out, j, hdr, stop, d, emitted, uses_handle);
                }
            }
        }
    }

    fn emit_loop(&self, out: &mut String, h: BlockId, cond: RegId, body: BlockId, d: usize, emitted: &mut [bool], uses_handle: bool) {
        if !self.is_loop_header(h) {
            panic!("structured codegen: Branch in block {h} is not a loop header");
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
            self.emit_seq(out, body, Some(h), None, d + 1, emitted, uses_handle);
            out.push_str(&format!("{ind}}}\n"));
        } else {
            // General fallback: everything in the header runs every
            // iteration. Never fires on the current corpus — it exists so a
            // surprising CFG degrades to correct-but-ugly, not wrong.
            out.push_str(&format!("{ind}loop {{\n"));
            for i in &block.instrs { self.emit_instr(out, i, d + 1, uses_handle); }
            out.push_str(&format!("{}if {} {{\n", indent(d + 1), self.bop_str(cond)));
            self.emit_seq(out, body, Some(h), None, d + 2, emitted, uses_handle);
            out.push_str(&format!("{}    }} else {{\n", indent(d + 1)));
            out.push_str(&format!("{}        break;\n", indent(d + 1)));
            out.push_str(&format!("{}    }}\n{ind}}}\n", indent(d + 1)));
        }
    }

    pub fn generate_rust_code(&self) -> String {
        let uses_handles = program_uses_handles(&self.program.blocks);

        let mut out = String::new();
        out.push_str("// target/release/build/phia-*/out/baked_native.rs\n\n");
        out.push_str("use crate::memory::Table;\n\n");
        out.push_str("#[allow(unused_variables, unused_mut, unused_assignments)]\n");
        out.push_str("pub fn run_baked() -> Vec<Box<Table>> {\n");

        let mut fast_phys: HashSet<RegId> = HashSet::new();
        for b in &self.program.blocks {
            for i in &b.instrs {
                if let Instruction::HoistRawPtr { table }
                    | Instruction::SetTableFast { table, .. }
                    | Instruction::GetTableFast { table, .. } = i {
                    fast_phys.insert(*table);
                }
            }
        }

        let (n_i, n_b, n_f, n_t, n_tf, base, fbase, tfbase) = if self.did_alloc {
            (self.n_int, self.n_bool, self.n_float, self.n_table, self.n_ftable,
             self.phys_base as usize, self.float_base as usize, self.ftable_base as usize)
        } else {
            let mut max: RegId = 0;
            for b in &self.program.blocks {
                for i in &b.instrs { if let Some(dd) = def_reg(i) { if dd > max { max = dd; } } }
            }
            let m = max as usize + 1;
            (m, m, m, m, 0usize, 0usize, 0usize, 0usize)
        };

        for r in base..base + n_i { out.push_str(&format!("    let mut i_r{r} = 0i64;\n")); }
        for r in base..base + n_b { out.push_str(&format!("    let mut b_r{r} = false;\n")); }
        for r in fbase..fbase + n_f { out.push_str(&format!("    let mut f_r{r} = 0f64;\n")); }
        // handle tables: the shared id range first, then the disjoint
        // float-table range — same decl shape, pointer type from the pool
        for r in base..base + n_t { self.emit_table_decl(&mut out, r as RegId, uses_handles, &fast_phys); }
        for r in tfbase..tfbase + n_tf { self.emit_table_decl(&mut out, r as RegId, uses_handles, &fast_phys); }
        out.push_str("    let mut tables = Vec::<Box<Table>>::with_capacity(128);\n\n");

        let mut emitted = vec![false; self.program.blocks.len()];
        self.emit_seq(&mut out, 0, None, None, 1, &mut emitted, uses_handles);
        let orphans: Vec<usize> = emitted.iter().enumerate()
            .filter(|(_, e)| !**e).map(|(i, _)| i).collect();
        if !orphans.is_empty() {
            panic!("structured codegen: blocks never reached: {orphans:?}");
        }

        out.push_str("}\n");
        out
    }
}
