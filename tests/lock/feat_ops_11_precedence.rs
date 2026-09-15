// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
    let mut i_r4 = 0i64;
    let mut i_r5 = 0i64;
    let mut i_r6 = 0i64;
    let mut b_r23 = false;
    let mut b_r24 = false;
    let mut b_r25 = false;
    let mut t_r28: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r0 = 0;
    loop {
        b_r23 = i_r0 < 1;
        if b_r23 {
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    i_r1 = i_r0 + 3;
    i_r2 = i_r0 + 1;
    i_r3 = i_r2 * 3;
    i_r2 = i_r0 * 2;
    i_r4 = 10 - i_r2;
    i_r2 = i_r4 - 3;
    i_r4 = -i_r0;
    i_r5 = i_r4 * 5;
    i_r4 = i_r0 + 5;
    i_r6 = -i_r4;
    b_r23 = 5 < i_r0;
    b_r24 = !b_r23;
    b_r23 = !b_r24;
    b_r25 = !b_r23;
    let mut new_table = Box::new(Table::new());
    t_r28 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r4 = i_r1 * 100000;
    i_r1 = i_r3 * 10000;
    i_r3 = i_r4 + i_r1;
    i_r1 = i_r2 * 1000;
    i_r2 = i_r3 + i_r1;
    i_r1 = i_r2 + 120;
    i_r2 = i_r1 + 4;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r28 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r2;
    }
    i_r2 = i_r5 * 100;
    i_r5 = i_r2 + i_r6;
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r28 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r5;
    }
    if b_r24 {
        i_r5 = 1;
    } else {
        i_r5 = 0;
    }
    if b_r25 {
        i_r2 = i_r5 + 2;
        i_r6 = i_r2;
    } else {
        i_r6 = i_r5;
    }
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r28 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r6;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=3;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=37;consts_b=0;consts_f=0;consts_s=0";
