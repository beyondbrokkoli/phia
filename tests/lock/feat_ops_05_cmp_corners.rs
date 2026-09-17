// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
    let mut b_r17 = false;
    let mut t_r21: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r0 = 0;
    loop {
        b_r17 = i_r0 < 1;
        if b_r17 {
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    i_r1 = 9223372036854775806 + i_r0;
    if true {
        i_r0 = 1;
    } else {
        i_r0 = 0;
    }
    b_r17 = 0 <= i_r1;
    if b_r17 {
        i_r2 = i_r0 + 10;
        i_r3 = i_r2;
    } else {
        i_r3 = i_r0;
    }
    b_r17 = i_r1 >= i_r1;
    if b_r17 {
        i_r2 = i_r3 + 100;
        i_r0 = i_r2;
    } else {
        i_r0 = i_r3;
    }
    b_r17 = 9223372036854775806 < i_r1;
    if b_r17 {
        i_r2 = i_r0 + 1000;
        i_r3 = i_r2;
    } else {
        i_r3 = i_r0;
    }
    if true {
        i_r2 = i_r3 + 10000;
        i_r0 = i_r2;
    } else {
        i_r0 = i_r3;
    }
    if true {
        i_r2 = i_r0 + 100000;
        i_r3 = i_r2;
    } else {
        i_r3 = i_r0;
    }
    if true {
        i_r2 = i_r3 + 1000000;
        i_r0 = i_r2;
    } else {
        i_r0 = i_r3;
    }
    if true {
        i_r2 = i_r0 + 10000000;
        i_r3 = i_r2;
    } else {
        i_r3 = i_r0;
    }
    let mut new_table = Box::new(Table::new());
    t_r21 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r21 };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = i_r3;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=18;consts_b=5;consts_f=4;consts_s=0";
