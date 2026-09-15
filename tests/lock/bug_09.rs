// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
    let mut i_r4 = 0i64;
    let mut b_r7 = false;
    let mut t_r10: *mut Table = std::ptr::null_mut();
    let mut p_r10: *mut i64 = std::ptr::null_mut();
    let mut len_r10 = 0usize;
    let mut t_r11: *mut Table = std::ptr::null_mut();
    let mut p_r11: *mut i64 = std::ptr::null_mut();
    let mut len_r11 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r10 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r11 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 2000;
    if lim > 0 {
        let t = unsafe { &mut *t_r10 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r10 = unsafe { (*t_r10).array.len() };
    p_r10 = unsafe { (*t_r10).array.as_mut_ptr() };
    i_r0 = 0;
    loop {
        b_r7 = i_r0 < 2000;
        if b_r7 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r10 {
                unsafe {
                    *p_r10.add(k as usize) = i_r0;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    i_r0 = 0;
    loop {
        b_r7 = i_r0 < 500;
        if b_r7 {
            let lim = 2000;
            if lim > 0 {
                let t = unsafe { &mut *t_r10 };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            len_r10 = unsafe { (*t_r10).array.len() };
            p_r10 = unsafe { (*t_r10).array.as_mut_ptr() };
            let lim = 2002;
            if lim > 0 {
                let t = unsafe { &mut *t_r11 };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            len_r11 = unsafe { (*t_r11).array.len() };
            p_r11 = unsafe { (*t_r11).array.as_mut_ptr() };
            i_r1 = 0;
            loop {
                b_r7 = i_r1 < 2000;
                if b_r7 {
                    let k = i_r1;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r10 {
                        i_r2 = unsafe { *p_r10.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r3 = i_r1 + 2;
                    i_r4 = i_r2 - 1;
                    i_r2 = i_r4 + 1;
                    let k = i_r3;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r11 {
                        unsafe {
                            *p_r11.add(k as usize) = i_r2;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
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

pub const STATS: &str = "fast_sets=2;fast_gets=1;dyn_sets=0;dyn_gets=0;hoists=3;hoist_ctx=0,1,1;consts_i=25;consts_b=0;consts_f=0;consts_s=0";
