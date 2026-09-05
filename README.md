# phia

An experimental Ahead-Of-Time (AOT) Lua-to-Rust transpiler.

Supported features: while, do, end, local, integers, integer-key tables with integers as values, addition, subtraction, less than

# Input
```lua
-- main.lua — "The Gauntlet"
-- Strict Phia subset: integers, tables, local, while, +, -, <.
-- 13 phases. Each pins one optimizer outcome class; each final table
-- state is exactly predictable. No multiplication — checksums are
-- derived with + and closed forms only.

-- PHASE A — The Stride (non-unit increment; EC zero-fills the odd slots)
local pa_t = {}
local pa_n = 400
local pa_i = 0
while pa_i < pa_n do
    pa_t[pa_i] = pa_i + 1
    pa_i = pa_i + 2
end

-- PHASE B — The Staircase (inner limit = outer idx; EC+HR re-hoisted per
-- outer iteration, INSIDE the outer body: hoist_ctx=1)
local pb_t = {}
local pb_n = 300
local pb_i = 0
while pb_i < pb_n do
    local pb_j = 0
    while pb_j < pb_i do
        pb_t[pb_j] = pb_j + 1
        pb_j = pb_j + 1
    end
    pb_i = pb_i + 1
end

-- PHASE C — The Polisher (sibling loops, read-modify-write, both fast;
-- same table hoisted twice, second EC a no-op)
local pc_t = {}
local pc_n = 250
local pc_i = 0
while pc_i < pc_n do
    pc_t[pc_i] = 5
    pc_i = pc_i + 1
end
local pc_j = 0
while pc_j < pc_n do
    pc_t[pc_j] = pc_t[pc_j] + 1
    pc_j = pc_j + 1
end

-- PHASE D — The Sentinel (literal-bound fill stays dynamic — the bug-#1
-- canary, alive in the showcase; then data-dependent termination: the
-- search loop runs until it reads the sentinel)
local pd_t = {}
local pd_fill = 0
while pd_fill < 30 do
    pd_t[pd_fill] = 1
    pd_fill = pd_fill + 1
end
pd_t[30] = 60
local pd_lim = 60
local pd_i = 0
while pd_t[pd_i] < pd_lim do
    pd_i = pd_i + 1
end
local pd_w = {}
pd_w[0] = pd_i

-- PHASE E — The Frozen Handoff (alias defined at outer level, never
-- redefined in the nested region: survives the entry-kill — the SOUND
-- half of #15; the hoist lands BEFORE the outer loop)
local pe_t = {}
local pe_n = 200
local pe_m = 3
local pe_i = 0
while pe_i < pe_n do
    local pe_d = pe_i
    local pe_j = 0
    while pe_j < pe_m do
        pe_t[pe_d] = 7
        pe_j = pe_j + 1
    end
    pe_i = pe_i + 1
end

-- PHASE F — The Carried Alias (the #15 shape itself: key defined inside
-- the nested region; the enclosing candidate abstains, the INNER one
-- converts and hoists at ctx=1 with its own limit)
local pf_t = {}
local pf_n = 150
local pf_i = 0
while pf_i < pf_n do
    local pf_d = pf_i
    while pf_d < pf_n do
        pf_t[pf_d] = pf_d + 1
        pf_d = pf_d + 1
    end
    pf_i = pf_i + 1
end

-- PHASE G — The Mirror (fast reads from a hoisted table feeding dynamic
-- writes at computed keys into a second table)
local pg_src = {}
local pg_dst = {}
local pg_n = 180
local pg_i = 0
while pg_i < pg_n do
    pg_src[pg_i] = pg_i + 1
    pg_i = pg_i + 1
end
local pg_j = 0
while pg_j < pg_n do
    local pg_rev = pg_n - 1 - pg_j
    pg_dst[pg_rev] = pg_src[pg_j]
    pg_j = pg_j + 1
end

-- PHASE H — The Abacus (fast reads accumulating into a scalar, exported
-- through a witness table)
local ph_t = {}
local ph_n = 120
local ph_i = 0
while ph_i < ph_n do
    ph_t[ph_i] = ph_i + 1
    ph_i = ph_i + 1
end
local ph_sum = 0
local ph_j = 0
while ph_j < ph_n do
    ph_sum = ph_sum + ph_t[ph_j]
    ph_j = ph_j + 1
end
local ph_w = {}
ph_w[0] = ph_sum

-- PHASE I — The One-Shot (boolean flag as the only branch mechanism; no
-- candidate forms — header scan breaks at WhileCondition; all dynamic)
local pi_t = {}
local pi_flag = 0 < 1
local pi_k = 0
while pi_flag do
    pi_t[pi_k] = 100
    pi_flag = 0 < 0
    pi_k = pi_k + 1
end

-- PHASE J — The Terraces (one table grown in two epochs from a nonzero
-- start; the second EC reallocs and the second HR re-derives; the witness
-- probes the zero-filled gap and both boundaries)
local pj_t = {}
local pj_hi1 = 200
local pj_i = 100
while pj_i < pj_hi1 do
    pj_t[pj_i] = pj_i - 100
    pj_i = pj_i + 1
end
local pj_hi2 = 350
local pj_j = 200
while pj_j < pj_hi2 do
    pj_t[pj_j] = pj_j
    pj_j = pj_j + 1
end
local pj_w = {}
pj_w[0] = pj_t[150]
pj_w[1] = pj_t[250]
pj_w[2] = pj_t[349]
pj_w[3] = pj_t[99]

-- PHASE K — The Cube (three-deep nesting; only the innermost candidate
-- converts — the hoist sits at depth 2)
local pk_t = {}
local pk_n = 60
local pk_i = 0
while pk_i < pk_n do
    local pk_j = 0
    while pk_j < pk_n do
        local pk_k = 0
        while pk_k < pk_n do
            pk_t[pk_k] = pk_k + pk_j
            pk_k = pk_k + 1
        end
        pk_j = pk_j + 1
    end
    pk_i = pk_i + 1
end

-- PHASE L — The Poisoned Chalice (three alias-keyed writes, one broken
-- alias: the dynamic access retroactively poisons the table for the whole
-- candidate — all three stay dynamic)
local pl_t = {}
local pl_n = 80
local pl_i = 0
while pl_i < pl_n do
    local pl_a = pl_i
    pl_t[pl_a] = 1
    local pl_b = pl_a
    pl_b = pl_b + 0
    pl_t[pl_b] = 2
    local pl_c = pl_i
    pl_t[pl_c] = 3
    pl_i = pl_i + 1
end

-- PHASE M — The Handoff (two registers, one object: both views fast-path
-- with separate EC+HR pairs into the same allocation)
local pm_a = {}
local pm_b = pm_a
local pm_n = 90
local pm_i = 0
while pm_i < pm_n do
    pm_a[pm_i] = pm_i + 1
    pm_b[pm_i] = pm_i + 2
    pm_i = pm_i + 1
end
```

