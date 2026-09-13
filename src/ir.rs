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
