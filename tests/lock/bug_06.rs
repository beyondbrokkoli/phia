// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r24 = 0i64;
    let mut i_r25 = 0i64;
    let mut i_r26 = 0i64;
    let mut b_r24 = false;
    let mut t_r24: *mut Table = std::ptr::null_mut();
    let mut p_r24: *mut i64 = std::ptr::null_mut();
    let mut len_r24 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r24 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r24 };
    i_r24 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let mut new_table = Box::new(Table::new());
    t_r24 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 5;
    if lim > 0 {
        let t = unsafe { &mut *t_r24 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r24 = unsafe { (*t_r24).array.len() };
    p_r24 = unsafe { (*t_r24).array.as_mut_ptr() };
    i_r25 = 0;
    while i_r25 < 5 {
        i_r26 = i_r25 + 100;
        let k = i_r25;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r24 {
            unsafe {
                *p_r24.add(k as usize) = i_r26;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r25 = i_r25 + 1;
    }
    let k = 10;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r24 };
    i_r26 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r24 };
    i_r25 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let mut new_table = Box::new(Table::new());
    t_r24 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r24 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r24;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r24 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r26;
    }
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r24 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r25;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=3;dyn_gets=3;hoists=1;hoist_ctx=0;consts_i=10;consts_b=0;consts_f=0;consts_s=0";
