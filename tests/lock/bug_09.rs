// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r54 = 0i64;
    let mut i_r55 = 0i64;
    let mut i_r56 = 0i64;
    let mut i_r57 = 0i64;
    let mut i_r58 = 0i64;
    let mut b_r54 = false;
    let mut t_r54: *mut Table = std::ptr::null_mut();
    let mut p_r54: *mut i64 = std::ptr::null_mut();
    let mut len_r54 = 0usize;
    let mut t_r55: *mut Table = std::ptr::null_mut();
    let mut p_r55: *mut i64 = std::ptr::null_mut();
    let mut len_r55 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r54 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r55 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 2000;
    if lim > 0 {
        let t = unsafe { &mut *t_r54 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r54 = unsafe { (*t_r54).array.len() };
    p_r54 = unsafe { (*t_r54).array.as_mut_ptr() };
    i_r54 = 0;
    loop {
        b_r54 = i_r54 < 2000;
        if b_r54 {
            let k = i_r54;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r54 {
                unsafe {
                    *p_r54.add(k as usize) = i_r54;
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
        b_r54 = i_r54 < 500;
        if b_r54 {
            let lim = 2000;
            if lim > 0 {
                let t = unsafe { &mut *t_r54 };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            len_r54 = unsafe { (*t_r54).array.len() };
            p_r54 = unsafe { (*t_r54).array.as_mut_ptr() };
            let lim = 2002;
            if lim > 0 {
                let t = unsafe { &mut *t_r55 };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            len_r55 = unsafe { (*t_r55).array.len() };
            p_r55 = unsafe { (*t_r55).array.as_mut_ptr() };
            i_r55 = 0;
            loop {
                b_r54 = i_r55 < 2000;
                if b_r54 {
                    let k = i_r55;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r54 {
                        i_r56 = unsafe { *p_r54.add(k as usize) };
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
                    if (k as usize) < len_r55 {
                        unsafe {
                            *p_r55.add(k as usize) = i_r56;
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

pub const STATS: &str = "fast_sets=2;fast_gets=1;dyn_sets=0;dyn_gets=0;hoists=3;hoist_ctx=0,1,1";
