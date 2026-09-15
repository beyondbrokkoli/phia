// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r28 = 0i64;
    let mut i_r29 = 0i64;
    let mut i_r30 = 0i64;
    let mut i_r31 = 0i64;
    let mut b_r32 = false;
    let mut t_r35: *mut Table = std::ptr::null_mut();
    let mut p_r35: *mut i64 = std::ptr::null_mut();
    let mut len_r35 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r35 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r28 = 0;
    loop {
        b_r32 = i_r28 < 60;
        if b_r32 {
            i_r29 = 0;
            loop {
                b_r32 = i_r29 < 60;
                if b_r32 {
                    let lim = 60;
                    if lim > 0 {
                        let t = unsafe { &mut *t_r35 };
                        if (lim as usize) > t.array.len() {
                            t.array.resize(lim as usize, 0);
                        }
                    }
                    len_r35 = unsafe { (*t_r35).array.len() };
                    p_r35 = unsafe { (*t_r35).array.as_mut_ptr() };
                    i_r30 = 0;
                    loop {
                        b_r32 = i_r30 < 60;
                        if b_r32 {
                            i_r31 = i_r30 + i_r29;
                            let k = i_r30;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r35 {
                                unsafe {
                                    *p_r35.add(k as usize) = i_r31;
                                }
                            } else {
                                panic!(
                                    "optimizer invariant violated: fast-path bounds check failed"
                                );
                            }
                            i_r30 = i_r30 + 1;
                        } else {
                            break;
                        }
                    }
                    i_r29 = i_r29 + 1;
                } else {
                    break;
                }
            }
            i_r28 = i_r28 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=2;consts_i=7;consts_b=0;consts_f=0;consts_s=0";
