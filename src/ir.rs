// src/ir.rs
#![allow(dead_code)]

use crate::ast::StaticType;

pub type BlockId = usize;

/// ARCHITECTURAL SPEC — THE RegId ID-SPACE.
///
/// RegId is a layered namespace. Every layer has exactly one owner
/// phase, ownership follows pipeline order, and no two layers are ever
/// simultaneously queryable: layer B is range-disjoint from everything
/// by its reserved-high base, and layer C replaces layer A wholesale —
/// the rewrite erases A's ids in the same pass that mints C's — so no
/// two live layers ever share a range.
///
/// ```text
/// layer A   vregs      [0, V)               owner: lowerer (the
///                                             optimizer's next_vreg
///                                             mints into it); destroyed
///                                             by allocate_registers
/// layer B   consts     [CONST_REG_BASE, …)  owner: propagate_constants
///                                             (the remint; reserved
///                                             high range, numerically
///                                             disjoint from A and C)
/// layer C   physicals  [0, P)               owner: allocate_registers;
///                                             minted from ZERO (not
///                                             max-vreg+1 — physical
///                                             numbering inherits no
///                                             offset from the vreg
///                                             namespace) in EIGHT
///                                             consecutive per-pool
///                                             ranges on ONE global
///                                             timeline: int, bool,
///                                             table, string, float,
///                                             ftable, tstr, btable
/// ```
/// The numeric overlap of [0,V) and [0,P) is unobservable: A and C
/// never coexist (Invariant 1).
///
/// THE FOUR INVARIANTS.
///
/// 1. TOTAL REWRITE — NO IDENTITY FALLBACK. Layer C replaces layer A
///    wholesale. The remap closure in `allocate_registers` rewrites
///    every def, use, and branch condition through the vreg→physical
///    map and panics on any non-const id the map lacks; the identity
///    fallback `_ => r` was removed by design. An unmapped vreg would
///    keep its low id, numerically collide with a minted physical, and
///    silently access a declared local of the wrong pool. Coverage is
///    exactly `remap_instr` + branch conds — the positions actually
///    rewritten — so a future instruction kind cannot slip a position
///    past a `use_regs`/`def_reg` match that has drifted out of sync.
///
/// 2. CONSTANT REMINTING — LAYER-B ISOLATION. Ids never migrate: a
///    folded vreg is reminted above `CONST_REG_BASE`, never reused, and
///    all four const maps are keyed by the reminted ids, so a const-map
///    key can never name a vreg or a physical (Boundary Defense B1).
///    Layer B is disjoint by construction, not by defense. The `skip`
///    set in `allocate_registers` is exactly layer-B membership: const
///    ids never enter a liveness interval and never receive a physical
///    slot; every use renders as a literal. Symmetrically, layer-C ids
///    reach the const maps only through `is_const_reg`, which is
///    tautologically false below the reserved base.
///
/// 3. POOL RANGES AS TYPES. An operand's kind has exactly one source,
///    chosen by layer. Pre-alloc (layers A/B) the operand's
///    `StaticType` is the sole kind source — vreg ids carry no ranges;
///    this is why `Eq` carries `ty` (the const fold's arm selector) and
///    `DebugProbe` carries per-operand kinds (the pool map's seed).
///    Post-alloc (layer C) the ID RANGE IS THE TYPE: eight consecutive
///    per-pool ranges on one global mint timeline give every physical
///    id exactly one pool, so pool membership is a pure integer range
///    check and `i_r12`/`b_r12` co-numbering cannot arise (Boundary
///    Defense B3). Four orthogonal questions, four mechanisms, never
///    mixed: `is_const_reg` answers membership; the const maps answer
///    value; pool range (C) or `StaticType` (A/B) answers kind;
///    `c{n}`/literal vs `i_r{n}` rendering follows the layer.
///
/// 4. THE FINAL-CFG AUDIT. `audit_id_space` (reg_alloc.rs) re-verifies
///    invariants 1–3 over the FINAL program — one scan, real asserts,
///    no mutation: remint purity, membership, pool purity, bool branch
///    conds, no dangling uses. It runs inside `allocate_registers` on
///    every compile and is the required regression harness for any
///    future IR mutation; tests/reg_alloc_audit.rs pins its failure
///    modes.
///
/// BOUNDARY DEFENSES — recorded hazard classes, each impossible by
/// construction and asserted anyway:
///
///   * B1 (fuzzer_01, stale-key): a const map keyed on a pre-remint low
///     id would shadow a minted physical at emission's const early-outs
///     — a map outliving its namespace. Prevented by the remint
///     (Invariant 2); re-checked by remint purity (Invariant 4).
///   * B2 (feat_ops_10): an arith target whose operands all const-folded
///     while its own result stayed runtime (the `is_finite` guard
///     declining inf/NaN, or a multi-def slot) mints no physical unless
///     the type fixpoint consults the const maps (`const_pool` in
///     reg_alloc) — the one deliberate bridge from layer B into the
///     pool map; it reads pool facts without reusing any layer-A id.
///   * B3 (co-numbered pools): a shared id range across
///     Int/Bool/Table/String made every "which pool is this id?"
///     question type-context-dependent. Prevented by the one global
///     mint timeline (Invariant 3); re-checked by pool purity
///     (Invariant 4).
///
/// Any future consumer of raw ids across the whole IR must make these
/// layer exclusions explicitly or cite the invariant that guarantees
/// them: the question is always "which layers does my scan see, and do
/// any two of them coexist?" consts_f and consts_s mint from the same
/// layer B under the same law; the zero-base mint touches no base math
/// outside allocate_registers itself.
pub type RegId = u32;

