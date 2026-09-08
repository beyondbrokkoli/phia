// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r29 = 0i64;
    let mut i_r30 = 0i64;
    let mut i_r31 = 0i64;
    let mut b_r29 = false;
    let mut t_r29: *mut Table = std::ptr::null_mut();
    let mut p_r29: *mut i64 = std::ptr::null_mut();
    let mut len_r29 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r29 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r29 };
    i_r29 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let mut new_table = Box::new(Table::new());
    t_r29 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 5;
    if lim > 0 {
        let t = unsafe { &mut *t_r29 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r29 = unsafe { (*t_r29).array.len() };
    p_r29 = unsafe { (*t_r29).array.as_mut_ptr() };
    i_r30 = 0;
    loop {
        b_r29 = i_r30 < 5;
        if b_r29 {
            i_r31 = i_r30 + 100;
            let k = i_r30;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r29 {
                unsafe {
                    *p_r29.add(k as usize) = i_r31;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r30 = i_r30 + 1;
        } else {
            break;
        }
    }
    let k = 10;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r29 };
    i_r31 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r29 };
    i_r30 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let mut new_table = Box::new(Table::new());
    t_r29 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r29 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r29;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r29 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r31;
    }
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r29 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r30;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=3;dyn_gets=3;hoists=1;hoist_ctx=0";
