// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r78 = 0i64;
    let mut i_r79 = 0i64;
    let mut i_r80 = 0i64;
    let mut i_r81 = 0i64;
    let mut i_r82 = 0i64;
    let mut i_r83 = 0i64;
    let mut i_r84 = 0i64;
    let mut i_r85 = 0i64;
    let mut b_r78 = false;
    let mut t_r78: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    if true {
        i_r78 = 2;
    } else {
        i_r78 = 3;
    }
    if true {
        i_r79 = 15;
    } else {
        i_r79 = 10;
    }
    if false {
        i_r80 = 101;
    } else {
        i_r80 = 102;
    }
    if true {
        if false {
            i_r81 = 1;
        } else {
            i_r81 = 2;
        }
    } else {
        i_r81 = 3;
    }
    if false {
        i_r82 = 10;
    } else {
        if false {
            i_r82 = 20;
        } else {
            if true {
                i_r82 = 30;
            } else {
                i_r82 = 40;
            }
        }
    }
    i_r83 = 0;
    i_r84 = 0;
    loop {
        b_r78 = i_r84 < 6;
        if b_r78 {
            b_r78 = i_r84 >= 3;
            if b_r78 {
                i_r85 = i_r83 + i_r84;
                i_r83 = i_r85;
            }
            i_r84 = i_r84 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r78 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r78 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r78;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r78 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r79;
    }
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r78 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r80;
    }
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r78 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r81;
    }
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r78 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r82;
    }
    let k = 5;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r78 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r83;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=6;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=42;consts_b=8;consts_f=0;consts_s=0";
