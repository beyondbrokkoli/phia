// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut b_r3 = false;
    let mut b_r4 = false;
    let mut b_r5 = false;
    let mut t_r9: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r9 = &mut *new_table as *mut Table;
    tables.push(new_table);
    if false {
        let k = -1;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &*t_r9 };
        i_r0 = if idx < t.as_int().len() {
            unsafe { *t.as_int().get_unchecked(idx) }
        } else {
            0
        };
        b_r3 = 0 < i_r0;
        b_r4 = b_r3;
    } else {
        b_r4 = false;
    }
    if false {
        i_r0 = 10 / 0 - i64::from(10 % 0 != 0 && (10 < 0) != (0 < 0));
        b_r3 = 0 < i_r0;
        b_r5 = b_r3;
    } else {
        b_r5 = false;
    }
    println!("{}\t{}\t{}", "guard", b_r4, b_r5);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r9 };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = 5;
    }
    if true {
        let k = 0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &*t_r9 };
        i_r0 = if idx < t.as_int().len() {
            unsafe { *t.as_int().get_unchecked(idx) }
        } else {
            0
        };
        b_r5 = 0 < i_r0;
        b_r3 = b_r5;
    } else {
        b_r3 = false;
    }
    println!("{}\t{}", "guard2", b_r3);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=2;hoists=0;hoist_ctx=;consts_i=16;consts_b=7;consts_f=0;consts_s=0";
