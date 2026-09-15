// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut b_r1 = false;
    let mut f_r2 = 0f64;
    let mut f_r3 = 0f64;
    let mut t_r4: *mut Table = std::ptr::null_mut();
    let mut p_r4: *mut f64 = std::ptr::null_mut();
    let mut len_r4 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_float());
    t_r4 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r4 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 0.75;
    }
    let lim = 4;
    if lim > 0 {
        let t = unsafe { &mut *t_r4 };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    len_r4 = unsafe { (*t_r4).farray.len() };
    p_r4 = unsafe { (*t_r4).farray.as_mut_ptr() };
    i_r0 = 0;
    while i_r0 < 4 {
        let k = i_r0;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r4 {
            f_r2 = unsafe { *p_r4.add(k as usize) };
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        f_r3 = f_r2 + 0.125;
        let k = i_r0;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r4 {
            unsafe {
                *p_r4.add(k as usize) = f_r3;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r0 = i_r0 + 1;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=1;dyn_gets=0;hoists=1;hoist_ctx=0;consts_i=4;consts_b=0;consts_f=4;consts_s=0";
