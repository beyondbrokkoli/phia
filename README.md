# phia

An experimental Ahead-Of-Time (AOT) Lua-to-Rust transpiler.

Supported features: while, do, end, local, integers, integer-key tables with integers as values

## Input
```lua
-- main.lua
-- Tests scope shadowing, register recycling, and optimizer fallbacks.

local size = 50000
local data_a = {}
local data_b = {}

-- 1. Initialize data_a using the FAST PATH (Optimizer will hoist this)
local i = 0
while i < size do
    data_a[i] = i
    i = i + 1
end

-- 2. The Torture Loop
local iter = 0
while iter < 50000 do
    local idx = 0

    -- Register Exhaustion Test:
    -- This creates a massive temporary expression tree.
    local crazy_math = 0 + 1 + 2 + 3 + 4 + 5 - 15 + iter

    while idx < size do
        -- Scope Shadowing Test:
        -- 'crazy_math' exists outside, but we declare it AGAIN inside.
        local crazy_math = data_a[idx]

        -- Optimizer Bypass Test:
        -- 'offset_idx' is NOT the loop variable. The compiler cannot prove
        -- the bounds ahead of time. It MUST fall back to the dynamic resize()
        -- path instead of the C-style raw pointer path.
        local offset_idx = idx + 2

        -- Write to a new table using the un-hoistable index
        data_b[offset_idx] = crazy_math - 1 + 1

        idx = idx + 1
    end

    iter = iter + 1
end
```

## Output
```rust
use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
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
    let mut b_r4 = false;
    let mut b_r6 = false;
    let mut b_r7 = false;
    let mut t_r1: *mut Table = std::ptr::null_mut();
    let mut p_r1: *mut i64 = std::ptr::null_mut();
    let mut t_r2: *mut Table = std::ptr::null_mut();
    let mut p_r2: *mut i64 = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r0 = 50000;
    let mut new_table = Box::new(Table::new());
    t_r1 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r2 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r3 = 0;
    let cap_limit = i_r0 as usize;
    let t = unsafe { &mut *t_r1 };
    if cap_limit > t.array.len() {
        t.array.resize(cap_limit, 0);
    }
    p_r1 = unsafe { (*t_r1).array.as_mut_ptr() };
    loop {
    b_r4 = i_r3 < i_r0;
    if !b_r4 { break; }
    unsafe { *p_r1.add(i_r3 as usize) = i_r3; }
    i_r4 = 1;
    i_r3 = i_r3 + i_r4;
    }
    i_r4 = 0;
    let cap_limit = i_r0 as usize;
    let t = unsafe { &mut *t_r1 };
    if cap_limit > t.array.len() {
        t.array.resize(cap_limit, 0);
    }
    p_r1 = unsafe { (*t_r1).array.as_mut_ptr() };
    loop {
    i_r5 = 50000;
    b_r6 = i_r4 < i_r5;
    if !b_r6 { break; }
    i_r5 = 0;
    i_r7 = 0;
    i_r8 = 1;
    i_r9 = i_r7 + i_r8;
    i_r10 = 2;
    i_r11 = i_r9 + i_r10;
    i_r12 = 3;
    i_r13 = i_r11 + i_r12;
    i_r14 = 4;
    i_r15 = i_r13 + i_r14;
    i_r16 = 5;
    i_r17 = i_r15 + i_r16;
    i_r18 = 15;
    i_r19 = i_r17 - i_r18;
    i_r6 = i_r19 + i_r4;
    loop {
    b_r7 = i_r5 < i_r0;
    if !b_r7 { break; }
    i_r7 = unsafe { *p_r1.add(i_r5 as usize) };
    i_r9 = 2;
    i_r8 = i_r5 + i_r9;
    i_r9 = 1;
    i_r10 = i_r7 - i_r9;
    i_r11 = 1;
    i_r12 = i_r10 + i_r11;
    let idx = i_r8 as usize;
    let t = unsafe { &mut *t_r2 };
    if idx >= t.array.len() {
        if idx == t.array.len() {
            t.array.push(0);
        } else {
            t.array.resize(idx + 1, 0);
        }
    }
    unsafe { *t.array.get_unchecked_mut(idx) = i_r12; }
    i_r9 = 1;
    i_r5 = i_r5 + i_r9;
    }
    i_r7 = 1;
    i_r4 = i_r4 + i_r7;
    }

    tables
}
```
