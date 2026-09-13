// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r26 = 0i64;
    let mut i_r27 = 0i64;
    let mut b_r26 = false;
    let mut t_r26: *mut Table = std::ptr::null_mut();
    let mut p_r26: *mut i64 = std::ptr::null_mut();
    let mut len_r26 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r26 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 30;
    if lim > 0 {
        let t = unsafe { &mut *t_r26 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r26 = unsafe { (*t_r26).array.len() };
    p_r26 = unsafe { (*t_r26).array.as_mut_ptr() };
    i_r26 = 0;
    loop {
        b_r26 = i_r26 < 30;
        if b_r26 {
            let k = i_r26;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r26 {
                unsafe {
                    *p_r26.add(k as usize) = 1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r26 = i_r26 + 1;
        } else {
            break;
        }
    }
    let k = 30;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r26 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 60;
    }
    i_r26 = 0;
    loop {
        let k = i_r26;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &*t_r26 };
        i_r27 = if idx < t.array.len() {
            unsafe { *t.array.get_unchecked(idx) }
        } else {
            0
        };
        b_r26 = i_r27 < 60;
        if b_r26 {
            i_r26 = i_r26 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r26 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r26 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r26;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=2;dyn_gets=1;hoists=1;hoist_ctx=0";