# Output
```rust
// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut b_r0 = false;
    let mut t_r0: *mut Table = std::ptr::null_mut();
    let mut p_r0: *mut i64 = std::ptr::null_mut();
    let mut len_r0 = 0usize;
    let mut i_r1 = 0i64;
    let mut b_r1 = false;
    let mut t_r1: *mut Table = std::ptr::null_mut();
    let mut p_r1: *mut i64 = std::ptr::null_mut();
    let mut len_r1 = 0usize;
    let mut i_r2 = 0i64;
    let mut b_r2 = false;
    let mut t_r2: *mut Table = std::ptr::null_mut();
    let mut p_r2: *mut i64 = std::ptr::null_mut();
    let mut len_r2 = 0usize;
    let mut i_r3 = 0i64;
    let mut b_r3 = false;
    let mut t_r3: *mut Table = std::ptr::null_mut();
    let mut p_r3: *mut i64 = std::ptr::null_mut();
    let mut len_r3 = 0usize;
    let mut i_r4 = 0i64;
    let mut b_r4 = false;
    let mut t_r4: *mut Table = std::ptr::null_mut();
    let mut p_r4: *mut i64 = std::ptr::null_mut();
    let mut len_r4 = 0usize;
    let mut i_r5 = 0i64;
    let mut b_r5 = false;
    let mut t_r5: *mut Table = std::ptr::null_mut();
    let mut p_r5: *mut i64 = std::ptr::null_mut();
    let mut len_r5 = 0usize;
    let mut i_r6 = 0i64;
    let mut b_r6 = false;
    let mut t_r6: *mut Table = std::ptr::null_mut();
    let mut p_r6: *mut i64 = std::ptr::null_mut();
    let mut len_r6 = 0usize;
    let mut i_r7 = 0i64;
    let mut b_r7 = false;
    let mut t_r7: *mut Table = std::ptr::null_mut();
    let mut p_r7: *mut i64 = std::ptr::null_mut();
    let mut len_r7 = 0usize;
    let mut i_r8 = 0i64;
    let mut b_r8 = false;
    let mut t_r8: *mut Table = std::ptr::null_mut();
    let mut p_r8: *mut i64 = std::ptr::null_mut();
    let mut len_r8 = 0usize;
    let mut i_r9 = 0i64;
    let mut b_r9 = false;
    let mut t_r9: *mut Table = std::ptr::null_mut();
    let mut p_r9: *mut i64 = std::ptr::null_mut();
    let mut len_r9 = 0usize;
    let mut i_r10 = 0i64;
    let mut b_r10 = false;
    let mut t_r10: *mut Table = std::ptr::null_mut();
    let mut p_r10: *mut i64 = std::ptr::null_mut();
    let mut len_r10 = 0usize;
    let mut i_r11 = 0i64;
    let mut b_r11 = false;
    let mut t_r11: *mut Table = std::ptr::null_mut();
    let mut p_r11: *mut i64 = std::ptr::null_mut();
    let mut len_r11 = 0usize;
    let mut i_r12 = 0i64;
    let mut b_r12 = false;
    let mut t_r12: *mut Table = std::ptr::null_mut();
    let mut p_r12: *mut i64 = std::ptr::null_mut();
    let mut len_r12 = 0usize;
    let mut i_r13 = 0i64;
    let mut b_r13 = false;
    let mut t_r13: *mut Table = std::ptr::null_mut();
    let mut p_r13: *mut i64 = std::ptr::null_mut();
    let mut len_r13 = 0usize;
    let mut i_r14 = 0i64;
    let mut b_r14 = false;
    let mut t_r14: *mut Table = std::ptr::null_mut();
    let mut p_r14: *mut i64 = std::ptr::null_mut();
    let mut len_r14 = 0usize;
    let mut i_r15 = 0i64;
    let mut b_r15 = false;
    let mut t_r15: *mut Table = std::ptr::null_mut();
    let mut p_r15: *mut i64 = std::ptr::null_mut();
    let mut len_r15 = 0usize;
    let mut i_r16 = 0i64;
    let mut b_r16 = false;
    let mut t_r16: *mut Table = std::ptr::null_mut();
    let mut p_r16: *mut i64 = std::ptr::null_mut();
    let mut len_r16 = 0usize;
    let mut i_r17 = 0i64;
    let mut b_r17 = false;
    let mut t_r17: *mut Table = std::ptr::null_mut();
    let mut p_r17: *mut i64 = std::ptr::null_mut();
    let mut len_r17 = 0usize;
    let mut i_r18 = 0i64;
    let mut b_r18 = false;
    let mut t_r18: *mut Table = std::ptr::null_mut();
    let mut p_r18: *mut i64 = std::ptr::null_mut();
    let mut len_r18 = 0usize;
    let mut i_r19 = 0i64;
    let mut b_r19 = false;
    let mut t_r19: *mut Table = std::ptr::null_mut();
    let mut p_r19: *mut i64 = std::ptr::null_mut();
    let mut len_r19 = 0usize;
    let mut i_r20 = 0i64;
    let mut b_r20 = false;
    let mut t_r20: *mut Table = std::ptr::null_mut();
    let mut p_r20: *mut i64 = std::ptr::null_mut();
    let mut len_r20 = 0usize;
    let mut i_r21 = 0i64;
    let mut b_r21 = false;
    let mut t_r21: *mut Table = std::ptr::null_mut();
    let mut p_r21: *mut i64 = std::ptr::null_mut();
    let mut len_r21 = 0usize;
    let mut i_r22 = 0i64;
    let mut b_r22 = false;
    let mut t_r22: *mut Table = std::ptr::null_mut();
    let mut p_r22: *mut i64 = std::ptr::null_mut();
    let mut len_r22 = 0usize;
    let mut i_r23 = 0i64;
    let mut b_r23 = false;
    let mut t_r23: *mut Table = std::ptr::null_mut();
    let mut p_r23: *mut i64 = std::ptr::null_mut();
    let mut len_r23 = 0usize;
    let mut i_r24 = 0i64;
    let mut b_r24 = false;
    let mut t_r24: *mut Table = std::ptr::null_mut();
    let mut p_r24: *mut i64 = std::ptr::null_mut();
    let mut len_r24 = 0usize;
    let mut i_r25 = 0i64;
    let mut b_r25 = false;
    let mut t_r25: *mut Table = std::ptr::null_mut();
    let mut p_r25: *mut i64 = std::ptr::null_mut();
    let mut len_r25 = 0usize;
    let mut i_r26 = 0i64;
    let mut b_r26 = false;
    let mut t_r26: *mut Table = std::ptr::null_mut();
    let mut p_r26: *mut i64 = std::ptr::null_mut();
    let mut len_r26 = 0usize;
    let mut i_r27 = 0i64;
    let mut b_r27 = false;
    let mut t_r27: *mut Table = std::ptr::null_mut();
    let mut p_r27: *mut i64 = std::ptr::null_mut();
    let mut len_r27 = 0usize;
    let mut i_r28 = 0i64;
    let mut b_r28 = false;
    let mut t_r28: *mut Table = std::ptr::null_mut();
    let mut p_r28: *mut i64 = std::ptr::null_mut();
    let mut len_r28 = 0usize;
    let mut i_r29 = 0i64;
    let mut b_r29 = false;
    let mut t_r29: *mut Table = std::ptr::null_mut();
    let mut p_r29: *mut i64 = std::ptr::null_mut();
    let mut len_r29 = 0usize;
    let mut i_r30 = 0i64;
    let mut b_r30 = false;
    let mut t_r30: *mut Table = std::ptr::null_mut();
    let mut p_r30: *mut i64 = std::ptr::null_mut();
    let mut len_r30 = 0usize;
    let mut i_r31 = 0i64;
    let mut b_r31 = false;
    let mut t_r31: *mut Table = std::ptr::null_mut();
    let mut p_r31: *mut i64 = std::ptr::null_mut();
    let mut len_r31 = 0usize;
    let mut i_r32 = 0i64;
    let mut b_r32 = false;
    let mut t_r32: *mut Table = std::ptr::null_mut();
    let mut p_r32: *mut i64 = std::ptr::null_mut();
    let mut len_r32 = 0usize;
    let mut i_r33 = 0i64;
    let mut b_r33 = false;
    let mut t_r33: *mut Table = std::ptr::null_mut();
    let mut p_r33: *mut i64 = std::ptr::null_mut();
    let mut len_r33 = 0usize;
    let mut i_r34 = 0i64;
    let mut b_r34 = false;
    let mut t_r34: *mut Table = std::ptr::null_mut();
    let mut p_r34: *mut i64 = std::ptr::null_mut();
    let mut len_r34 = 0usize;
    let mut i_r35 = 0i64;
    let mut b_r35 = false;
    let mut t_r35: *mut Table = std::ptr::null_mut();
    let mut p_r35: *mut i64 = std::ptr::null_mut();
    let mut len_r35 = 0usize;
    let mut i_r36 = 0i64;
    let mut b_r36 = false;
    let mut t_r36: *mut Table = std::ptr::null_mut();
    let mut p_r36: *mut i64 = std::ptr::null_mut();
    let mut len_r36 = 0usize;
    let mut i_r37 = 0i64;
    let mut b_r37 = false;
    let mut t_r37: *mut Table = std::ptr::null_mut();
    let mut p_r37: *mut i64 = std::ptr::null_mut();
    let mut len_r37 = 0usize;
    let mut i_r38 = 0i64;
    let mut b_r38 = false;
    let mut t_r38: *mut Table = std::ptr::null_mut();
    let mut p_r38: *mut i64 = std::ptr::null_mut();
    let mut len_r38 = 0usize;
    let mut i_r39 = 0i64;
    let mut b_r39 = false;
    let mut t_r39: *mut Table = std::ptr::null_mut();
    let mut p_r39: *mut i64 = std::ptr::null_mut();
    let mut len_r39 = 0usize;
    let mut i_r40 = 0i64;
    let mut b_r40 = false;
    let mut t_r40: *mut Table = std::ptr::null_mut();
    let mut p_r40: *mut i64 = std::ptr::null_mut();
    let mut len_r40 = 0usize;
    let mut i_r41 = 0i64;
    let mut b_r41 = false;
    let mut t_r41: *mut Table = std::ptr::null_mut();
    let mut p_r41: *mut i64 = std::ptr::null_mut();
    let mut len_r41 = 0usize;
    let mut i_r42 = 0i64;
    let mut b_r42 = false;
    let mut t_r42: *mut Table = std::ptr::null_mut();
    let mut p_r42: *mut i64 = std::ptr::null_mut();
    let mut len_r42 = 0usize;
    let mut i_r43 = 0i64;
    let mut b_r43 = false;
    let mut t_r43: *mut Table = std::ptr::null_mut();
    let mut p_r43: *mut i64 = std::ptr::null_mut();
    let mut len_r43 = 0usize;
    let mut i_r44 = 0i64;
    let mut b_r44 = false;
    let mut t_r44: *mut Table = std::ptr::null_mut();
    let mut p_r44: *mut i64 = std::ptr::null_mut();
    let mut len_r44 = 0usize;
    let mut i_r45 = 0i64;
    let mut b_r45 = false;
    let mut t_r45: *mut Table = std::ptr::null_mut();
    let mut p_r45: *mut i64 = std::ptr::null_mut();
    let mut len_r45 = 0usize;
    let mut i_r46 = 0i64;
    let mut b_r46 = false;
    let mut t_r46: *mut Table = std::ptr::null_mut();
    let mut p_r46: *mut i64 = std::ptr::null_mut();
    let mut len_r46 = 0usize;
    let mut i_r47 = 0i64;
    let mut b_r47 = false;
    let mut t_r47: *mut Table = std::ptr::null_mut();
    let mut p_r47: *mut i64 = std::ptr::null_mut();
    let mut len_r47 = 0usize;
    let mut i_r48 = 0i64;
    let mut b_r48 = false;
    let mut t_r48: *mut Table = std::ptr::null_mut();
    let mut p_r48: *mut i64 = std::ptr::null_mut();
    let mut len_r48 = 0usize;
    let mut i_r49 = 0i64;
    let mut b_r49 = false;
    let mut t_r49: *mut Table = std::ptr::null_mut();
    let mut p_r49: *mut i64 = std::ptr::null_mut();
    let mut len_r49 = 0usize;
    let mut i_r50 = 0i64;
    let mut b_r50 = false;
    let mut t_r50: *mut Table = std::ptr::null_mut();
    let mut p_r50: *mut i64 = std::ptr::null_mut();
    let mut len_r50 = 0usize;
    let mut i_r51 = 0i64;
    let mut b_r51 = false;
    let mut t_r51: *mut Table = std::ptr::null_mut();
    let mut p_r51: *mut i64 = std::ptr::null_mut();
    let mut len_r51 = 0usize;
    let mut i_r52 = 0i64;
    let mut b_r52 = false;
    let mut t_r52: *mut Table = std::ptr::null_mut();
    let mut p_r52: *mut i64 = std::ptr::null_mut();
    let mut len_r52 = 0usize;
    let mut i_r53 = 0i64;
    let mut b_r53 = false;
    let mut t_r53: *mut Table = std::ptr::null_mut();
    let mut p_r53: *mut i64 = std::ptr::null_mut();
    let mut len_r53 = 0usize;
    let mut i_r54 = 0i64;
    let mut b_r54 = false;
    let mut t_r54: *mut Table = std::ptr::null_mut();
    let mut p_r54: *mut i64 = std::ptr::null_mut();
    let mut len_r54 = 0usize;
    let mut i_r55 = 0i64;
    let mut b_r55 = false;
    let mut t_r55: *mut Table = std::ptr::null_mut();
    let mut p_r55: *mut i64 = std::ptr::null_mut();
    let mut len_r55 = 0usize;
    let mut i_r56 = 0i64;
    let mut b_r56 = false;
    let mut t_r56: *mut Table = std::ptr::null_mut();
    let mut p_r56: *mut i64 = std::ptr::null_mut();
    let mut len_r56 = 0usize;
    let mut i_r57 = 0i64;
    let mut b_r57 = false;
    let mut t_r57: *mut Table = std::ptr::null_mut();
    let mut p_r57: *mut i64 = std::ptr::null_mut();
    let mut len_r57 = 0usize;
    let mut i_r58 = 0i64;
    let mut b_r58 = false;
    let mut t_r58: *mut Table = std::ptr::null_mut();
    let mut p_r58: *mut i64 = std::ptr::null_mut();
    let mut len_r58 = 0usize;
    let mut i_r59 = 0i64;
    let mut b_r59 = false;
    let mut t_r59: *mut Table = std::ptr::null_mut();
    let mut p_r59: *mut i64 = std::ptr::null_mut();
    let mut len_r59 = 0usize;
    let mut i_r60 = 0i64;
    let mut b_r60 = false;
    let mut t_r60: *mut Table = std::ptr::null_mut();
    let mut p_r60: *mut i64 = std::ptr::null_mut();
    let mut len_r60 = 0usize;
    let mut i_r61 = 0i64;
    let mut b_r61 = false;
    let mut t_r61: *mut Table = std::ptr::null_mut();
    let mut p_r61: *mut i64 = std::ptr::null_mut();
    let mut len_r61 = 0usize;
    let mut i_r62 = 0i64;
    let mut b_r62 = false;
    let mut t_r62: *mut Table = std::ptr::null_mut();
    let mut p_r62: *mut i64 = std::ptr::null_mut();
    let mut len_r62 = 0usize;
    let mut i_r63 = 0i64;
    let mut b_r63 = false;
    let mut t_r63: *mut Table = std::ptr::null_mut();
    let mut p_r63: *mut i64 = std::ptr::null_mut();
    let mut len_r63 = 0usize;
    let mut i_r64 = 0i64;
    let mut b_r64 = false;
    let mut t_r64: *mut Table = std::ptr::null_mut();
    let mut p_r64: *mut i64 = std::ptr::null_mut();
    let mut len_r64 = 0usize;
    let mut i_r65 = 0i64;
    let mut b_r65 = false;
    let mut t_r65: *mut Table = std::ptr::null_mut();
    let mut p_r65: *mut i64 = std::ptr::null_mut();
    let mut len_r65 = 0usize;
    let mut i_r66 = 0i64;
    let mut b_r66 = false;
    let mut t_r66: *mut Table = std::ptr::null_mut();
    let mut p_r66: *mut i64 = std::ptr::null_mut();
    let mut len_r66 = 0usize;
    let mut i_r67 = 0i64;
    let mut b_r67 = false;
    let mut t_r67: *mut Table = std::ptr::null_mut();
    let mut p_r67: *mut i64 = std::ptr::null_mut();
    let mut len_r67 = 0usize;
    let mut i_r68 = 0i64;
    let mut b_r68 = false;
    let mut t_r68: *mut Table = std::ptr::null_mut();
    let mut p_r68: *mut i64 = std::ptr::null_mut();
    let mut len_r68 = 0usize;
    let mut i_r69 = 0i64;
    let mut b_r69 = false;
    let mut t_r69: *mut Table = std::ptr::null_mut();
    let mut p_r69: *mut i64 = std::ptr::null_mut();
    let mut len_r69 = 0usize;
    let mut i_r70 = 0i64;
    let mut b_r70 = false;
    let mut t_r70: *mut Table = std::ptr::null_mut();
    let mut p_r70: *mut i64 = std::ptr::null_mut();
    let mut len_r70 = 0usize;
    let mut i_r71 = 0i64;
    let mut b_r71 = false;
    let mut t_r71: *mut Table = std::ptr::null_mut();
    let mut p_r71: *mut i64 = std::ptr::null_mut();
    let mut len_r71 = 0usize;
    let mut i_r72 = 0i64;
    let mut b_r72 = false;
    let mut t_r72: *mut Table = std::ptr::null_mut();
    let mut p_r72: *mut i64 = std::ptr::null_mut();
    let mut len_r72 = 0usize;
    let mut i_r73 = 0i64;
    let mut b_r73 = false;
    let mut t_r73: *mut Table = std::ptr::null_mut();
    let mut p_r73: *mut i64 = std::ptr::null_mut();
    let mut len_r73 = 0usize;
    let mut i_r74 = 0i64;
    let mut b_r74 = false;
    let mut t_r74: *mut Table = std::ptr::null_mut();
    let mut p_r74: *mut i64 = std::ptr::null_mut();
    let mut len_r74 = 0usize;
    let mut i_r75 = 0i64;
    let mut b_r75 = false;
    let mut t_r75: *mut Table = std::ptr::null_mut();
    let mut p_r75: *mut i64 = std::ptr::null_mut();
    let mut len_r75 = 0usize;
    let mut i_r76 = 0i64;
    let mut b_r76 = false;
    let mut t_r76: *mut Table = std::ptr::null_mut();
    let mut p_r76: *mut i64 = std::ptr::null_mut();
    let mut len_r76 = 0usize;
    let mut i_r77 = 0i64;
    let mut b_r77 = false;
    let mut t_r77: *mut Table = std::ptr::null_mut();
    let mut p_r77: *mut i64 = std::ptr::null_mut();
    let mut len_r77 = 0usize;
    let mut i_r78 = 0i64;
    let mut b_r78 = false;
    let mut t_r78: *mut Table = std::ptr::null_mut();
    let mut p_r78: *mut i64 = std::ptr::null_mut();
    let mut len_r78 = 0usize;
    let mut i_r79 = 0i64;
    let mut b_r79 = false;
    let mut t_r79: *mut Table = std::ptr::null_mut();
    let mut p_r79: *mut i64 = std::ptr::null_mut();
    let mut len_r79 = 0usize;
    let mut i_r80 = 0i64;
    let mut b_r80 = false;
    let mut t_r80: *mut Table = std::ptr::null_mut();
    let mut p_r80: *mut i64 = std::ptr::null_mut();
    let mut len_r80 = 0usize;
    let mut i_r81 = 0i64;
    let mut b_r81 = false;
    let mut t_r81: *mut Table = std::ptr::null_mut();
    let mut p_r81: *mut i64 = std::ptr::null_mut();
    let mut len_r81 = 0usize;
    let mut i_r82 = 0i64;
    let mut b_r82 = false;
    let mut t_r82: *mut Table = std::ptr::null_mut();
    let mut p_r82: *mut i64 = std::ptr::null_mut();
    let mut len_r82 = 0usize;
    let mut i_r83 = 0i64;
    let mut b_r83 = false;
    let mut t_r83: *mut Table = std::ptr::null_mut();
    let mut p_r83: *mut i64 = std::ptr::null_mut();
    let mut len_r83 = 0usize;
    let mut i_r84 = 0i64;
    let mut b_r84 = false;
    let mut t_r84: *mut Table = std::ptr::null_mut();
    let mut p_r84: *mut i64 = std::ptr::null_mut();
    let mut len_r84 = 0usize;
    let mut i_r85 = 0i64;
    let mut b_r85 = false;
    let mut t_r85: *mut Table = std::ptr::null_mut();
    let mut p_r85: *mut i64 = std::ptr::null_mut();
    let mut len_r85 = 0usize;
    let mut i_r86 = 0i64;
    let mut b_r86 = false;
    let mut t_r86: *mut Table = std::ptr::null_mut();
    let mut p_r86: *mut i64 = std::ptr::null_mut();
    let mut len_r86 = 0usize;
    let mut i_r87 = 0i64;
    let mut b_r87 = false;
    let mut t_r87: *mut Table = std::ptr::null_mut();
    let mut p_r87: *mut i64 = std::ptr::null_mut();
    let mut len_r87 = 0usize;
    let mut i_r88 = 0i64;
    let mut b_r88 = false;
    let mut t_r88: *mut Table = std::ptr::null_mut();
    let mut p_r88: *mut i64 = std::ptr::null_mut();
    let mut len_r88 = 0usize;
    let mut i_r89 = 0i64;
    let mut b_r89 = false;
    let mut t_r89: *mut Table = std::ptr::null_mut();
    let mut p_r89: *mut i64 = std::ptr::null_mut();
    let mut len_r89 = 0usize;
    let mut i_r90 = 0i64;
    let mut b_r90 = false;
    let mut t_r90: *mut Table = std::ptr::null_mut();
    let mut p_r90: *mut i64 = std::ptr::null_mut();
    let mut len_r90 = 0usize;
    let mut i_r91 = 0i64;
    let mut b_r91 = false;
    let mut t_r91: *mut Table = std::ptr::null_mut();
    let mut p_r91: *mut i64 = std::ptr::null_mut();
    let mut len_r91 = 0usize;
    let mut i_r92 = 0i64;
    let mut b_r92 = false;
    let mut t_r92: *mut Table = std::ptr::null_mut();
    let mut p_r92: *mut i64 = std::ptr::null_mut();
    let mut len_r92 = 0usize;
    let mut i_r93 = 0i64;
    let mut b_r93 = false;
    let mut t_r93: *mut Table = std::ptr::null_mut();
    let mut p_r93: *mut i64 = std::ptr::null_mut();
    let mut len_r93 = 0usize;
    let mut i_r94 = 0i64;
    let mut b_r94 = false;
    let mut t_r94: *mut Table = std::ptr::null_mut();
    let mut p_r94: *mut i64 = std::ptr::null_mut();
    let mut len_r94 = 0usize;
    let mut i_r95 = 0i64;
    let mut b_r95 = false;
    let mut t_r95: *mut Table = std::ptr::null_mut();
    let mut p_r95: *mut i64 = std::ptr::null_mut();
    let mut len_r95 = 0usize;
    let mut i_r96 = 0i64;
    let mut b_r96 = false;
    let mut t_r96: *mut Table = std::ptr::null_mut();
    let mut p_r96: *mut i64 = std::ptr::null_mut();
    let mut len_r96 = 0usize;
    let mut i_r97 = 0i64;
    let mut b_r97 = false;
    let mut t_r97: *mut Table = std::ptr::null_mut();
    let mut p_r97: *mut i64 = std::ptr::null_mut();
    let mut len_r97 = 0usize;
    let mut i_r98 = 0i64;
    let mut b_r98 = false;
    let mut t_r98: *mut Table = std::ptr::null_mut();
    let mut p_r98: *mut i64 = std::ptr::null_mut();
    let mut len_r98 = 0usize;
    let mut i_r99 = 0i64;
    let mut b_r99 = false;
    let mut t_r99: *mut Table = std::ptr::null_mut();
    let mut p_r99: *mut i64 = std::ptr::null_mut();
    let mut len_r99 = 0usize;
    let mut i_r100 = 0i64;
    let mut b_r100 = false;
    let mut t_r100: *mut Table = std::ptr::null_mut();
    let mut p_r100: *mut i64 = std::ptr::null_mut();
    let mut len_r100 = 0usize;
    let mut i_r101 = 0i64;
    let mut b_r101 = false;
    let mut t_r101: *mut Table = std::ptr::null_mut();
    let mut p_r101: *mut i64 = std::ptr::null_mut();
    let mut len_r101 = 0usize;
    let mut i_r102 = 0i64;
    let mut b_r102 = false;
    let mut t_r102: *mut Table = std::ptr::null_mut();
    let mut p_r102: *mut i64 = std::ptr::null_mut();
    let mut len_r102 = 0usize;
    let mut i_r103 = 0i64;
    let mut b_r103 = false;
    let mut t_r103: *mut Table = std::ptr::null_mut();
    let mut p_r103: *mut i64 = std::ptr::null_mut();
    let mut len_r103 = 0usize;
    let mut i_r104 = 0i64;
    let mut b_r104 = false;
    let mut t_r104: *mut Table = std::ptr::null_mut();
    let mut p_r104: *mut i64 = std::ptr::null_mut();
    let mut len_r104 = 0usize;
    let mut i_r105 = 0i64;
    let mut b_r105 = false;
    let mut t_r105: *mut Table = std::ptr::null_mut();
    let mut p_r105: *mut i64 = std::ptr::null_mut();
    let mut len_r105 = 0usize;
    let mut i_r106 = 0i64;
    let mut b_r106 = false;
    let mut t_r106: *mut Table = std::ptr::null_mut();
    let mut p_r106: *mut i64 = std::ptr::null_mut();
    let mut len_r106 = 0usize;
    let mut i_r107 = 0i64;
    let mut b_r107 = false;
    let mut t_r107: *mut Table = std::ptr::null_mut();
    let mut p_r107: *mut i64 = std::ptr::null_mut();
    let mut len_r107 = 0usize;
    let mut i_r108 = 0i64;
    let mut b_r108 = false;
    let mut t_r108: *mut Table = std::ptr::null_mut();
    let mut p_r108: *mut i64 = std::ptr::null_mut();
    let mut len_r108 = 0usize;
    let mut i_r109 = 0i64;
    let mut b_r109 = false;
    let mut t_r109: *mut Table = std::ptr::null_mut();
    let mut p_r109: *mut i64 = std::ptr::null_mut();
    let mut len_r109 = 0usize;
    let mut i_r110 = 0i64;
    let mut b_r110 = false;
    let mut t_r110: *mut Table = std::ptr::null_mut();
    let mut p_r110: *mut i64 = std::ptr::null_mut();
    let mut len_r110 = 0usize;
    let mut i_r111 = 0i64;
    let mut b_r111 = false;
    let mut t_r111: *mut Table = std::ptr::null_mut();
    let mut p_r111: *mut i64 = std::ptr::null_mut();
    let mut len_r111 = 0usize;
    let mut i_r112 = 0i64;
    let mut b_r112 = false;
    let mut t_r112: *mut Table = std::ptr::null_mut();
    let mut p_r112: *mut i64 = std::ptr::null_mut();
    let mut len_r112 = 0usize;
    let mut i_r113 = 0i64;
    let mut b_r113 = false;
    let mut t_r113: *mut Table = std::ptr::null_mut();
    let mut p_r113: *mut i64 = std::ptr::null_mut();
    let mut len_r113 = 0usize;
    let mut i_r114 = 0i64;
    let mut b_r114 = false;
    let mut t_r114: *mut Table = std::ptr::null_mut();
    let mut p_r114: *mut i64 = std::ptr::null_mut();
    let mut len_r114 = 0usize;
    let mut i_r115 = 0i64;
    let mut b_r115 = false;
    let mut t_r115: *mut Table = std::ptr::null_mut();
    let mut p_r115: *mut i64 = std::ptr::null_mut();
    let mut len_r115 = 0usize;
    let mut i_r116 = 0i64;
    let mut b_r116 = false;
    let mut t_r116: *mut Table = std::ptr::null_mut();
    let mut p_r116: *mut i64 = std::ptr::null_mut();
    let mut len_r116 = 0usize;
    let mut i_r117 = 0i64;
    let mut b_r117 = false;
    let mut t_r117: *mut Table = std::ptr::null_mut();
    let mut p_r117: *mut i64 = std::ptr::null_mut();
    let mut len_r117 = 0usize;
    let mut i_r118 = 0i64;
    let mut b_r118 = false;
    let mut t_r118: *mut Table = std::ptr::null_mut();
    let mut p_r118: *mut i64 = std::ptr::null_mut();
    let mut len_r118 = 0usize;
    let mut i_r119 = 0i64;
    let mut b_r119 = false;
    let mut t_r119: *mut Table = std::ptr::null_mut();
    let mut p_r119: *mut i64 = std::ptr::null_mut();
    let mut len_r119 = 0usize;
    let mut i_r120 = 0i64;
    let mut b_r120 = false;
    let mut t_r120: *mut Table = std::ptr::null_mut();
    let mut p_r120: *mut i64 = std::ptr::null_mut();
    let mut len_r120 = 0usize;
    let mut i_r121 = 0i64;
    let mut b_r121 = false;
    let mut t_r121: *mut Table = std::ptr::null_mut();
    let mut p_r121: *mut i64 = std::ptr::null_mut();
    let mut len_r121 = 0usize;
    let mut i_r122 = 0i64;
    let mut b_r122 = false;
    let mut t_r122: *mut Table = std::ptr::null_mut();
    let mut p_r122: *mut i64 = std::ptr::null_mut();
    let mut len_r122 = 0usize;
    let mut i_r123 = 0i64;
    let mut b_r123 = false;
    let mut t_r123: *mut Table = std::ptr::null_mut();
    let mut p_r123: *mut i64 = std::ptr::null_mut();
    let mut len_r123 = 0usize;
    let mut i_r124 = 0i64;
    let mut b_r124 = false;
    let mut t_r124: *mut Table = std::ptr::null_mut();
    let mut p_r124: *mut i64 = std::ptr::null_mut();
    let mut len_r124 = 0usize;
    let mut i_r125 = 0i64;
    let mut b_r125 = false;
    let mut t_r125: *mut Table = std::ptr::null_mut();
    let mut p_r125: *mut i64 = std::ptr::null_mut();
    let mut len_r125 = 0usize;
    let mut i_r126 = 0i64;
    let mut b_r126 = false;
    let mut t_r126: *mut Table = std::ptr::null_mut();
    let mut p_r126: *mut i64 = std::ptr::null_mut();
    let mut len_r126 = 0usize;
    let mut i_r127 = 0i64;
    let mut b_r127 = false;
    let mut t_r127: *mut Table = std::ptr::null_mut();
    let mut p_r127: *mut i64 = std::ptr::null_mut();
    let mut len_r127 = 0usize;
    let mut i_r128 = 0i64;
    let mut b_r128 = false;
    let mut t_r128: *mut Table = std::ptr::null_mut();
    let mut p_r128: *mut i64 = std::ptr::null_mut();
    let mut len_r128 = 0usize;
    let mut i_r129 = 0i64;
    let mut b_r129 = false;
    let mut t_r129: *mut Table = std::ptr::null_mut();
    let mut p_r129: *mut i64 = std::ptr::null_mut();
    let mut len_r129 = 0usize;
    let mut i_r130 = 0i64;
    let mut b_r130 = false;
    let mut t_r130: *mut Table = std::ptr::null_mut();
    let mut p_r130: *mut i64 = std::ptr::null_mut();
    let mut len_r130 = 0usize;
    let mut i_r131 = 0i64;
    let mut b_r131 = false;
    let mut t_r131: *mut Table = std::ptr::null_mut();
    let mut p_r131: *mut i64 = std::ptr::null_mut();
    let mut len_r131 = 0usize;
    let mut i_r132 = 0i64;
    let mut b_r132 = false;
    let mut t_r132: *mut Table = std::ptr::null_mut();
    let mut p_r132: *mut i64 = std::ptr::null_mut();
    let mut len_r132 = 0usize;
    let mut i_r133 = 0i64;
    let mut b_r133 = false;
    let mut t_r133: *mut Table = std::ptr::null_mut();
    let mut p_r133: *mut i64 = std::ptr::null_mut();
    let mut len_r133 = 0usize;
    let mut i_r134 = 0i64;
    let mut b_r134 = false;
    let mut t_r134: *mut Table = std::ptr::null_mut();
    let mut p_r134: *mut i64 = std::ptr::null_mut();
    let mut len_r134 = 0usize;
    let mut i_r135 = 0i64;
    let mut b_r135 = false;
    let mut t_r135: *mut Table = std::ptr::null_mut();
    let mut p_r135: *mut i64 = std::ptr::null_mut();
    let mut len_r135 = 0usize;
    let mut i_r136 = 0i64;
    let mut b_r136 = false;
    let mut t_r136: *mut Table = std::ptr::null_mut();
    let mut p_r136: *mut i64 = std::ptr::null_mut();
    let mut len_r136 = 0usize;
    let mut i_r137 = 0i64;
    let mut b_r137 = false;
    let mut t_r137: *mut Table = std::ptr::null_mut();
    let mut p_r137: *mut i64 = std::ptr::null_mut();
    let mut len_r137 = 0usize;
    let mut i_r138 = 0i64;
    let mut b_r138 = false;
    let mut t_r138: *mut Table = std::ptr::null_mut();
    let mut p_r138: *mut i64 = std::ptr::null_mut();
    let mut len_r138 = 0usize;
    let mut i_r139 = 0i64;
    let mut b_r139 = false;
    let mut t_r139: *mut Table = std::ptr::null_mut();
    let mut p_r139: *mut i64 = std::ptr::null_mut();
    let mut len_r139 = 0usize;
    let mut i_r140 = 0i64;
    let mut b_r140 = false;
    let mut t_r140: *mut Table = std::ptr::null_mut();
    let mut p_r140: *mut i64 = std::ptr::null_mut();
    let mut len_r140 = 0usize;
    let mut i_r141 = 0i64;
    let mut b_r141 = false;
    let mut t_r141: *mut Table = std::ptr::null_mut();
    let mut p_r141: *mut i64 = std::ptr::null_mut();
    let mut len_r141 = 0usize;
    let mut i_r142 = 0i64;
    let mut b_r142 = false;
    let mut t_r142: *mut Table = std::ptr::null_mut();
    let mut p_r142: *mut i64 = std::ptr::null_mut();
    let mut len_r142 = 0usize;
    let mut i_r143 = 0i64;
    let mut b_r143 = false;
    let mut t_r143: *mut Table = std::ptr::null_mut();
    let mut p_r143: *mut i64 = std::ptr::null_mut();
    let mut len_r143 = 0usize;
    let mut i_r144 = 0i64;
    let mut b_r144 = false;
    let mut t_r144: *mut Table = std::ptr::null_mut();
    let mut p_r144: *mut i64 = std::ptr::null_mut();
    let mut len_r144 = 0usize;
    let mut i_r145 = 0i64;
    let mut b_r145 = false;
    let mut t_r145: *mut Table = std::ptr::null_mut();
    let mut p_r145: *mut i64 = std::ptr::null_mut();
    let mut len_r145 = 0usize;
    let mut i_r146 = 0i64;
    let mut b_r146 = false;
    let mut t_r146: *mut Table = std::ptr::null_mut();
    let mut p_r146: *mut i64 = std::ptr::null_mut();
    let mut len_r146 = 0usize;
    let mut i_r147 = 0i64;
    let mut b_r147 = false;
    let mut t_r147: *mut Table = std::ptr::null_mut();
    let mut p_r147: *mut i64 = std::ptr::null_mut();
    let mut len_r147 = 0usize;
    let mut i_r148 = 0i64;
    let mut b_r148 = false;
    let mut t_r148: *mut Table = std::ptr::null_mut();
    let mut p_r148: *mut i64 = std::ptr::null_mut();
    let mut len_r148 = 0usize;
    let mut i_r149 = 0i64;
    let mut b_r149 = false;
    let mut t_r149: *mut Table = std::ptr::null_mut();
    let mut p_r149: *mut i64 = std::ptr::null_mut();
    let mut len_r149 = 0usize;
    let mut i_r150 = 0i64;
    let mut b_r150 = false;
    let mut t_r150: *mut Table = std::ptr::null_mut();
    let mut p_r150: *mut i64 = std::ptr::null_mut();
    let mut len_r150 = 0usize;
    let mut i_r151 = 0i64;
    let mut b_r151 = false;
    let mut t_r151: *mut Table = std::ptr::null_mut();
    let mut p_r151: *mut i64 = std::ptr::null_mut();
    let mut len_r151 = 0usize;
    let mut i_r152 = 0i64;
    let mut b_r152 = false;
    let mut t_r152: *mut Table = std::ptr::null_mut();
    let mut p_r152: *mut i64 = std::ptr::null_mut();
    let mut len_r152 = 0usize;
    let mut i_r153 = 0i64;
    let mut b_r153 = false;
    let mut t_r153: *mut Table = std::ptr::null_mut();
    let mut p_r153: *mut i64 = std::ptr::null_mut();
    let mut len_r153 = 0usize;
    let mut i_r154 = 0i64;
    let mut b_r154 = false;
    let mut t_r154: *mut Table = std::ptr::null_mut();
    let mut p_r154: *mut i64 = std::ptr::null_mut();
    let mut len_r154 = 0usize;
    let mut i_r155 = 0i64;
    let mut b_r155 = false;
    let mut t_r155: *mut Table = std::ptr::null_mut();
    let mut p_r155: *mut i64 = std::ptr::null_mut();
    let mut len_r155 = 0usize;
    let mut i_r156 = 0i64;
    let mut b_r156 = false;
    let mut t_r156: *mut Table = std::ptr::null_mut();
    let mut p_r156: *mut i64 = std::ptr::null_mut();
    let mut len_r156 = 0usize;
    let mut i_r157 = 0i64;
    let mut b_r157 = false;
    let mut t_r157: *mut Table = std::ptr::null_mut();
    let mut p_r157: *mut i64 = std::ptr::null_mut();
    let mut len_r157 = 0usize;
    let mut i_r158 = 0i64;
    let mut b_r158 = false;
    let mut t_r158: *mut Table = std::ptr::null_mut();
    let mut p_r158: *mut i64 = std::ptr::null_mut();
    let mut len_r158 = 0usize;
    let mut i_r159 = 0i64;
    let mut b_r159 = false;
    let mut t_r159: *mut Table = std::ptr::null_mut();
    let mut p_r159: *mut i64 = std::ptr::null_mut();
    let mut len_r159 = 0usize;
    let mut i_r160 = 0i64;
    let mut b_r160 = false;
    let mut t_r160: *mut Table = std::ptr::null_mut();
    let mut p_r160: *mut i64 = std::ptr::null_mut();
    let mut len_r160 = 0usize;
    let mut i_r161 = 0i64;
    let mut b_r161 = false;
    let mut t_r161: *mut Table = std::ptr::null_mut();
    let mut p_r161: *mut i64 = std::ptr::null_mut();
    let mut len_r161 = 0usize;
    let mut i_r162 = 0i64;
    let mut b_r162 = false;
    let mut t_r162: *mut Table = std::ptr::null_mut();
    let mut p_r162: *mut i64 = std::ptr::null_mut();
    let mut len_r162 = 0usize;
    let mut i_r163 = 0i64;
    let mut b_r163 = false;
    let mut t_r163: *mut Table = std::ptr::null_mut();
    let mut p_r163: *mut i64 = std::ptr::null_mut();
    let mut len_r163 = 0usize;
    let mut i_r164 = 0i64;
    let mut b_r164 = false;
    let mut t_r164: *mut Table = std::ptr::null_mut();
    let mut p_r164: *mut i64 = std::ptr::null_mut();
    let mut len_r164 = 0usize;
    let mut i_r165 = 0i64;
    let mut b_r165 = false;
    let mut t_r165: *mut Table = std::ptr::null_mut();
    let mut p_r165: *mut i64 = std::ptr::null_mut();
    let mut len_r165 = 0usize;
    let mut i_r166 = 0i64;
    let mut b_r166 = false;
    let mut t_r166: *mut Table = std::ptr::null_mut();
    let mut p_r166: *mut i64 = std::ptr::null_mut();
    let mut len_r166 = 0usize;
    let mut i_r167 = 0i64;
    let mut b_r167 = false;
    let mut t_r167: *mut Table = std::ptr::null_mut();
    let mut p_r167: *mut i64 = std::ptr::null_mut();
    let mut len_r167 = 0usize;
    let mut i_r168 = 0i64;
    let mut b_r168 = false;
    let mut t_r168: *mut Table = std::ptr::null_mut();
    let mut p_r168: *mut i64 = std::ptr::null_mut();
    let mut len_r168 = 0usize;
    let mut i_r169 = 0i64;
    let mut b_r169 = false;
    let mut t_r169: *mut Table = std::ptr::null_mut();
    let mut p_r169: *mut i64 = std::ptr::null_mut();
    let mut len_r169 = 0usize;
    let mut i_r170 = 0i64;
    let mut b_r170 = false;
    let mut t_r170: *mut Table = std::ptr::null_mut();
    let mut p_r170: *mut i64 = std::ptr::null_mut();
    let mut len_r170 = 0usize;
    let mut i_r171 = 0i64;
    let mut b_r171 = false;
    let mut t_r171: *mut Table = std::ptr::null_mut();
    let mut p_r171: *mut i64 = std::ptr::null_mut();
    let mut len_r171 = 0usize;
    let mut i_r172 = 0i64;
    let mut b_r172 = false;
    let mut t_r172: *mut Table = std::ptr::null_mut();
    let mut p_r172: *mut i64 = std::ptr::null_mut();
    let mut len_r172 = 0usize;
    let mut i_r173 = 0i64;
    let mut b_r173 = false;
    let mut t_r173: *mut Table = std::ptr::null_mut();
    let mut p_r173: *mut i64 = std::ptr::null_mut();
    let mut len_r173 = 0usize;
    let mut i_r174 = 0i64;
    let mut b_r174 = false;
    let mut t_r174: *mut Table = std::ptr::null_mut();
    let mut p_r174: *mut i64 = std::ptr::null_mut();
    let mut len_r174 = 0usize;
    let mut i_r175 = 0i64;
    let mut b_r175 = false;
    let mut t_r175: *mut Table = std::ptr::null_mut();
    let mut p_r175: *mut i64 = std::ptr::null_mut();
    let mut len_r175 = 0usize;
    let mut i_r176 = 0i64;
    let mut b_r176 = false;
    let mut t_r176: *mut Table = std::ptr::null_mut();
    let mut p_r176: *mut i64 = std::ptr::null_mut();
    let mut len_r176 = 0usize;
    let mut i_r177 = 0i64;
    let mut b_r177 = false;
    let mut t_r177: *mut Table = std::ptr::null_mut();
    let mut p_r177: *mut i64 = std::ptr::null_mut();
    let mut len_r177 = 0usize;
    let mut i_r178 = 0i64;
    let mut b_r178 = false;
    let mut t_r178: *mut Table = std::ptr::null_mut();
    let mut p_r178: *mut i64 = std::ptr::null_mut();
    let mut len_r178 = 0usize;
    let mut i_r179 = 0i64;
    let mut b_r179 = false;
    let mut t_r179: *mut Table = std::ptr::null_mut();
    let mut p_r179: *mut i64 = std::ptr::null_mut();
    let mut len_r179 = 0usize;
    let mut i_r180 = 0i64;
    let mut b_r180 = false;
    let mut t_r180: *mut Table = std::ptr::null_mut();
    let mut p_r180: *mut i64 = std::ptr::null_mut();
    let mut len_r180 = 0usize;
    let mut i_r181 = 0i64;
    let mut b_r181 = false;
    let mut t_r181: *mut Table = std::ptr::null_mut();
    let mut p_r181: *mut i64 = std::ptr::null_mut();
    let mut len_r181 = 0usize;
    let mut i_r182 = 0i64;
    let mut b_r182 = false;
    let mut t_r182: *mut Table = std::ptr::null_mut();
    let mut p_r182: *mut i64 = std::ptr::null_mut();
    let mut len_r182 = 0usize;
    let mut i_r183 = 0i64;
    let mut b_r183 = false;
    let mut t_r183: *mut Table = std::ptr::null_mut();
    let mut p_r183: *mut i64 = std::ptr::null_mut();
    let mut len_r183 = 0usize;
    let mut i_r184 = 0i64;
    let mut b_r184 = false;
    let mut t_r184: *mut Table = std::ptr::null_mut();
    let mut p_r184: *mut i64 = std::ptr::null_mut();
    let mut len_r184 = 0usize;
    let mut i_r185 = 0i64;
    let mut b_r185 = false;
    let mut t_r185: *mut Table = std::ptr::null_mut();
    let mut p_r185: *mut i64 = std::ptr::null_mut();
    let mut len_r185 = 0usize;
    let mut i_r186 = 0i64;
    let mut b_r186 = false;
    let mut t_r186: *mut Table = std::ptr::null_mut();
    let mut p_r186: *mut i64 = std::ptr::null_mut();
    let mut len_r186 = 0usize;
    let mut i_r187 = 0i64;
    let mut b_r187 = false;
    let mut t_r187: *mut Table = std::ptr::null_mut();
    let mut p_r187: *mut i64 = std::ptr::null_mut();
    let mut len_r187 = 0usize;
    let mut i_r188 = 0i64;
    let mut b_r188 = false;
    let mut t_r188: *mut Table = std::ptr::null_mut();
    let mut p_r188: *mut i64 = std::ptr::null_mut();
    let mut len_r188 = 0usize;
    let mut i_r189 = 0i64;
    let mut b_r189 = false;
    let mut t_r189: *mut Table = std::ptr::null_mut();
    let mut p_r189: *mut i64 = std::ptr::null_mut();
    let mut len_r189 = 0usize;
    let mut i_r190 = 0i64;
    let mut b_r190 = false;
    let mut t_r190: *mut Table = std::ptr::null_mut();
    let mut p_r190: *mut i64 = std::ptr::null_mut();
    let mut len_r190 = 0usize;
    let mut i_r191 = 0i64;
    let mut b_r191 = false;
    let mut t_r191: *mut Table = std::ptr::null_mut();
    let mut p_r191: *mut i64 = std::ptr::null_mut();
    let mut len_r191 = 0usize;
    let mut i_r192 = 0i64;
    let mut b_r192 = false;
    let mut t_r192: *mut Table = std::ptr::null_mut();
    let mut p_r192: *mut i64 = std::ptr::null_mut();
    let mut len_r192 = 0usize;
    let mut i_r193 = 0i64;
    let mut b_r193 = false;
    let mut t_r193: *mut Table = std::ptr::null_mut();
    let mut p_r193: *mut i64 = std::ptr::null_mut();
    let mut len_r193 = 0usize;
    let mut i_r194 = 0i64;
    let mut b_r194 = false;
    let mut t_r194: *mut Table = std::ptr::null_mut();
    let mut p_r194: *mut i64 = std::ptr::null_mut();
    let mut len_r194 = 0usize;
    let mut i_r195 = 0i64;
    let mut b_r195 = false;
    let mut t_r195: *mut Table = std::ptr::null_mut();
    let mut p_r195: *mut i64 = std::ptr::null_mut();
    let mut len_r195 = 0usize;
    let mut i_r196 = 0i64;
    let mut b_r196 = false;
    let mut t_r196: *mut Table = std::ptr::null_mut();
    let mut p_r196: *mut i64 = std::ptr::null_mut();
    let mut len_r196 = 0usize;
    let mut i_r197 = 0i64;
    let mut b_r197 = false;
    let mut t_r197: *mut Table = std::ptr::null_mut();
    let mut p_r197: *mut i64 = std::ptr::null_mut();
    let mut len_r197 = 0usize;
    let mut i_r198 = 0i64;
    let mut b_r198 = false;
    let mut t_r198: *mut Table = std::ptr::null_mut();
    let mut p_r198: *mut i64 = std::ptr::null_mut();
    let mut len_r198 = 0usize;
    let mut i_r199 = 0i64;
    let mut b_r199 = false;
    let mut t_r199: *mut Table = std::ptr::null_mut();
    let mut p_r199: *mut i64 = std::ptr::null_mut();
    let mut len_r199 = 0usize;
    let mut i_r200 = 0i64;
    let mut b_r200 = false;
    let mut t_r200: *mut Table = std::ptr::null_mut();
    let mut p_r200: *mut i64 = std::ptr::null_mut();
    let mut len_r200 = 0usize;
    let mut i_r201 = 0i64;
    let mut b_r201 = false;
    let mut t_r201: *mut Table = std::ptr::null_mut();
    let mut p_r201: *mut i64 = std::ptr::null_mut();
    let mut len_r201 = 0usize;
    let mut i_r202 = 0i64;
    let mut b_r202 = false;
    let mut t_r202: *mut Table = std::ptr::null_mut();
    let mut p_r202: *mut i64 = std::ptr::null_mut();
    let mut len_r202 = 0usize;
    let mut i_r203 = 0i64;
    let mut b_r203 = false;
    let mut t_r203: *mut Table = std::ptr::null_mut();
    let mut p_r203: *mut i64 = std::ptr::null_mut();
    let mut len_r203 = 0usize;
    let mut i_r204 = 0i64;
    let mut b_r204 = false;
    let mut t_r204: *mut Table = std::ptr::null_mut();
    let mut p_r204: *mut i64 = std::ptr::null_mut();
    let mut len_r204 = 0usize;
    let mut i_r205 = 0i64;
    let mut b_r205 = false;
    let mut t_r205: *mut Table = std::ptr::null_mut();
    let mut p_r205: *mut i64 = std::ptr::null_mut();
    let mut len_r205 = 0usize;
    let mut i_r206 = 0i64;
    let mut b_r206 = false;
    let mut t_r206: *mut Table = std::ptr::null_mut();
    let mut p_r206: *mut i64 = std::ptr::null_mut();
    let mut len_r206 = 0usize;
    let mut i_r207 = 0i64;
    let mut b_r207 = false;
    let mut t_r207: *mut Table = std::ptr::null_mut();
    let mut p_r207: *mut i64 = std::ptr::null_mut();
    let mut len_r207 = 0usize;
    let mut i_r208 = 0i64;
    let mut b_r208 = false;
    let mut t_r208: *mut Table = std::ptr::null_mut();
    let mut p_r208: *mut i64 = std::ptr::null_mut();
    let mut len_r208 = 0usize;
    let mut i_r209 = 0i64;
    let mut b_r209 = false;
    let mut t_r209: *mut Table = std::ptr::null_mut();
    let mut p_r209: *mut i64 = std::ptr::null_mut();
    let mut len_r209 = 0usize;
    let mut i_r210 = 0i64;
    let mut b_r210 = false;
    let mut t_r210: *mut Table = std::ptr::null_mut();
    let mut p_r210: *mut i64 = std::ptr::null_mut();
    let mut len_r210 = 0usize;
    let mut i_r211 = 0i64;
    let mut b_r211 = false;
    let mut t_r211: *mut Table = std::ptr::null_mut();
    let mut p_r211: *mut i64 = std::ptr::null_mut();
    let mut len_r211 = 0usize;
    let mut i_r212 = 0i64;
    let mut b_r212 = false;
    let mut t_r212: *mut Table = std::ptr::null_mut();
    let mut p_r212: *mut i64 = std::ptr::null_mut();
    let mut len_r212 = 0usize;
    let mut i_r213 = 0i64;
    let mut b_r213 = false;
    let mut t_r213: *mut Table = std::ptr::null_mut();
    let mut p_r213: *mut i64 = std::ptr::null_mut();
    let mut len_r213 = 0usize;
    let mut i_r214 = 0i64;
    let mut b_r214 = false;
    let mut t_r214: *mut Table = std::ptr::null_mut();
    let mut p_r214: *mut i64 = std::ptr::null_mut();
    let mut len_r214 = 0usize;
    let mut i_r215 = 0i64;
    let mut b_r215 = false;
    let mut t_r215: *mut Table = std::ptr::null_mut();
    let mut p_r215: *mut i64 = std::ptr::null_mut();
    let mut len_r215 = 0usize;
    let mut i_r216 = 0i64;
    let mut b_r216 = false;
    let mut t_r216: *mut Table = std::ptr::null_mut();
    let mut p_r216: *mut i64 = std::ptr::null_mut();
    let mut len_r216 = 0usize;
    let mut i_r217 = 0i64;
    let mut b_r217 = false;
    let mut t_r217: *mut Table = std::ptr::null_mut();
    let mut p_r217: *mut i64 = std::ptr::null_mut();
    let mut len_r217 = 0usize;
    let mut i_r218 = 0i64;
    let mut b_r218 = false;
    let mut t_r218: *mut Table = std::ptr::null_mut();
    let mut p_r218: *mut i64 = std::ptr::null_mut();
    let mut len_r218 = 0usize;
    let mut i_r219 = 0i64;
    let mut b_r219 = false;
    let mut t_r219: *mut Table = std::ptr::null_mut();
    let mut p_r219: *mut i64 = std::ptr::null_mut();
    let mut len_r219 = 0usize;
    let mut i_r220 = 0i64;
    let mut b_r220 = false;
    let mut t_r220: *mut Table = std::ptr::null_mut();
    let mut p_r220: *mut i64 = std::ptr::null_mut();
    let mut len_r220 = 0usize;
    let mut i_r221 = 0i64;
    let mut b_r221 = false;
    let mut t_r221: *mut Table = std::ptr::null_mut();
    let mut p_r221: *mut i64 = std::ptr::null_mut();
    let mut len_r221 = 0usize;
    let mut i_r222 = 0i64;
    let mut b_r222 = false;
    let mut t_r222: *mut Table = std::ptr::null_mut();
    let mut p_r222: *mut i64 = std::ptr::null_mut();
    let mut len_r222 = 0usize;
    let mut i_r223 = 0i64;
    let mut b_r223 = false;
    let mut t_r223: *mut Table = std::ptr::null_mut();
    let mut p_r223: *mut i64 = std::ptr::null_mut();
    let mut len_r223 = 0usize;
    let mut i_r224 = 0i64;
    let mut b_r224 = false;
    let mut t_r224: *mut Table = std::ptr::null_mut();
    let mut p_r224: *mut i64 = std::ptr::null_mut();
    let mut len_r224 = 0usize;
    let mut i_r225 = 0i64;
    let mut b_r225 = false;
    let mut t_r225: *mut Table = std::ptr::null_mut();
    let mut p_r225: *mut i64 = std::ptr::null_mut();
    let mut len_r225 = 0usize;
    let mut i_r226 = 0i64;
    let mut b_r226 = false;
    let mut t_r226: *mut Table = std::ptr::null_mut();
    let mut p_r226: *mut i64 = std::ptr::null_mut();
    let mut len_r226 = 0usize;
    let mut i_r227 = 0i64;
    let mut b_r227 = false;
    let mut t_r227: *mut Table = std::ptr::null_mut();
    let mut p_r227: *mut i64 = std::ptr::null_mut();
    let mut len_r227 = 0usize;
    let mut i_r228 = 0i64;
    let mut b_r228 = false;
    let mut t_r228: *mut Table = std::ptr::null_mut();
    let mut p_r228: *mut i64 = std::ptr::null_mut();
    let mut len_r228 = 0usize;
    let mut i_r229 = 0i64;
    let mut b_r229 = false;
    let mut t_r229: *mut Table = std::ptr::null_mut();
    let mut p_r229: *mut i64 = std::ptr::null_mut();
    let mut len_r229 = 0usize;
    let mut i_r230 = 0i64;
    let mut b_r230 = false;
    let mut t_r230: *mut Table = std::ptr::null_mut();
    let mut p_r230: *mut i64 = std::ptr::null_mut();
    let mut len_r230 = 0usize;
    let mut i_r231 = 0i64;
    let mut b_r231 = false;
    let mut t_r231: *mut Table = std::ptr::null_mut();
    let mut p_r231: *mut i64 = std::ptr::null_mut();
    let mut len_r231 = 0usize;
    let mut i_r232 = 0i64;
    let mut b_r232 = false;
    let mut t_r232: *mut Table = std::ptr::null_mut();
    let mut p_r232: *mut i64 = std::ptr::null_mut();
    let mut len_r232 = 0usize;
    let mut i_r233 = 0i64;
    let mut b_r233 = false;
    let mut t_r233: *mut Table = std::ptr::null_mut();
    let mut p_r233: *mut i64 = std::ptr::null_mut();
    let mut len_r233 = 0usize;
    let mut i_r234 = 0i64;
    let mut b_r234 = false;
    let mut t_r234: *mut Table = std::ptr::null_mut();
    let mut p_r234: *mut i64 = std::ptr::null_mut();
    let mut len_r234 = 0usize;
    let mut i_r235 = 0i64;
    let mut b_r235 = false;
    let mut t_r235: *mut Table = std::ptr::null_mut();
    let mut p_r235: *mut i64 = std::ptr::null_mut();
    let mut len_r235 = 0usize;
    let mut i_r236 = 0i64;
    let mut b_r236 = false;
    let mut t_r236: *mut Table = std::ptr::null_mut();
    let mut p_r236: *mut i64 = std::ptr::null_mut();
    let mut len_r236 = 0usize;
    let mut i_r237 = 0i64;
    let mut b_r237 = false;
    let mut t_r237: *mut Table = std::ptr::null_mut();
    let mut p_r237: *mut i64 = std::ptr::null_mut();
    let mut len_r237 = 0usize;
    let mut i_r238 = 0i64;
    let mut b_r238 = false;
    let mut t_r238: *mut Table = std::ptr::null_mut();
    let mut p_r238: *mut i64 = std::ptr::null_mut();
    let mut len_r238 = 0usize;
    let mut i_r239 = 0i64;
    let mut b_r239 = false;
    let mut t_r239: *mut Table = std::ptr::null_mut();
    let mut p_r239: *mut i64 = std::ptr::null_mut();
    let mut len_r239 = 0usize;
    let mut i_r240 = 0i64;
    let mut b_r240 = false;
    let mut t_r240: *mut Table = std::ptr::null_mut();
    let mut p_r240: *mut i64 = std::ptr::null_mut();
    let mut len_r240 = 0usize;
    let mut i_r241 = 0i64;
    let mut b_r241 = false;
    let mut t_r241: *mut Table = std::ptr::null_mut();
    let mut p_r241: *mut i64 = std::ptr::null_mut();
    let mut len_r241 = 0usize;
    let mut i_r242 = 0i64;
    let mut b_r242 = false;
    let mut t_r242: *mut Table = std::ptr::null_mut();
    let mut p_r242: *mut i64 = std::ptr::null_mut();
    let mut len_r242 = 0usize;
    let mut i_r243 = 0i64;
    let mut b_r243 = false;
    let mut t_r243: *mut Table = std::ptr::null_mut();
    let mut p_r243: *mut i64 = std::ptr::null_mut();
    let mut len_r243 = 0usize;
    let mut i_r244 = 0i64;
    let mut b_r244 = false;
    let mut t_r244: *mut Table = std::ptr::null_mut();
    let mut p_r244: *mut i64 = std::ptr::null_mut();
    let mut len_r244 = 0usize;
    let mut i_r245 = 0i64;
    let mut b_r245 = false;
    let mut t_r245: *mut Table = std::ptr::null_mut();
    let mut p_r245: *mut i64 = std::ptr::null_mut();
    let mut len_r245 = 0usize;
    let mut i_r246 = 0i64;
    let mut b_r246 = false;
    let mut t_r246: *mut Table = std::ptr::null_mut();
    let mut p_r246: *mut i64 = std::ptr::null_mut();
    let mut len_r246 = 0usize;
    let mut i_r247 = 0i64;
    let mut b_r247 = false;
    let mut t_r247: *mut Table = std::ptr::null_mut();
    let mut p_r247: *mut i64 = std::ptr::null_mut();
    let mut len_r247 = 0usize;
    let mut i_r248 = 0i64;
    let mut b_r248 = false;
    let mut t_r248: *mut Table = std::ptr::null_mut();
    let mut p_r248: *mut i64 = std::ptr::null_mut();
    let mut len_r248 = 0usize;
    let mut i_r249 = 0i64;
    let mut b_r249 = false;
    let mut t_r249: *mut Table = std::ptr::null_mut();
    let mut p_r249: *mut i64 = std::ptr::null_mut();
    let mut len_r249 = 0usize;
    let mut i_r250 = 0i64;
    let mut b_r250 = false;
    let mut t_r250: *mut Table = std::ptr::null_mut();
    let mut p_r250: *mut i64 = std::ptr::null_mut();
    let mut len_r250 = 0usize;
    let mut i_r251 = 0i64;
    let mut b_r251 = false;
    let mut t_r251: *mut Table = std::ptr::null_mut();
    let mut p_r251: *mut i64 = std::ptr::null_mut();
    let mut len_r251 = 0usize;
    let mut i_r252 = 0i64;
    let mut b_r252 = false;
    let mut t_r252: *mut Table = std::ptr::null_mut();
    let mut p_r252: *mut i64 = std::ptr::null_mut();
    let mut len_r252 = 0usize;
    let mut i_r253 = 0i64;
    let mut b_r253 = false;
    let mut t_r253: *mut Table = std::ptr::null_mut();
    let mut p_r253: *mut i64 = std::ptr::null_mut();
    let mut len_r253 = 0usize;
    let mut i_r254 = 0i64;
    let mut b_r254 = false;
    let mut t_r254: *mut Table = std::ptr::null_mut();
    let mut p_r254: *mut i64 = std::ptr::null_mut();
    let mut len_r254 = 0usize;
    let mut i_r255 = 0i64;
    let mut b_r255 = false;
    let mut t_r255: *mut Table = std::ptr::null_mut();
    let mut p_r255: *mut i64 = std::ptr::null_mut();
    let mut len_r255 = 0usize;
    let mut i_r256 = 0i64;
    let mut b_r256 = false;
    let mut t_r256: *mut Table = std::ptr::null_mut();
    let mut p_r256: *mut i64 = std::ptr::null_mut();
    let mut len_r256 = 0usize;
    let mut i_r257 = 0i64;
    let mut b_r257 = false;
    let mut t_r257: *mut Table = std::ptr::null_mut();
    let mut p_r257: *mut i64 = std::ptr::null_mut();
    let mut len_r257 = 0usize;
    let mut i_r258 = 0i64;
    let mut b_r258 = false;
    let mut t_r258: *mut Table = std::ptr::null_mut();
    let mut p_r258: *mut i64 = std::ptr::null_mut();
    let mut len_r258 = 0usize;
    let mut i_r259 = 0i64;
    let mut b_r259 = false;
    let mut t_r259: *mut Table = std::ptr::null_mut();
    let mut p_r259: *mut i64 = std::ptr::null_mut();
    let mut len_r259 = 0usize;
    let mut i_r260 = 0i64;
    let mut b_r260 = false;
    let mut t_r260: *mut Table = std::ptr::null_mut();
    let mut p_r260: *mut i64 = std::ptr::null_mut();
    let mut len_r260 = 0usize;
    let mut i_r261 = 0i64;
    let mut b_r261 = false;
    let mut t_r261: *mut Table = std::ptr::null_mut();
    let mut p_r261: *mut i64 = std::ptr::null_mut();
    let mut len_r261 = 0usize;
    let mut i_r262 = 0i64;
    let mut b_r262 = false;
    let mut t_r262: *mut Table = std::ptr::null_mut();
    let mut p_r262: *mut i64 = std::ptr::null_mut();
    let mut len_r262 = 0usize;
    let mut i_r263 = 0i64;
    let mut b_r263 = false;
    let mut t_r263: *mut Table = std::ptr::null_mut();
    let mut p_r263: *mut i64 = std::ptr::null_mut();
    let mut len_r263 = 0usize;
    let mut i_r264 = 0i64;
    let mut b_r264 = false;
    let mut t_r264: *mut Table = std::ptr::null_mut();
    let mut p_r264: *mut i64 = std::ptr::null_mut();
    let mut len_r264 = 0usize;
    let mut i_r265 = 0i64;
    let mut b_r265 = false;
    let mut t_r265: *mut Table = std::ptr::null_mut();
    let mut p_r265: *mut i64 = std::ptr::null_mut();
    let mut len_r265 = 0usize;
    let mut i_r266 = 0i64;
    let mut b_r266 = false;
    let mut t_r266: *mut Table = std::ptr::null_mut();
    let mut p_r266: *mut i64 = std::ptr::null_mut();
    let mut len_r266 = 0usize;
    let mut i_r267 = 0i64;
    let mut b_r267 = false;
    let mut t_r267: *mut Table = std::ptr::null_mut();
    let mut p_r267: *mut i64 = std::ptr::null_mut();
    let mut len_r267 = 0usize;
    let mut i_r268 = 0i64;
    let mut b_r268 = false;
    let mut t_r268: *mut Table = std::ptr::null_mut();
    let mut p_r268: *mut i64 = std::ptr::null_mut();
    let mut len_r268 = 0usize;
    let mut i_r269 = 0i64;
    let mut b_r269 = false;
    let mut t_r269: *mut Table = std::ptr::null_mut();
    let mut p_r269: *mut i64 = std::ptr::null_mut();
    let mut len_r269 = 0usize;
    let mut i_r270 = 0i64;
    let mut b_r270 = false;
    let mut t_r270: *mut Table = std::ptr::null_mut();
    let mut p_r270: *mut i64 = std::ptr::null_mut();
    let mut len_r270 = 0usize;
    let mut i_r271 = 0i64;
    let mut b_r271 = false;
    let mut t_r271: *mut Table = std::ptr::null_mut();
    let mut p_r271: *mut i64 = std::ptr::null_mut();
    let mut len_r271 = 0usize;
    let mut i_r272 = 0i64;
    let mut b_r272 = false;
    let mut t_r272: *mut Table = std::ptr::null_mut();
    let mut p_r272: *mut i64 = std::ptr::null_mut();
    let mut len_r272 = 0usize;
    let mut i_r273 = 0i64;
    let mut b_r273 = false;
    let mut t_r273: *mut Table = std::ptr::null_mut();
    let mut p_r273: *mut i64 = std::ptr::null_mut();
    let mut len_r273 = 0usize;
    let mut i_r274 = 0i64;
    let mut b_r274 = false;
    let mut t_r274: *mut Table = std::ptr::null_mut();
    let mut p_r274: *mut i64 = std::ptr::null_mut();
    let mut len_r274 = 0usize;
    let mut i_r275 = 0i64;
    let mut b_r275 = false;
    let mut t_r275: *mut Table = std::ptr::null_mut();
    let mut p_r275: *mut i64 = std::ptr::null_mut();
    let mut len_r275 = 0usize;
    let mut i_r276 = 0i64;
    let mut b_r276 = false;
    let mut t_r276: *mut Table = std::ptr::null_mut();
    let mut p_r276: *mut i64 = std::ptr::null_mut();
    let mut len_r276 = 0usize;
    let mut i_r277 = 0i64;
    let mut b_r277 = false;
    let mut t_r277: *mut Table = std::ptr::null_mut();
    let mut p_r277: *mut i64 = std::ptr::null_mut();
    let mut len_r277 = 0usize;
    let mut i_r278 = 0i64;
    let mut b_r278 = false;
    let mut t_r278: *mut Table = std::ptr::null_mut();
    let mut p_r278: *mut i64 = std::ptr::null_mut();
    let mut len_r278 = 0usize;
    let mut i_r279 = 0i64;
    let mut b_r279 = false;
    let mut t_r279: *mut Table = std::ptr::null_mut();
    let mut p_r279: *mut i64 = std::ptr::null_mut();
    let mut len_r279 = 0usize;
    let mut i_r280 = 0i64;
    let mut b_r280 = false;
    let mut t_r280: *mut Table = std::ptr::null_mut();
    let mut p_r280: *mut i64 = std::ptr::null_mut();
    let mut len_r280 = 0usize;
    let mut i_r281 = 0i64;
    let mut b_r281 = false;
    let mut t_r281: *mut Table = std::ptr::null_mut();
    let mut p_r281: *mut i64 = std::ptr::null_mut();
    let mut len_r281 = 0usize;
    let mut i_r282 = 0i64;
    let mut b_r282 = false;
    let mut t_r282: *mut Table = std::ptr::null_mut();
    let mut p_r282: *mut i64 = std::ptr::null_mut();
    let mut len_r282 = 0usize;
    let mut i_r283 = 0i64;
    let mut b_r283 = false;
    let mut t_r283: *mut Table = std::ptr::null_mut();
    let mut p_r283: *mut i64 = std::ptr::null_mut();
    let mut len_r283 = 0usize;
    let mut i_r284 = 0i64;
    let mut b_r284 = false;
    let mut t_r284: *mut Table = std::ptr::null_mut();
    let mut p_r284: *mut i64 = std::ptr::null_mut();
    let mut len_r284 = 0usize;
    let mut i_r285 = 0i64;
    let mut b_r285 = false;
    let mut t_r285: *mut Table = std::ptr::null_mut();
    let mut p_r285: *mut i64 = std::ptr::null_mut();
    let mut len_r285 = 0usize;
    let mut i_r286 = 0i64;
    let mut b_r286 = false;
    let mut t_r286: *mut Table = std::ptr::null_mut();
    let mut p_r286: *mut i64 = std::ptr::null_mut();
    let mut len_r286 = 0usize;
    let mut i_r287 = 0i64;
    let mut b_r287 = false;
    let mut t_r287: *mut Table = std::ptr::null_mut();
    let mut p_r287: *mut i64 = std::ptr::null_mut();
    let mut len_r287 = 0usize;
    let mut i_r288 = 0i64;
    let mut b_r288 = false;
    let mut t_r288: *mut Table = std::ptr::null_mut();
    let mut p_r288: *mut i64 = std::ptr::null_mut();
    let mut len_r288 = 0usize;
    let mut i_r289 = 0i64;
    let mut b_r289 = false;
    let mut t_r289: *mut Table = std::ptr::null_mut();
    let mut p_r289: *mut i64 = std::ptr::null_mut();
    let mut len_r289 = 0usize;
    let mut i_r290 = 0i64;
    let mut b_r290 = false;
    let mut t_r290: *mut Table = std::ptr::null_mut();
    let mut p_r290: *mut i64 = std::ptr::null_mut();
    let mut len_r290 = 0usize;
    let mut i_r291 = 0i64;
    let mut b_r291 = false;
    let mut t_r291: *mut Table = std::ptr::null_mut();
    let mut p_r291: *mut i64 = std::ptr::null_mut();
    let mut len_r291 = 0usize;
    let mut i_r292 = 0i64;
    let mut b_r292 = false;
    let mut t_r292: *mut Table = std::ptr::null_mut();
    let mut p_r292: *mut i64 = std::ptr::null_mut();
    let mut len_r292 = 0usize;
    let mut i_r293 = 0i64;
    let mut b_r293 = false;
    let mut t_r293: *mut Table = std::ptr::null_mut();
    let mut p_r293: *mut i64 = std::ptr::null_mut();
    let mut len_r293 = 0usize;
    let mut i_r294 = 0i64;
    let mut b_r294 = false;
    let mut t_r294: *mut Table = std::ptr::null_mut();
    let mut p_r294: *mut i64 = std::ptr::null_mut();
    let mut len_r294 = 0usize;
    let mut i_r295 = 0i64;
    let mut b_r295 = false;
    let mut t_r295: *mut Table = std::ptr::null_mut();
    let mut p_r295: *mut i64 = std::ptr::null_mut();
    let mut len_r295 = 0usize;
    let mut i_r296 = 0i64;
    let mut b_r296 = false;
    let mut t_r296: *mut Table = std::ptr::null_mut();
    let mut p_r296: *mut i64 = std::ptr::null_mut();
    let mut len_r296 = 0usize;
    let mut i_r297 = 0i64;
    let mut b_r297 = false;
    let mut t_r297: *mut Table = std::ptr::null_mut();
    let mut p_r297: *mut i64 = std::ptr::null_mut();
    let mut len_r297 = 0usize;
    let mut i_r298 = 0i64;
    let mut b_r298 = false;
    let mut t_r298: *mut Table = std::ptr::null_mut();
    let mut p_r298: *mut i64 = std::ptr::null_mut();
    let mut len_r298 = 0usize;
    let mut i_r299 = 0i64;
    let mut b_r299 = false;
    let mut t_r299: *mut Table = std::ptr::null_mut();
    let mut p_r299: *mut i64 = std::ptr::null_mut();
    let mut len_r299 = 0usize;
    let mut i_r300 = 0i64;
    let mut b_r300 = false;
    let mut t_r300: *mut Table = std::ptr::null_mut();
    let mut p_r300: *mut i64 = std::ptr::null_mut();
    let mut len_r300 = 0usize;
    let mut i_r301 = 0i64;
    let mut b_r301 = false;
    let mut t_r301: *mut Table = std::ptr::null_mut();
    let mut p_r301: *mut i64 = std::ptr::null_mut();
    let mut len_r301 = 0usize;
    let mut i_r302 = 0i64;
    let mut b_r302 = false;
    let mut t_r302: *mut Table = std::ptr::null_mut();
    let mut p_r302: *mut i64 = std::ptr::null_mut();
    let mut len_r302 = 0usize;
    let mut i_r303 = 0i64;
    let mut b_r303 = false;
    let mut t_r303: *mut Table = std::ptr::null_mut();
    let mut p_r303: *mut i64 = std::ptr::null_mut();
    let mut len_r303 = 0usize;
    let mut i_r304 = 0i64;
    let mut b_r304 = false;
    let mut t_r304: *mut Table = std::ptr::null_mut();
    let mut p_r304: *mut i64 = std::ptr::null_mut();
    let mut len_r304 = 0usize;
    let mut i_r305 = 0i64;
    let mut b_r305 = false;
    let mut t_r305: *mut Table = std::ptr::null_mut();
    let mut p_r305: *mut i64 = std::ptr::null_mut();
    let mut len_r305 = 0usize;
    let mut i_r306 = 0i64;
    let mut b_r306 = false;
    let mut t_r306: *mut Table = std::ptr::null_mut();
    let mut p_r306: *mut i64 = std::ptr::null_mut();
    let mut len_r306 = 0usize;
    let mut i_r307 = 0i64;
    let mut b_r307 = false;
    let mut t_r307: *mut Table = std::ptr::null_mut();
    let mut p_r307: *mut i64 = std::ptr::null_mut();
    let mut len_r307 = 0usize;
    let mut i_r308 = 0i64;
    let mut b_r308 = false;
    let mut t_r308: *mut Table = std::ptr::null_mut();
    let mut p_r308: *mut i64 = std::ptr::null_mut();
    let mut len_r308 = 0usize;
    let mut i_r309 = 0i64;
    let mut b_r309 = false;
    let mut t_r309: *mut Table = std::ptr::null_mut();
    let mut p_r309: *mut i64 = std::ptr::null_mut();
    let mut len_r309 = 0usize;
    let mut i_r310 = 0i64;
    let mut b_r310 = false;
    let mut t_r310: *mut Table = std::ptr::null_mut();
    let mut p_r310: *mut i64 = std::ptr::null_mut();
    let mut len_r310 = 0usize;
    let mut i_r311 = 0i64;
    let mut b_r311 = false;
    let mut t_r311: *mut Table = std::ptr::null_mut();
    let mut p_r311: *mut i64 = std::ptr::null_mut();
    let mut len_r311 = 0usize;
    let mut i_r312 = 0i64;
    let mut b_r312 = false;
    let mut t_r312: *mut Table = std::ptr::null_mut();
    let mut p_r312: *mut i64 = std::ptr::null_mut();
    let mut len_r312 = 0usize;
    let mut i_r313 = 0i64;
    let mut b_r313 = false;
    let mut t_r313: *mut Table = std::ptr::null_mut();
    let mut p_r313: *mut i64 = std::ptr::null_mut();
    let mut len_r313 = 0usize;
    let mut i_r314 = 0i64;
    let mut b_r314 = false;
    let mut t_r314: *mut Table = std::ptr::null_mut();
    let mut p_r314: *mut i64 = std::ptr::null_mut();
    let mut len_r314 = 0usize;
    let mut i_r315 = 0i64;
    let mut b_r315 = false;
    let mut t_r315: *mut Table = std::ptr::null_mut();
    let mut p_r315: *mut i64 = std::ptr::null_mut();
    let mut len_r315 = 0usize;
    let mut i_r316 = 0i64;
    let mut b_r316 = false;
    let mut t_r316: *mut Table = std::ptr::null_mut();
    let mut p_r316: *mut i64 = std::ptr::null_mut();
    let mut len_r316 = 0usize;
    let mut i_r317 = 0i64;
    let mut b_r317 = false;
    let mut t_r317: *mut Table = std::ptr::null_mut();
    let mut p_r317: *mut i64 = std::ptr::null_mut();
    let mut len_r317 = 0usize;
    let mut i_r318 = 0i64;
    let mut b_r318 = false;
    let mut t_r318: *mut Table = std::ptr::null_mut();
    let mut p_r318: *mut i64 = std::ptr::null_mut();
    let mut len_r318 = 0usize;
    let mut i_r319 = 0i64;
    let mut b_r319 = false;
    let mut t_r319: *mut Table = std::ptr::null_mut();
    let mut p_r319: *mut i64 = std::ptr::null_mut();
    let mut len_r319 = 0usize;
    let mut i_r320 = 0i64;
    let mut b_r320 = false;
    let mut t_r320: *mut Table = std::ptr::null_mut();
    let mut p_r320: *mut i64 = std::ptr::null_mut();
    let mut len_r320 = 0usize;
    let mut i_r321 = 0i64;
    let mut b_r321 = false;
    let mut t_r321: *mut Table = std::ptr::null_mut();
    let mut p_r321: *mut i64 = std::ptr::null_mut();
    let mut len_r321 = 0usize;
    let mut i_r322 = 0i64;
    let mut b_r322 = false;
    let mut t_r322: *mut Table = std::ptr::null_mut();
    let mut p_r322: *mut i64 = std::ptr::null_mut();
    let mut len_r322 = 0usize;
    let mut i_r323 = 0i64;
    let mut b_r323 = false;
    let mut t_r323: *mut Table = std::ptr::null_mut();
    let mut p_r323: *mut i64 = std::ptr::null_mut();
    let mut len_r323 = 0usize;
    let mut i_r324 = 0i64;
    let mut b_r324 = false;
    let mut t_r324: *mut Table = std::ptr::null_mut();
    let mut p_r324: *mut i64 = std::ptr::null_mut();
    let mut len_r324 = 0usize;
    let mut i_r325 = 0i64;
    let mut b_r325 = false;
    let mut t_r325: *mut Table = std::ptr::null_mut();
    let mut p_r325: *mut i64 = std::ptr::null_mut();
    let mut len_r325 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut current_block = 0;
    'cfg: loop {
        match current_block {
            0 => {
                let mut new_table = Box::new(Table::new());
                t_r0 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r1 = 400;
                i_r2 = 0;
                let lim = i_r1;
                if lim > 0 {
                    let t = unsafe { &mut *t_r0 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r0 = unsafe { (*t_r0).array.len() };
                p_r0 = unsafe { (*t_r0).array.as_mut_ptr() };
                i_r3 = i_r2;
                current_block = 1;
            }
            1 => {
                b_r4 = i_r3 < i_r1;
                current_block = if b_r4 { 2 } else { 3 };
            }
            2 => {
                i_r10 = 1;
                i_r8 = i_r3 + i_r10;
                let k = i_r3;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r0 {
                    unsafe {
                        *p_r0.add(k as usize) = i_r8;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r13 = 2;
                i_r11 = i_r3 + i_r13;
                i_r3 = i_r11;
                current_block = 1;
            }
            3 => {
                let mut new_table = Box::new(Table::new());
                t_r14 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r15 = 300;
                i_r16 = 0;
                i_r17 = i_r16;
                current_block = 4;
            }
            4 => {
                b_r18 = i_r17 < i_r15;
                current_block = if b_r18 { 5 } else { 6 };
            }
            5 => {
                i_r21 = 0;
                let lim = i_r17;
                if lim > 0 {
                    let t = unsafe { &mut *t_r14 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r14 = unsafe { (*t_r14).array.len() };
                p_r14 = unsafe { (*t_r14).array.as_mut_ptr() };
                i_r22 = i_r21;
                current_block = 7;
            }
            6 => {
                let mut new_table = Box::new(Table::new());
                t_r36 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r37 = 250;
                i_r38 = 0;
                let lim = i_r37;
                if lim > 0 {
                    let t = unsafe { &mut *t_r36 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r36 = unsafe { (*t_r36).array.len() };
                p_r36 = unsafe { (*t_r36).array.as_mut_ptr() };
                i_r39 = i_r38;
                current_block = 10;
            }
            7 => {
                b_r23 = i_r22 < i_r17;
                current_block = if b_r23 { 8 } else { 9 };
            }
            8 => {
                i_r29 = 1;
                i_r27 = i_r22 + i_r29;
                let k = i_r22;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r14 {
                    unsafe {
                        *p_r14.add(k as usize) = i_r27;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r32 = 1;
                i_r30 = i_r22 + i_r32;
                i_r22 = i_r30;
                current_block = 7;
            }
            9 => {
                i_r35 = 1;
                i_r33 = i_r17 + i_r35;
                i_r17 = i_r33;
                current_block = 4;
            }
            10 => {
                b_r40 = i_r39 < i_r37;
                current_block = if b_r40 { 11 } else { 12 };
            }
            11 => {
                i_r44 = 5;
                let k = i_r39;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r36 {
                    unsafe {
                        *p_r36.add(k as usize) = i_r44;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r47 = 1;
                i_r45 = i_r39 + i_r47;
                i_r39 = i_r45;
                current_block = 10;
            }
            12 => {
                i_r48 = 0;
                let lim = i_r37;
                if lim > 0 {
                    let t = unsafe { &mut *t_r36 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r36 = unsafe { (*t_r36).array.len() };
                p_r36 = unsafe { (*t_r36).array.as_mut_ptr() };
                i_r49 = i_r48;
                current_block = 13;
            }
            13 => {
                b_r50 = i_r49 < i_r37;
                current_block = if b_r50 { 14 } else { 15 };
            }
            14 => {
                let k = i_r49;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r36 {
                    i_r55 = unsafe { *p_r36.add(k as usize) };
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r58 = 1;
                i_r54 = i_r55 + i_r58;
                let k = i_r49;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r36 {
                    unsafe {
                        *p_r36.add(k as usize) = i_r54;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r61 = 1;
                i_r59 = i_r49 + i_r61;
                i_r49 = i_r59;
                current_block = 13;
            }
            15 => {
                let mut new_table = Box::new(Table::new());
                t_r62 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r63 = 0;
                i_r64 = i_r63;
                current_block = 16;
            }
            16 => {
                i_r67 = 30;
                b_r65 = i_r64 < i_r67;
                current_block = if b_r65 { 17 } else { 18 };
            }
            17 => {
                i_r69 = 1;
                let k = i_r64;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r62 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r69;
                }
                i_r72 = 1;
                i_r70 = i_r64 + i_r72;
                i_r64 = i_r70;
                current_block = 16;
            }
            18 => {
                i_r73 = 30;
                i_r74 = 60;
                let k = i_r73;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r62 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r74;
                }
                i_r75 = 60;
                i_r76 = 0;
                i_r77 = i_r76;
                current_block = 19;
            }
            19 => {
                let k = i_r77;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &*t_r62 };
                i_r79 = if idx < t.array.len() {
                    unsafe { *t.array.get_unchecked(idx) }
                } else {
                    0
                };
                b_r78 = i_r79 < i_r75;
                current_block = if b_r78 { 20 } else { 21 };
            }
            20 => {
                i_r85 = 1;
                i_r83 = i_r77 + i_r85;
                i_r77 = i_r83;
                current_block = 19;
            }
            21 => {
                let mut new_table = Box::new(Table::new());
                t_r86 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r87 = 0;
                let k = i_r87;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r86 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r83;
                }
                let mut new_table = Box::new(Table::new());
                t_r89 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r90 = 200;
                i_r91 = 3;
                i_r92 = 0;
                i_r93 = i_r92;
                current_block = 22;
            }
            22 => {
                b_r94 = i_r93 < i_r90;
                current_block = if b_r94 { 23 } else { 24 };
            }
            23 => {
                i_r97 = i_r93;
                i_r98 = 0;
                i_r99 = i_r98;
                current_block = 25;
            }
            24 => {
                let mut new_table = Box::new(Table::new());
                t_r111 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r112 = 150;
                i_r113 = 0;
                i_r114 = i_r113;
                current_block = 28;
            }
            25 => {
                b_r100 = i_r99 < i_r91;
                current_block = if b_r100 { 26 } else { 27 };
            }
            26 => {
                i_r104 = 7;
                let k = i_r97;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r89 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r104;
                }
                i_r107 = 1;
                i_r105 = i_r99 + i_r107;
                i_r99 = i_r105;
                current_block = 25;
            }
            27 => {
                i_r110 = 1;
                i_r108 = i_r93 + i_r110;
                i_r93 = i_r108;
                current_block = 22;
            }
            28 => {
                b_r115 = i_r114 < i_r112;
                current_block = if b_r115 { 29 } else { 30 };
            }
            29 => {
                i_r118 = i_r114;
                let lim = i_r112;
                if lim > 0 {
                    let t = unsafe { &mut *t_r111 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r111 = unsafe { (*t_r111).array.len() };
                p_r111 = unsafe { (*t_r111).array.as_mut_ptr() };
                i_r119 = i_r118;
                current_block = 31;
            }
            30 => {
                let mut new_table = Box::new(Table::new());
                t_r133 = &mut *new_table as *mut Table;
                tables.push(new_table);
                let mut new_table = Box::new(Table::new());
                t_r134 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r135 = 180;
                i_r136 = 0;
                let lim = i_r135;
                if lim > 0 {
                    let t = unsafe { &mut *t_r133 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r133 = unsafe { (*t_r133).array.len() };
                p_r133 = unsafe { (*t_r133).array.as_mut_ptr() };
                i_r137 = i_r136;
                current_block = 34;
            }
            31 => {
                b_r120 = i_r119 < i_r112;
                current_block = if b_r120 { 32 } else { 33 };
            }
            32 => {
                i_r126 = 1;
                i_r124 = i_r119 + i_r126;
                let k = i_r119;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r111 {
                    unsafe {
                        *p_r111.add(k as usize) = i_r124;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r129 = 1;
                i_r127 = i_r119 + i_r129;
                i_r119 = i_r127;
                current_block = 31;
            }
            33 => {
                i_r132 = 1;
                i_r130 = i_r114 + i_r132;
                i_r114 = i_r130;
                current_block = 28;
            }
            34 => {
                b_r138 = i_r137 < i_r135;
                current_block = if b_r138 { 35 } else { 36 };
            }
            35 => {
                i_r144 = 1;
                i_r142 = i_r137 + i_r144;
                let k = i_r137;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r133 {
                    unsafe {
                        *p_r133.add(k as usize) = i_r142;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r147 = 1;
                i_r145 = i_r137 + i_r147;
                i_r137 = i_r145;
                current_block = 34;
            }
            36 => {
                i_r148 = 0;
                let lim = i_r135;
                if lim > 0 {
                    let t = unsafe { &mut *t_r133 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r133 = unsafe { (*t_r133).array.len() };
                p_r133 = unsafe { (*t_r133).array.as_mut_ptr() };
                i_r149 = i_r148;
                current_block = 37;
            }
            37 => {
                b_r150 = i_r149 < i_r135;
                current_block = if b_r150 { 38 } else { 39 };
            }
            38 => {
                i_r156 = 1;
                i_r154 = i_r135 - i_r156;
                i_r153 = i_r154 - i_r149;
                let k = i_r149;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r133 {
                    i_r159 = unsafe { *p_r133.add(k as usize) };
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                let k = i_r153;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r134 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r159;
                }
                i_r164 = 1;
                i_r162 = i_r149 + i_r164;
                i_r149 = i_r162;
                current_block = 37;
            }
            39 => {
                let mut new_table = Box::new(Table::new());
                t_r165 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r166 = 120;
                i_r167 = 0;
                let lim = i_r166;
                if lim > 0 {
                    let t = unsafe { &mut *t_r165 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r165 = unsafe { (*t_r165).array.len() };
                p_r165 = unsafe { (*t_r165).array.as_mut_ptr() };
                i_r168 = i_r167;
                current_block = 40;
            }
            40 => {
                b_r169 = i_r168 < i_r166;
                current_block = if b_r169 { 41 } else { 42 };
            }
            41 => {
                i_r175 = 1;
                i_r173 = i_r168 + i_r175;
                let k = i_r168;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r165 {
                    unsafe {
                        *p_r165.add(k as usize) = i_r173;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r178 = 1;
                i_r176 = i_r168 + i_r178;
                i_r168 = i_r176;
                current_block = 40;
            }
            42 => {
                i_r179 = 0;
                i_r180 = 0;
                let lim = i_r166;
                if lim > 0 {
                    let t = unsafe { &mut *t_r165 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r165 = unsafe { (*t_r165).array.len() };
                p_r165 = unsafe { (*t_r165).array.as_mut_ptr() };
                i_r181 = i_r180;
                i_r182 = i_r179;
                current_block = 43;
            }
            43 => {
                b_r183 = i_r181 < i_r166;
                current_block = if b_r183 { 44 } else { 45 };
            }
            44 => {
                let k = i_r181;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r165 {
                    i_r188 = unsafe { *p_r165.add(k as usize) };
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r186 = i_r182 + i_r188;
                i_r193 = 1;
                i_r191 = i_r181 + i_r193;
                i_r181 = i_r191;
                i_r182 = i_r186;
                current_block = 43;
            }
            45 => {
                let mut new_table = Box::new(Table::new());
                t_r194 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r195 = 0;
                let k = i_r195;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r194 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r186;
                }
                let mut new_table = Box::new(Table::new());
                t_r197 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r199 = 0;
                i_r200 = 1;
                b_r198 = i_r199 < i_r200;
                i_r201 = 0;
                i_r202 = i_r201;
                b_r203 = b_r198;
                current_block = 46;
            }
            46 => {
                current_block = if b_r203 { 47 } else { 48 };
            }
            47 => {
                i_r206 = 100;
                let k = i_r202;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r197 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r206;
                }
                i_r208 = 0;
                i_r209 = 0;
                b_r207 = i_r208 < i_r209;
                i_r212 = 1;
                i_r210 = i_r202 + i_r212;
                i_r202 = i_r210;
                b_r203 = b_r207;
                current_block = 46;
            }
            48 => {
                let mut new_table = Box::new(Table::new());
                t_r213 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r214 = 200;
                i_r215 = 100;
                let lim = i_r214;
                if lim > 0 {
                    let t = unsafe { &mut *t_r213 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r213 = unsafe { (*t_r213).array.len() };
                p_r213 = unsafe { (*t_r213).array.as_mut_ptr() };
                i_r216 = i_r215;
                current_block = 49;
            }
            49 => {
                b_r217 = i_r216 < i_r214;
                current_block = if b_r217 { 50 } else { 51 };
            }
            50 => {
                i_r223 = 100;
                i_r221 = i_r216 - i_r223;
                let k = i_r216;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r213 {
                    unsafe {
                        *p_r213.add(k as usize) = i_r221;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r226 = 1;
                i_r224 = i_r216 + i_r226;
                i_r216 = i_r224;
                current_block = 49;
            }
            51 => {
                i_r227 = 350;
                i_r228 = 200;
                let lim = i_r227;
                if lim > 0 {
                    let t = unsafe { &mut *t_r213 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r213 = unsafe { (*t_r213).array.len() };
                p_r213 = unsafe { (*t_r213).array.as_mut_ptr() };
                i_r229 = i_r228;
                current_block = 52;
            }
            52 => {
                b_r230 = i_r229 < i_r227;
                current_block = if b_r230 { 53 } else { 54 };
            }
            53 => {
                let k = i_r229;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r213 {
                    unsafe {
                        *p_r213.add(k as usize) = i_r229;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r237 = 1;
                i_r235 = i_r229 + i_r237;
                i_r229 = i_r235;
                current_block = 52;
            }
            54 => {
                let mut new_table = Box::new(Table::new());
                t_r238 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r239 = 0;
                i_r242 = 150;
                let k = i_r242;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &*t_r213 };
                i_r240 = if idx < t.array.len() {
                    unsafe { *t.array.get_unchecked(idx) }
                } else {
                    0
                };
                let k = i_r239;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r238 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r240;
                }
                i_r243 = 1;
                i_r246 = 250;
                let k = i_r246;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &*t_r213 };
                i_r244 = if idx < t.array.len() {
                    unsafe { *t.array.get_unchecked(idx) }
                } else {
                    0
                };
                let k = i_r243;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r238 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r244;
                }
                i_r247 = 2;
                i_r250 = 349;
                let k = i_r250;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &*t_r213 };
                i_r248 = if idx < t.array.len() {
                    unsafe { *t.array.get_unchecked(idx) }
                } else {
                    0
                };
                let k = i_r247;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r238 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r248;
                }
                i_r251 = 3;
                i_r254 = 99;
                let k = i_r254;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &*t_r213 };
                i_r252 = if idx < t.array.len() {
                    unsafe { *t.array.get_unchecked(idx) }
                } else {
                    0
                };
                let k = i_r251;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r238 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r252;
                }
                let mut new_table = Box::new(Table::new());
                t_r255 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r256 = 60;
                i_r257 = 0;
                i_r258 = i_r257;
                current_block = 55;
            }
            55 => {
                b_r259 = i_r258 < i_r256;
                current_block = if b_r259 { 56 } else { 57 };
            }
            56 => {
                i_r262 = 0;
                i_r263 = i_r262;
                current_block = 58;
            }
            57 => {
                let mut new_table = Box::new(Table::new());
                t_r285 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r286 = 80;
                i_r287 = 0;
                i_r288 = i_r287;
                current_block = 64;
            }
            58 => {
                b_r264 = i_r263 < i_r256;
                current_block = if b_r264 { 59 } else { 60 };
            }
            59 => {
                i_r267 = 0;
                let lim = i_r256;
                if lim > 0 {
                    let t = unsafe { &mut *t_r255 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r255 = unsafe { (*t_r255).array.len() };
                p_r255 = unsafe { (*t_r255).array.as_mut_ptr() };
                i_r268 = i_r267;
                current_block = 61;
            }
            60 => {
                i_r284 = 1;
                i_r282 = i_r258 + i_r284;
                i_r258 = i_r282;
                current_block = 55;
            }
            61 => {
                b_r269 = i_r268 < i_r256;
                current_block = if b_r269 { 62 } else { 63 };
            }
            62 => {
                i_r273 = i_r268 + i_r263;
                let k = i_r268;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r255 {
                    unsafe {
                        *p_r255.add(k as usize) = i_r273;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r278 = 1;
                i_r276 = i_r268 + i_r278;
                i_r268 = i_r276;
                current_block = 61;
            }
            63 => {
                i_r281 = 1;
                i_r279 = i_r263 + i_r281;
                i_r263 = i_r279;
                current_block = 58;
            }
            64 => {
                b_r289 = i_r288 < i_r286;
                current_block = if b_r289 { 65 } else { 66 };
            }
            65 => {
                i_r292 = i_r288;
                i_r294 = 1;
                let k = i_r292;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r285 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r294;
                }
                i_r295 = i_r292;
                i_r298 = 0;
                i_r296 = i_r295 + i_r298;
                i_r300 = 2;
                let k = i_r296;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r285 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r300;
                }
                i_r301 = i_r288;
                i_r303 = 3;
                let k = i_r301;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r285 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r303;
                }
                i_r306 = 1;
                i_r304 = i_r288 + i_r306;
                i_r288 = i_r304;
                current_block = 64;
            }
            66 => {
                let mut new_table = Box::new(Table::new());
                t_r307 = &mut *new_table as *mut Table;
                tables.push(new_table);
                t_r308 = t_r307;
                i_r309 = 90;
                i_r310 = 0;
                let lim = i_r309;
                if lim > 0 {
                    let t = unsafe { &mut *t_r308 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r308 = unsafe { (*t_r308).array.len() };
                p_r308 = unsafe { (*t_r308).array.as_mut_ptr() };
                let lim = i_r309;
                if lim > 0 {
                    let t = unsafe { &mut *t_r307 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r307 = unsafe { (*t_r307).array.len() };
                p_r307 = unsafe { (*t_r307).array.as_mut_ptr() };
                i_r311 = i_r310;
                current_block = 67;
            }
            67 => {
                b_r312 = i_r311 < i_r309;
                current_block = if b_r312 { 68 } else { 69 };
            }
            68 => {
                i_r318 = 1;
                i_r316 = i_r311 + i_r318;
                let k = i_r311;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r307 {
                    unsafe {
                        *p_r307.add(k as usize) = i_r316;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r322 = 2;
                i_r320 = i_r311 + i_r322;
                let k = i_r311;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r308 {
                    unsafe {
                        *p_r308.add(k as usize) = i_r320;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r325 = 1;
                i_r323 = i_r311 + i_r325;
                i_r311 = i_r323;
                current_block = 67;
            }
            69 => {
                break 'cfg;
            }
            _ => unreachable!(),
        }
    }
    tables
}

pub const STATS: &str = "fast_sets=12;fast_gets=3;dyn_sets=14;dyn_gets=5;hoists=14;hoist_ctx=0,1,0,0,1,0,0,0,0,0,0,2,0,0";
```
