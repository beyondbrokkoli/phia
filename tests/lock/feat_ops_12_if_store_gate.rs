// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r18 = 0i64;
    let mut i_r19 = 0i64;
    let mut b_r21 = false;
    let mut t_r23: *mut Table = std::ptr::null_mut();
    let mut p_r23: *mut i64 = std::ptr::null_mut();
    let mut len_r23 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r23 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 6;
    if lim > 0 {
        let t = unsafe { &mut *t_r23 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r23 = unsafe { (*t_r23).array.len() };
    p_r23 = unsafe { (*t_r23).array.as_mut_ptr() };
    i_r18 = 0;
    loop {
        b_r21 = i_r18 < 6;
        if b_r21 {
            i_r19 = i_r18 % 2 + i64::from(i_r18 % 2 != 0 && (i_r18 % 2 < 0) != (2 < 0)) * 2;
            b_r21 = i_r19 == 0;
            if b_r21 {
                i_r19 = i_r18 * 10;
                let k = i_r18;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r23 {
                    unsafe {
                        *p_r23.add(k as usize) = i_r19;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
            }
            i_r18 = i_r18 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=0;consts_i=8;consts_b=0;consts_f=0;consts_s=0";
