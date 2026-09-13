// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r20 = 0i64;
    let mut i_r21 = 0i64;
    let mut b_r20 = false;
    let mut t_r20: *mut Table = std::ptr::null_mut();
    let mut p_r20: *mut i64 = std::ptr::null_mut();
    let mut len_r20 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r20 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 6;
    if lim > 0 {
        let t = unsafe { &mut *t_r20 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r20 = unsafe { (*t_r20).array.len() };
    p_r20 = unsafe { (*t_r20).array.as_mut_ptr() };
    i_r20 = 0;
    loop {
        b_r20 = i_r20 < 6;
        if b_r20 {
            i_r21 = i_r20 % 2 + i64::from(i_r20 % 2 != 0 && (i_r20 % 2 < 0) != (2 < 0)) * 2;
            b_r20 = i_r21 == 0;
            if b_r20 {
                i_r21 = i_r20 * 10;
                let k = i_r20;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r20 {
                    unsafe {
                        *p_r20.add(k as usize) = i_r21;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
            }
            i_r20 = i_r20 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=0";
