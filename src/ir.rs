// src/ir.rs
#![allow(dead_code)]

use crate::ast::StaticType;

pub type BlockId = usize;

/// THE ID-SPACE CONSTITUTION.
///
/// RegId is a layered namespace. Every layer has exactly one owner phase,
/// and ownership follows pipeline order — and layers NEVER COEXIST
/// QUERYABLE: layer B is range-disjoint from everything by its
/// reserved-high base, and layer C replaces layer A wholesale (the
/// rewrite erases A's ids in the same pass that mints C's), so no two
/// live layers ever share a range:
///
/// ```text
/// layer A   vregs      [0, V)                owner: lowerer (+ optimizer's
///                                              next_vreg mints into it);
///                                              dies at allocate_registers
/// layer B   consts     [CONST_REG_BASE, …)   owner: propagate_constants
///                                              (the remint; reserved high
///                                              range, disjoint from every
///                                              other layer numerically)
/// layer C   physicals  [0, P)                owner: allocate_registers
///                   minted from ZERO — the pass rewrites the layer-A
///                   ids out of the IR as it mints, so A and C never
///                   coexist and the numeric overlap of [0,V) and
///                   [0,P) is unobservable (the old base = max-vreg+1
///                   layout carried the scars of the vreg namespace
///                   into every program's physical numbering)
///                   sub-layered in mint order — int, bool, table,
///                   string, float, ftable, tstr, btable — EIGHT
///                   consecutive per-pool ranges on ONE global
///                   timeline: every physical id belongs to exactly
///                   one pool, and no two physical registers ever
///                   share a number (i_r12/b_r12 co-numbering is
///                   gone; pool membership is a pure range check)
/// ```
///
/// The three laws:
///
/// 1. **One layer, one owner, pipeline order — and overlapping layers
///    never coexist.** Ownership follows the pipeline; disjointness is
///    a VISIBILITY property, not just base arithmetic: layer B is
///    range-disjoint from everything (reserved high), and layer C
///    rewrites the layer-A namespace out of the IR as it mints. The
///    spelled-out-in-code instances: reg_alloc's skip set (consts
///    never enter a pool) and its remap-completeness assert (every
///    non-const id MUST enter a pool before the rewrite — an unmapped
///    vreg would survive and numerically collide with minted
///    physicals). Any future consumer of raw ids over the whole IR
///    must make the same exclusions, or cite this law instead of
///    remembering the war story.
/// 2. **Membership by range, value by map, kind by range (physicals) or
///    static type (vregs), rendering by layer.** Four orthogonal
///    questions, four mechanisms, never mixed: `is_const_reg` answers
///    membership; the consts maps answer value; an id's POOL RANGE
///    answers its kind once layer C has minted it (pre-alloc, vregs
///    have no ranges — there the operand's `StaticType` is the only
///    kind source, which is why Eq and DebugProbe carry types at all);
///    `c{n}` / literal vs `i_r{n}` follows the layer.
/// 3. **Ids never migrate.** A folded vreg is REMINTED into layer B, never
///    reused; layer C ids are never queried against const maps except as
///    tautologically-false range checks.
///
/// A future id consumer asks "which layers does my scan see, and do any
/// two of them coexist?" — consts_f and consts_s minted from the same
/// layer B under the same law, and the zero-base mint touched no base
/// math outside allocate_registers itself.
pub type RegId = u32;

/// Layer B's base: const-folded register ids mint from this reserved high
/// range, disjoint from the vreg/physical low half BY CONSTRUCTION — the
/// two namespaces can never collide numerically (the fuzzer_01
/// stale-key/physical collision class is impossible rather than defended
/// against). Realistic id counts are in the hundreds; nothing else ever
/// mints this high.
pub const CONST_REG_BASE: RegId = 1 << 31;

pub fn is_const_reg(r: RegId) -> bool { r >= CONST_REG_BASE }

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
