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
    NewTable { target: RegId, ty: StaticType },
    SetTable { table: RegId, key: RegId, val: RegId, ty: StaticType },
    GetTable { target: RegId, table: RegId, key: RegId, ty: StaticType },
    Move { target: RegId, source: RegId, ty: StaticType },

    Add { target: RegId, left: RegId, right: RegId },
    Sub { target: RegId, left: RegId, right: RegId },
    Less { target: RegId, left: RegId, right: RegId },

    Phi { target: RegId, ty: StaticType, args: Vec<(BlockId, RegId)> },

    EnsureCapacity { table: RegId, limit: RegId },
    HoistRawPtr { table: RegId },
    SetTableFast { table: RegId, key: RegId, val: RegId, ty: StaticType },
    GetTableFast { target: RegId, table: RegId, key: RegId, ty: StaticType },
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
