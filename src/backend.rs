// src/backend.rs
use crate::ir::{IrProgram, Instruction, Terminator, BasicBlock, BlockId, RegId, ConstVal};
use std::collections::{HashMap, HashSet};
use crate::ast::StaticType;
use crate::reg_alloc::AllocInfo;

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

// Pool membership is answered by the mint ranges, NOT a phys-id -> pool
// map: allocate_registers lays the eight pools out as consecutive ranges
// on ONE global timeline, so every physical id belongs to exactly one
// pool and range membership equals "this pool minted this id". The old
// shared Int/Bool/Table/String range (i_r12/b_r12/t_r12/s_r12
// co-numbering four distinct variables) is what once made a map
// last-writer-wins and a bare-id lookup a coin flip — the uniqueness
// timeline removes the hazard class entirely. Const ids (layer B,
// >= CONST_REG_BASE) sit far above every range, so they can never fall
// inside one either.
pub fn is_int_reg(r: RegId, alloc: &AllocInfo) -> bool {
    r >= alloc.int_base && r < alloc.int_base + alloc.n_int as RegId
}

pub fn is_bool_reg(r: RegId, alloc: &AllocInfo) -> bool {
    r >= alloc.bool_base && r < alloc.bool_base + alloc.n_bool as RegId
}

// Plain int-element/handle tables ([table_base, table_base + n_table));
// the element-kind table pools below are their own ranges.
pub fn is_table_reg(r: RegId, alloc: &AllocInfo) -> bool {
    r >= alloc.table_base && r < alloc.table_base + alloc.n_table as RegId
}

pub fn is_str_reg(r: RegId, alloc: &AllocInfo) -> bool {
    r >= alloc.str_base && r < alloc.str_base + alloc.n_str as RegId
}

pub fn is_float_reg(r: RegId, alloc: &AllocInfo) -> bool {
    r >= alloc.float_base && r < alloc.float_base + alloc.n_float as RegId
}

// Storage side of a hoisted/EC'd table physical: float-ELEMENT tables
// live in their own pool ([ftable_base, ftable_base + n_ftable)), so
// the id alone answers farray vs array (and *mut f64 vs *mut i64 in
// the decl block) unambiguously.
fn is_ftable_reg(r: RegId, alloc: &AllocInfo) -> bool {
    r >= alloc.ftable_base && r < alloc.ftable_base + alloc.n_ftable as RegId
}

// String-ELEMENT tables: same disjoint-range argument, sarray side.
fn is_tstr_reg(r: RegId, alloc: &AllocInfo) -> bool {
    r >= alloc.tstr_base && r < alloc.tstr_base + alloc.n_tstr as RegId
}

// Bool-ELEMENT tables: same disjoint-range argument, barray side.
fn is_btable_reg(r: RegId, alloc: &AllocInfo) -> bool {
    r >= alloc.btable_base && r < alloc.btable_base + alloc.n_btable as RegId
}

// ---- THE STORAGE-SIDE ORACLE + THE pool==ty VERIFICATION FRONTIER ----
//
// Element kind of a table physical is answered ONCE, by the pool the id
// was minted into (Layer C: the range is the type). Every emission site
// that needs "which Table accessor / pointer type / resize zero?" asks
// these functions — the decl block's p_r type, NewTable's constructor,
// the dyn getter/setters' fld, EC's (fld, zero), HR's fld, and the
// probe's length token — so two sites can no longer disagree by
// construction. That was the wrong-fld hazard class: the answers used
// to be re-derived at six sites, half from the instruction-carried
// StaticType, half from the pool, and a divergence between the two
// derivations emits a program that compiles and reads the wrong Vec at
// runtime.
//
// The carried StaticType is not silently ignored either — it is
// VERIFIED. Wherever an instruction still rides a ty (NewTable, the
// dyn/fast table ops, Eq, probe operands), emission asserts the
// ty-derived answer equals the pool-derived one; a mismatch means the
// id-space law broke upstream and fails the build HERE, at the
// emission site, rather than downstream as wrong code. This is the
// gradual frontier: pre-alloc consumers (the const fold, reg_alloc's
// pool map) still CONSUME ty — pre-alloc, ids carry no ranges and ty is
// the only kind source (Invariant 3, ir.rs) — and emission's Move/Phi
// arms still render by ty. The sites below are where the pool already
// answers, so the ty riding along is pure tripwire.

// Storage side (Table accessor) of a table physical, by pool. The
// plain-table range is the i64 side: integer elements and nested-table
// handles (handle mode) share it. Sites render `{fld}()` for shared
// reads and `{fld}_mut()` where they resize/write — same one oracle,
// two spellings.
fn table_fld(r: RegId, alloc: &AllocInfo) -> &'static str {
    if is_ftable_reg(r, alloc) { "as_float" }
    else if is_tstr_reg(r, alloc) { "as_string" }
    else if is_btable_reg(r, alloc) { "as_bool" }
    else { "as_int" }
}

// Storage side plus the pool's resize zero (EC and the dyn ops' grow
// path): the absent-key default of the element pool.
fn table_fld_zero(r: RegId, alloc: &AllocInfo) -> (&'static str, &'static str) {
    if is_ftable_reg(r, alloc) { ("as_float", "0.0") }
    else if is_tstr_reg(r, alloc) { ("as_string", "String::new()") }
    else if is_btable_reg(r, alloc) { ("as_bool", "false") }
    else { ("as_int", "0") }
}

// Hoisted-pointer element type — the decl block's p_r declaration and
// the fast ops' raw pointer base.
fn table_ptr_ty(r: RegId, alloc: &AllocInfo) -> &'static str {
    if is_ftable_reg(r, alloc) { "*mut f64" }
    else if is_tstr_reg(r, alloc) { "*mut String" }
    else if is_btable_reg(r, alloc) { "*mut bool" }
    else { "*mut i64" }
}

// The same storage-side question asked of an instruction-carried
// ELEMENT type — the pre-alloc spelling (where ty is the only kind
// source). Integer elements and nested-table handles share the i64
// side; an UnknownTable element is an undecided element, i.e. the i64
// side's pool.
fn elem_fld(elem: &StaticType) -> &'static str {
    match elem {
        StaticType::Float => "as_float",
        StaticType::String => "as_string",
        StaticType::Boolean => "as_bool",
        _ => "as_int",
    }
}

// pool == ty, asserted wherever an instruction carries both. The panic
// is a tripwire, not defense: audit_id_space already enforces the law
// over the final CFG, so a hit here means the law broke between the
// audit and this emission site — fail the build, never emit the
// wrong-fld program.
fn assert_fld(table: RegId, elem: &StaticType, alloc: &AllocInfo, op: &str) {
    let (pool, ty) = (table_fld(table, alloc), elem_fld(elem));
    assert!(pool == ty,
        "{op}: table physical {table} sits on the {pool} side but its carried \
         element type {elem:?} says {ty} — pool/ty divergence, the id-space \
         law broke upstream (audit_id_space should have named it first)");
}

// Post-alloc operand kind: the id ALONE answers (Invariant 3). A const
// id (layer B) sits outside every mint range — its ConstVal variant is
// its kind; a physical id's pool range is its kind. Panic on an id in
// no layer (pool_prefixed precedent): audit_id_space guarantees every
// emission operand is classified, so a hit means the law broke between
// the audit and this site.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum OpKind { Int, Bool, Float, Str, Table }

