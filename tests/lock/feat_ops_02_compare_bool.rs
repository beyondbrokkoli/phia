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
    let mut t_r10: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r0 = 0;
    i_r1 = 0;
    loop {
        b_r8 = -4 < i_r0;
        if b_r8 {
            if true {
                i_r2 = i_r1 + 3;
                i_r3 = i_r2;
            } else {
                i_r3 = i_r1;
            }
            b_r8 = true == true;
            if b_r8 {
                i_r2 = i_r3 + 2;
                i_r4 = i_r2;
            } else {
                i_r4 = i_r3;
            }
            if false {
                i_r2 = i_r4 + 1000;
                i_r1 = i_r2;
            } else {
                i_r1 = i_r4;
            }
            i_r0 = i_r0 - 1;
        } else {
            break;
        }
    }
    if true {
        i_r2 = 5;
    } else {
        i_r2 = 0;
    }
    let mut new_table = Box::new(Table::new());
    t_r10 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r10 };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = i_r1;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r10 };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = i_r2;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=2;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=17;consts_b=7;consts_f=0;consts_s=0";
