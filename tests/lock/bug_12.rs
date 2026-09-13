// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r21 = 0i64;
    let mut i_r22 = 0i64;
    let mut b_r21 = false;
    let mut t_r21: *mut Table = std::ptr::null_mut();
    let mut p_r21: *mut i64 = std::ptr::null_mut();
    let mut len_r21 = 0usize;
    let mut t_r22: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r21 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r22 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 310;
    if lim > 0 {
        let t = unsafe { &mut *t_r21 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r21 = unsafe { (*t_r21).array.len() };
    p_r21 = unsafe { (*t_r21).array.as_mut_ptr() };
    i_r21 = 0;
    loop {
        b_r21 = i_r21 < 10;
        if b_r21 {
            i_r22 = i_r21 + 300;
            let k = i_r22;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r21 {
                unsafe {
                    *p_r21.add(k as usize) = 1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = i_r21;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r21 {
                unsafe {
                    *p_r21.add(k as usize) = 1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r21 = i_r21 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=2;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=0";
