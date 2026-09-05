// src/ir.rs
#![allow(dead_code)]

use crate::ast::StaticType;

#[derive(Debug, Clone)]
pub enum Instruction {
    LoadInt { target: u32, val: i64 },
    NewTable { target: u32 },
    SetTable { table: u32, key: u32, val: u32 },
    GetTable { target: u32, table: u32, key: u32 },
    Move { target: u32, source: u32, ty: StaticType },
    Add { target: u32, left: u32, right: u32 },
    Sub { target: u32, left: u32, right: u32 },
    Less { target: u32, left: u32, right: u32 },
    BeginWhile,
    WhileCondition { cond_reg: u32 },
    EndWhile,
    EnsureCapacity { table: u32, limit: u32 },
    HoistRawPtr { table: u32 },
    SetTableFast { table: u32, key: u32, val: u32 },
    GetTableFast { target: u32, table: u32, key: u32 },
}
