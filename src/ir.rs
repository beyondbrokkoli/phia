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
    // 1. Memory & Literals
    LoadInt { target: RegId, val: i64 },
    NewTable { target: RegId },
    SetTable { table: RegId, key: RegId, val: RegId },
    GetTable { target: RegId, table: RegId, key: RegId },
    Move { target: RegId, source: RegId, ty: StaticType },

    // 2. Math & Logic
    Add { target: RegId, left: RegId, right: RegId },
    Sub { target: RegId, left: RegId, right: RegId },
    Less { target: RegId, left: RegId, right: RegId },

    // 3. THE MAGIC SSA NODE
    // "If we came from block X, use register Y."
    Phi { target: RegId, ty: StaticType, args: Vec<(BlockId, RegId)> },

    // 4. Fast Paths (Populated later by the optimizer)
    EnsureCapacity { table: RegId, limit: RegId },
    HoistRawPtr { table: RegId },
    SetTableFast { table: RegId, key: RegId, val: RegId },
    GetTableFast { target: RegId, table: RegId, key: RegId },
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: BlockId,
    pub depth: usize, // <--- ADDED
    pub instrs: Vec<Instruction>,
    pub terminator: Option<Terminator>,
}

impl BasicBlock {
    pub fn new(id: BlockId, depth: usize) -> Self { // <--- UPDATED
        Self { id, depth, instrs: Vec::new(), terminator: None }
    }
}

// Your backend will now accept a CFG instead of a Vec<Instruction>
#[derive(Debug, Clone)]
pub struct IrProgram {
    pub blocks: Vec<BasicBlock>,
}
