// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut b_r4 = false;
    let mut t_r6: *mut Table = std::ptr::null_mut();
    let mut p_r6: *mut i64 = std::ptr::null_mut();
    let mut len_r6 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r6 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 250;
    if lim > 0 {
        let t = unsafe { &mut *t_r6 };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    len_r6 = unsafe { (*t_r6).as_int_mut().len() };
    p_r6 = unsafe { (*t_r6).as_int_mut().as_mut_ptr() };
    i_r0 = 0;
    loop {
        b_r4 = i_r0 < 250;
        if b_r4 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r6 {
                unsafe {
                    *p_r6.add(k as usize) = 5;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let lim = 250;
    if lim > 0 {
        let t = unsafe { &mut *t_r6 };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    len_r6 = unsafe { (*t_r6).as_int_mut().len() };
    p_r6 = unsafe { (*t_r6).as_int_mut().as_mut_ptr() };
    i_r0 = 0;
    loop {
        b_r4 = i_r0 < 250;
        if b_r4 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r6 {
                i_r1 = unsafe { *p_r6.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r2 = i_r1 + 1;
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r6 {
                unsafe {
                    *p_r6.add(k as usize) = i_r2;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=2;fast_gets=1;dyn_sets=0;dyn_gets=0;hoists=2;hoist_ctx=0,0;consts_i=7;consts_b=0;consts_f=0;consts_s=0";