fn operand_kind(r: RegId, consts: &HashMap<RegId, ConstVal>, alloc: &AllocInfo) -> OpKind {
    match consts.get(&r) {
        Some(ConstVal::Int(_)) => return OpKind::Int,
        Some(ConstVal::Bool(_)) => return OpKind::Bool,
        Some(ConstVal::Float(_)) => return OpKind::Float,
        Some(ConstVal::String(_)) => return OpKind::Str,
        _ => {}
    }
    if is_float_reg(r, alloc) { OpKind::Float }
    else if is_bool_reg(r, alloc) { OpKind::Bool }
    else if is_str_reg(r, alloc) { OpKind::Str }
    else if is_table_reg(r, alloc) || is_ftable_reg(r, alloc)
        || is_tstr_reg(r, alloc) || is_btable_reg(r, alloc) { OpKind::Table }
    else if is_int_reg(r, alloc) { OpKind::Int }
    else { panic!("operand_kind: reg {r} is neither const nor any pool's physical — \
                   post-allocation id-space law violated") }
}

// The kind an instruction-carried StaticType claims — the ty half of
// the verification.
fn ty_kind(t: &StaticType) -> OpKind {
    match t {
        StaticType::Integer => OpKind::Int,
        StaticType::Float => OpKind::Float,
        StaticType::Boolean => OpKind::Bool,
        StaticType::String => OpKind::Str,
        StaticType::Table(_) | StaticType::UnknownTable(_) => OpKind::Table,
    }
}

// Float-operand test for the comparison forks and the pretty-while
// decline: the float POOL range plus the const map's Float variant (a
// folded float's layer-B id sits outside every range).
fn is_float_operand(r: RegId, consts: &HashMap<RegId, ConstVal>, alloc: &AllocInfo) -> bool {
    is_float_reg(r, alloc) || matches!(consts.get(&r), Some(ConstVal::Float(_)))
}

// A const-folded vreg renders as its literal; everything else renders
// as its pool-prefixed physical register. Renderers are MACROS, not
// functions: each expansion defines a tiny block-local Display wrapper
// — a value that owns the register id and a shared reference to the
// unified const map — so the lookup happens inside fmt() and no
// intermediate String is ever allocated. Each macro answers with its
// OWN ConstVal variant only (the map is one, the kinds are four — the
// variant IS the kind, Invariant 3's layer-B half); anything else in
// the slot falls through to the physical spelling. The wrapper is used
// directly as a format! argument in the hot path; at the few sites
// where the value must be stored or mixed with plain format! arms, the
// call site appends .to_string() (one allocation — exactly the value
// the old helper returned).
macro_rules! iop_str {
    ($r:expr, $consts:expr) => {{
        struct Formatter<'a>(&'a HashMap<RegId, ConstVal>, RegId);
        impl<'a> std::fmt::Display for Formatter<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.0.get(&self.1) {
                    Some(ConstVal::Int(v)) => write!(f, "{v}"),
                    _ => write!(f, "i_r{}", self.1),
                }
            }
        }
        Formatter($consts, $r)
    }};
}
macro_rules! bop_str {
    ($r:expr, $consts:expr) => {{
        struct Formatter<'a>(&'a HashMap<RegId, ConstVal>, RegId);
        impl<'a> std::fmt::Display for Formatter<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.0.get(&self.1) {
                    Some(ConstVal::Bool(v)) => write!(f, "{v}"),
                    _ => write!(f, "b_r{}", self.1),
                }
            }
        }
        Formatter($consts, $r)
    }};
}
// Floats fold under the Float variant: a float operand renders as its
// `{:?}` literal (finite only — the fold guard declines inf/NaN, which
// do not spell valid Rust literals) or its physical register.
macro_rules! fop_str {
    ($r:expr, $consts:expr) => {{
        struct Formatter<'a>(&'a HashMap<RegId, ConstVal>, RegId);
        impl<'a> std::fmt::Display for Formatter<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.0.get(&self.1) {
                    Some(ConstVal::Float(v)) => write!(f, "{:?}", v),
                    _ => write!(f, "f_r{}", self.1),
                }
            }
        }
        Formatter($consts, $r)
    }};
}
// Strings likewise (the String variant): a string operand renders as
// its Debug-escaped literal (the LoadString invariant: raw user text
// always renders a valid Rust literal) or its physical register. Sites
// that need an OWNED value must special-case the const arm — `.clone()`
// on a literal resolves to &str's Clone and yields the wrong type;
// those sites render `"...".to_string()` instead (LoadFloat/LoadString
// precedent).
macro_rules! sop_str {
    ($r:expr, $consts:expr) => {{
        struct Formatter<'a>(&'a HashMap<RegId, ConstVal>, RegId);
        impl<'a> std::fmt::Display for Formatter<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.0.get(&self.1) {
                    Some(ConstVal::String(v)) => write!(f, "{:?}", v),
                    _ => write!(f, "s_r{}", self.1),
                }
            }
        }
        Formatter($consts, $r)
    }};
}

// The dump's id renderer. A const-folded vreg (layer-B id, >=
// CONST_REG_BASE) renders `c{n}` (n = offset from the const base):
// self-documenting, and visually distinct from both physicals and the
// vreg namespace. Every other id renders as its pool-prefixed physical
// register, and the id ALONE answers the pool — one global timeline,
// one range per pool — so no StaticType hint is needed (or accepted:
// the hint used to paper over the shared Int/Bool/Table/String range
// by trusting the caller's guess). The panic is a tripwire, not
// defense: post-allocate_registers every id in the IR is either a
// layer-B const or a minted physical, so an unmatched id means the
// layer law broke and the dump must not name a register that does not
// exist.
pub fn pool_prefixed(
    r: RegId,
    consts: &HashMap<RegId, ConstVal>,
    alloc: &AllocInfo,
) -> String {
    if consts.contains_key(&r) {
        return format!("c{}", r - crate::ir::CONST_REG_BASE);
    }
    if is_int_reg(r, alloc) { return format!("i_r{r}"); }
    if is_bool_reg(r, alloc) { return format!("b_r{r}"); }
    if is_table_reg(r, alloc) { return format!("t_r{r}"); }
    if is_str_reg(r, alloc) { return format!("s_r{r}"); }
    if is_float_reg(r, alloc) { return format!("f_r{r}"); }
    if is_ftable_reg(r, alloc) || is_tstr_reg(r, alloc) || is_btable_reg(r, alloc) {
        return format!("t_r{r}");
    }
    panic!("pool_prefixed: reg {r} is neither const nor any pool's physical — \
            post-allocation id-space law violated");
}

// Total uses of `r` across instructions and branch conditions. With
// unique physical ids every use of the id IS a use of this register —
// the question the pretty-while guard asks ("does anything but the
// Branch read the Less's result?") is answered exactly by the raw
// count, no type-context enumeration needed. The old
// bool-context-only counter (bool_reg_uses) existed because Int and
// Bool physicals co-numbered one shared range and a co-numbered int's
// uses inflated the count; the one-global-timeline mint retired it.
// Phis are gone by emission time (resolve_phis), so this enumerates
// plain use_regs + Branch conds and nothing else.
fn count_uses(program: &IrProgram, r: RegId) -> usize {
    let mut n = 0;
    for b in &program.blocks {
        for i in &b.instrs {
            if i.use_regs().contains(&r) { n += 1; }
        }
        if let Some(Terminator::Branch { cond, .. }) = &b.terminator
            && *cond == r { n += 1; }
    }
    n
}

