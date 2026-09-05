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
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
    let mut i_r4 = 0i64;
    let mut i_r5 = 0i64;
    let mut i_r6 = 0i64;
    let mut i_r7 = 0i64;
    let mut i_r8 = 0i64;
    let mut i_r9 = 0i64;
    let mut b_r0 = false;
    let mut b_r1 = false;
    let mut t_r0: *mut Table = std::ptr::null_mut();
    let mut p_r0: *mut i64 = std::ptr::null_mut();
    let mut len_r0 = 0usize;
    let mut t_r1: *mut Table = std::ptr::null_mut();
    let mut p_r1: *mut i64 = std::ptr::null_mut();
    let mut len_r1 = 0usize;
    let mut t_r2: *mut Table = std::ptr::null_mut();
    let mut p_r2: *mut i64 = std::ptr::null_mut();
    let mut len_r2 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut current_block = 0;
    'cfg: loop {
        match current_block {
            0 => {
                let mut new_table = Box::new(Table::new());
                t_r0 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r2 = 400;
                i_r3 = 0;
                let lim = i_r2;
                if lim > 0 {
                    let t = unsafe { &mut *t_r0 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r0 = unsafe { (*t_r0).array.len() };
                p_r0 = unsafe { (*t_r0).array.as_mut_ptr() };
                i_r4 = i_r3;
                current_block = 1;
            }
            1 => {
                b_r0 = i_r4 < i_r2;
                current_block = if b_r0 { 2 } else { 3 };
            }
            2 => {
                i_r3 = 1;
                i_r5 = i_r4 + i_r3;
                let k = i_r4;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r0 {
                    unsafe {
                        *p_r0.add(k as usize) = i_r5;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r5 = 2;
                i_r3 = i_r4 + i_r5;
                i_r4 = i_r3;
                current_block = 1;
            }
            3 => {
                let mut new_table = Box::new(Table::new());
                t_r0 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r3 = 300;
                i_r5 = 0;
                i_r4 = i_r5;
                current_block = 4;
            }
            4 => {
                b_r0 = i_r4 < i_r3;
                current_block = if b_r0 { 5 } else { 6 };
            }
            5 => {
                i_r5 = 0;
                let lim = i_r4;
                if lim > 0 {
                    let t = unsafe { &mut *t_r0 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r0 = unsafe { (*t_r0).array.len() };
                p_r0 = unsafe { (*t_r0).array.as_mut_ptr() };
                i_r2 = i_r5;
                current_block = 7;
            }
            6 => {
                let mut new_table = Box::new(Table::new());
                t_r1 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r5 = 250;
                i_r6 = 0;
                let lim = i_r5;
                if lim > 0 {
                    let t = unsafe { &mut *t_r1 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r1 = unsafe { (*t_r1).array.len() };
                p_r1 = unsafe { (*t_r1).array.as_mut_ptr() };
                i_r7 = i_r6;
                current_block = 10;
            }
            7 => {
                b_r0 = i_r2 < i_r4;
                current_block = if b_r0 { 8 } else { 9 };
            }
            8 => {
                i_r6 = 1;
                i_r8 = i_r2 + i_r6;
                let k = i_r2;
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
                i_r8 = 1;
                i_r6 = i_r2 + i_r8;
                i_r2 = i_r6;
                current_block = 7;
            }
            9 => {
                i_r6 = 1;
                i_r8 = i_r4 + i_r6;
                i_r4 = i_r8;
                current_block = 4;
            }
            10 => {
                b_r0 = i_r7 < i_r5;
                current_block = if b_r0 { 11 } else { 12 };
            }
            11 => {
                i_r8 = 5;
                let k = i_r7;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r1 {
                    unsafe {
                        *p_r1.add(k as usize) = i_r8;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r8 = 1;
                i_r6 = i_r7 + i_r8;
                i_r7 = i_r6;
                current_block = 10;
            }
            12 => {
                i_r6 = 0;
                let lim = i_r5;
                if lim > 0 {
                    let t = unsafe { &mut *t_r1 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r1 = unsafe { (*t_r1).array.len() };
                p_r1 = unsafe { (*t_r1).array.as_mut_ptr() };
                i_r8 = i_r6;
                current_block = 13;
            }
            13 => {
                b_r0 = i_r8 < i_r5;
                current_block = if b_r0 { 14 } else { 15 };
            }
            14 => {
                let k = i_r8;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r1 {
                    i_r6 = unsafe { *p_r1.add(k as usize) };
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r7 = 1;
                i_r4 = i_r6 + i_r7;
                let k = i_r8;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r1 {
                    unsafe {
                        *p_r1.add(k as usize) = i_r4;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r4 = 1;
                i_r7 = i_r8 + i_r4;
                i_r8 = i_r7;
                current_block = 13;
            }
            15 => {
                let mut new_table = Box::new(Table::new());
                t_r1 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r7 = 0;
                i_r4 = i_r7;
                current_block = 16;
            }
            16 => {
                i_r7 = 30;
                b_r0 = i_r4 < i_r7;
                current_block = if b_r0 { 17 } else { 18 };
            }
            17 => {
                i_r7 = 1;
                let k = i_r4;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r1 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r7;
                }
                i_r7 = 1;
                i_r8 = i_r4 + i_r7;
                i_r4 = i_r8;
                current_block = 16;
            }
            18 => {
                i_r8 = 30;
                i_r7 = 60;
                let k = i_r8;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r1 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r7;
                }
                i_r7 = 60;
                i_r8 = 0;
                i_r4 = i_r8;
                current_block = 19;
            }
            19 => {
                let k = i_r4;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &*t_r1 };
                i_r8 = if idx < t.array.len() {
                    unsafe { *t.array.get_unchecked(idx) }
                } else {
                    0
                };
                b_r0 = i_r8 < i_r7;
                current_block = if b_r0 { 20 } else { 21 };
            }
            20 => {
                i_r8 = 1;
                i_r0 = i_r4 + i_r8;
                i_r4 = i_r0;
                current_block = 19;
            }
            21 => {
                let mut new_table = Box::new(Table::new());
                t_r1 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r8 = 0;
                let k = i_r8;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r1 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r0;
                }
                let mut new_table = Box::new(Table::new());
                t_r1 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r8 = 200;
                i_r0 = 3;
                i_r4 = 0;
                i_r7 = i_r4;
                current_block = 22;
            }
            22 => {
                b_r0 = i_r7 < i_r8;
                current_block = if b_r0 { 23 } else { 24 };
            }
            23 => {
                i_r4 = i_r7;
                i_r5 = 0;
                i_r6 = i_r5;
                current_block = 25;
            }
            24 => {
                let mut new_table = Box::new(Table::new());
                t_r0 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r5 = 150;
                i_r3 = 0;
                i_r2 = i_r3;
                current_block = 28;
            }
            25 => {
                b_r0 = i_r6 < i_r0;
                current_block = if b_r0 { 26 } else { 27 };
            }
            26 => {
                i_r3 = 7;
                let k = i_r4;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r1 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r3;
                }
                i_r3 = 1;
                i_r9 = i_r6 + i_r3;
                i_r6 = i_r9;
                current_block = 25;
            }
            27 => {
                i_r9 = 1;
                i_r3 = i_r7 + i_r9;
                i_r7 = i_r3;
                current_block = 22;
            }
            28 => {
                b_r0 = i_r2 < i_r5;
                current_block = if b_r0 { 29 } else { 30 };
            }
            29 => {
                i_r3 = i_r2;
                let lim = i_r5;
                if lim > 0 {
                    let t = unsafe { &mut *t_r0 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r0 = unsafe { (*t_r0).array.len() };
                p_r0 = unsafe { (*t_r0).array.as_mut_ptr() };
                i_r9 = i_r3;
                current_block = 31;
            }
            30 => {
                let mut new_table = Box::new(Table::new());
                t_r1 = &mut *new_table as *mut Table;
                tables.push(new_table);
                let mut new_table = Box::new(Table::new());
                t_r2 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r3 = 180;
                i_r7 = 0;
                let lim = i_r3;
                if lim > 0 {
                    let t = unsafe { &mut *t_r1 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r1 = unsafe { (*t_r1).array.len() };
                p_r1 = unsafe { (*t_r1).array.as_mut_ptr() };
                i_r0 = i_r7;
                current_block = 34;
            }
            31 => {
                b_r0 = i_r9 < i_r5;
                current_block = if b_r0 { 32 } else { 33 };
            }
            32 => {
                i_r7 = 1;
                i_r8 = i_r9 + i_r7;
                let k = i_r9;
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
                i_r8 = 1;
                i_r7 = i_r9 + i_r8;
                i_r9 = i_r7;
                current_block = 31;
            }
            33 => {
                i_r7 = 1;
                i_r8 = i_r2 + i_r7;
                i_r2 = i_r8;
                current_block = 28;
            }
            34 => {
                b_r0 = i_r0 < i_r3;
                current_block = if b_r0 { 35 } else { 36 };
            }
            35 => {
                i_r8 = 1;
                i_r7 = i_r0 + i_r8;
                let k = i_r0;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r1 {
                    unsafe {
                        *p_r1.add(k as usize) = i_r7;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r7 = 1;
                i_r8 = i_r0 + i_r7;
                i_r0 = i_r8;
                current_block = 34;
            }
            36 => {
                i_r8 = 0;
                let lim = i_r3;
                if lim > 0 {
                    let t = unsafe { &mut *t_r1 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r1 = unsafe { (*t_r1).array.len() };
                p_r1 = unsafe { (*t_r1).array.as_mut_ptr() };
                i_r7 = i_r8;
                current_block = 37;
            }
            37 => {
                b_r0 = i_r7 < i_r3;
                current_block = if b_r0 { 38 } else { 39 };
            }
            38 => {
                i_r8 = 1;
                i_r0 = i_r3 - i_r8;
                i_r8 = i_r0 - i_r7;
                let k = i_r7;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r1 {
                    i_r0 = unsafe { *p_r1.add(k as usize) };
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                let k = i_r8;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r2 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r0;
                }
                i_r0 = 1;
                i_r8 = i_r7 + i_r0;
                i_r7 = i_r8;
                current_block = 37;
            }
            39 => {
                let mut new_table = Box::new(Table::new());
                t_r2 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r8 = 120;
                i_r0 = 0;
                let lim = i_r8;
                if lim > 0 {
                    let t = unsafe { &mut *t_r2 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r2 = unsafe { (*t_r2).array.len() };
                p_r2 = unsafe { (*t_r2).array.as_mut_ptr() };
                i_r7 = i_r0;
                current_block = 40;
            }
            40 => {
                b_r0 = i_r7 < i_r8;
                current_block = if b_r0 { 41 } else { 42 };
            }
            41 => {
                i_r0 = 1;
                i_r3 = i_r7 + i_r0;
                let k = i_r7;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r2 {
                    unsafe {
                        *p_r2.add(k as usize) = i_r3;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r3 = 1;
                i_r0 = i_r7 + i_r3;
                i_r7 = i_r0;
                current_block = 40;
            }
            42 => {
                i_r0 = 0;
                i_r3 = 0;
                let lim = i_r8;
                if lim > 0 {
                    let t = unsafe { &mut *t_r2 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r2 = unsafe { (*t_r2).array.len() };
                p_r2 = unsafe { (*t_r2).array.as_mut_ptr() };
                i_r7 = i_r0;
                i_r0 = i_r3;
                current_block = 43;
            }
            43 => {
                b_r0 = i_r0 < i_r8;
                current_block = if b_r0 { 44 } else { 45 };
            }
            44 => {
                let k = i_r0;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r2 {
                    i_r3 = unsafe { *p_r2.add(k as usize) };
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r1 = i_r7 + i_r3;
                i_r3 = 1;
                i_r2 = i_r0 + i_r3;
                i_r7 = i_r1;
                i_r0 = i_r2;
                current_block = 43;
            }
            45 => {
                let mut new_table = Box::new(Table::new());
                t_r2 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r2 = 0;
                let k = i_r2;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r2 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r1;
                }
                let mut new_table = Box::new(Table::new());
                t_r2 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r2 = 0;
                i_r1 = 1;
                b_r0 = i_r2 < i_r1;
                i_r1 = 0;
                b_r1 = b_r0;
                i_r2 = i_r1;
                current_block = 46;
            }
            46 => {
                current_block = if b_r1 { 47 } else { 48 };
            }
            47 => {
                i_r1 = 100;
                let k = i_r2;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r2 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r1;
                }
                i_r1 = 0;
                i_r3 = 0;
                b_r0 = i_r1 < i_r3;
                i_r3 = 1;
                i_r1 = i_r2 + i_r3;
                b_r1 = b_r0;
                i_r2 = i_r1;
                current_block = 46;
            }
            48 => {
                let mut new_table = Box::new(Table::new());
                t_r2 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r1 = 200;
                i_r3 = 100;
                let lim = i_r1;
                if lim > 0 {
                    let t = unsafe { &mut *t_r2 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r2 = unsafe { (*t_r2).array.len() };
                p_r2 = unsafe { (*t_r2).array.as_mut_ptr() };
                i_r2 = i_r3;
                current_block = 49;
            }
            49 => {
                b_r0 = i_r2 < i_r1;
                current_block = if b_r0 { 50 } else { 51 };
            }
            50 => {
                i_r3 = 100;
                i_r0 = i_r2 - i_r3;
                let k = i_r2;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r2 {
                    unsafe {
                        *p_r2.add(k as usize) = i_r0;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r0 = 1;
                i_r3 = i_r2 + i_r0;
                i_r2 = i_r3;
                current_block = 49;
            }
            51 => {
                i_r3 = 350;
                i_r0 = 200;
                let lim = i_r3;
                if lim > 0 {
                    let t = unsafe { &mut *t_r2 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r2 = unsafe { (*t_r2).array.len() };
                p_r2 = unsafe { (*t_r2).array.as_mut_ptr() };
                i_r2 = i_r0;
                current_block = 52;
            }
            52 => {
                b_r0 = i_r2 < i_r3;
                current_block = if b_r0 { 53 } else { 54 };
            }
            53 => {
                let k = i_r2;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r2 {
                    unsafe {
                        *p_r2.add(k as usize) = i_r2;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r0 = 1;
                i_r1 = i_r2 + i_r0;
                i_r2 = i_r1;
                current_block = 52;
            }
            54 => {
                let mut new_table = Box::new(Table::new());
                t_r1 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r1 = 0;
                i_r0 = 150;
                let k = i_r0;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &*t_r2 };
                i_r2 = if idx < t.array.len() {
                    unsafe { *t.array.get_unchecked(idx) }
                } else {
                    0
                };
                let k = i_r1;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r1 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r2;
                }
                i_r2 = 1;
                i_r0 = 250;
                let k = i_r0;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &*t_r2 };
                i_r1 = if idx < t.array.len() {
                    unsafe { *t.array.get_unchecked(idx) }
                } else {
                    0
                };
                let k = i_r2;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r1 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r1;
                }
                i_r1 = 2;
                i_r0 = 349;
                let k = i_r0;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &*t_r2 };
                i_r2 = if idx < t.array.len() {
                    unsafe { *t.array.get_unchecked(idx) }
                } else {
                    0
                };
                let k = i_r1;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r1 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r2;
                }
                i_r2 = 3;
                i_r0 = 99;
                let k = i_r0;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &*t_r2 };
                i_r1 = if idx < t.array.len() {
                    unsafe { *t.array.get_unchecked(idx) }
                } else {
                    0
                };
                let k = i_r2;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r1 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r1;
                }
                let mut new_table = Box::new(Table::new());
                t_r1 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r1 = 60;
                i_r0 = 0;
                i_r2 = i_r0;
                current_block = 55;
            }
            55 => {
                b_r0 = i_r2 < i_r1;
                current_block = if b_r0 { 56 } else { 57 };
            }
            56 => {
                i_r0 = 0;
                i_r3 = i_r0;
                current_block = 58;
            }
            57 => {
                let mut new_table = Box::new(Table::new());
                t_r2 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r0 = 80;
                i_r7 = 0;
                i_r8 = i_r7;
                current_block = 64;
            }
            58 => {
                b_r0 = i_r3 < i_r1;
                current_block = if b_r0 { 59 } else { 60 };
            }
            59 => {
                i_r7 = 0;
                let lim = i_r1;
                if lim > 0 {
                    let t = unsafe { &mut *t_r1 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r1 = unsafe { (*t_r1).array.len() };
                p_r1 = unsafe { (*t_r1).array.as_mut_ptr() };
                i_r5 = i_r7;
                current_block = 61;
            }
            60 => {
                i_r7 = 1;
                i_r9 = i_r2 + i_r7;
                i_r2 = i_r9;
                current_block = 55;
            }
            61 => {
                b_r0 = i_r5 < i_r1;
                current_block = if b_r0 { 62 } else { 63 };
            }
            62 => {
                i_r9 = i_r5 + i_r3;
                let k = i_r5;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r1 {
                    unsafe {
                        *p_r1.add(k as usize) = i_r9;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r9 = 1;
                i_r7 = i_r5 + i_r9;
                i_r5 = i_r7;
                current_block = 61;
            }
            63 => {
                i_r7 = 1;
                i_r9 = i_r3 + i_r7;
                i_r3 = i_r9;
                current_block = 58;
            }
            64 => {
                b_r0 = i_r8 < i_r0;
                current_block = if b_r0 { 65 } else { 66 };
            }
            65 => {
                i_r9 = i_r8;
                i_r7 = 1;
                let k = i_r9;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r2 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r7;
                }
                i_r7 = 0;
                i_r3 = i_r9 + i_r7;
                i_r7 = 2;
                let k = i_r3;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r2 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r7;
                }
                i_r7 = i_r8;
                i_r3 = 3;
                let k = i_r7;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r2 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r3;
                }
                i_r3 = 1;
                i_r7 = i_r8 + i_r3;
                i_r8 = i_r7;
                current_block = 64;
            }
            66 => {
                let mut new_table = Box::new(Table::new());
                t_r2 = &mut *new_table as *mut Table;
                tables.push(new_table);
                i_r7 = 90;
                i_r3 = 0;
                let lim = i_r7;
                if lim > 0 {
                    let t = unsafe { &mut *t_r2 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r2 = unsafe { (*t_r2).array.len() };
                p_r2 = unsafe { (*t_r2).array.as_mut_ptr() };
                let lim = i_r7;
                if lim > 0 {
                    let t = unsafe { &mut *t_r2 };
                    if (lim as usize) > t.array.len() {
                        t.array.resize(lim as usize, 0);
                    }
                }
                len_r2 = unsafe { (*t_r2).array.len() };
                p_r2 = unsafe { (*t_r2).array.as_mut_ptr() };
                i_r8 = i_r3;
                current_block = 67;
            }
            67 => {
                b_r0 = i_r8 < i_r7;
                current_block = if b_r0 { 68 } else { 69 };
            }
            68 => {
                i_r3 = 1;
                i_r0 = i_r8 + i_r3;
                let k = i_r8;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r2 {
                    unsafe {
                        *p_r2.add(k as usize) = i_r0;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r0 = 2;
                i_r3 = i_r8 + i_r0;
                let k = i_r8;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r2 {
                    unsafe {
                        *p_r2.add(k as usize) = i_r3;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r3 = 1;
                i_r0 = i_r8 + i_r3;
                i_r8 = i_r0;
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
