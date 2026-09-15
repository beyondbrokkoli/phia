// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r20 = 0i64;
    let mut i_r21 = 0i64;
    let mut i_r22 = 0i64;
    let mut b_r24 = false;
    let mut t_r25: *mut Table = std::ptr::null_mut();
    let mut p_r25: *mut i64 = std::ptr::null_mut();
    let mut len_r25 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r25 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 80;
    if lim > 0 {
        let t = unsafe { &mut *t_r25 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r25 = unsafe { (*t_r25).array.len() };
    p_r25 = unsafe { (*t_r25).array.as_mut_ptr() };
    i_r20 = 0;
    while i_r20 < 80 {
        i_r21 = i_r20;
        let k = i_r21;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r25 {
            unsafe {
                *p_r25.add(k as usize) = 1;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r22 = i_r21 + 0;
        let k = i_r22;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r25 {
            unsafe {
                *p_r25.add(k as usize) = 2;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r22 = i_r20;
        let k = i_r22;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r25 {
            unsafe {
                *p_r25.add(k as usize) = 3;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r20 = i_r20 + 1;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=3;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=0;consts_i=7;consts_b=0;consts_f=0;consts_s=0";
