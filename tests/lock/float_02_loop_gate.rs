// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r14 = 0i64;
    let mut b_r15 = false;
    let mut f_r16 = 0f64;
    let mut t_r17: *mut Table = std::ptr::null_mut();
    let mut p_r17: *mut f64 = std::ptr::null_mut();
    let mut len_r17 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_float());
    t_r17 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 8;
    if lim > 0 {
        let t = unsafe { &mut *t_r17 };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    len_r17 = unsafe { (*t_r17).farray.len() };
    p_r17 = unsafe { (*t_r17).farray.as_mut_ptr() };
    f_r16 = 0.0;
    i_r14 = 0;
    while i_r14 < 8 {
        let k = i_r14;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r17 {
            unsafe {
                *p_r17.add(k as usize) = f_r16;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        f_r16 = f_r16 + 0.25;
        i_r14 = i_r14 + 1;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=0;consts_i=3;consts_b=0;consts_f=2;consts_s=0";
