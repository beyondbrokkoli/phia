// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
    let mut i_r4 = 0i64;
    let mut b_r48 = false;
    let mut t_r71: *mut Table = std::ptr::null_mut();
    let mut p_r71: *mut i64 = std::ptr::null_mut();
    let mut len_r71 = 0usize;
    let mut t_r72: *mut Table = std::ptr::null_mut();
    let mut p_r72: *mut i64 = std::ptr::null_mut();
    let mut len_r72 = 0usize;
    let mut t_r73: *mut Table = std::ptr::null_mut();
    let mut p_r73: *mut i64 = std::ptr::null_mut();
    let mut len_r73 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r71 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 8000000;
    if lim > 0 {
        let t = unsafe { &mut *t_r71 };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    len_r71 = unsafe { (*t_r71).as_int_mut().len() };
    p_r71 = unsafe { (*t_r71).as_int_mut().as_mut_ptr() };
    i_r0 = 0;
    loop {
        b_r48 = i_r0 < 8000000;
        if b_r48 {
            i_r1 = i_r0 + 1;
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r71 {
                unsafe {
                    *p_r71.add(k as usize) = i_r1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 2;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r71 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r1 = 0;
    loop {
        b_r48 = i_r1 < 100000;
        if b_r48 {
            let lim = i_r1;
            if lim > 0 {
                let t = unsafe { &mut *t_r71 };
                if (lim as usize) > t.as_int_mut().len() {
                    t.as_int_mut().resize(lim as usize, 0);
                }
            }
            len_r71 = unsafe { (*t_r71).as_int_mut().len() };
            p_r71 = unsafe { (*t_r71).as_int_mut().as_mut_ptr() };
            i_r0 = 0;
            loop {
                b_r48 = i_r0 < i_r1;
                if b_r48 {
                    i_r3 = i_r0 + 1;
                    let k = i_r0;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r71 {
                        unsafe {
                            *p_r71.add(k as usize) = i_r3;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
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
    let mut new_table = Box::new(Table::new());
    t_r72 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 2500000;
    if lim > 0 {
        let t = unsafe { &mut *t_r72 };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    len_r72 = unsafe { (*t_r72).as_int_mut().len() };
    p_r72 = unsafe { (*t_r72).as_int_mut().as_mut_ptr() };
    i_r2 = 0;
    loop {
        b_r48 = i_r2 < 2500000;
        if b_r48 {
            let k = i_r2;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r72 {
                unsafe {
                    *p_r72.add(k as usize) = 5;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r2 = i_r2 + 1;
        } else {
            break;
        }
    }
    let lim = 2500000;
    if lim > 0 {
        let t = unsafe { &mut *t_r72 };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    len_r72 = unsafe { (*t_r72).as_int_mut().len() };
    p_r72 = unsafe { (*t_r72).as_int_mut().as_mut_ptr() };
    i_r3 = 0;
    loop {
        b_r48 = i_r3 < 2500000;
        if b_r48 {
            let k = i_r3;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r72 {
                i_r2 = unsafe { *p_r72.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r2 + 1;
            let k = i_r3;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r72 {
                unsafe {
                    *p_r72.add(k as usize) = i_r0;
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
    t_r72 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 30000;
    if lim > 0 {
        let t = unsafe { &mut *t_r72 };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    len_r72 = unsafe { (*t_r72).as_int_mut().len() };
    p_r72 = unsafe { (*t_r72).as_int_mut().as_mut_ptr() };
    i_r0 = 0;
    loop {
        b_r48 = i_r0 < 30000;
        if b_r48 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r72 {
                unsafe {
                    *p_r72.add(k as usize) = 1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let k = 30;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r72 };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = 60;
    }
    i_r0 = 0;
    loop {
        let k = i_r0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &*t_r72 };
        i_r2 = if idx < t.as_int().len() {
            unsafe { *t.as_int().get_unchecked(idx) }
        } else {
            0
        };
        b_r48 = i_r2 < 60;
        if b_r48 {
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r72 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r72 };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = i_r0;
    }
    let mut new_table = Box::new(Table::new());
    t_r72 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 2000000;
    if lim > 0 {
        let t = unsafe { &mut *t_r72 };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    len_r72 = unsafe { (*t_r72).as_int_mut().len() };
    p_r72 = unsafe { (*t_r72).as_int_mut().as_mut_ptr() };
    i_r2 = 0;
    loop {
        b_r48 = i_r2 < 2000000;
        if b_r48 {
            i_r0 = i_r2;
            i_r3 = 0;
            loop {
                b_r48 = i_r3 < 3;
                if b_r48 {
                    let k = i_r0;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r72 {
                        unsafe {
                            *p_r72.add(k as usize) = 7;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r3 = i_r3 + 1;
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
    t_r71 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r1 = 0;
    loop {
        b_r48 = i_r1 < 60000;
        if b_r48 {
            i_r3 = i_r1;
            let lim = 60000;
            if lim > 0 {
                let t = unsafe { &mut *t_r71 };
                if (lim as usize) > t.as_int_mut().len() {
                    t.as_int_mut().resize(lim as usize, 0);
                }
            }
            len_r71 = unsafe { (*t_r71).as_int_mut().len() };
            p_r71 = unsafe { (*t_r71).as_int_mut().as_mut_ptr() };
            i_r0 = i_r3;
            loop {
                b_r48 = i_r0 < 60000;
                if b_r48 {
                    i_r2 = i_r0 + 1;
                    let k = i_r0;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r71 {
                        unsafe {
                            *p_r71.add(k as usize) = i_r2;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
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
    let mut new_table = Box::new(Table::new());
    t_r72 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r73 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 180;
    if lim > 0 {
        let t = unsafe { &mut *t_r72 };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    len_r72 = unsafe { (*t_r72).as_int_mut().len() };
    p_r72 = unsafe { (*t_r72).as_int_mut().as_mut_ptr() };
    i_r3 = 0;
    loop {
        b_r48 = i_r3 < 180;
        if b_r48 {
            i_r2 = i_r3 + 1;
            let k = i_r3;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r72 {
                unsafe {
                    *p_r72.add(k as usize) = i_r2;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r3 = i_r3 + 1;
        } else {
            break;
        }
    }
    let lim = 180;
    if lim > 0 {
        let t = unsafe { &mut *t_r72 };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    len_r72 = unsafe { (*t_r72).as_int_mut().len() };
    p_r72 = unsafe { (*t_r72).as_int_mut().as_mut_ptr() };
    i_r2 = 0;
    loop {
        b_r48 = i_r2 < 180;
        if b_r48 {
            i_r3 = 179 - i_r2;
            let k = i_r2;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r72 {
                i_r0 = unsafe { *p_r72.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = i_r3;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r73 };
            if idx >= t.as_int_mut().len() {
                t.as_int_mut().resize(idx + 1, 0);
            }
            unsafe {
                *t.as_int_mut().get_unchecked_mut(idx) = i_r0;
            }
            i_r2 = i_r2 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r73 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 1000000;
    if lim > 0 {
        let t = unsafe { &mut *t_r73 };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    len_r73 = unsafe { (*t_r73).as_int_mut().len() };
    p_r73 = unsafe { (*t_r73).as_int_mut().as_mut_ptr() };
    i_r0 = 0;
    loop {
        b_r48 = i_r0 < 1000000;
        if b_r48 {
            i_r3 = i_r0 + 1;
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r73 {
                unsafe {
                    *p_r73.add(k as usize) = i_r3;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let lim = 1000000;
    if lim > 0 {
        let t = unsafe { &mut *t_r73 };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    len_r73 = unsafe { (*t_r73).as_int_mut().len() };
    p_r73 = unsafe { (*t_r73).as_int_mut().as_mut_ptr() };
    i_r3 = 0;
    i_r0 = 0;
    loop {
        b_r48 = i_r0 < 1000000;
        if b_r48 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r73 {
                i_r2 = unsafe { *p_r73.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r3 = i_r3 + i_r2;
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r73 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r73 };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = i_r3;
    }
    let mut new_table = Box::new(Table::new());
    t_r73 = &mut *new_table as *mut Table;
    tables.push(new_table);
    b_r48 = true;
    i_r2 = 0;
    while b_r48 {
        let k = i_r2;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r73 };
        if idx >= t.as_int_mut().len() {
            t.as_int_mut().resize(idx + 1, 0);
        }
        unsafe {
            *t.as_int_mut().get_unchecked_mut(idx) = 100;
        }
        b_r48 = 0 < 0;
        i_r2 = i_r2 + 1;
    }
    let mut new_table = Box::new(Table::new());
    t_r73 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 200;
    if lim > 0 {
        let t = unsafe { &mut *t_r73 };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    len_r73 = unsafe { (*t_r73).as_int_mut().len() };
    p_r73 = unsafe { (*t_r73).as_int_mut().as_mut_ptr() };
    i_r2 = 100;
    loop {
        b_r48 = i_r2 < 200;
        if b_r48 {
            i_r0 = i_r2 - 100;
            let k = i_r2;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r73 {
                unsafe {
                    *p_r73.add(k as usize) = i_r0;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r2 = i_r2 + 1;
        } else {
            break;
        }
    }
    let lim = 350;
    if lim > 0 {
        let t = unsafe { &mut *t_r73 };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    len_r73 = unsafe { (*t_r73).as_int_mut().len() };
    p_r73 = unsafe { (*t_r73).as_int_mut().as_mut_ptr() };
    i_r0 = 200;
    loop {
        b_r48 = i_r0 < 350;
        if b_r48 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r73 {
                unsafe {
                    *p_r73.add(k as usize) = i_r0;
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
    t_r72 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 150;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r73 };
    i_r0 = if idx < t.as_int().len() {
        unsafe { *t.as_int().get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r72 };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = i_r0;
    }
    let k = 250;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r73 };
    i_r0 = if idx < t.as_int().len() {
        unsafe { *t.as_int().get_unchecked(idx) }
    } else {
        0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r72 };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = i_r0;
    }
    let k = 349;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r73 };
    i_r0 = if idx < t.as_int().len() {
        unsafe { *t.as_int().get_unchecked(idx) }
    } else {
        0
    };
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r72 };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = i_r0;
    }
    let k = 99;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r73 };
    i_r0 = if idx < t.as_int().len() {
        unsafe { *t.as_int().get_unchecked(idx) }
    } else {
        0
    };
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r72 };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = i_r0;
    }
    let mut new_table = Box::new(Table::new());
    t_r72 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r0 = 0;
    loop {
        b_r48 = i_r0 < 2000;
        if b_r48 {
            i_r2 = 0;
            loop {
                b_r48 = i_r2 < 2000;
                if b_r48 {
                    let lim = 2000;
                    if lim > 0 {
                        let t = unsafe { &mut *t_r72 };
                        if (lim as usize) > t.as_int_mut().len() {
                            t.as_int_mut().resize(lim as usize, 0);
                        }
                    }
                    len_r72 = unsafe { (*t_r72).as_int_mut().len() };
                    p_r72 = unsafe { (*t_r72).as_int_mut().as_mut_ptr() };
                    i_r1 = 0;
                    loop {
                        b_r48 = i_r1 < 2000;
                        if b_r48 {
                            i_r4 = i_r1 + i_r2;
                            let k = i_r1;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r72 {
                                unsafe {
                                    *p_r72.add(k as usize) = i_r4;
                                }
                            } else {
                                panic!(
                                    "optimizer invariant violated: fast-path bounds check failed"
                                );
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
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r73 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 8000;
    if lim > 0 {
        let t = unsafe { &mut *t_r73 };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    len_r73 = unsafe { (*t_r73).as_int_mut().len() };
    p_r73 = unsafe { (*t_r73).as_int_mut().as_mut_ptr() };
    i_r3 = 0;
    loop {
        b_r48 = i_r3 < 8000;
        if b_r48 {
            i_r4 = i_r3;
            let k = i_r4;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r73 {
                unsafe {
                    *p_r73.add(k as usize) = 1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r1 = i_r4 + 0;
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r73 {
                unsafe {
                    *p_r73.add(k as usize) = 2;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r1 = i_r3;
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r73 {
                unsafe {
                    *p_r73.add(k as usize) = 3;
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
    t_r73 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 2000000;
    if lim > 0 {
        let t = unsafe { &mut *t_r73 };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    len_r73 = unsafe { (*t_r73).as_int_mut().len() };
    p_r73 = unsafe { (*t_r73).as_int_mut().as_mut_ptr() };
    i_r1 = 0;
    loop {
        b_r48 = i_r1 < 2000000;
        if b_r48 {
            i_r3 = i_r1 + 1;
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r73 {
                unsafe {
                    *p_r73.add(k as usize) = i_r3;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r3 = i_r1 + 2;
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r73 {
                unsafe {
                    *p_r73.add(k as usize) = i_r3;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=17;fast_gets=3;dyn_sets=9;dyn_gets=5;hoists=16;hoist_ctx=0,1,0,0,0,0,1,0,0,0,0,0,0,0,2,0;consts_i=96;consts_b=1;consts_f=0;consts_s=0";
