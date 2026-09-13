// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r57 = 0i64;
    let mut i_r58 = 0i64;
    let mut i_r59 = 0i64;
    let mut i_r60 = 0i64;
    let mut i_r61 = 0i64;
    let mut b_r57 = false;
    let mut t_r57: *mut Table = std::ptr::null_mut();
    let mut p_r57: *mut i64 = std::ptr::null_mut();
    let mut len_r57 = 0usize;
    let mut t_r58: *mut Table = std::ptr::null_mut();
    let mut p_r58: *mut i64 = std::ptr::null_mut();
    let mut len_r58 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r57 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r58 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 2000;
    if lim > 0 {
        let t = unsafe { &mut *t_r57 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r57 = unsafe { (*t_r57).array.len() };
    p_r57 = unsafe { (*t_r57).array.as_mut_ptr() };
    i_r57 = 0;
    loop {
        b_r57 = i_r57 < 2000;
        if b_r57 {
            let k = i_r57;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r57 {
                unsafe {
                    *p_r57.add(k as usize) = i_r57;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r57 = i_r57 + 1;
        } else {
            break;
        }
    }
    i_r57 = 0;
    loop {
        b_r57 = i_r57 < 500;
        if b_r57 {
            let lim = 2000;
            if lim > 0 {
                let t = unsafe { &mut *t_r57 };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            len_r57 = unsafe { (*t_r57).array.len() };
            p_r57 = unsafe { (*t_r57).array.as_mut_ptr() };
            let lim = 2002;
            if lim > 0 {
                let t = unsafe { &mut *t_r58 };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            len_r58 = unsafe { (*t_r58).array.len() };
            p_r58 = unsafe { (*t_r58).array.as_mut_ptr() };
            i_r58 = 0;
            loop {
                b_r57 = i_r58 < 2000;
                if b_r57 {
                    let k = i_r58;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r57 {
                        i_r59 = unsafe { *p_r57.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r60 = i_r58 + 2;
                    i_r61 = i_r59 - 1;
                    i_r59 = i_r61 + 1;
                    let k = i_r60;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r58 {
                        unsafe {
                            *p_r58.add(k as usize) = i_r59;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r58 = i_r58 + 1;
                } else {
                    break;
                }
            }
            i_r57 = i_r57 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=2;fast_gets=1;dyn_sets=0;dyn_gets=0;hoists=3;hoist_ctx=0,1,1";
