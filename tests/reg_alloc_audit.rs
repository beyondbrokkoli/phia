// tests/reg_alloc_audit.rs — rust-side pins for the id-space law.
//
// The compiler pipeline lives in build.rs's crate (src/main.rs is only
// the runtime harness), so src/*.rs modules are never unit-testable in
// place. This crate re-mounts the sources with the same #[path] trick
// build.rs uses and pins the two tripwires that guard the zero-base
// unique-id world:
//   * the rewrite inside allocate_registers is TOTAL AND LOUD — a vreg
//     that never entered a pool panics instead of riding an identity
//     fallback into a numeric collision with minted physicals;
//   * the post-alloc id-space audit — remint purity, membership, pool
//     purity (one id, one pool), bool conds, no dangling uses.
// Run: cargo test (dev profile — release sets panic=abort, which the
// should_panic harness cannot catch).

#![allow(dead_code)]

#[path = "../src/ast.rs"] mod ast;
#[path = "../src/ir.rs"] mod ir;
#[path = "../src/reg_alloc.rs"] mod reg_alloc;

use std::collections::HashMap;
use ast::StaticType;
use ir::{BasicBlock, Instruction, IrProgram, ConstVal, RegId, Terminator};
use reg_alloc::{AllocInfo, allocate_registers, audit_id_space};

fn empty_consts() -> HashMap<RegId, ConstVal> {
    HashMap::new()
}

// one int physical, everything else empty: id 0 mints Int, nothing
// else is addressable
fn one_int_alloc() -> AllocInfo {
    AllocInfo {
        n_int: 1, n_bool: 0, n_float: 0, n_str: 0, n_table: 0,
        n_ftable: 0, n_tstr: 0, n_btable: 0,
        int_base: 0, bool_base: 1, table_base: 1, str_base: 1,
        float_base: 1, ftable_base: 1, tstr_base: 1, btable_base: 1,
    }
}

/// REMAP-COMPLETENESS regression pin: a vreg that never entered a pool
/// must abort the build inside the rewrite, not ride an identity
/// fallback into a numeric collision with minted physicals. The
/// injection vector is a Phi — the one instruction whose ARG positions
/// the pre-mint typing pass does not carry (post-de_ssa compiles
/// contain no phis; a future instruction kind forgotten in the ty-map
/// is the real-world version of this shape). The target IS typed
/// (the def-side pass reads def_type), so only the arg trips.
#[test]
#[should_panic(expected = "never entered a pool")]
fn unmapped_vreg_trips_the_rewrite() {
    let mut program = IrProgram { blocks: vec![
        BasicBlock::new(0, 0),
        BasicBlock::new(1, 1),
    ] };
    program.blocks[0].terminator = Some(Terminator::Jump(1));
    program.blocks[1].instrs.push(Instruction::Phi {
        target: 0,
        ty: StaticType::Integer,
        args: vec![(0, 1)], // vreg 1: defined nowhere, typed nowhere
    });
    program.blocks[1].terminator = Some(Terminator::Halt);
    let consts = empty_consts();
    allocate_registers(&mut program, &consts);
}

/// Pool-purity pin: a bool-producing instruction defining an id in the
/// int range is the silent-wrong-code class (i_r12/b_r12 co-numbering
/// returning) — the audit must name it.
#[test]
#[should_panic(expected = "pool purity broken")]
fn wrong_pool_def_trips_the_audit() {
    let mut program = IrProgram { blocks: vec![BasicBlock::new(0, 0)] };
    program.blocks[0].instrs.push(Instruction::LoadBool { target: 0, val: true });
    program.blocks[0].terminator = Some(Terminator::Halt);
    let consts = empty_consts();
    audit_id_space(&program, &one_int_alloc(), &consts);
}

/// Membership pin: a use of an id outside every minted range means
/// something unminted survived the rewrite — the audit's end-to-end
/// catch (the rewrite's own assert only covers remap_instr positions).
#[test]
#[should_panic(expected = "unminted id survived the rewrite")]
fn unminted_use_trips_the_audit() {
    let mut program = IrProgram { blocks: vec![BasicBlock::new(0, 0)] };
    program.blocks[0].instrs.push(Instruction::LoadInt { target: 0, val: 1 });
    program.blocks[0].instrs.push(Instruction::Move {
        target: 0, source: 7, ty: StaticType::Integer, // 7: never minted
    });
    program.blocks[0].terminator = Some(Terminator::Halt);
    let consts = empty_consts();
    audit_id_space(&program, &one_int_alloc(), &consts);
}
