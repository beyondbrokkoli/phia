// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut b_r3 = false;
    let mut t_r5: *mut Table = std::ptr::null_mut();
    let mut p_r5: *mut i64 = std::ptr::null_mut();
    let mut len_r5 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r5 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 30;
    if lim > 0 {
        let t = unsafe { &mut *t_r5 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r5 = unsafe { (*t_r5).array.len() };
    p_r5 = unsafe { (*t_r5).array.as_mut_ptr() };
    i_r0 = 0;
    loop {
        b_r3 = i_r0 < 30;
        if b_r3 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r5 {
                unsafe {
                    *p_r5.add(k as usize) = 1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let k = 30;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r5 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 60;
    }
    i_r0 = 0;
    loop {
        let k = i_r0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &*t_r5 };
        i_r1 = if idx < t.array.len() {
            unsafe { *t.array.get_unchecked(idx) }
        } else {
            0
        };
        b_r3 = i_r1 < 60;
        if b_r3 {
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r5 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r5 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r0;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=2;dyn_gets=1;hoists=1;hoist_ctx=0;consts_i=10;consts_b=0;consts_f=0;consts_s=0";
