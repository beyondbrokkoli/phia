# phia

An experimental Ahead-Of-Time (AOT) Lua-to-Rust transpiler.

Supported features: while, do, end, local, integers, integer-key tables with integers as values, addition, subtraction, less than

## Input (main.lua)
```lua
-- main.lua
-- Features: Strict Phia subset (integers, tables, local, while, +, -)

-- PHASE 1: The Autobahn (Massive Linear Allocation)
-- Goal: Test pure `SetTableFast` throughput and basic loop hoisting.
-- Size: 50 Million iterations.
local p1_table = {}
local p1_size = 50000000
local p1_i = 0
while p1_i < p1_size do
    p1_table[p1_i] = p1_i
    p1_i = p1_i + 1
end


-- PHASE 2: The Alias Trap (Safeguards #12 & #13)
-- Goal: Force the optimizer to recognize aliases and safely disable
-- fast-paths mid-loop without crashing or hanging.
-- Size: 10 Million iterations.
local p2_table = {}
local p2_alias = p2_table
local p2_size = 10000000
local p2_i = 0
while p2_i < p2_size do
    -- Fast write via original
    p2_table[p2_i] = p2_i + 100

    -- The trap: dynamic write via alias (forces clobber)
    local j = p2_i
    p2_alias[j + 1000] = 1

    p2_i = p2_i + 1
end


-- PHASE 3: The Dynamic Resize Minefield (Safeguard #8)
-- Goal: Interleave sequential writes with out-of-bounds jumps to force
-- the arena to dynamically grow, testing pointer invalidation.
-- Size: 20 Million iterations (40 Million total table writes).
local p3_table = {}
local p3_size = 20000000
local p3_i = 0
while p3_i < p3_size do
    -- Expected fast set
    p3_table[p3_i] = 7

    -- OOB jump (disables hoisting for this write, forces arena resize)
    local jump = p3_i + 30000
    p3_table[jump] = 8

    p3_i = p3_i + 1
end


-- PHASE 4: The Matrix (Safeguards #4 & #9)
-- Goal: Deep loop depth hoisting and register survival across massive iterations.
-- Size: 150,000 * 150,000 = 22.5 BILLION iterations.
local p4_src = {}
local p4_dst = {}
local p4_size = 150000

-- Setup source table
local p4_setup = 0
while p4_setup < p4_size do
    p4_src[p4_setup] = p4_setup
    p4_setup = p4_setup + 1
end

-- The 22.5 Billion Iteration Meat Grinder
local outer = 0
while outer < p4_size do
    local inner = 0
    -- Fake math noise to ensure the register allocator doesn't just sleep
    local math_noise = outer + 1 - 1 + 5 - 5

    while inner < p4_size do
        -- Fast Get from src, Fast Set to dst (testing context depth 1)
        local val = p4_src[inner]
        local offset = inner + 2
        p4_dst[offset] = val + math_noise

        inner = inner + 1
    end
    outer = outer + 1
end
```

