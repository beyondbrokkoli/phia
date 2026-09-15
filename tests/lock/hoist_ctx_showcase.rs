// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
    let mut b_r13 = false;
    let mut t_r26: *mut Table = std::ptr::null_mut();
    let mut p_r26: *mut i64 = std::ptr::null_mut();
    let mut len_r26 = 0usize;
    let mut t_r27: *mut Table = std::ptr::null_mut();
    let mut p_r27: *mut i64 = std::ptr::null_mut();
    let mut len_r27 = 0usize;
    let mut t_r28: *mut Table = std::ptr::null_mut();
    let mut p_r28: *mut i64 = std::ptr::null_mut();
    let mut len_r28 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r26 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 4;
    if lim > 0 {
        let t = unsafe { &mut *t_r26 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r26 = unsafe { (*t_r26).array.len() };
    p_r26 = unsafe { (*t_r26).array.as_mut_ptr() };
    i_r0 = 0;
    loop {
        b_r13 = i_r0 < 4;
        if b_r13 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r26 {
                unsafe {
                    *p_r26.add(k as usize) = i_r0;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r26 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r0 = 0;
    loop {
        b_r13 = i_r0 < 3;
        if b_r13 {
            let lim = 3;
            if lim > 0 {
                let t = unsafe { &mut *t_r26 };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            len_r26 = unsafe { (*t_r26).array.len() };
            p_r26 = unsafe { (*t_r26).array.as_mut_ptr() };
            i_r1 = 0;
            loop {
                b_r13 = i_r1 < 3;
                if b_r13 {
                    let k = i_r1;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r26 {
                        unsafe {
                            *p_r26.add(k as usize) = i_r1;
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
    let mut new_table = Box::new(Table::new());
    t_r27 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r2 = 0;
    loop {
        b_r13 = i_r2 < 2;
        if b_r13 {
            i_r1 = 0;
            loop {
                b_r13 = i_r1 < 2;
                if b_r13 {
                    let lim = 2;
                    if lim > 0 {
                        let t = unsafe { &mut *t_r27 };
                        if (lim as usize) > t.array.len() {
                            t.array.resize(lim as usize, 0);
                        }
                    }
                    len_r27 = unsafe { (*t_r27).array.len() };
                    p_r27 = unsafe { (*t_r27).array.as_mut_ptr() };
                    i_r3 = 0;
                    loop {
                        b_r13 = i_r3 < 2;
                        if b_r13 {
                            let k = i_r3;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r27 {
                                unsafe {
                                    *p_r27.add(k as usize) = i_r3;
                                }
                            } else {
                                panic!(
                                    "optimizer invariant violated: fast-path bounds check failed"
                                );
                            }
                            i_r3 = i_r3 + 1;
                        } else {
                            break;
                        }
                    }
                    i_r1 = i_r1 + 1;
                } else {
                    break;
                }
            }
            i_r2 = i_r2 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r26 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r28 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 3;
    if lim > 0 {
        let t = unsafe { &mut *t_r26 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r26 = unsafe { (*t_r26).array.len() };
    p_r26 = unsafe { (*t_r26).array.as_mut_ptr() };
    let lim = 3;
    if lim > 0 {
        let t = unsafe { &mut *t_r28 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r28 = unsafe { (*t_r28).array.len() };
    p_r28 = unsafe { (*t_r28).array.as_mut_ptr() };
    i_r0 = 0;
    loop {
        b_r13 = i_r0 < 3;
        if b_r13 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r26 {
                unsafe {
                    *p_r26.add(k as usize) = i_r0;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r28 {
                unsafe {
                    *p_r28.add(k as usize) = i_r0;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r28 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 3;
    if lim > 0 {
        let t = unsafe { &mut *t_r28 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r28 = unsafe { (*t_r28).array.len() };
    p_r28 = unsafe { (*t_r28).array.as_mut_ptr() };
    i_r3 = 0;
    loop {
        b_r13 = i_r3 < 3;
        if b_r13 {
            let k = i_r3;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r28 {
                unsafe {
                    *p_r28.add(k as usize) = i_r3;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r3 = i_r3 + 1;
        } else {
            break;
        }
    }
    let lim = 3;
    if lim > 0 {
        let t = unsafe { &mut *t_r28 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r28 = unsafe { (*t_r28).array.len() };
    p_r28 = unsafe { (*t_r28).array.as_mut_ptr() };
    i_r3 = 0;
    loop {
        b_r13 = i_r3 < 3;
        if b_r13 {
            let k = i_r3;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r28 {
                unsafe {
                    *p_r28.add(k as usize) = i_r3;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r3 = i_r3 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r28 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 3;
    if lim > 0 {
        let t = unsafe { &mut *t_r28 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r28 = unsafe { (*t_r28).array.len() };
    p_r28 = unsafe { (*t_r28).array.as_mut_ptr() };
    i_r3 = 0;
    loop {
        b_r13 = i_r3 < 3;
        if b_r13 {
            i_r0 = 0;
            loop {
                b_r13 = i_r0 < 2;
                if b_r13 {
                    i_r0 = i_r0 + 1;
                } else {
                    break;
                }
            }
            let k = i_r3;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r28 {
                unsafe {
                    *p_r28.add(k as usize) = i_r3;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r3 = i_r3 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r26 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r1 = 0;
    loop {
        b_r13 = i_r1 < 4;
        if b_r13 {
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r26 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = 1;
            }
            i_r0 = 0;
            loop {
                b_r13 = i_r0 < 2;
                if b_r13 {
                    let k = 999;
                    if k < 0 {
                        panic!("Runtime Error: Negative table index");
                    }
                    let idx = k as usize;
                    let t = unsafe { &mut *t_r26 };
                    if idx >= t.array.len() {
                        t.array.resize(idx + 1, 0);
                    }
                    unsafe {
                        *t.array.get_unchecked_mut(idx) = i_r0;
                    }
                    i_r0 = i_r0 + 1;
                } else {
                    break;
                }
            }
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=8;fast_gets=0;dyn_sets=2;dyn_gets=0;hoists=8;hoist_ctx=0,1,0,0,2,0,0,0;consts_i=41;consts_b=0;consts_f=0;consts_s=0";
