// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r32 = 0i64;
    let mut i_r33 = 0i64;
    let mut i_r34 = 0i64;
    let mut b_r32 = false;
    let mut t_r32: *mut Table = std::ptr::null_mut();
    let mut p_r32: *mut i64 = std::ptr::null_mut();
    let mut len_r32 = 0usize;
    let mut t_r33: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r32 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r33 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 180;
    if lim > 0 {
        let t = unsafe { &mut *t_r32 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r32 = unsafe { (*t_r32).array.len() };
    p_r32 = unsafe { (*t_r32).array.as_mut_ptr() };
    i_r32 = 0;
    loop {
        b_r32 = i_r32 < 180;
        if b_r32 {
            i_r33 = i_r32 + 1;
            let k = i_r32;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r32 {
                unsafe {
                    *p_r32.add(k as usize) = i_r33;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r32 = i_r32 + 1;
        } else {
            break;
        }
    }
    let lim = 180;
    if lim > 0 {
        let t = unsafe { &mut *t_r32 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r32 = unsafe { (*t_r32).array.len() };
    p_r32 = unsafe { (*t_r32).array.as_mut_ptr() };
    i_r33 = 0;
    loop {
        b_r32 = i_r33 < 180;
        if b_r32 {
            i_r32 = 179 - i_r33;
            let k = i_r33;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r32 {
                i_r34 = unsafe { *p_r32.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = i_r32;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r33 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = i_r34;
            }
            i_r33 = i_r33 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=1;dyn_gets=0;hoists=2;hoist_ctx=0,0";
