// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r12 = 0i64;
    let mut b_r12 = false;
    let mut t_r12: *mut Table = std::ptr::null_mut();
    let mut p_r12: *mut i64 = std::ptr::null_mut();
    let mut len_r12 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r12 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 100000;
    if lim > 0 {
        let t = unsafe { &mut *t_r12 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r12 = unsafe { (*t_r12).array.len() };
    p_r12 = unsafe { (*t_r12).array.as_mut_ptr() };
    i_r12 = 0;
    loop {
        b_r12 = i_r12 < 100000;
        if b_r12 {
            let k = i_r12;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r12 {
                unsafe {
                    *p_r12.add(k as usize) = i_r12;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r12 = i_r12 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=0";
