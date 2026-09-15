// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r24 = 0i64;
    let mut b_r24 = false;
    let mut f_r26 = 0f64;
    let mut f_r27 = 0f64;
    let mut t_r30: *mut Table = std::ptr::null_mut();
    let mut p_r30: *mut f64 = std::ptr::null_mut();
    let mut len_r30 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_float());
    t_r30 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 8;
    if lim > 0 {
        let t = unsafe { &mut *t_r30 };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    len_r30 = unsafe { (*t_r30).farray.len() };
    p_r30 = unsafe { (*t_r30).farray.as_mut_ptr() };
    i_r24 = 0;
    loop {
        b_r24 = i_r24 < 8;
        if b_r24 {
            f_r26 = 0.5;
            let k = i_r24;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r30 {
                unsafe {
                    *p_r30.add(k as usize) = f_r26;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r24 = i_r24 + 1;
        } else {
            break;
        }
    }
    f_r26 = 0.0;
    let lim = 8;
    if lim > 0 {
        let t = unsafe { &mut *t_r30 };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    len_r30 = unsafe { (*t_r30).farray.len() };
    p_r30 = unsafe { (*t_r30).farray.as_mut_ptr() };
    f_r27 = f_r26;
    i_r24 = 0;
    loop {
        b_r24 = i_r24 < 8;
        if b_r24 {
            let k = i_r24;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r30 {
                f_r26 = unsafe { *p_r30.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            f_r27 = f_r27 + f_r26;
            i_r24 = i_r24 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=0;dyn_gets=0;hoists=2;hoist_ctx=0,0";
