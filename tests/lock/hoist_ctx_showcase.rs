// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r130 = 0i64;
    let mut i_r131 = 0i64;
    let mut i_r132 = 0i64;
    let mut i_r133 = 0i64;
    let mut b_r130 = false;
    let mut t_r130: *mut Table = std::ptr::null_mut();
    let mut p_r130: *mut i64 = std::ptr::null_mut();
    let mut len_r130 = 0usize;
    let mut t_r131: *mut Table = std::ptr::null_mut();
    let mut p_r131: *mut i64 = std::ptr::null_mut();
    let mut len_r131 = 0usize;
    let mut t_r132: *mut Table = std::ptr::null_mut();
    let mut p_r132: *mut i64 = std::ptr::null_mut();
    let mut len_r132 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r130 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 4;
    if lim > 0 {
        let t = unsafe { &mut *t_r130 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r130 = unsafe { (*t_r130).array.len() };
    p_r130 = unsafe { (*t_r130).array.as_mut_ptr() };
    i_r130 = 0;
    loop {
        b_r130 = i_r130 < 4;
        if b_r130 {
            let k = i_r130;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r130 {
                unsafe {
                    *p_r130.add(k as usize) = i_r130;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r130 = i_r130 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r130 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r130 = 0;
    loop {
        b_r130 = i_r130 < 3;
        if b_r130 {
            let lim = 3;
            if lim > 0 {
                let t = unsafe { &mut *t_r130 };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            len_r130 = unsafe { (*t_r130).array.len() };
            p_r130 = unsafe { (*t_r130).array.as_mut_ptr() };
            i_r131 = 0;
            loop {
                b_r130 = i_r131 < 3;
                if b_r130 {
                    let k = i_r131;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r130 {
                        unsafe {
                            *p_r130.add(k as usize) = i_r131;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r131 = i_r131 + 1;
                } else {
                    break;
                }
            }
            i_r130 = i_r130 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r131 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r132 = 0;
    loop {
        b_r130 = i_r132 < 2;
        if b_r130 {
            i_r131 = 0;
            loop {
                b_r130 = i_r131 < 2;
                if b_r130 {
                    let lim = 2;
                    if lim > 0 {
                        let t = unsafe { &mut *t_r131 };
                        if (lim as usize) > t.array.len() {
                            t.array.resize(lim as usize, 0);
                        }
                    }
                    len_r131 = unsafe { (*t_r131).array.len() };
                    p_r131 = unsafe { (*t_r131).array.as_mut_ptr() };
                    i_r133 = 0;
                    loop {
                        b_r130 = i_r133 < 2;
                        if b_r130 {
                            let k = i_r133;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r131 {
                                unsafe {
                                    *p_r131.add(k as usize) = i_r133;
                                }
                            } else {
                                panic!(
                                    "optimizer invariant violated: fast-path bounds check failed"
                                );
                            }
                            i_r133 = i_r133 + 1;
                        } else {
                            break;
                        }
                    }
                    i_r131 = i_r131 + 1;
                } else {
                    break;
                }
            }
            i_r132 = i_r132 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r130 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r132 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 3;
    if lim > 0 {
        let t = unsafe { &mut *t_r130 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r130 = unsafe { (*t_r130).array.len() };
    p_r130 = unsafe { (*t_r130).array.as_mut_ptr() };
    let lim = 3;
    if lim > 0 {
        let t = unsafe { &mut *t_r132 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r132 = unsafe { (*t_r132).array.len() };
    p_r132 = unsafe { (*t_r132).array.as_mut_ptr() };
    i_r130 = 0;
    loop {
        b_r130 = i_r130 < 3;
        if b_r130 {
            let k = i_r130;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r130 {
                unsafe {
                    *p_r130.add(k as usize) = i_r130;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = i_r130;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r132 {
                unsafe {
                    *p_r132.add(k as usize) = i_r130;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r130 = i_r130 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r132 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 3;
    if lim > 0 {
        let t = unsafe { &mut *t_r132 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r132 = unsafe { (*t_r132).array.len() };
    p_r132 = unsafe { (*t_r132).array.as_mut_ptr() };
    i_r133 = 0;
    loop {
        b_r130 = i_r133 < 3;
        if b_r130 {
            let k = i_r133;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r132 {
                unsafe {
                    *p_r132.add(k as usize) = i_r133;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r133 = i_r133 + 1;
        } else {
            break;
        }
    }
    let lim = 3;
    if lim > 0 {
        let t = unsafe { &mut *t_r132 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r132 = unsafe { (*t_r132).array.len() };
    p_r132 = unsafe { (*t_r132).array.as_mut_ptr() };
    i_r133 = 0;
    loop {
        b_r130 = i_r133 < 3;
        if b_r130 {
            let k = i_r133;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r132 {
                unsafe {
                    *p_r132.add(k as usize) = i_r133;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r133 = i_r133 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r132 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 3;
    if lim > 0 {
        let t = unsafe { &mut *t_r132 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r132 = unsafe { (*t_r132).array.len() };
    p_r132 = unsafe { (*t_r132).array.as_mut_ptr() };
    i_r133 = 0;
    loop {
        b_r130 = i_r133 < 3;
        if b_r130 {
            i_r130 = 0;
            loop {
                b_r130 = i_r130 < 2;
                if b_r130 {
                    i_r130 = i_r130 + 1;
                } else {
                    break;
                }
            }
            let k = i_r133;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r132 {
                unsafe {
                    *p_r132.add(k as usize) = i_r133;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r133 = i_r133 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r130 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r131 = 0;
    loop {
        b_r130 = i_r131 < 4;
        if b_r130 {
            let k = i_r131;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r130 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = 1;
            }
            i_r130 = 0;
            loop {
                b_r130 = i_r130 < 2;
                if b_r130 {
                    let k = 999;
                    if k < 0 {
                        panic!("Runtime Error: Negative table index");
                    }
                    let idx = k as usize;
                    let t = unsafe { &mut *t_r130 };
                    if idx >= t.array.len() {
                        t.array.resize(idx + 1, 0);
                    }
                    unsafe {
                        *t.array.get_unchecked_mut(idx) = i_r130;
                    }
                    i_r130 = i_r130 + 1;
                } else {
                    break;
                }
            }
            i_r131 = i_r131 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=8;fast_gets=0;dyn_sets=2;dyn_gets=0;hoists=8;hoist_ctx=0,1,0,0,2,0,0,0;consts_i=41;consts_b=0;consts_f=0;consts_s=0";