## Output (target/release/build/phia-HASH/out/baked_native.rs)
```rust
use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
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
    let mut b_r3 = false;
    let mut b_r7 = false;
    let mut b_r10 = false;
    let mut b_r14 = false;
    let mut b_r15 = false;
    let mut b_r17 = false;
    let mut t_r0: *mut Table = std::ptr::null_mut();
    let mut p_r0: *mut i64 = std::ptr::null_mut();
    let mut t_r3: *mut Table = std::ptr::null_mut();
    let mut p_r3: *mut i64 = std::ptr::null_mut();
    let mut t_r4: *mut Table = std::ptr::null_mut();
    let mut p_r4: *mut i64 = std::ptr::null_mut();
    let mut t_r7: *mut Table = std::ptr::null_mut();
    let mut p_r7: *mut i64 = std::ptr::null_mut();
    let mut t_r10: *mut Table = std::ptr::null_mut();
    let mut p_r10: *mut i64 = std::ptr::null_mut();
    let mut t_r11: *mut Table = std::ptr::null_mut();
    let mut p_r11: *mut i64 = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r0 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r1 = 50000000;
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
    if !b_r3 { break; }
    let k = i_r2;
if k < 0 { panic!("Runtime Error: Negative index in fast path"); }
if (k as usize) < len_r0 {
unsafe { *p_r0.add(k as usize) = i_r2; }
}
    i_r3 = 1;
    i_r2 = i_r2 + i_r3;
    }
    let mut new_table = Box::new(Table::new());
    t_r3 = &mut *new_table as *mut Table;
    tables.push(new_table);
    t_r4 = t_r3;
    i_r5 = 10000000;
    i_r6 = 0;
    loop {
    b_r7 = i_r6 < i_r5;
    if !b_r7 { break; }
    i_r7 = 100;
    i_r8 = i_r6 + i_r7;
    let k = i_r6;
if k < 0 { panic!("Runtime Error: Negative table index"); }
let idx = k as usize;
let t = unsafe { &mut *t_r3 };
if idx >= t.array.len() {
if idx == t.array.len() {
t.array.push(0);
} else {
t.array.resize(idx + 1, 0);
}
}
unsafe { *t.array.get_unchecked_mut(idx) = i_r8; }
    i_r7 = i_r6;
    i_r8 = 1000;
    i_r9 = i_r7 + i_r8;
    i_r10 = 1;
    let k = i_r9;
if k < 0 { panic!("Runtime Error: Negative table index"); }
let idx = k as usize;
let t = unsafe { &mut *t_r4 };
if idx >= t.array.len() {
if idx == t.array.len() {
t.array.push(0);
} else {
t.array.resize(idx + 1, 0);
}
}
unsafe { *t.array.get_unchecked_mut(idx) = i_r10; }
    i_r8 = 1;
    i_r6 = i_r6 + i_r8;
    }
    let mut new_table = Box::new(Table::new());
    t_r7 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r8 = 20000000;
    i_r9 = 0;
    loop {
    b_r10 = i_r9 < i_r8;
    if !b_r10 { break; }
    i_r10 = 7;
    let k = i_r9;
if k < 0 { panic!("Runtime Error: Negative table index"); }
let idx = k as usize;
let t = unsafe { &mut *t_r7 };
if idx >= t.array.len() {
if idx == t.array.len() {
t.array.push(0);
} else {
t.array.resize(idx + 1, 0);
}
}
unsafe { *t.array.get_unchecked_mut(idx) = i_r10; }
    i_r11 = 30000;
    i_r10 = i_r9 + i_r11;
    i_r11 = 8;
    let k = i_r10;
if k < 0 { panic!("Runtime Error: Negative table index"); }
let idx = k as usize;
let t = unsafe { &mut *t_r7 };
if idx >= t.array.len() {
if idx == t.array.len() {
t.array.push(0);
} else {
t.array.resize(idx + 1, 0);
}
}
unsafe { *t.array.get_unchecked_mut(idx) = i_r11; }
    i_r11 = 1;
    i_r9 = i_r9 + i_r11;
    }
    let mut new_table = Box::new(Table::new());
    t_r10 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r11 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r12 = 150000;
    i_r13 = 0;
    let lim = i_r12;
if lim > 0 {
let cap_limit = lim as usize;
let t = unsafe { &mut *t_r10 };
if cap_limit > t.array.len() {
t.array.resize(cap_limit, 0);
}
}
    let len_r10 = unsafe { (*t_r10).array.len() };
p_r10 = unsafe { (*t_r10).array.as_mut_ptr() };
    loop {
    b_r14 = i_r13 < i_r12;
    if !b_r14 { break; }
    let k = i_r13;
if k < 0 { panic!("Runtime Error: Negative index in fast path"); }
if (k as usize) < len_r10 {
unsafe { *p_r10.add(k as usize) = i_r13; }
}
    i_r14 = 1;
    i_r13 = i_r13 + i_r14;
    }
    i_r14 = 0;
    loop {
    b_r15 = i_r14 < i_r12;
    if !b_r15 { break; }
    i_r15 = 0;
    i_r17 = 1;
    i_r18 = i_r14 + i_r17;
    i_r19 = 1;
    i_r20 = i_r18 - i_r19;
    i_r21 = 5;
    i_r22 = i_r20 + i_r21;
    i_r23 = 5;
    i_r16 = i_r22 - i_r23;
    let lim = i_r12;
if lim > 0 {
let cap_limit = lim as usize;
let t = unsafe { &mut *t_r10 };
if cap_limit > t.array.len() {
t.array.resize(cap_limit, 0);
}
}
    let len_r10 = unsafe { (*t_r10).array.len() };
p_r10 = unsafe { (*t_r10).array.as_mut_ptr() };
    loop {
    b_r17 = i_r15 < i_r12;
    if !b_r17 { break; }
    let k = i_r15;
if k < 0 { panic!("Runtime Error: Negative index in fast path"); }
i_r17 = if (k as usize) < len_r10 {
unsafe { *p_r10.add(k as usize) }
} else {
0
};
    i_r19 = 2;
    i_r18 = i_r15 + i_r19;
    i_r19 = i_r17 + i_r16;
    let k = i_r18;
if k < 0 { panic!("Runtime Error: Negative table index"); }
let idx = k as usize;
let t = unsafe { &mut *t_r11 };
if idx >= t.array.len() {
if idx == t.array.len() {
t.array.push(0);
} else {
t.array.resize(idx + 1, 0);
}
}
unsafe { *t.array.get_unchecked_mut(idx) = i_r19; }
    i_r19 = 1;
    i_r15 = i_r15 + i_r19;
    }
    i_r17 = 1;
    i_r14 = i_r14 + i_r17;
    }

    tables
}

pub const STATS: &str = "fast_sets=2;fast_gets=1;dyn_sets=5;dyn_gets=0;hoists=3;hoist_ctx=0,0,1";
```
