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
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
    let mut i_r4 = 0i64;
    let mut i_r5 = 0i64;
    let mut i_r6 = 0i64;
    let mut i_r7 = 0i64;
    let mut i_r8 = 0i64;
    let mut i_r9 = 0i64;
    let mut i_r10 = 0i64;
    let mut i_r11 = 0i64;
    let mut i_r12 = 0i64;
    let mut i_r13 = 0i64;
    let mut i_r14 = 0i64;
    let mut i_r15 = 0i64;
    let mut i_r16 = 0i64;
    let mut i_r17 = 0i64;
    let mut i_r18 = 0i64;
    let mut i_r19 = 0i64;
    let mut i_r20 = 0i64;
    let mut i_r21 = 0i64;
    let mut i_r22 = 0i64;
    let mut i_r23 = 0i64;
    let mut i_r24 = 0i64;
    let mut i_r25 = 0i64;
    let mut i_r26 = 0i64;
    let mut i_r27 = 0i64;
    let mut i_r28 = 0i64;
    let mut i_r29 = 0i64;
    let mut i_r30 = 0i64;
    let mut i_r31 = 0i64;
    let mut i_r32 = 0i64;
    let mut i_r33 = 0i64;
    let mut i_r35 = 0i64;
    let mut i_r36 = 0i64;
    let mut i_r37 = 0i64;
    let mut i_r38 = 0i64;
    let mut i_r39 = 0i64;
    let mut i_r40 = 0i64;
    let mut i_r41 = 0i64;
    let mut i_r42 = 0i64;
    let mut i_r43 = 0i64;
    let mut i_r44 = 0i64;
    let mut i_r45 = 0i64;
    let mut i_r46 = 0i64;
    let mut i_r47 = 0i64;
    let mut i_r48 = 0i64;
    let mut i_r49 = 0i64;
    let mut i_r50 = 0i64;
    let mut i_r51 = 0i64;
    let mut i_r52 = 0i64;
    let mut i_r53 = 0i64;
    let mut b_r3 = false;
    let mut b_r6 = false;
    let mut b_r7 = false;
    let mut b_r9 = false;
    let mut b_r10 = false;
    let mut b_r13 = false;
    let mut b_r15 = false;
    let mut b_r19 = false;
    let mut b_r21 = false;
    let mut b_r22 = false;
    let mut b_r23 = false;
    let mut b_r26 = false;
    let mut b_r27 = false;
    let mut b_r30 = false;
    let mut b_r32 = false;
    let mut b_r34 = false;
    let mut b_r39 = false;
    let mut b_r41 = false;
    let mut b_r45 = false;
    let mut b_r46 = false;
    let mut b_r47 = false;
    let mut b_r48 = false;
    let mut b_r52 = false;
    let mut t_r0: *mut Table = std::ptr::null_mut();
    let mut p_r0: *mut i64 = std::ptr::null_mut();
    let mut t_r3: *mut Table = std::ptr::null_mut();
    let mut p_r3: *mut i64 = std::ptr::null_mut();
    let mut t_r6: *mut Table = std::ptr::null_mut();
    let mut p_r6: *mut i64 = std::ptr::null_mut();
    let mut t_r10: *mut Table = std::ptr::null_mut();
    let mut p_r10: *mut i64 = std::ptr::null_mut();
    let mut t_r14: *mut Table = std::ptr::null_mut();
    let mut p_r14: *mut i64 = std::ptr::null_mut();
    let mut t_r15: *mut Table = std::ptr::null_mut();
    let mut p_r15: *mut i64 = std::ptr::null_mut();
    let mut t_r19: *mut Table = std::ptr::null_mut();
    let mut p_r19: *mut i64 = std::ptr::null_mut();
    let mut t_r22: *mut Table = std::ptr::null_mut();
    let mut p_r22: *mut i64 = std::ptr::null_mut();
    let mut t_r23: *mut Table = std::ptr::null_mut();
    let mut p_r23: *mut i64 = std::ptr::null_mut();
    let mut t_r27: *mut Table = std::ptr::null_mut();
    let mut p_r27: *mut i64 = std::ptr::null_mut();
    let mut t_r32: *mut Table = std::ptr::null_mut();
    let mut p_r32: *mut i64 = std::ptr::null_mut();
    let mut t_r33: *mut Table = std::ptr::null_mut();
    let mut p_r33: *mut i64 = std::ptr::null_mut();
    let mut t_r36: *mut Table = std::ptr::null_mut();
    let mut p_r36: *mut i64 = std::ptr::null_mut();
    let mut t_r41: *mut Table = std::ptr::null_mut();
    let mut p_r41: *mut i64 = std::ptr::null_mut();
    let mut t_r42: *mut Table = std::ptr::null_mut();
    let mut p_r42: *mut i64 = std::ptr::null_mut();
    let mut t_r45: *mut Table = std::ptr::null_mut();
    let mut p_r45: *mut i64 = std::ptr::null_mut();
    let mut t_r48: *mut Table = std::ptr::null_mut();
    let mut p_r48: *mut i64 = std::ptr::null_mut();
    let mut t_r49: *mut Table = std::ptr::null_mut();
    let mut p_r49: *mut i64 = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r0 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r1 = 400;
    i_r2 = 0;
    let lim = i_r1;
    if lim > 0 {
        let cap_limit = lim as usize;
        let t = unsafe { &mut *t_r0 };
        if cap_limit > t.array.len() {
            t.array.resize(cap_limit, 0);
        }
    }
    let len_r0 = unsafe { (*t_r0).array.len() };
    p_r0 = unsafe { (*t_r0).array.as_mut_ptr() };
    loop {
        b_r3 = i_r2 < i_r1;
        if !b_r3 {
            break;
        }
        i_r3 = 1;
        i_r4 = i_r2 + i_r3;
        let k = i_r2;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r0 {
            unsafe {
                *p_r0.add(k as usize) = i_r4;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r3 = 2;
        i_r2 = i_r2 + i_r3;
    }
    let mut new_table = Box::new(Table::new());
    t_r3 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r4 = 300;
    i_r5 = 0;
    loop {
        b_r6 = i_r5 < i_r4;
        if !b_r6 {
            break;
        }
        i_r6 = 0;
        let lim = i_r5;
        if lim > 0 {
            let cap_limit = lim as usize;
            let t = unsafe { &mut *t_r3 };
            if cap_limit > t.array.len() {
                t.array.resize(cap_limit, 0);
            }
        }
        let len_r3 = unsafe { (*t_r3).array.len() };
        p_r3 = unsafe { (*t_r3).array.as_mut_ptr() };
        loop {
            b_r7 = i_r6 < i_r5;
            if !b_r7 {
                break;
            }
            i_r7 = 1;
            i_r8 = i_r6 + i_r7;
            let k = i_r6;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r3 {
                unsafe {
                    *p_r3.add(k as usize) = i_r8;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r7 = 1;
            i_r6 = i_r6 + i_r7;
        }
        i_r7 = 1;
        i_r5 = i_r5 + i_r7;
    }
    let mut new_table = Box::new(Table::new());
    t_r6 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r7 = 250;
    i_r8 = 0;
    let lim = i_r7;
    if lim > 0 {
        let cap_limit = lim as usize;
        let t = unsafe { &mut *t_r6 };
        if cap_limit > t.array.len() {
            t.array.resize(cap_limit, 0);
        }
    }
    let len_r6 = unsafe { (*t_r6).array.len() };
    p_r6 = unsafe { (*t_r6).array.as_mut_ptr() };
    loop {
        b_r9 = i_r8 < i_r7;
        if !b_r9 {
            break;
        }
        i_r9 = 5;
        let k = i_r8;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r6 {
            unsafe {
                *p_r6.add(k as usize) = i_r9;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r9 = 1;
        i_r8 = i_r8 + i_r9;
    }
    i_r9 = 0;
    let lim = i_r7;
    if lim > 0 {
        let cap_limit = lim as usize;
        let t = unsafe { &mut *t_r6 };
        if cap_limit > t.array.len() {
            t.array.resize(cap_limit, 0);
        }
    }
    let len_r6 = unsafe { (*t_r6).array.len() };
    p_r6 = unsafe { (*t_r6).array.as_mut_ptr() };
    loop {
        b_r10 = i_r9 < i_r7;
        if !b_r10 {
            break;
        }
        let k = i_r9;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        i_r10 = if (k as usize) < len_r6 {
            unsafe { *p_r6.add(k as usize) }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        };
        i_r11 = 1;
        i_r12 = i_r10 + i_r11;
        let k = i_r9;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r6 {
            unsafe {
                *p_r6.add(k as usize) = i_r12;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r10 = 1;
        i_r9 = i_r9 + i_r10;
    }
    let mut new_table = Box::new(Table::new());
    t_r10 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r11 = 0;
    loop {
        i_r12 = 30;
        b_r13 = i_r11 < i_r12;
        if !b_r13 {
            break;
        }
        i_r12 = 1;
        let k = i_r11;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r10 };
        if idx >= t.array.len() {
            if idx == t.array.len() {
                t.array.push(0);
            } else {
                t.array.resize(idx + 1, 0);
            }
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = i_r12;
        }
        i_r12 = 1;
        i_r11 = i_r11 + i_r12;
    }
    i_r12 = 30;
    i_r13 = 60;
    let k = i_r12;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r10 };
    if idx >= t.array.len() {
        if idx == t.array.len() {
            t.array.push(0);
        } else {
            t.array.resize(idx + 1, 0);
        }
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r13;
    }
    i_r12 = 60;
    i_r13 = 0;
    loop {
        let k = i_r13;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &*t_r10 };
        i_r14 = if idx < t.array.len() {
            unsafe { *t.array.get_unchecked(idx) }
        } else {
            0
        };
        b_r15 = i_r14 < i_r12;
        if !b_r15 {
            break;
        }
        i_r14 = 1;
        i_r13 = i_r13 + i_r14;
    }
    let mut new_table = Box::new(Table::new());
    t_r14 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r15 = 0;
    let k = i_r15;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r14 };
    if idx >= t.array.len() {
        if idx == t.array.len() {
            t.array.push(0);
        } else {
            t.array.resize(idx + 1, 0);
        }
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r13;
    }
    let mut new_table = Box::new(Table::new());
    t_r15 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r16 = 200;
    i_r17 = 3;
    i_r18 = 0;
    let lim = i_r16;
    if lim > 0 {
        let cap_limit = lim as usize;
        let t = unsafe { &mut *t_r15 };
        if cap_limit > t.array.len() {
            t.array.resize(cap_limit, 0);
        }
    }
    let len_r15 = unsafe { (*t_r15).array.len() };
    p_r15 = unsafe { (*t_r15).array.as_mut_ptr() };
    loop {
        b_r19 = i_r18 < i_r16;
        if !b_r19 {
            break;
        }
        i_r19 = i_r18;
        i_r20 = 0;
        loop {
            b_r21 = i_r20 < i_r17;
            if !b_r21 {
                break;
            }
            i_r21 = 7;
            let k = i_r19;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r15 {
                unsafe {
                    *p_r15.add(k as usize) = i_r21;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r21 = 1;
            i_r20 = i_r20 + i_r21;
        }
        i_r21 = 1;
        i_r18 = i_r18 + i_r21;
    }
    let mut new_table = Box::new(Table::new());
    t_r19 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r20 = 150;
    i_r21 = 0;
    loop {
        b_r22 = i_r21 < i_r20;
        if !b_r22 {
            break;
        }
        i_r22 = i_r21;
        let lim = i_r20;
        if lim > 0 {
            let cap_limit = lim as usize;
            let t = unsafe { &mut *t_r19 };
            if cap_limit > t.array.len() {
                t.array.resize(cap_limit, 0);
            }
        }
        let len_r19 = unsafe { (*t_r19).array.len() };
        p_r19 = unsafe { (*t_r19).array.as_mut_ptr() };
        loop {
            b_r23 = i_r22 < i_r20;
            if !b_r23 {
                break;
            }
            i_r23 = 1;
            i_r24 = i_r22 + i_r23;
            let k = i_r22;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r19 {
                unsafe {
                    *p_r19.add(k as usize) = i_r24;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r23 = 1;
            i_r22 = i_r22 + i_r23;
        }
        i_r23 = 1;
        i_r21 = i_r21 + i_r23;
    }
    let mut new_table = Box::new(Table::new());
    t_r22 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r23 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r24 = 180;
    i_r25 = 0;
    let lim = i_r24;
    if lim > 0 {
        let cap_limit = lim as usize;
        let t = unsafe { &mut *t_r22 };
        if cap_limit > t.array.len() {
            t.array.resize(cap_limit, 0);
        }
    }
    let len_r22 = unsafe { (*t_r22).array.len() };
    p_r22 = unsafe { (*t_r22).array.as_mut_ptr() };
    loop {
        b_r26 = i_r25 < i_r24;
        if !b_r26 {
            break;
        }
        i_r26 = 1;
        i_r27 = i_r25 + i_r26;
        let k = i_r25;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r22 {
            unsafe {
                *p_r22.add(k as usize) = i_r27;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r26 = 1;
        i_r25 = i_r25 + i_r26;
    }
    i_r26 = 0;
    let lim = i_r24;
    if lim > 0 {
        let cap_limit = lim as usize;
        let t = unsafe { &mut *t_r22 };
        if cap_limit > t.array.len() {
            t.array.resize(cap_limit, 0);
        }
    }
    let len_r22 = unsafe { (*t_r22).array.len() };
    p_r22 = unsafe { (*t_r22).array.as_mut_ptr() };
    loop {
        b_r27 = i_r26 < i_r24;
        if !b_r27 {
            break;
        }
        i_r28 = 1;
        i_r29 = i_r24 - i_r28;
        i_r27 = i_r29 - i_r26;
        let k = i_r26;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        i_r28 = if (k as usize) < len_r22 {
            unsafe { *p_r22.add(k as usize) }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        };
        let k = i_r27;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r23 };
        if idx >= t.array.len() {
            if idx == t.array.len() {
                t.array.push(0);
            } else {
                t.array.resize(idx + 1, 0);
            }
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = i_r28;
        }
        i_r28 = 1;
        i_r26 = i_r26 + i_r28;
    }
    let mut new_table = Box::new(Table::new());
    t_r27 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r28 = 120;
    i_r29 = 0;
    let lim = i_r28;
    if lim > 0 {
        let cap_limit = lim as usize;
        let t = unsafe { &mut *t_r27 };
        if cap_limit > t.array.len() {
            t.array.resize(cap_limit, 0);
        }
    }
    let len_r27 = unsafe { (*t_r27).array.len() };
    p_r27 = unsafe { (*t_r27).array.as_mut_ptr() };
    loop {
        b_r30 = i_r29 < i_r28;
        if !b_r30 {
            break;
        }
        i_r30 = 1;
        i_r31 = i_r29 + i_r30;
        let k = i_r29;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r27 {
            unsafe {
                *p_r27.add(k as usize) = i_r31;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r30 = 1;
        i_r29 = i_r29 + i_r30;
    }
    i_r30 = 0;
    i_r31 = 0;
    let lim = i_r28;
    if lim > 0 {
        let cap_limit = lim as usize;
        let t = unsafe { &mut *t_r27 };
        if cap_limit > t.array.len() {
            t.array.resize(cap_limit, 0);
        }
    }
    let len_r27 = unsafe { (*t_r27).array.len() };
    p_r27 = unsafe { (*t_r27).array.as_mut_ptr() };
    loop {
        b_r32 = i_r31 < i_r28;
        if !b_r32 {
            break;
        }
        let k = i_r31;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        i_r32 = if (k as usize) < len_r27 {
            unsafe { *p_r27.add(k as usize) }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        };
        i_r30 = i_r30 + i_r32;
        i_r32 = 1;
        i_r31 = i_r31 + i_r32;
    }
    let mut new_table = Box::new(Table::new());
    t_r32 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r33 = 0;
    let k = i_r33;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r32 };
    if idx >= t.array.len() {
        if idx == t.array.len() {
            t.array.push(0);
        } else {
            t.array.resize(idx + 1, 0);
        }
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r30;
    }
    let mut new_table = Box::new(Table::new());
    t_r33 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r35 = 0;
    i_r36 = 1;
    b_r34 = i_r35 < i_r36;
    i_r35 = 0;
    loop {
        if !b_r34 {
            break;
        }
        i_r36 = 100;
        let k = i_r35;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r33 };
        if idx >= t.array.len() {
            if idx == t.array.len() {
                t.array.push(0);
            } else {
                t.array.resize(idx + 1, 0);
            }
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = i_r36;
        }
        i_r36 = 0;
        i_r37 = 0;
        b_r34 = i_r36 < i_r37;
        i_r36 = 1;
        i_r35 = i_r35 + i_r36;
    }
    let mut new_table = Box::new(Table::new());
    t_r36 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r37 = 200;
    i_r38 = 100;
    let lim = i_r37;
    if lim > 0 {
        let cap_limit = lim as usize;
        let t = unsafe { &mut *t_r36 };
        if cap_limit > t.array.len() {
            t.array.resize(cap_limit, 0);
        }
    }
    let len_r36 = unsafe { (*t_r36).array.len() };
    p_r36 = unsafe { (*t_r36).array.as_mut_ptr() };
    loop {
        b_r39 = i_r38 < i_r37;
        if !b_r39 {
            break;
        }
        i_r39 = 100;
        i_r40 = i_r38 - i_r39;
        let k = i_r38;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r36 {
            unsafe {
                *p_r36.add(k as usize) = i_r40;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r39 = 1;
        i_r38 = i_r38 + i_r39;
    }
    i_r39 = 350;
    i_r40 = 200;
    let lim = i_r39;
    if lim > 0 {
        let cap_limit = lim as usize;
        let t = unsafe { &mut *t_r36 };
        if cap_limit > t.array.len() {
            t.array.resize(cap_limit, 0);
        }
    }
    let len_r36 = unsafe { (*t_r36).array.len() };
    p_r36 = unsafe { (*t_r36).array.as_mut_ptr() };
    loop {
        b_r41 = i_r40 < i_r39;
        if !b_r41 {
            break;
        }
        let k = i_r40;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r36 {
            unsafe {
                *p_r36.add(k as usize) = i_r40;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r41 = 1;
        i_r40 = i_r40 + i_r41;
    }
    let mut new_table = Box::new(Table::new());
    t_r41 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r42 = 0;
    i_r43 = 150;
    let k = i_r43;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r36 };
    i_r44 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = i_r42;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r41 };
    if idx >= t.array.len() {
        if idx == t.array.len() {
            t.array.push(0);
        } else {
            t.array.resize(idx + 1, 0);
        }
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r44;
    }
    i_r42 = 1;
    i_r43 = 250;
    let k = i_r43;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r36 };
    i_r44 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = i_r42;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r41 };
    if idx >= t.array.len() {
        if idx == t.array.len() {
            t.array.push(0);
        } else {
            t.array.resize(idx + 1, 0);
        }
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r44;
    }
    i_r42 = 2;
    i_r43 = 349;
    let k = i_r43;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r36 };
    i_r44 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = i_r42;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r41 };
    if idx >= t.array.len() {
        if idx == t.array.len() {
            t.array.push(0);
        } else {
            t.array.resize(idx + 1, 0);
        }
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r44;
    }
    i_r42 = 3;
    i_r43 = 99;
    let k = i_r43;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r36 };
    i_r44 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = i_r42;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r41 };
    if idx >= t.array.len() {
        if idx == t.array.len() {
            t.array.push(0);
        } else {
            t.array.resize(idx + 1, 0);
        }
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r44;
    }
    let mut new_table = Box::new(Table::new());
    t_r42 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r43 = 60;
    i_r44 = 0;
    loop {
        b_r45 = i_r44 < i_r43;
        if !b_r45 {
            break;
        }
        i_r45 = 0;
        loop {
            b_r46 = i_r45 < i_r43;
            if !b_r46 {
                break;
            }
            i_r46 = 0;
            let lim = i_r43;
            if lim > 0 {
                let cap_limit = lim as usize;
                let t = unsafe { &mut *t_r42 };
                if cap_limit > t.array.len() {
                    t.array.resize(cap_limit, 0);
                }
            }
            let len_r42 = unsafe { (*t_r42).array.len() };
            p_r42 = unsafe { (*t_r42).array.as_mut_ptr() };
            loop {
                b_r47 = i_r46 < i_r43;
                if !b_r47 {
                    break;
                }
                i_r47 = i_r46 + i_r45;
                let k = i_r46;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r42 {
                    unsafe {
                        *p_r42.add(k as usize) = i_r47;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
                i_r47 = 1;
                i_r46 = i_r46 + i_r47;
            }
            i_r47 = 1;
            i_r45 = i_r45 + i_r47;
        }
        i_r46 = 1;
        i_r44 = i_r44 + i_r46;
    }
    let mut new_table = Box::new(Table::new());
    t_r45 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r46 = 80;
    i_r47 = 0;
    loop {
        b_r48 = i_r47 < i_r46;
        if !b_r48 {
            break;
        }
        i_r48 = i_r47;
        i_r49 = 1;
        let k = i_r48;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r45 };
        if idx >= t.array.len() {
            if idx == t.array.len() {
                t.array.push(0);
            } else {
                t.array.resize(idx + 1, 0);
            }
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = i_r49;
        }
        i_r49 = i_r48;
        i_r50 = 0;
        i_r49 = i_r49 + i_r50;
        i_r50 = 2;
        let k = i_r49;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r45 };
        if idx >= t.array.len() {
            if idx == t.array.len() {
                t.array.push(0);
            } else {
                t.array.resize(idx + 1, 0);
            }
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = i_r50;
        }
        i_r50 = i_r47;
        i_r51 = 3;
        let k = i_r50;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r45 };
        if idx >= t.array.len() {
            if idx == t.array.len() {
                t.array.push(0);
            } else {
                t.array.resize(idx + 1, 0);
            }
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = i_r51;
        }
        i_r51 = 1;
        i_r47 = i_r47 + i_r51;
    }
    let mut new_table = Box::new(Table::new());
    t_r48 = &mut *new_table as *mut Table;
    tables.push(new_table);
    t_r49 = t_r48;
    i_r50 = 90;
    i_r51 = 0;
    let lim = i_r50;
    if lim > 0 {
        let cap_limit = lim as usize;
        let t = unsafe { &mut *t_r48 };
        if cap_limit > t.array.len() {
            t.array.resize(cap_limit, 0);
        }
    }
    let len_r48 = unsafe { (*t_r48).array.len() };
    p_r48 = unsafe { (*t_r48).array.as_mut_ptr() };
    let lim = i_r50;
    if lim > 0 {
        let cap_limit = lim as usize;
        let t = unsafe { &mut *t_r49 };
        if cap_limit > t.array.len() {
            t.array.resize(cap_limit, 0);
        }
    }
    let len_r49 = unsafe { (*t_r49).array.len() };
    p_r49 = unsafe { (*t_r49).array.as_mut_ptr() };
    loop {
        b_r52 = i_r51 < i_r50;
        if !b_r52 {
            break;
        }
        i_r52 = 1;
        i_r53 = i_r51 + i_r52;
        let k = i_r51;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r48 {
            unsafe {
                *p_r48.add(k as usize) = i_r53;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r52 = 2;
        i_r53 = i_r51 + i_r52;
        let k = i_r51;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r49 {
            unsafe {
                *p_r49.add(k as usize) = i_r53;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r52 = 1;
        i_r51 = i_r51 + i_r52;
    }

    tables
}

pub const STATS: &str = "fast_sets=13;fast_gets=3;dyn_sets=13;dyn_gets=5;hoists=15;hoist_ctx=0,1,0,0,0,1,0,0,0,0,0,0,2,0,0";
```
