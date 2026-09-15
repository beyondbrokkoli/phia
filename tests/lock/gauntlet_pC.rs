// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r24 = 0i64;
    let mut i_r25 = 0i64;
    let mut i_r26 = 0i64;
    let mut b_r28 = false;
    let mut t_r30: *mut Table = std::ptr::null_mut();
    let mut p_r30: *mut i64 = std::ptr::null_mut();
    let mut len_r30 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r30 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 250;
    if lim > 0 {
        let t = unsafe { &mut *t_r30 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r30 = unsafe { (*t_r30).array.len() };
    p_r30 = unsafe { (*t_r30).array.as_mut_ptr() };
    i_r24 = 0;
    loop {
        b_r28 = i_r24 < 250;
        if b_r28 {
            let k = i_r24;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r30 {
                unsafe {
                    *p_r30.add(k as usize) = 5;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r24 = i_r24 + 1;
        } else {
            break;
        }
    }
    let lim = 250;
    if lim > 0 {
        let t = unsafe { &mut *t_r30 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r30 = unsafe { (*t_r30).array.len() };
    p_r30 = unsafe { (*t_r30).array.as_mut_ptr() };
    i_r24 = 0;
    loop {
        b_r28 = i_r24 < 250;
        if b_r28 {
            let k = i_r24;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r30 {
                i_r25 = unsafe { *p_r30.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r26 = i_r25 + 1;
            let k = i_r24;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r30 {
                unsafe {
                    *p_r30.add(k as usize) = i_r26;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r24 = i_r24 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=2;fast_gets=1;dyn_sets=0;dyn_gets=0;hoists=2;hoist_ctx=0,0;consts_i=7;consts_b=0;consts_f=0;consts_s=0";
