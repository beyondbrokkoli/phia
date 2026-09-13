// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r42 = 0i64;
    let mut i_r43 = 0i64;
    let mut b_r42 = false;
    let mut t_r42: *mut Table = std::ptr::null_mut();
    let mut p_r42: *mut i64 = std::ptr::null_mut();
    let mut len_r42 = 0usize;
    let mut t_r43: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r42 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 200;
    if lim > 0 {
        let t = unsafe { &mut *t_r42 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r42 = unsafe { (*t_r42).array.len() };
    p_r42 = unsafe { (*t_r42).array.as_mut_ptr() };
    i_r42 = 100;
    loop {
        b_r42 = i_r42 < 200;
        if b_r42 {
            i_r43 = i_r42 - 100;
            let k = i_r42;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r42 {
                unsafe {
                    *p_r42.add(k as usize) = i_r43;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r42 = i_r42 + 1;
        } else {
            break;
        }
    }
    let lim = 350;
    if lim > 0 {
        let t = unsafe { &mut *t_r42 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r42 = unsafe { (*t_r42).array.len() };
    p_r42 = unsafe { (*t_r42).array.as_mut_ptr() };
    i_r43 = 200;
    loop {
        b_r42 = i_r43 < 350;
        if b_r42 {
            let k = i_r43;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r42 {
                unsafe {
                    *p_r42.add(k as usize) = i_r43;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r43 = i_r43 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r43 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 150;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r42 };
    i_r43 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r43 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r43;
    }
    let k = 250;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r42 };
    i_r43 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r43 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r43;
    }
    let k = 349;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r42 };
    i_r43 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r43 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r43;
    }
    let k = 99;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r42 };
    i_r43 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r43 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r43;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=2;fast_gets=0;dyn_sets=4;dyn_gets=4;hoists=2;hoist_ctx=0,0";
