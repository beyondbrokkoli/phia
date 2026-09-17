// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
    let mut b_r6 = false;
    let mut b_r7 = false;
    let mut t_r12: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r12 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r12 };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = 1;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r12 };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = 1;
    }
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r12 };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = 1;
    }
    i_r0 = 3;
    loop {
        b_r6 = 0 < i_r0;
        if b_r6 {
            i_r2 = i_r0 - 1;
            let k = i_r2;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &*t_r12 };
            i_r3 = if idx < t.as_int().len() {
                unsafe { *t.as_int().get_unchecked(idx) }
            } else {
                0
            };
            b_r6 = 0 < i_r3;
            b_r7 = b_r6;
        } else {
            b_r7 = false;
        }
        if b_r7 {
            i_r0 = i_r0 - 1;
        } else {
            break;
        }
    }
    println!("{}\t{}", "walk", i_r0);
    i_r1 = 0;
    loop {
        b_r7 = 0 < i_r1;
        if b_r7 {
            i_r3 = i_r1 - 1;
            let k = i_r3;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &*t_r12 };
            i_r2 = if idx < t.as_int().len() {
                unsafe { *t.as_int().get_unchecked(idx) }
            } else {
                0
            };
            b_r7 = 0 < i_r2;
            b_r6 = b_r7;
        } else {
            b_r6 = false;
        }
        if b_r6 {
            i_r1 = i_r1 - 1;
        } else {
            break;
        }
    }
    println!("{}\t{}", "zero", i_r1);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=3;dyn_gets=2;hoists=0;hoist_ctx=;consts_i=16;consts_b=2;consts_f=0;consts_s=0";
