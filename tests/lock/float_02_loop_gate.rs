// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r16 = 0i64;
    let mut b_r16 = false;
    let mut f_r17 = 0f64;
    let mut f_r18 = 0f64;
    let mut t_r16: *mut Table = std::ptr::null_mut();
    let mut p_r16: *mut f64 = std::ptr::null_mut();
    let mut len_r16 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_float());
    t_r16 = &mut *new_table as *mut Table;
    tables.push(new_table);
    f_r17 = 0.0;
    let lim = 8;
    if lim > 0 {
        let t = unsafe { &mut *t_r16 };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    len_r16 = unsafe { (*t_r16).farray.len() };
    p_r16 = unsafe { (*t_r16).farray.as_mut_ptr() };
    f_r18 = f_r17;
    i_r16 = 0;
    loop {
        b_r16 = i_r16 < 8;
        if b_r16 {
            let k = i_r16;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r16 {
                unsafe {
                    *p_r16.add(k as usize) = f_r18;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            f_r17 = 0.25;
            f_r18 = f_r18 + f_r17;
            i_r16 = i_r16 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=0";
