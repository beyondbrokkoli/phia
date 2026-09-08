// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r14 = 0i64;
    let mut b_r14 = false;
    let mut t_r14: *mut Table = std::ptr::null_mut();
    let mut p_r14: *mut i64 = std::ptr::null_mut();
    let mut len_r14 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r14 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 10;
    if lim > 0 {
        let t = unsafe { &mut *t_r14 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r14 = unsafe { (*t_r14).array.len() };
    p_r14 = unsafe { (*t_r14).array.as_mut_ptr() };
    i_r14 = 0;
    loop {
        b_r14 = i_r14 < 10;
        if b_r14 {
            let k = i_r14;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r14 {
                unsafe {
                    *p_r14.add(k as usize) = 1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r14 = i_r14 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=0";
