// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r54 = 0i64;
    let mut i_r55 = 0i64;
    let mut i_r56 = 0i64;
    let mut i_r57 = 0i64;
    let mut i_r58 = 0i64;
    let mut b_r61 = false;
    let mut t_r64: *mut Table = std::ptr::null_mut();
    let mut p_r64: *mut i64 = std::ptr::null_mut();
    let mut len_r64 = 0usize;
    let mut t_r65: *mut Table = std::ptr::null_mut();
    let mut p_r65: *mut i64 = std::ptr::null_mut();
    let mut len_r65 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r64 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r65 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 2000;
    if lim > 0 {
        let t = unsafe { &mut *t_r64 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r64 = unsafe { (*t_r64).array.len() };
    p_r64 = unsafe { (*t_r64).array.as_mut_ptr() };
    i_r54 = 0;
    loop {
        b_r61 = i_r54 < 2000;
        if b_r61 {
            let k = i_r54;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r64 {
                unsafe {
                    *p_r64.add(k as usize) = i_r54;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r54 = i_r54 + 1;
        } else {
            break;
        }
    }
    i_r54 = 0;
    loop {
        b_r61 = i_r54 < 500;
        if b_r61 {
            let lim = 2000;
            if lim > 0 {
                let t = unsafe { &mut *t_r64 };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            len_r64 = unsafe { (*t_r64).array.len() };
            p_r64 = unsafe { (*t_r64).array.as_mut_ptr() };
            let lim = 2002;
            if lim > 0 {
                let t = unsafe { &mut *t_r65 };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            len_r65 = unsafe { (*t_r65).array.len() };
            p_r65 = unsafe { (*t_r65).array.as_mut_ptr() };
            i_r55 = 0;
            loop {
                b_r61 = i_r55 < 2000;
                if b_r61 {
                    let k = i_r55;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r64 {
                        i_r56 = unsafe { *p_r64.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r57 = i_r55 + 2;
                    i_r58 = i_r56 - 1;
                    i_r56 = i_r58 + 1;
                    let k = i_r57;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r65 {
                        unsafe {
                            *p_r65.add(k as usize) = i_r56;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r55 = i_r55 + 1;
                } else {
                    break;
                }
            }
            i_r54 = i_r54 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=2;fast_gets=1;dyn_sets=0;dyn_gets=0;hoists=3;hoist_ctx=0,1,1;consts_i=25;consts_b=0;consts_f=0;consts_s=0";
