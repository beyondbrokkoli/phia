// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut b_r3 = false;
    let mut b_r4 = false;
    let mut b_r5 = false;
    let mut t_r11: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    b_r3 = false;
    i_r0 = 0;
    loop {
        if true {
            b_r4 = b_r3;
        } else {
            b_r4 = false;
        }
        b_r5 = !b_r4;
        if b_r5 {
            i_r0 = i_r0 + 1;
            b_r4 = 4 < i_r0;
            if b_r4 {
                b_r3 = true;
            }
        } else {
            break;
        }
    }
    println!("{}\t{}", "notchain", i_r0);
    let mut new_table = Box::new(Table::new_bool());
    t_r11 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r11 };
    if idx >= t.as_bool_mut().len() {
        t.as_bool_mut().resize(idx + 1, false);
    }
    unsafe {
        *t.as_bool_mut().get_unchecked_mut(idx) = true;
    }
    i_r1 = 0;
    loop {
        let k = 0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &*t_r11 };
        b_r5 = if idx < t.as_bool().len() {
            unsafe { *t.as_bool().get_unchecked(idx) }
        } else {
            false
        };
        if b_r5 {
            b_r5 = i_r1 < 2;
            b_r4 = b_r5;
        } else {
            b_r4 = false;
        }
        if b_r4 {
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    println!("{}\t{}", "btab", i_r1);
    i_r0 = 0;
    loop {
        b_r4 = i_r0 < 10;
        if b_r4 {
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    println!("{}\t{}", "plain", i_r0);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=1;hoists=0;hoist_ctx=;consts_i=11;consts_b=6;consts_f=0;consts_s=0";
