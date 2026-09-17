// src/memory.rs

// 32 bytes: one tag + one Vec payload (24 bytes, align 8). The compiler
// proves tables strictly monomorphic — the pool oracle (reg_alloc) picks
// the variant at NewTable and it never changes — so the element sides
// share one payload slot instead of costing four Vec headers plus flags
// (the old struct-of-sides was 104 bytes per table). The Box<Table>
// indirection in run_baked's arena keeps the header address stable
// across arena growth and data reallocs: t_r{N} pointers and the
// p_r{N}/len_r{N} cache re-derived by HoistRawPtr stay valid — the
// Phase J contract, preserved verbatim.
pub enum Table {
    Int(Vec<i64>),
    Float(Vec<f64>),
    String(Vec<String>),
    // Vec<bool> is one BYTE per element in std (never bit-packed); the
    // bitmap-table claim the old struct made here was false — indexing
    // semantics were always byte-wise and stay byte-wise.
    Bool(Vec<bool>),
}

const _: () = assert!(std::mem::size_of::<Table>() == 32);

impl Table {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Table::Int(Vec::with_capacity(256))
    }

    #[allow(dead_code)]
    pub fn new_float() -> Self {
        Table::Float(Vec::with_capacity(256))
    }

    #[allow(dead_code)]
    pub fn new_string() -> Self {
        Table::String(Vec::with_capacity(256))
    }

    #[allow(dead_code)]
    pub fn new_bool() -> Self {
        Table::Bool(Vec::with_capacity(256))
    }

    // Monomorphic accessors: the pool oracle guarantees exactly one
    // variant is ever consulted per table, so the wrong-variant arms
    // are unreachable BY CONSTRUCTION (assert_fld is the build-time
    // tripwire on the transpiler side; the debug_assert keeps a wrong
    // pool observable in debug builds, where unreachable_unchecked
    // alone would be silent UB). After inlining, the discriminant
    // switch folds away — the default arm is unreachable and the tag
    // load loses its last use — leaving a plain load at a constant
    // offset from the header: the same instruction count the old
    // struct field access compiled to.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn as_int(&self) -> &Vec<i64> {
        debug_assert!(matches!(self, Table::Int(_)));
        match self {
            Table::Int(arr) => arr,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn as_int_mut(&mut self) -> &mut Vec<i64> {
        debug_assert!(matches!(self, Table::Int(_)));
        match self {
            Table::Int(arr) => arr,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn as_float(&self) -> &Vec<f64> {
        debug_assert!(matches!(self, Table::Float(_)));
        match self {
            Table::Float(arr) => arr,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn as_float_mut(&mut self) -> &mut Vec<f64> {
        debug_assert!(matches!(self, Table::Float(_)));
        match self {
            Table::Float(arr) => arr,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn as_string(&self) -> &Vec<String> {
        debug_assert!(matches!(self, Table::String(_)));
        match self {
            Table::String(arr) => arr,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn as_string_mut(&mut self) -> &mut Vec<String> {
        debug_assert!(matches!(self, Table::String(_)));
        match self {
            Table::String(arr) => arr,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn as_bool(&self) -> &Vec<bool> {
        debug_assert!(matches!(self, Table::Bool(_)));
        match self {
            Table::Bool(arr) => arr,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn as_bool_mut(&mut self) -> &mut Vec<bool> {
        debug_assert!(matches!(self, Table::Bool(_)));
        match self {
            Table::Bool(arr) => arr,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}