// a dominates b iff b is unreachable from the entry once a is removed.
// Small structured CFGs: this DFS runs once per back-edge candidate.
fn dominates(program: &IrProgram, a: BlockId, b: BlockId) -> bool {
    if a == b { return true; }
    let mut seen = HashSet::new();
    let mut stack = vec![0usize];
    while let Some(x) = stack.pop() {
        if x == a || !seen.insert(x) { continue; }
        match &program.blocks[x].terminator {
            Some(Terminator::Jump(t)) => stack.push(*t),
            Some(Terminator::Branch { true_block, false_block, .. }) => {
                stack.push(*true_block);
                stack.push(*false_block);
            }
            _ => {}
        }
    }
    !seen.contains(&b)
}

fn is_loop_header(program: &IrProgram, h: BlockId) -> bool {
    // A loop header has a back edge: a LATER block jumping to it that
    // the header DOMINATES — the natural-loop definition (every trip
    // to the back edge passes through the header). Mere id-space
    // backwardness is not enough: the lowerer mints if-arm blocks
    // (including nested ifs' joins) AFTER the enclosing join, so a
    // nested if's inner join jumps backward to the outer join. The old
    // reachability guard ("an if-join's jumper sits in a sibling arm
    // nothing downstream reaches") holds only OUTSIDE loops — inside a
    // while, a join's successors flow to the latch, around the back
    // edge, and back through the whole body, so every nested-if join
    // inside a loop graded as a header and emit_loop wrapped and
    // re-entered it (differential fuzzer seed 157: "header 15 reached
    // twice"). Dominance has no such blind spot: a join never
    // dominates its own then-arm's nested join, while a real while
    // header dominates its latch.
    let Some(Terminator::Branch { .. }) = &program.blocks[h].terminator else { return false };
    program.blocks[h + 1..].iter().enumerate().any(|(i, p)| {
        matches!(&p.terminator, Some(Terminator::Jump(t)) if *t == h)
            && dominates(program, h, h + 1 + i)
    })
}

fn emit_table_decl(out: &mut String, alloc: &AllocInfo, r: RegId, uses_handles: bool, fast_phys: &HashSet<RegId>) {
    if uses_handles {
        out.push_str(&format!("    let mut t_r{r} = 0i64;\n"));
    } else {
        out.push_str(&format!("    let mut t_r{r}: *mut Table = std::ptr::null_mut();\n"));
    }
    if fast_phys.contains(&r) {
        let ptr_ty = table_ptr_ty(r, alloc);
        out.push_str(&format!("    let mut p_r{r}: {ptr_ty} = std::ptr::null_mut();\n"));
        out.push_str(&format!("    let mut len_r{r} = 0usize;\n"));
    }
}

// The read-only emission environment: program, allocation, the unified
// const map, and the handle-mode flag. A tuple alias, not a state
// struct — it is assembled once in generate_rust_code, passed by value
// (all fields Copy) through the recursive emit_* chain, and each frame
// destructures only the projections it reads itself, handing the whole
// tuple down unchanged.
type EmitEnv<'a> = (
    &'a IrProgram,
    &'a AllocInfo,
    &'a HashMap<RegId, ConstVal>,
    bool,
);

// The float floor-division subexpression `x / y`, E0689-proofed: when
// BOTH operands const-folded to literals the division is `{float}`
// and a `.floor()` method call on it is f32/f64-ambiguous — pin it
// with `as f64`. Any register operand types the division already, and
// that spelling stays byte-identical to the frozen locks.
fn floor_div_str(
    left: RegId,
    right: RegId,
    consts: &HashMap<RegId, ConstVal>,
) -> String {
    let fconst = |r: RegId| matches!(consts.get(&r), Some(ConstVal::Float(_)));
    if fconst(left) && fconst(right) {
        format!("(({} / {}) as f64)", fop_str!(left, consts), fop_str!(right, consts))
    } else {
        format!("({} / {})", fop_str!(left, consts), fop_str!(right, consts))
    }
}

// Lua-faithful float literal rendering: Rust's `{:?}` spells
// non-finite f64s `inf`/`-inf`/`NaN`, none of which is a Rust
// expression, so they render as the named constants instead — the
// Lua rule (strtod semantics) is that an overflowing numeral IS its
// infinite value. The lexer's `d.d` shape can only overflow to +inf
// (negation is a separate Neg op, NaN has no literal spelling), but
// the renderer is total over all four cases for the day the lexer
// grows exponents.
fn render_f64(val: f64) -> String {
    if val.is_nan() {
        "f64::NAN".to_string()
    } else if val.is_infinite() {
        if val.is_sign_negative() {
            "f64::NEG_INFINITY".to_string()
        } else {
            "f64::INFINITY".to_string()
        }
    } else {
        format!("{val:?}")
    }
}

