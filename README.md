# i_we

**Intermediate / Working Environment**

An experimental Ahead-Of-Time (AOT) Lua-to-Rust transpiler architected to isolate and evaluate the maximum theoretical optimization ceiling of Lua logic mapped to LLVM IR.

## Input
```lua
-- test.lua
local table_size = 200
local outer_loops = 50000000
local data = {}
local i = 0
while i < table_size do
    data[i] = i
    i = i + 1
end
local iter = 0
while iter < outer_loops do
    local j = 0
    while j < table_size do
        local current = data[j]
        data[j] = current + iter - j
        j = j + 1
    end
    iter = iter + 1
end
```

## Output
```rust
use crate::memory::{Value, Table};

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r3 = 0i64;
    let mut i_r4 = 0i64;
    let mut i_r5 = 0i64;
    let mut i_r6 = 0i64;
    let mut i_r7 = 0i64;
    let mut i_r8 = 0i64;
    let mut b_r4 = false;
    let mut b_r5 = false;
    let mut b_r6 = false;
    let mut t_r2: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r0 = 200;
    i_r1 = 5000000;
    let mut new_table = Box::new(Table::new());
    t_r2 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r3 = 0;
    let cap_limit = i_r0 as usize;
    let t = unsafe { &mut *t_r2 };
    if cap_limit > t.array.len() {
        t.array.resize(cap_limit, Value::nil());
    }
    loop {
    b_r4 = i_r3 < i_r0;
    if !b_r4 { break; }
    i_r4 = i_r3;
    i_r5 = i_r3;
    let idx = i_r4 as usize;
    let t = unsafe { &mut *t_r2 };
    // HOT PATH: pure unchecked assignment, capacity proven!
    unsafe { *t.array.get_unchecked_mut(idx) = Value::integer(i_r5 as i32); }
    i_r4 = 1;
    i_r3 = i_r3 + i_r4;
    }
    i_r4 = 0;
    let cap_limit = i_r0 as usize;
    let t = unsafe { &mut *t_r2 };
    if cap_limit > t.array.len() {
        t.array.resize(cap_limit, Value::nil());
    }
    loop {
    b_r5 = i_r4 < i_r1;
    if !b_r5 { break; }
    i_r5 = 0;
    loop {
    b_r6 = i_r5 < i_r0;
    if !b_r6 { break; }
    i_r7 = i_r5;
    let idx = i_r7 as usize;
    // Explicitly create a safe reference first to satisfy the borrow checker
    let t = unsafe { &*t_r2 };
    let raw_val = unsafe { *t.array.get_unchecked(idx) };
    i_r6 = (raw_val.0 & 0xFFFF_FFFF) as i32 as i64;
    i_r7 = i_r5;
    i_r8 = i_r6 + i_r4;
    i_r8 = i_r8 - i_r5;
    let idx = i_r7 as usize;
    let t = unsafe { &mut *t_r2 };
    // HOT PATH: pure unchecked assignment, capacity proven!
    unsafe { *t.array.get_unchecked_mut(idx) = Value::integer(i_r8 as i32); }
    i_r7 = 1;
    i_r5 = i_r5 + i_r7;
    }
    i_r6 = 1;
    i_r4 = i_r4 + i_r6;
    }

    tables
}
```
