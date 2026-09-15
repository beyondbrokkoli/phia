// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut b_r5 = false;
    let mut t_r7: *mut Table = std::ptr::null_mut();
    let mut p_r7: *mut i64 = std::ptr::null_mut();
    let mut len_r7 = 0usize;
    let mut t_r8: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r7 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r8 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 180;
    if lim > 0 {
        let t = unsafe { &mut *t_r7 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r7 = unsafe { (*t_r7).array.len() };
    p_r7 = unsafe { (*t_r7).array.as_mut_ptr() };
    i_r0 = 0;
    loop {
        b_r5 = i_r0 < 180;
        if b_r5 {
            i_r1 = i_r0 + 1;
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r7 {
                unsafe {
                    *p_r7.add(k as usize) = i_r1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let lim = 180;
    if lim > 0 {
        let t = unsafe { &mut *t_r7 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r7 = unsafe { (*t_r7).array.len() };
    p_r7 = unsafe { (*t_r7).array.as_mut_ptr() };
    i_r1 = 0;
    loop {
        b_r5 = i_r1 < 180;
        if b_r5 {
            i_r0 = 179 - i_r1;
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r7 {
                i_r2 = unsafe { *p_r7.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r8 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = i_r2;
            }
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=1;dyn_gets=0;hoists=2;hoist_ctx=0,0;consts_i=8;consts_b=0;consts_f=0;consts_s=0";
