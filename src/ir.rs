// src/ir.rs
#![allow(dead_code)]

use crate::ast::StaticType;

pub type BlockId = usize;
pub type RegId = u32;

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
    // ty = the OPERAND type (Integer | Float | Boolean — the checker
    // guarantees both sides agree). Eq operands are the only polymorphic
    // operands in the IR without an instruction-carried type, and Int and
    // Bool physicals deliberately share one id range, so the pool tables
    // cannot disambiguate them — the rendering needs this field.
    Eq { target: RegId, left: RegId, right: RegId, ty: StaticType },
    Not { target: RegId, source: RegId },
    // String concatenation. Monomorphic — String-only operands by the
    // checker — so it carries no kind: the polymorphic-operand rule
    // (Int/Bool sharing an id range) cannot apply to it.
    Concat { target: RegId, left: RegId, right: RegId },

    Phi { target: RegId, ty: StaticType, args: Vec<(BlockId, RegId)> },

    EnsureCapacity { table: RegId, limit: RegId },
    HoistRawPtr { table: RegId },
    SetTableFast { table: RegId, key: RegId, val: RegId, ty: StaticType },
    GetTableFast { target: RegId, table: RegId, key: RegId, ty: StaticType },
    /// Runtime observation point: prints its operands' values under the
    /// tag. Defines nothing; carries each operand's kind because Int and
    /// Bool physicals share one id range and nothing else disambiguates
    /// them at emission (the polymorphic-operand rule). Never DCE'd: it
    /// has no def to be dead and it is not in the pure set.
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
