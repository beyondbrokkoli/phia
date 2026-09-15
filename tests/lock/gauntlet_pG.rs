// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r30 = 0i64;
    let mut i_r31 = 0i64;
    let mut i_r32 = 0i64;
    let mut b_r35 = false;
    let mut t_r37: *mut Table = std::ptr::null_mut();
    let mut p_r37: *mut i64 = std::ptr::null_mut();
    let mut len_r37 = 0usize;
    let mut t_r38: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r37 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r38 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 180;
    if lim > 0 {
        let t = unsafe { &mut *t_r37 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r37 = unsafe { (*t_r37).array.len() };
    p_r37 = unsafe { (*t_r37).array.as_mut_ptr() };
    i_r30 = 0;
    loop {
        b_r35 = i_r30 < 180;
        if b_r35 {
            i_r31 = i_r30 + 1;
            let k = i_r30;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r37 {
                unsafe {
                    *p_r37.add(k as usize) = i_r31;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r30 = i_r30 + 1;
        } else {
            break;
        }
    }
    let lim = 180;
    if lim > 0 {
        let t = unsafe { &mut *t_r37 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r37 = unsafe { (*t_r37).array.len() };
    p_r37 = unsafe { (*t_r37).array.as_mut_ptr() };
    i_r31 = 0;
    loop {
        b_r35 = i_r31 < 180;
        if b_r35 {
            i_r30 = 179 - i_r31;
            let k = i_r31;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r37 {
                i_r32 = unsafe { *p_r37.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = i_r30;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r38 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = i_r32;
            }
            i_r31 = i_r31 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=1;dyn_gets=0;hoists=2;hoist_ctx=0,0;consts_i=8;consts_b=0;consts_f=0;consts_s=0";
