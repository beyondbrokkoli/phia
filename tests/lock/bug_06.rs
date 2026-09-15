// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut b_r5 = false;
    let mut t_r6: *mut Table = std::ptr::null_mut();
    let mut p_r6: *mut i64 = std::ptr::null_mut();
    let mut len_r6 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r6 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r6 };
    i_r0 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let mut new_table = Box::new(Table::new());
    t_r6 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 5;
    if lim > 0 {
        let t = unsafe { &mut *t_r6 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r6 = unsafe { (*t_r6).array.len() };
    p_r6 = unsafe { (*t_r6).array.as_mut_ptr() };
    i_r1 = 0;
    while i_r1 < 5 {
        i_r2 = i_r1 + 100;
        let k = i_r1;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r6 {
            unsafe {
                *p_r6.add(k as usize) = i_r2;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r1 = i_r1 + 1;
    }
    let k = 10;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r6 };
    i_r2 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r6 };
    i_r1 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let mut new_table = Box::new(Table::new());
    t_r6 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r6 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r0;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r6 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r2;
    }
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r6 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r1;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=3;dyn_gets=3;hoists=1;hoist_ctx=0;consts_i=10;consts_b=0;consts_f=0;consts_s=0";
