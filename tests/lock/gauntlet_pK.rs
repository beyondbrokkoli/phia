// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
    let mut b_r4 = false;
    let mut t_r7: *mut Table = std::ptr::null_mut();
    let mut p_r7: *mut i64 = std::ptr::null_mut();
    let mut len_r7 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r7 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r0 = 0;
    loop {
        b_r4 = i_r0 < 60;
        if b_r4 {
            i_r1 = 0;
            loop {
                b_r4 = i_r1 < 60;
                if b_r4 {
                    let lim = 60;
                    if lim > 0 {
                        let t = unsafe { &mut *t_r7 };
                        if (lim as usize) > t.array.len() {
                            t.array.resize(lim as usize, 0);
                        }
                    }
                    len_r7 = unsafe { (*t_r7).array.len() };
                    p_r7 = unsafe { (*t_r7).array.as_mut_ptr() };
                    i_r2 = 0;
                    loop {
                        b_r4 = i_r2 < 60;
                        if b_r4 {
                            i_r3 = i_r2 + i_r1;
                            let k = i_r2;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r7 {
                                unsafe {
                                    *p_r7.add(k as usize) = i_r3;
                                }
                            } else {
                                panic!(
                                    "optimizer invariant violated: fast-path bounds check failed"
                                );
                            }
                            i_r2 = i_r2 + 1;
                        } else {
                            break;
                        }
                    }
                    i_r1 = i_r1 + 1;
                } else {
                    break;
                }
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=2;consts_i=7;consts_b=0;consts_f=0;consts_s=0";