/// Layer B's base: const-folded register ids mint from this reserved
/// high range, disjoint from the vreg/physical low half by construction
/// (Boundary Defense B1). Realistic id counts are in the hundreds;
/// nothing else ever mints this high.
pub const CONST_REG_BASE: RegId = 1 << 31;

pub fn is_const_reg(r: RegId) -> bool { r >= CONST_REG_BASE }

/// Layer B's VALUE domain — one map, four variants. The const layer is
/// a single namespace (one remint timeline, Invariant 2), so its values
/// ride ONE map keyed by the reminted ids: the variant is the value's
/// kind, and a single lookup answers both "is this id const?" and "of
/// what kind?". (History: four parallel maps keyed by the same ids —
/// every pipeline signature carried four HashMaps and every membership
/// question was four probes. The fold still works kind-locally inside
/// propagate_constants; only the published artifact is unified.)
#[derive(Debug, Clone)]
pub enum ConstVal {
    Int(i64),
    Bool(bool),
    Float(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub enum Terminator {
    /// Unconditional jump to another block
    Jump(BlockId),
    /// Conditional branch based on a boolean register
    Branch { cond: RegId, true_block: BlockId, false_block: BlockId },
    /// End of the program
    Halt,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    LoadInt { target: RegId, val: i64 },
    LoadFloat { target: RegId, val: f64 },
    LoadBool { target: RegId, val: bool },
    LoadString { target: RegId, val: String },
    NewTable { target: RegId, ty: StaticType },
    SetTable { table: RegId, key: RegId, val: RegId, ty: StaticType },
    GetTable { target: RegId, table: RegId, key: RegId, ty: StaticType },
    Move { target: RegId, source: RegId, ty: StaticType },

    Add { target: RegId, left: RegId, right: RegId },
    Sub { target: RegId, left: RegId, right: RegId },
    Mul { target: RegId, left: RegId, right: RegId },
    Div { target: RegId, left: RegId, right: RegId },
    IntDiv { target: RegId, left: RegId, right: RegId },
    Mod { target: RegId, left: RegId, right: RegId },
    Neg { target: RegId, source: RegId },
    Less { target: RegId, left: RegId, right: RegId },
    // Native <= / >= — comparisons that cannot ride the Less desugar
    // (a <= b => a < b+1 wraps at i64::MAX and rounds wrong for floats
    // at 2^53) lower to these instead. The tier4 while-gate shapes keep
    // the desugar: literal-bound `while i <= n` still materializes the
    // pre-header +1 and opens the fast path exactly as before.
    Leq { target: RegId, left: RegId, right: RegId },
    Geq { target: RegId, left: RegId, right: RegId },
    // ty = the OPERAND type (Integer | Float | Boolean | String — the
    // checker guarantees both sides agree). Eq operands are the only
    // polymorphic operands in the IR without an instruction-carried
    // type, and at FOLD time (pre-alloc) vreg ids carry no pool ranges,
    // so the const maps need this field to know which of them to
    // consult. Post-alloc the operands' pool ranges answer on their
    // own; emission still rides the ty because it is already here.
    Eq { target: RegId, left: RegId, right: RegId, ty: StaticType },
    Not { target: RegId, source: RegId },
    // String concatenation. Monomorphic — String-only operands by the
    // checker — so it carries no kind: there is nothing to
    // disambiguate.
    Concat { target: RegId, left: RegId, right: RegId },

    Phi { target: RegId, ty: StaticType, args: Vec<(BlockId, RegId)> },

    EnsureCapacity { table: RegId, limit: RegId },
    HoistRawPtr { table: RegId },
    SetTableFast { table: RegId, key: RegId, val: RegId, ty: StaticType },
    GetTableFast { target: RegId, table: RegId, key: RegId, ty: StaticType },
    /// Runtime observation point: prints its operands' values under the
    /// tag. Defines nothing; carries each operand's kind for PRE-ALLOC
    /// typing (reg_alloc's pool map — vreg ids carry no ranges yet) and
    /// for the print-format choice ({} vs {:?}). Never DCE'd: it has no
    /// def to be dead and it is not in the pure set.
    DebugProbe { tag: String, operands: Vec<(RegId, StaticType)> },
}

impl Instruction {
    pub fn def_reg(&self) -> Option<RegId> {
        match self {
            Instruction::LoadInt { target, .. } | Instruction::LoadFloat { target, .. }
            | Instruction::LoadBool { target, .. } | Instruction::LoadString { target, .. }
            | Instruction::NewTable { target, .. }
            | Instruction::GetTable { target, .. } | Instruction::GetTableFast { target, .. }
            | Instruction::Move { target, .. } | Instruction::Add { target, .. }
            | Instruction::Sub { target, .. } | Instruction::Less { target, .. }
            | Instruction::Mul { target, .. } | Instruction::Div { target, .. }
            | Instruction::IntDiv { target, .. } | Instruction::Mod { target, .. }
            | Instruction::Neg { target, .. }
            | Instruction::Leq { target, .. } | Instruction::Geq { target, .. }
            | Instruction::Eq { target, .. } | Instruction::Not { target, .. }
            | Instruction::Concat { target, .. }
            | Instruction::Phi { target, .. } => Some(*target),
            _ => None,
        }
    }
    pub fn def_type(&self) -> Option<StaticType> {
        match self {
            Instruction::LoadInt { .. } | Instruction::Add { .. } | Instruction::Sub { .. }
            | Instruction::Mul { .. } | Instruction::Div { .. } | Instruction::IntDiv { .. }
            | Instruction::Mod { .. } | Instruction::Neg { .. } => Some(StaticType::Integer),
            Instruction::LoadFloat { .. } => Some(StaticType::Float),
            Instruction::LoadString { .. } | Instruction::Concat { .. } => Some(StaticType::String),
            Instruction::GetTable { ty, .. } | Instruction::GetTableFast { ty, .. } => Some(ty.clone()),
            Instruction::Less { .. } | Instruction::Leq { .. } | Instruction::Geq { .. }
            | Instruction::Eq { .. } | Instruction::Not { .. }
            | Instruction::LoadBool { .. } => Some(StaticType::Boolean),
            Instruction::NewTable { ty, .. } => Some(ty.clone()),
            Instruction::Move { ty, .. } | Instruction::Phi { ty, .. } => Some(ty.clone()),
            _ => None,
        }
    }
    pub fn use_regs(&self) -> Vec<RegId> {
        match self {
            Instruction::LoadInt { .. } | Instruction::LoadFloat { .. } | Instruction::LoadBool { .. }
            | Instruction::LoadString { .. } | Instruction::NewTable { .. } => vec![],
            Instruction::Move { source, .. } | Instruction::Neg { source, .. } => vec![*source],
            Instruction::Concat { left, right, .. } => vec![*left, *right],
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
    pub fn remap_instr<F: Fn(RegId) -> RegId>(&mut self, f: &F) {
        let g = |r: &mut RegId| *r = f(*r);
        match self {
            Instruction::LoadInt { target, .. } | Instruction::LoadFloat { target, .. }
            | Instruction::LoadBool { target, .. } | Instruction::LoadString { target, .. }
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
            Instruction::Concat { target, left, right } => { g(target); g(left); g(right); }
            Instruction::Phi { target, args, .. } => { g(target); for (_, r) in args.iter_mut() { g(r); } }
            Instruction::EnsureCapacity { table, limit } => { g(table); g(limit); }
            Instruction::HoistRawPtr { table } => g(table),
            Instruction::DebugProbe { operands, .. } =>
                { for (r, _) in operands.iter_mut() { g(r); } }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: BlockId,
    pub depth: usize, // <--- ADDED
    pub instrs: Vec<Instruction>,
    pub terminator: Option<Terminator>,
}

impl BasicBlock {
    pub fn new(id: BlockId, depth: usize) -> Self {
        Self { id, depth, instrs: Vec::new(), terminator: None }
    }
}

// Your backend will now accept a CFG instead of a Vec<Instruction>
#[derive(Debug, Clone)]
pub struct IrProgram {
    pub blocks: Vec<BasicBlock>,
}