fn emit_instr(
    out: &mut String,
    env: EmitEnv,
    instr: &Instruction,
) {
    let (_, alloc, consts, uses_handles) = env;
    // compile-time-computed defs emit nothing: their uses are literals
    if let Some(t) = instr.def_reg() && consts.contains_key(&t) {
        return;
    }
    // In handle mode a table-typed operand renders as its t_r handle
    // reg, resolved through checked arena access (tables.get/get_mut +
    // nil panic) instead of get_unchecked: even a hypothetical checker
    // bug degrades to a clean "Runtime Error", never UB. Handle mode
    // only — the pointer templates below are frozen, byte-identical to
    // the milestone locks.
    match instr {
        Instruction::LoadInt { target, val } =>
            out.push_str(&format!("i_r{target} = {val};\n")),
        Instruction::LoadFloat { target, val } =>
            // non-finite literals reach here (extreme decimals the
            // lexer parses to inf; the fold declines them — cf is
            // finite-only), so the rendering must be the named
            // constants, never `{:?}`'s bare `inf`
            out.push_str(&format!("f_r{target} = {};\n", render_f64(*val))),
        Instruction::LoadBool { target, val } =>
            out.push_str(&format!("b_r{target} = {val};\n")),
        Instruction::LoadString { target, val } =>
            // {:?} on a &str renders a valid escaped Rust literal
            out.push_str(&format!("s_r{target} = {val:?}.to_string();\n")),
        Instruction::Concat { target, left, right } =>
            out.push_str(&format!(
                "s_r{target} = format!(\"{{}}{{}}\", {}, {});\n",
                sop_str!(*left, consts), sop_str!(*right, consts)
            )),
        Instruction::NewTable { target, ty } => {
            // A table is monomorphic: its pool — minted from the static
            // element type — picks the storage side at construction and
            // it never changes. The constructor is derived from the
            // TARGET's pool (the one oracle); the carried table type is
            // verified against it, never consulted for the choice.
            let elem: &StaticType = match ty {
                StaticType::Table(inner) => inner,
                // an undecided element kinds as the i64 side's pool
                other => other,
            };
            assert_fld(*target, elem, alloc, "NewTable");
            let ctor = if is_ftable_reg(*target, alloc) { "Table::new_float()" }
                else if is_tstr_reg(*target, alloc) { "Table::new_string()" }
                else if is_btable_reg(*target, alloc) { "Table::new_bool()" }
                else { "Table::new()" };
            if uses_handles {
                // 1-based arena handle; 0 stays reserved for null
                out.push_str(&format!(
                    "tables.push(Box::new({ctor}));\n\
                     t_r{target} = tables.len() as i64;\n"
                ));
            } else {
                out.push_str(&format!(
                    "let mut new_table = Box::new({ctor});\n\
                     t_r{target} = &mut *new_table as *mut Table;\n\
                     tables.push(new_table);\n"
                ));
            }
        }
        Instruction::Move { target, source, ty } => match ty {
            StaticType::Integer => out.push_str(&format!("i_r{target} = {};\n", iop_str!(*source, consts))),
            StaticType::Boolean => out.push_str(&format!("b_r{target} = {};\n", bop_str!(*source, consts))),
            StaticType::Float => out.push_str(&format!("f_r{target} = {};\n", fop_str!(*source, consts))),
            // strings are owned: a Move clones (SSA semantics — the
            // source slot may still be read on another path). A CONST
            // source cannot ride `.clone()` — that resolves to &str's
            // Clone and yields the wrong type — so it materializes with
            // .to_string(), the LoadString emission shape.
            StaticType::String => {
                if let Some(ConstVal::String(v)) = consts.get(source) {
                    out.push_str(&format!("s_r{target} = {v:?}.to_string();\n"));
                } else {
                    out.push_str(&format!("s_r{target} = {}.clone();\n", sop_str!(*source, consts)));
                }
            }
            StaticType::Table(_) | StaticType::UnknownTable(_) => out.push_str(&format!("t_r{target} = t_r{source};\n")),
        },
        Instruction::Add { target, left, right } => {
            if is_float_reg(*target, alloc) {
                out.push_str(&format!("f_r{target} = {} + {};\n", fop_str!(*left, consts), fop_str!(*right, consts)))
            } else {
                out.push_str(&format!("i_r{target} = {} + {};\n", iop_str!(*left, consts), iop_str!(*right, consts)))
            }
        }
        Instruction::Sub { target, left, right } => {
            if is_float_reg(*target, alloc) {
                out.push_str(&format!("f_r{target} = {} - {};\n", fop_str!(*left, consts), fop_str!(*right, consts)))
            } else {
                out.push_str(&format!("i_r{target} = {} - {};\n", iop_str!(*left, consts), iop_str!(*right, consts)))
            }
        }
        Instruction::Mul { target, left, right } => {
            if is_float_reg(*target, alloc) {
                out.push_str(&format!("f_r{target} = {} * {};\n", fop_str!(*left, consts), fop_str!(*right, consts)))
            } else {
                out.push_str(&format!("i_r{target} = {} * {};\n", iop_str!(*left, consts), iop_str!(*right, consts)))
            }
        }
        Instruction::Div { target, left, right } => {
            if is_float_reg(*target, alloc) {
                out.push_str(&format!("f_r{target} = {} / {};\n", fop_str!(*left, consts), fop_str!(*right, consts)))
            } else {
                out.push_str(&format!("i_r{target} = {} / {};\n", iop_str!(*left, consts), iop_str!(*right, consts)))
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
        // inside the leading L / R. The float templates pin an
        // all-literal division with `as f64`: `.floor()` on {float}
        // is f32/f64-ambiguous (E0689) — a register operand types the
        // division, but the float fold can leave BOTH operands
        // literal (fuzzer seed 65; fconst_04 pins it).
        Instruction::IntDiv { target, left, right } => {
            if is_float_reg(*target, alloc) {
                let div = floor_div_str(*left, *right, consts);
                out.push_str(&format!("f_r{target} = {div}.floor();\n"))
            } else {
                out.push_str(&format!(
                    "i_r{target} = {} / {} - i64::from({} % {} != 0 && ({} < 0) != ({} < 0));\n",
                    iop_str!(*left, consts), iop_str!(*right, consts),
                    iop_str!(*left, consts), iop_str!(*right, consts),
                    iop_str!(*left, consts), iop_str!(*right, consts)))
            }
        }
        // Lua modulo: result takes the divisor's sign (-7 % 3 == 2),
        // unlike Rust's truncated remainder — adjust the remainder by
        // the divisor exactly when the two signs disagree.
        Instruction::Mod { target, left, right } => {
            if is_float_reg(*target, alloc) {
                // same E0689 pin as IntDiv: the inner (l / r) riding
                // .floor() is all-literal when both operands folded
                let div = floor_div_str(*left, *right, consts);
                out.push_str(&format!("f_r{target} = {} - {div}.floor() * {};\n",
                    fop_str!(*left, consts), fop_str!(*right, consts)))
            } else {
                out.push_str(&format!(
                    "i_r{target} = {} % {} + i64::from({} % {} != 0 && ({} % {} < 0) != ({} < 0)) * {};\n",
                    iop_str!(*left, consts), iop_str!(*right, consts),
                    iop_str!(*left, consts), iop_str!(*right, consts),
                    iop_str!(*left, consts), iop_str!(*right, consts),
                    iop_str!(*right, consts), iop_str!(*right, consts)))
            }
        }
        Instruction::Neg { target, source } => {
            if is_float_reg(*target, alloc) {
                out.push_str(&format!("f_r{target} = -{};\n", fop_str!(*source, consts)))
            } else {
                out.push_str(&format!("i_r{target} = -{};\n", iop_str!(*source, consts)))
            }
        }
        Instruction::Less { target, left, right } => {
            // operand-based float fork: a CONST-float left (layer-B id) is
            // not in the float pool's mint range, so the map check rides
            // shotgun — without it a multi-def Less over folded floats
            // renders i_r<const-id>, an undeclared register
            if is_float_operand(*left, consts, alloc) {
                out.push_str(&format!("b_r{target} = {} < {};\n", fop_str!(*left, consts), fop_str!(*right, consts)))
            } else {
                out.push_str(&format!("b_r{target} = {} < {};\n", iop_str!(*left, consts), iop_str!(*right, consts)))
            }
        }
        Instruction::Leq { target, left, right } => {
            if is_float_operand(*left, consts, alloc) {
                out.push_str(&format!("b_r{target} = {} <= {};\n", fop_str!(*left, consts), fop_str!(*right, consts)))
            } else {
                out.push_str(&format!("b_r{target} = {} <= {};\n", iop_str!(*left, consts), iop_str!(*right, consts)))
            }
        }
        Instruction::Geq { target, left, right } => {
            if is_float_operand(*left, consts, alloc) {
                out.push_str(&format!("b_r{target} = {} >= {};\n", fop_str!(*left, consts), fop_str!(*right, consts)))
            } else {
                out.push_str(&format!("b_r{target} = {} >= {};\n", iop_str!(*left, consts), iop_str!(*right, consts)))
            }
        }
        // The OPERANDS' layer drives the rendering (pool range or const
        // variant — the id alone answers); the carried ty is VERIFIED,
        // not consulted. It is still carried for the const FOLD, which
        // runs pre-alloc where ids carry no ranges (Invariant 3), and
        // the checker guarantees Eq is homogeneous with a scalar ty —
        // so left's kind is the instruction's kind. History: under the
        // old shared Int/Bool range, ty was the ONLY correct
        // disambiguator here; the unique-id mint made the operands'
        // own pools authoritative, and the assert keeps the two
        // answers from ever drifting apart silently.
        Instruction::Eq { target, left, right, ty } => {
            let kind = operand_kind(*left, consts, alloc);
            assert!(kind == ty_kind(ty),
                "Eq: operand {left} kinds as {kind:?} but the instruction carries \
                 ty {ty:?} — pool/ty divergence, the id-space law broke upstream");
            match kind {
                OpKind::Float =>
                    out.push_str(&format!("b_r{target} = {} == {};\n", fop_str!(*left, consts), fop_str!(*right, consts))),
                OpKind::Str =>
                    out.push_str(&format!("b_r{target} = {} == {};\n", sop_str!(*left, consts), sop_str!(*right, consts))),
                OpKind::Bool =>
                    out.push_str(&format!("b_r{target} = {} == {};\n", bop_str!(*left, consts), bop_str!(*right, consts))),
                // Int — the checker's only remaining scalar; table
                // comparisons never reach emission (checker rejection)
                _ =>
                    out.push_str(&format!("b_r{target} = {} == {};\n", iop_str!(*left, consts), iop_str!(*right, consts))),
            }
        }
        Instruction::Not { target, source } =>
            out.push_str(&format!("b_r{target} = !{};\n", bop_str!(*source, consts))),

        // Runtime observation, Lua-style: the line prints exactly what
        // print received — the literal tag (first argument, when present)
        // as the first tab-separated field, then each operand's value,
        // strings raw and unquoted. No PROBE prefix, no register names:
        // the register mapping lives in the probe_map.txt sidecar
        // (build.rs scan_probes), which pins each probe site's
        // pool-prefixed physical registers, block, and depth; the tag and
        // line order join the two. Determinism contract unchanged:
        // values, handles and arena-derived lengths only — never
        // addresses. A null table handle (handle mode) prints `nil` —
        // the one observable nil in the language; a live table prints
        // its 1-based arena handle and materialized length (pointer
        // mode has no handle: `table(len=N)`).
        Instruction::DebugProbe { tag, operands } => {
            let mut fields: Vec<String> = Vec::new();
            let mut args: Vec<String> = Vec::new();
            // The tag is RAW user text (the lexer does no escape
            // processing), so it can never sit in format-template
            // position: a backslash there is re-read as a Rust escape —
            // an invalid one (\p) breaks the build, a valid one (\n)
            // silently corrupts the output. It rides as the first
            // println! ARGUMENT, embedded with {:?} — the LoadString
            // invariant: Debug escaping always renders a valid Rust
            // literal, and runtime {} prints it byte-for-byte.
            if !tag.is_empty() {
                fields.push("{}".to_string());
                args.push(format!("{:?}", tag));
            }
            for &(r, ref t) in operands {
                // The operand's own layer kinds it (pool range or const
                // variant); the carried kind is asserted, not consulted
                // — the same pool==ty frontier as Eq. reg_alloc's pool
                // map seeds probe operands FROM these kinds, so the
                // assert can only fire if the law broke after minting.
                let kind = operand_kind(r, consts, alloc);
                assert!(kind == ty_kind(t),
                    "DebugProbe: operand {r} kinds as {kind:?} but carries {t:?} — \
                     pool/ty divergence, the id-space law broke upstream");
                match kind {
                    OpKind::Int => {
                        fields.push("{}".to_string());
                        args.push(iop_str!(r, consts).to_string());
                    }
                    OpKind::Float => {
                        fields.push("{:?}".to_string());
                        args.push(fop_str!(r, consts).to_string());
                    }
                    OpKind::Bool => {
                        fields.push("{}".to_string());
                        args.push(bop_str!(r, consts).to_string());
                    }
                    OpKind::Str => {
                        fields.push("{}".to_string());
                        args.push(sop_str!(r, consts).to_string());
                    }
                    OpKind::Table => {
                        let fld = table_fld(r, alloc);
                        if uses_handles {
                            fields.push("{}".to_string());
                            args.push(format!(
                                "match tables.get((t_r{r} - 1) as usize) \
                                 {{ Some(t) => format!(\"table#{{}}(len={{}})\", t_r{r}, t.{fld}().len()), \
                                 None => \"nil\".to_string() }}"
                            ));
                        } else {
                            // pointer mode: a table-typed operand is
                            // always NewTable-defined before use (only
                            // nested programs read tables out of tables,
                            // and those render handle mode), so the
                            // deref cannot see null
                            fields.push("table(len={})".to_string());
                            args.push(format!("unsafe {{ (*t_r{r}).{fld}().len() }}"));
                        }
                    }
                }
            }
            if fields.is_empty() {
                // print(): a bare newline, exactly like Lua
                out.push_str("println!();\n");
                return;
            }
            // print("tag") needs no special case: the tag is an argument
            // like any operand, so a non-empty fields list always has one
            out.push_str(&format!(
                "println!(\"{}\", {});\n",
                fields.join("\\t"),
                args.join(", ")
            ));
        }

        Instruction::EnsureCapacity { table, limit } => {
            // Storage side and resize zero from the table's pool (the
            // one oracle). Monomorphism guarantees the fast ops riding
            // this EC agree — they ask the same oracle.
            let (fld, zero) = table_fld_zero(*table, alloc);
            if uses_handles {
                out.push_str(&format!(
                    "let lim = {lim};\n\
                     if lim > 0 {{\n\
                         if t_r{table} == 0 {{ panic!(\"Runtime Error: table is nil\"); }}\n\
                         let t = match tables.get_mut((t_r{table} - 1) as usize) {{ Some(t) => &mut **t, None => panic!(\"Runtime Error: table is nil\") }};\n\
                         if (lim as usize) > t.{fld}_mut().len() {{\n\
                             t.{fld}_mut().resize(lim as usize, {zero});\n\
                         }}\n\
                     }}\n",
                    lim = iop_str!(*limit, consts)
                ));
            } else {
                out.push_str(&format!(
                    "let lim = {lim};\n\
                     if lim > 0 {{\n\
                         let t = unsafe {{ &mut *t_r{table} }};\n\
                         if (lim as usize) > t.{fld}_mut().len() {{\n\
                             t.{fld}_mut().resize(lim as usize, {zero});\n\
                         }}\n\
                     }}\n",
                    lim = iop_str!(*limit, consts)
                ));
            }
        }
        Instruction::HoistRawPtr { table } => {
            let fld = table_fld(*table, alloc);
            if uses_handles {
                out.push_str(&format!(
                    "if t_r{table} == 0 {{ panic!(\"Runtime Error: table is nil\"); }}\n\
                     let t = match tables.get_mut((t_r{table} - 1) as usize) {{ Some(t) => &mut **t, None => panic!(\"Runtime Error: table is nil\") }};\n\
                     len_r{table} = t.{fld}_mut().len();\n\
                     p_r{table} = t.{fld}_mut().as_mut_ptr();\n"
                ));
            } else {
                out.push_str(&format!(
                    "len_r{table} = unsafe {{ (*t_r{table}).{fld}_mut().len() }};\n\
                     p_r{table} = unsafe {{ (*t_r{table}).{fld}_mut().as_mut_ptr() }};\n"
                ));
            }
        }

        Instruction::SetTable { table, key, val, ty } => {
            // Storage side from the TABLE's pool, value rendering from
            // the VALUE operand's own layer (pool range or const
            // variant); the carried element type is verified against
            // the pool, never consulted for the choice. Integer and
            // table-element stores render byte-identically to the
            // frozen templates (fld="as_int", zero="0"); floats,
            // strings, bools take their own sides.
            assert_fld(*table, ty, alloc, "SetTable");
            let (fld, zero) = table_fld_zero(*table, alloc);
            let val_str = match operand_kind(*val, consts, alloc) {
                // a stored table is a raw handle copy (handle mode —
                // pure pointer programs never store tables)
                OpKind::Table => format!("t_r{val}"),
                OpKind::Float => fop_str!(*val, consts).to_string(),
                OpKind::Str =>
                    // clone: a store COPIES the immutable value in
                    // (a move would kill the source slot — the borrow
                    // checker rejects it at build time, and the source
                    // is exactly what a loop-carried string is). A CONST
                    // val materializes with .to_string() — `.clone()` on
                    // a literal resolves to &str's Clone (the Move-arm
                    // hazard, same fix).
                    match consts.get(val) {
                        Some(ConstVal::String(v)) => format!("{v:?}.to_string()"),
                        _ => format!("{}.clone()", sop_str!(*val, consts)),
                    },
                OpKind::Bool => bop_str!(*val, consts).to_string(),
                OpKind::Int => iop_str!(*val, consts).to_string(),
            };
            if uses_handles {
                out.push_str(&format!(
                    "let k = {key};\n\
                     if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                     let idx = k as usize;\n\
                     if t_r{table} == 0 {{ panic!(\"Runtime Error: table is nil\"); }}\n\
                     let t = match tables.get_mut((t_r{table} - 1) as usize) {{ Some(t) => &mut **t, None => panic!(\"Runtime Error: table is nil\") }};\n\
                     if idx >= t.{fld}_mut().len() {{ t.{fld}_mut().resize(idx + 1, {zero}); }}\n\
                     unsafe {{ *t.{fld}_mut().get_unchecked_mut(idx) = {val_str}; }}\n",
                    key = iop_str!(*key, consts)
                ));
            } else {
                out.push_str(&format!(
                    "let k = {key};\n\
                     if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                     let idx = k as usize;\n\
                     let t = unsafe {{ &mut *t_r{table} }};\n\
                     if idx >= t.{fld}_mut().len() {{ t.{fld}_mut().resize(idx + 1, {zero}); }}\n\
                     unsafe {{ *t.{fld}_mut().get_unchecked_mut(idx) = {val_str}; }}\n",
                    key = iop_str!(*key, consts)
                ));
            }
        }
        Instruction::GetTable { target, table, key, ty } => {
            // Storage side from the TABLE's pool, target register from
            // the TARGET's own pool; the carried element type is
            // verified, never consulted. String elements are owned,
            // not Copy: every read CLONES out of the Vec (dyn and
            // fast), and the absent-key default is the empty string —
            // nil-as-absence compiled to the pool's zero, exactly like
            // 0 and 0.0.
            assert_fld(*table, ty, alloc, "GetTable");
            if operand_kind(*target, consts, alloc) == OpKind::Str {
                if uses_handles {
                    out.push_str(&format!(
                        "let k = {key};\n\
                         if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         let idx = k as usize;\n\
                         if t_r{table} == 0 {{ panic!(\"Runtime Error: table is nil\"); }}\n\
                         let t = match tables.get((t_r{table} - 1) as usize) {{ Some(t) => &**t, None => panic!(\"Runtime Error: table is nil\") }};\n\
                         s_r{target} = if idx < t.as_string().len() {{ unsafe {{ t.as_string().get_unchecked(idx).clone() }} }} else {{ String::new() }};\n",
                        key = iop_str!(*key, consts)
                    ));
                } else {
                    out.push_str(&format!(
                        "let k = {key};\n\
                         if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         let idx = k as usize;\n\
                         let t = unsafe {{ &*t_r{table} }};\n\
                         s_r{target} = if idx < t.as_string().len() {{ unsafe {{ t.as_string().get_unchecked(idx).clone() }} }} else {{ String::new() }};\n",
                        key = iop_str!(*key, consts)
                    ));
                }
            } else {
                let (fld, zero) = table_fld_zero(*table, alloc);
                let target_str = match operand_kind(*target, consts, alloc) {
                    OpKind::Table => format!("t_r{target}"),
                    OpKind::Float => format!("f_r{target}"),
                    OpKind::Bool => format!("b_r{target}"),
                    _ => format!("i_r{target}"),
                };
                if uses_handles {
                    out.push_str(&format!(
                        "let k = {key};\n\
                         if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         let idx = k as usize;\n\
                         if t_r{table} == 0 {{ panic!(\"Runtime Error: table is nil\"); }}\n\
                         let t = match tables.get((t_r{table} - 1) as usize) {{ Some(t) => &**t, None => panic!(\"Runtime Error: table is nil\") }};\n\
                         {target_str} = if idx < t.{fld}().len() {{ unsafe {{ *t.{fld}().get_unchecked(idx) }} }} else {{ {zero} }};\n",
                        key = iop_str!(*key, consts)
                    ));
                } else {
                    out.push_str(&format!(
                        "let k = {key};\n\
                         if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         let idx = k as usize;\n\
                         let t = unsafe {{ &*t_r{table} }};\n\
                         {target_str} = if idx < t.{fld}().len() {{ unsafe {{ *t.{fld}().get_unchecked(idx) }} }} else {{ {zero} }};\n",
                        key = iop_str!(*key, consts)
                    ));
                }
            }
        }
        Instruction::SetTableFast { table, key, val, ty } => {
            // The store rides p_r{table} — a pointer whose type the DECL
            // block derived from the table's pool — so the value's own
            // layer must render it; the carried element type is verified
            // against the pool (a divergence is a generated-code type
            // error at best, memory corruption at worst — now it is a
            // build-time tripwire instead).
            assert_fld(*table, ty, alloc, "SetTableFast");
            let val_str = match operand_kind(*val, consts, alloc) {
                OpKind::Table => format!("t_r{val}"),
                OpKind::Float => fop_str!(*val, consts).to_string(),
                OpKind::Str =>
                    // clone — same immutable-copy semantics as the dyn store;
                    // const vals materialize via .to_string() (same &str-Clone
                    // hazard, same fix)
                    match consts.get(val) {
                        Some(ConstVal::String(v)) => format!("{v:?}.to_string()"),
                        _ => format!("{}.clone()", sop_str!(*val, consts)),
                    },
                OpKind::Bool => bop_str!(*val, consts).to_string(),
                OpKind::Int => iop_str!(*val, consts).to_string(),
            };
            out.push_str(&format!(
                "let k = {key};\n\
                 if k < 0 {{ panic!(\"Runtime Error: Negative index in fast path\"); }}\n\
                 if (k as usize) < len_r{table} {{\n\
                     unsafe {{ *p_r{table}.add(k as usize) = {val_str}; }}\n\
                 }} else {{\n\
                     panic!(\"optimizer invariant violated: fast-path bounds check failed\");\n\
                 }}\n",
                key = iop_str!(*key, consts)
            ));
        }
        Instruction::GetTableFast { target, table, key, ty } => {
            // The load rides p_r{table} (pool-typed in the decl block);
            // the target register's own pool picks its spelling, the
            // carried element type is verified. String fast reads clone
            // through the hoisted pointer — the shared template's plain
            // deref would move out of the Vec, which the borrow checker
            // (rightly) forbids.
            assert_fld(*table, ty, alloc, "GetTableFast");
            if operand_kind(*target, consts, alloc) == OpKind::Str {
                out.push_str(&format!(
                    "let k = {key};\n\
                     if k < 0 {{ panic!(\"Runtime Error: Negative index in fast path\"); }}\n\
                     if (k as usize) < len_r{table} {{\n\
                         s_r{target} = unsafe {{ (*p_r{table}.add(k as usize)).clone() }};\n\
                     }} else {{\n\
                         panic!(\"optimizer invariant violated: fast-path bounds check failed\");\n\
                     }}\n",
                    key = iop_str!(*key, consts)
                ));
            } else {
                let target_str = match operand_kind(*target, consts, alloc) {
                    OpKind::Table => format!("t_r{target}"),
                    OpKind::Float => format!("f_r{target}"),
                    OpKind::Bool => format!("b_r{target}"),
                    _ => format!("i_r{target}"),
                };
                out.push_str(&format!(
                    "let k = {key};\n\
                     if k < 0 {{ panic!(\"Runtime Error: Negative index in fast path\"); }}\n\
                     if (k as usize) < len_r{table} {{\n\
                         {target_str} = unsafe {{ *p_r{table}.add(k as usize) }};\n\
                     }} else {{\n\
                         panic!(\"optimizer invariant violated: fast-path bounds check failed\");\n\
                     }}\n",
                    key = iop_str!(*key, consts)
                ));
            }
        }
        Instruction::Phi { .. } => {} // deleted by resolve_phis
    }
}

/// Every block reachable from `b` via any terminator edge.
fn reachable_from(program: &IrProgram, b: BlockId) -> HashSet<BlockId> {
    let mut seen = HashSet::new();
    let mut stack = vec![b];
    while let Some(x) = stack.pop() {
        if !seen.insert(x) { continue; }
        match &program.blocks[x].terminator {
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
fn common_join(program: &IrProgram, tb: BlockId, fb: BlockId) -> Option<BlockId> {
    let rf = reachable_from(program, fb);
    reachable_from(program, tb).into_iter()
        .filter(|b| *b > tb.max(fb) && rf.contains(b))
        .min()
}

/// The natural loop of header `h`: `h` plus every block that can reach
/// `h`'s back edge without passing through `h` (pred-walk from the back
/// edge, stopping at `h`). The loop's condition chain — the desugared
/// and/or blocks sitting between the header and the body — is inside by
/// construction: it reaches the body, which reaches the back edge.
fn natural_loop_region(program: &IrProgram, h: BlockId) -> HashSet<BlockId> {
    let targets = |t: &Option<Terminator>| -> Vec<BlockId> {
        match t {
            Some(Terminator::Jump(t)) => vec![*t],
            Some(Terminator::Branch { true_block, false_block, .. }) => vec![*true_block, *false_block],
            _ => vec![],
        }
    };
    let mut region = HashSet::from([h]);
    for (id, block) in program.blocks.iter().enumerate() {
        // back edge: a later block targeting h that h dominates (the
        // is_loop_header definition; a Jump-only loop tail is the shape
        // this lowerer mints)
        if targets(&block.terminator).contains(&h) && dominates(program, h, id) {
            let mut stack = vec![id];
            while let Some(b) = stack.pop() {
                if region.insert(b) {
                    for (p, pblk) in program.blocks.iter().enumerate() {
                        if targets(&pblk.terminator).contains(&b) { stack.push(p); }
                    }
                }
            }
        }
    }
    region
}

/// The loop's condition gate: the unique branch inside the region whose
/// false edge leaves it. For a plain `while c` this is the header's own
/// Branch; for `while a and b` the header branches on `a` and the gate
/// sits at the end of the condition chain, branching on the join phi
/// (true -> body, false -> exit). Nothing else in this language branches
/// to a loop's exit, so a count other than one is a malformed CFG.
fn loop_gate(
    program: &IrProgram,
    region: &HashSet<BlockId>,
    h: BlockId,
) -> (BlockId, RegId, BlockId, BlockId) {
    let mut gates: Vec<(BlockId, RegId, BlockId, BlockId)> = Vec::new();
    for &b in region {
        if let Some(Terminator::Branch { cond, true_block, false_block }) = &program.blocks[b].terminator
            && !region.contains(false_block)
        {
            gates.push((b, *cond, *true_block, *false_block));
        }
    }
    match gates.as_slice() {
        [(g, c, t, f)] => (*g, *c, *t, *f),
        _ => panic!("structured codegen: loop at {h} has {} exit branches, expected exactly 1", gates.len()),
    }
}

/// Emit block `b` and everything that follows it, staying inside the loop
/// whose header is `hdr` (a back edge to `hdr` closes the loop body).
/// `stop` is the join block of an enclosing structured if: reaching it
/// ends this arm — the parent emits the join after both arms.
/// `gate` is the enclosing loop's exit block while walking a condition
/// chain: the branch whose false edge is the gate is the chain's last
/// block, emitted as `if cond { body } else { break; }` — the caller's
/// `loop {` wrapper (emit_loop) makes the break close the iteration.
fn emit_seq(
    out: &mut String,
    env: EmitEnv,
    b: BlockId,
    hdr: Option<BlockId>,
    stop: Option<BlockId>,
    gate: Option<BlockId>,
    emitted: &mut [bool],
) {
    let (program, _alloc, consts, _uses_handles) = env;
    if Some(b) == stop { return; }
    if emitted[b] { panic!("structured codegen: block {b} reached twice — CFG is not a tree"); }
    emitted[b] = true;
    let block = &program.blocks[b];
    for i in &block.instrs { emit_instr(out, env, i); }
    match &block.terminator {
        None | Some(Terminator::Halt) => {
            // early return == the dispatcher's `break 'cfg`: there is no
            // code after Halt, so jumping to the end is exactly a return
            out.push_str("return tables;\n");
        }
        Some(Terminator::Jump(t)) => {
            if Some(*t) == hdr {
                // back edge: this loop body is complete
            } else if Some(*t) == stop {
                // if-arm reached its join: the parent continues
            } else if *t < b {
                panic!("structured codegen: stray backward jump {b} -> {t}");
            } else if is_loop_header(program, *t) {
                // forward jump into a loop header = entering a loop
                let after = emit_loop(out, env, *t, emitted);
                emit_seq(out, env, after, hdr, stop, gate, emitted);
            } else {
                emit_seq(out, env, *t, hdr, stop, gate, emitted);
            }
        }
        Some(Terminator::Branch { cond, true_block, false_block })
            if is_loop_header(program, b) && gate.is_none() =>
        {
            // a header reached directly (not via its pre-header Jump):
            // same handling as the Jump-into-header case. Never fires
            // inside a condition chain — the chain walk entered that
            // header already (gate.is_some()), so this Branch is the
            // chain's first if, not a loop entry.
            let after = emit_loop(out, env, b, emitted);
            emit_seq(out, env, after, hdr, stop, gate, emitted);
        }
        Some(Terminator::Branch { cond, true_block, false_block }) => {
            // the tail of an and/or condition chain: the false edge is
            // the loop's exit. The true arm is the body (it ends at the
            // back edge); the false arm breaks — emit_loop's caller
            // continues from the exit after the loop.
            if Some(*false_block) == gate {
                out.push_str(&format!("if {} {{\n", bop_str!(*cond, consts)));
                emit_seq(out, env, *true_block, hdr, stop, None, emitted);
                out.push_str("    } else {\n        break;\n    }\n");
                return;
            }
            // non-header branch = structured if/else. Both arms converge
            // on the join block, which the parent emits after the arms.
            let join = common_join(program, *true_block, *false_block);
            out.push_str(&format!("if {} {{\n", bop_str!(*cond, consts)));
            emit_seq(out, env, *true_block, hdr, join, gate, emitted);
            // skip an `else` that would be empty: bare else-block with
            // no instructions jumping straight to the join. The block
            // is still CONSUMED — mark it emitted, or the orphan check
            // below fires on the common `if c then flag = true end`
            // inside a loop (the coalesced loop phi turns the else
            // arm's join Move into a removable self-copy, re-emptying
            // the block; found by probe_ops_loop_forms).
            let trivial_else = program.blocks[*false_block].instrs.is_empty()
                && matches!(&program.blocks[*false_block].terminator,
                            Some(Terminator::Jump(t)) if Some(*t) == join);
            if !trivial_else {
                out.push_str("} else {\n");
                emit_seq(out, env, *false_block, hdr, join, gate, emitted);
            } else {
                emitted[*false_block] = true;
            }
            out.push_str("}\n");
            if let Some(j) = join {
                emit_seq(out, env, j, hdr, stop, gate, emitted);
            }
        }
    }
}

/// Emit a whole loop. Derives the loop's condition gate from the natural
/// loop region (the unique in-region branch whose false edge exits it)
/// and returns the after-loop continuation block — the block emit_loop's
/// caller emits next, outside the loop.
///
/// Direct gate (every loop without and/or in its condition): the gate IS
/// the header's Branch, byte-identical to the historical emission.
///
/// Condition chain (`while a and b`): the header branches on `a`, the
/// chain's blocks form structured ifs inside the region, and the gate
/// sits at the chain's end (the join, branching on the merged phi). The
/// emission is `loop { <chain as if/else> ; if <gate> { body } else {
/// break; } }` — the condition re-evaluates every iteration, exactly
/// like a while header does.
fn emit_loop(
    out: &mut String,
    env: EmitEnv,
    h: BlockId,
    emitted: &mut [bool],
) -> BlockId {
    let (program, alloc, consts, _uses_handles) = env;
    if !is_loop_header(program, h) {
        panic!("structured codegen: Branch in block {h} is not a loop header");
    }
    if emitted[h] { panic!("structured codegen: header {h} reached twice"); }

    let region = natural_loop_region(program, h);
    let (gate_b, cond, body, exit) = loop_gate(program, &region, h);

    if gate_b != h {
        // Condition chain: the header's own Branch is the chain's first
        // if; emit_seq walks the chain (gate = the exit block marks the
        // chain's tail) and the body inside one Rust loop.
        out.push_str("loop {\n");
        emit_seq(out, env, h, Some(h), None, Some(exit), emitted);
        out.push_str("}\n");
        return exit;
    }

    emitted[h] = true;

    let block = &program.blocks[h];

    // Pretty form: the header holds nothing (identifier condition, e.g.
    // phase I / bug16a) or exactly the Less computing the branch
    // condition with no other readers of its result (count_uses == 1 —
    // exact since physical ids are unique; the pre-uniqueness era
    // needed a bool-CONTEXT counter because a co-numbered int's uses
    // inflated the raw count). The Less folds into the
    // while-condition — still evaluated every iteration.
    //
    // Float operands DECLINE the pretty arm: iop_str! below would render
    // them as `i_r<id>` — undeclared registers, a program that does not
    // compile (fwhile_01's shape: identifier-bound float bounds put the
    // bare Less alone in the header. Literal float bounds no longer load
    // in the header — the lowerer's loop-invariant load hoist relocates
    // them to the pre-header — so fwhile_02's shape reaches this arm as
    // a lone Less too, declined by the operand check alone). The check
    // spans the physical float pool AND the const map's Float variant —
    // a folded float operand's layer-B id is outside every mint range.
    // The fallback emits the Less through emit_instr, whose float arm is
    // correct — exactly what it exists for. Only Less needs the guard:
    // Leq/Geq never ride the pretty arm, and bool/string cannot be `<`
    // operands.
    let pretty = if block.instrs.is_empty() {
        Some(bop_str!(cond, consts).to_string())
    } else if block.instrs.len() == 1 {
        match &block.instrs[0] {
            Instruction::Less { target, left, right }
                if *target == cond && count_uses(program, cond) == 1
                    && !is_float_operand(*left, consts, alloc) =>
                Some(format!("{} < {}", iop_str!(*left, consts), iop_str!(*right, consts))),
            _ => None,
        }
    } else { None };

    if let Some(c) = pretty {
        out.push_str(&format!("while {c} {{\n"));
        emit_seq(out, env, body, Some(h), None, None, emitted);
        out.push_str("}\n");
    } else {
        // General fallback: everything in the header runs every
        // iteration. Float loop conditions land here since the
        // pretty-arm decline (fwhile_01 pins the shape) — it exists so a
        // surprising CFG degrades to correct-but-ugly, not wrong.
        out.push_str("loop {\n");
        for i in &block.instrs { emit_instr(out, env, i); }
        out.push_str(&format!("if {} {{\n", bop_str!(cond, consts)));
        emit_seq(out, env, body, Some(h), None, None, emitted);
        out.push_str("    } else {\n");
        out.push_str("        break;\n");
        out.push_str("    }\n}\n");
    }
    exit
}

pub fn generate_rust_code(
    program: &IrProgram,
    alloc: &AllocInfo,
    consts: &HashMap<RegId, ConstVal>,
) -> String {
    // AllocInfo is a named tuple of pool counts + range bases — pull out
    // every value once up front; emission below only reads these.
    let AllocInfo {
        n_int: n_i, n_bool: n_b, n_float: n_f, n_str: n_s, n_table: n_t,
        n_ftable: n_tf, n_tstr: n_ts, n_btable: n_tb,
        int_base: ibase, bool_base: bbase, table_base: tbase,
        str_base: sbase, float_base: fbase, ftable_base: tfbase,
        tstr_base: tsbase, btable_base: tbbase,
    } = *alloc;
    let (ibase, bbase, tbase, sbase, fbase, tfbase, tsbase, tbbase) = (
        ibase as usize, bbase as usize, tbase as usize, sbase as usize,
        fbase as usize, tfbase as usize, tsbase as usize, tbbase as usize,
    );

    let uses_handles = program_uses_handles(&program.blocks);

    let mut out = String::new();
    out.push_str("// target/release/build/phia-*/out/baked_native.rs\n\n");
    out.push_str("use crate::memory::Table;\n\n");
    // clippy::eq_op rides the fn-level allow: consts fold can leave a
    // live Less/Div whose OPERANDS both folded to the same literal
    // (`b_r326 = 0 < 0;`) — deny-by-default on generated code otherwise
    // fails `cargo clippy` on artifacts the compiler emitted on purpose
    // (the deliberate relock choice (b); target/ filtering was (a)).
    out.push_str("#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]\n");
    out.push_str("pub fn run_baked() -> Vec<Box<Table>> {\n");

    let mut fast_phys: HashSet<RegId> = HashSet::new();
    for b in &program.blocks {
        for i in &b.instrs {
            if let Instruction::HoistRawPtr { table }
                | Instruction::SetTableFast { table, .. }
                | Instruction::GetTableFast { table, .. } = i {
                fast_phys.insert(*table);
            }
        }
    }

    // Decl block walks the mint timeline: int, bool, table, string,
    // float, then the element-kind table pools — decl order IS range
    // order, so the generated locals read in the same order the ids
    // were handed out. Every id in these loops is unique across pools;
    // pure-int programs (and int+float programs) emit the same decl
    // bytes they always did.
    for r in ibase..ibase + n_i { out.push_str(&format!("    let mut i_r{r} = 0i64;\n")); }
    for r in bbase..bbase + n_b { out.push_str(&format!("    let mut b_r{r} = false;\n")); }
    for r in tbase..tbase + n_t { emit_table_decl(&mut out, alloc, r as RegId, uses_handles, &fast_phys); }
    for r in sbase..sbase + n_s { out.push_str(&format!("    let mut s_r{r} = String::new();\n")); }
    for r in fbase..fbase + n_f { out.push_str(&format!("    let mut f_r{r} = 0f64;\n")); }
    for r in tfbase..tfbase + n_tf { emit_table_decl(&mut out, alloc, r as RegId, uses_handles, &fast_phys); }
    for r in tsbase..tsbase + n_ts { emit_table_decl(&mut out, alloc, r as RegId, uses_handles, &fast_phys); }
    for r in tbbase..tbbase + n_tb { emit_table_decl(&mut out, alloc, r as RegId, uses_handles, &fast_phys); }
    out.push_str("    let mut tables = Vec::<Box<Table>>::with_capacity(128);\n\n");

    let env: EmitEnv = (program, alloc, consts, uses_handles);
    let mut emitted = vec![false; program.blocks.len()];
    emit_seq(&mut out, env, 0, None, None, None, &mut emitted);
    let orphans: Vec<usize> = emitted.iter().enumerate()
        .filter(|(_, e)| !**e).map(|(i, _)| i).collect();
    if !orphans.is_empty() {
        panic!("structured codegen: blocks never reached: {orphans:?}");
    }

    out.push_str("}\n");
    out
}
