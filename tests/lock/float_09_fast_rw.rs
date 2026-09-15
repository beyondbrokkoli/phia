// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut b_r2 = false;
    let mut f_r4 = 0f64;
    let mut f_r5 = 0f64;
    let mut t_r6: *mut Table = std::ptr::null_mut();
    let mut p_r6: *mut f64 = std::ptr::null_mut();
    let mut len_r6 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_float());
    t_r6 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 8;
    if lim > 0 {
        let t = unsafe { &mut *t_r6 };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    len_r6 = unsafe { (*t_r6).farray.len() };
    p_r6 = unsafe { (*t_r6).farray.as_mut_ptr() };
    i_r0 = 0;
    loop {
        b_r2 = i_r0 < 8;
        if b_r2 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r6 {
                unsafe {
                    *p_r6.add(k as usize) = 0.5;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let lim = 8;
    if lim > 0 {
        let t = unsafe { &mut *t_r6 };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    len_r6 = unsafe { (*t_r6).farray.len() };
    p_r6 = unsafe { (*t_r6).farray.as_mut_ptr() };
    f_r4 = 0.0;
    i_r0 = 0;
    loop {
        b_r2 = i_r0 < 8;
        if b_r2 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r6 {
                f_r5 = unsafe { *p_r6.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            f_r4 = f_r4 + f_r5;
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=0;dyn_gets=0;hoists=2;hoist_ctx=0,0;consts_i=6;consts_b=0;consts_f=2;consts_s=0";
