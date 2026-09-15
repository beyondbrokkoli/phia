// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
    let mut i_r4 = 0i64;
    let mut b_r8 = false;
    let mut b_r9 = false;
    let mut t_r13: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r0 = 0;
    i_r1 = 0;
    loop {
        b_r8 = i_r0 <= 3;
        if b_r8 {
            i_r1 = i_r1 + i_r0;
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    b_r8 = false;
    i_r0 = 0;
    loop {
        b_r9 = !b_r8;
        if b_r9 {
            i_r0 = i_r0 + 1;
            b_r9 = i_r0 >= 2;
            if b_r9 {
                b_r8 = true;
            }
        } else {
            break;
        }
    }
    i_r2 = 4;
    i_r3 = 0;
    loop {
        b_r9 = i_r2 >= 1;
        if b_r9 {
            i_r4 = i_r3 * 10;
            i_r3 = i_r4 + i_r2;
            i_r2 = i_r2 - 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r13 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r4 = i_r1 * 100;
    i_r1 = i_r4 + i_r0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r13 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r1;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r13 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r3;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=2;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=15;consts_b=2;consts_f=0;consts_s=0";
