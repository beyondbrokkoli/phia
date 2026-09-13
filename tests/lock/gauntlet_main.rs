// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r326 = 0i64;
    let mut i_r327 = 0i64;
    let mut i_r328 = 0i64;
    let mut i_r329 = 0i64;
    let mut i_r330 = 0i64;
    let mut b_r326 = false;
    let mut t_r326: *mut Table = std::ptr::null_mut();
    let mut p_r326: *mut i64 = std::ptr::null_mut();
    let mut len_r326 = 0usize;
    let mut t_r327: *mut Table = std::ptr::null_mut();
    let mut p_r327: *mut i64 = std::ptr::null_mut();
    let mut len_r327 = 0usize;
    let mut t_r328: *mut Table = std::ptr::null_mut();
    let mut p_r328: *mut i64 = std::ptr::null_mut();
    let mut len_r328 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r326 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 8000000;
    if lim > 0 {
        let t = unsafe { &mut *t_r326 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r326 = unsafe { (*t_r326).array.len() };
    p_r326 = unsafe { (*t_r326).array.as_mut_ptr() };
    i_r326 = 0;
    loop {
        b_r326 = i_r326 < 8000000;
        if b_r326 {
            i_r327 = i_r326 + 1;
            let k = i_r326;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r326 {
                unsafe {
                    *p_r326.add(k as usize) = i_r327;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r326 = i_r326 + 2;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r326 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r327 = 0;
    loop {
        b_r326 = i_r327 < 100000;
        if b_r326 {
            let lim = i_r327;
            if lim > 0 {
                let t = unsafe { &mut *t_r326 };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            len_r326 = unsafe { (*t_r326).array.len() };
            p_r326 = unsafe { (*t_r326).array.as_mut_ptr() };
            i_r326 = 0;
            loop {
                b_r326 = i_r326 < i_r327;
                if b_r326 {
                    i_r329 = i_r326 + 1;
                    let k = i_r326;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r326 {
                        unsafe {
                            *p_r326.add(k as usize) = i_r329;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r326 = i_r326 + 1;
                } else {
                    break;
                }
            }
            i_r327 = i_r327 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r327 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 2500000;
    if lim > 0 {
        let t = unsafe { &mut *t_r327 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r327 = unsafe { (*t_r327).array.len() };
    p_r327 = unsafe { (*t_r327).array.as_mut_ptr() };
    i_r328 = 0;
    loop {
        b_r326 = i_r328 < 2500000;
        if b_r326 {
            let k = i_r328;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r327 {
                unsafe {
                    *p_r327.add(k as usize) = 5;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r328 = i_r328 + 1;
        } else {
            break;
        }
    }
    let lim = 2500000;
    if lim > 0 {
        let t = unsafe { &mut *t_r327 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r327 = unsafe { (*t_r327).array.len() };
    p_r327 = unsafe { (*t_r327).array.as_mut_ptr() };
    i_r329 = 0;
    loop {
        b_r326 = i_r329 < 2500000;
        if b_r326 {
            let k = i_r329;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r327 {
                i_r328 = unsafe { *p_r327.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r326 = i_r328 + 1;
            let k = i_r329;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r327 {
                unsafe {
                    *p_r327.add(k as usize) = i_r326;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r329 = i_r329 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r327 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 30000;
    if lim > 0 {
        let t = unsafe { &mut *t_r327 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r327 = unsafe { (*t_r327).array.len() };
    p_r327 = unsafe { (*t_r327).array.as_mut_ptr() };
    i_r326 = 0;
    loop {
        b_r326 = i_r326 < 30000;
        if b_r326 {
            let k = i_r326;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r327 {
                unsafe {
                    *p_r327.add(k as usize) = 1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r326 = i_r326 + 1;
        } else {
            break;
        }
    }
    let k = 30;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r327 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 60;
    }
    i_r326 = 0;
    loop {
        let k = i_r326;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &*t_r327 };
        i_r328 = if idx < t.array.len() {
            unsafe { *t.array.get_unchecked(idx) }
        } else {
            0
        };
        b_r326 = i_r328 < 60;
        if b_r326 {
            i_r326 = i_r326 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r327 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r327 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r326;
    }
    let mut new_table = Box::new(Table::new());
    t_r327 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 2000000;
    if lim > 0 {
        let t = unsafe { &mut *t_r327 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r327 = unsafe { (*t_r327).array.len() };
    p_r327 = unsafe { (*t_r327).array.as_mut_ptr() };
    i_r328 = 0;
    loop {
        b_r326 = i_r328 < 2000000;
        if b_r326 {
            i_r326 = i_r328;
            i_r329 = 0;
            loop {
                b_r326 = i_r329 < 3;
                if b_r326 {
                    let k = i_r326;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r327 {
                        unsafe {
                            *p_r327.add(k as usize) = 7;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r329 = i_r329 + 1;
                } else {
                    break;
                }
            }
            i_r328 = i_r328 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r326 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r327 = 0;
    loop {
        b_r326 = i_r327 < 60000;
        if b_r326 {
            i_r329 = i_r327;
            let lim = 60000;
            if lim > 0 {
                let t = unsafe { &mut *t_r326 };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            len_r326 = unsafe { (*t_r326).array.len() };
            p_r326 = unsafe { (*t_r326).array.as_mut_ptr() };
            i_r326 = i_r329;
            loop {
                b_r326 = i_r326 < 60000;
                if b_r326 {
                    i_r328 = i_r326 + 1;
                    let k = i_r326;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r326 {
                        unsafe {
                            *p_r326.add(k as usize) = i_r328;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r326 = i_r326 + 1;
                } else {
                    break;
                }
            }
            i_r327 = i_r327 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r327 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r328 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 180;
    if lim > 0 {
        let t = unsafe { &mut *t_r327 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r327 = unsafe { (*t_r327).array.len() };
    p_r327 = unsafe { (*t_r327).array.as_mut_ptr() };
    i_r329 = 0;
    loop {
        b_r326 = i_r329 < 180;
        if b_r326 {
            i_r328 = i_r329 + 1;
            let k = i_r329;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r327 {
                unsafe {
                    *p_r327.add(k as usize) = i_r328;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r329 = i_r329 + 1;
        } else {
            break;
        }
    }
    let lim = 180;
    if lim > 0 {
        let t = unsafe { &mut *t_r327 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r327 = unsafe { (*t_r327).array.len() };
    p_r327 = unsafe { (*t_r327).array.as_mut_ptr() };
    i_r328 = 0;
    loop {
        b_r326 = i_r328 < 180;
        if b_r326 {
            i_r329 = 179 - i_r328;
            let k = i_r328;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r327 {
                i_r326 = unsafe { *p_r327.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = i_r329;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r328 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = i_r326;
            }
            i_r328 = i_r328 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r328 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 1000000;
    if lim > 0 {
        let t = unsafe { &mut *t_r328 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r328 = unsafe { (*t_r328).array.len() };
    p_r328 = unsafe { (*t_r328).array.as_mut_ptr() };
    i_r326 = 0;
    loop {
        b_r326 = i_r326 < 1000000;
        if b_r326 {
            i_r329 = i_r326 + 1;
            let k = i_r326;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = i_r329;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r326 = i_r326 + 1;
        } else {
            break;
        }
    }
    let lim = 1000000;
    if lim > 0 {
        let t = unsafe { &mut *t_r328 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r328 = unsafe { (*t_r328).array.len() };
    p_r328 = unsafe { (*t_r328).array.as_mut_ptr() };
    i_r329 = 0;
    i_r326 = 0;
    loop {
        b_r326 = i_r326 < 1000000;
        if b_r326 {
            let k = i_r326;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                i_r328 = unsafe { *p_r328.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r329 = i_r329 + i_r328;
            i_r326 = i_r326 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r328 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r328 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r329;
    }
    let mut new_table = Box::new(Table::new());
    t_r328 = &mut *new_table as *mut Table;
    tables.push(new_table);
    b_r326 = true;
    i_r328 = 0;
    while b_r326 {
        let k = i_r328;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r328 };
        if idx >= t.array.len() {
            t.array.resize(idx + 1, 0);
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = 100;
        }
        b_r326 = 0 < 0;
        i_r328 = i_r328 + 1;
    }
    let mut new_table = Box::new(Table::new());
    t_r328 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 200;
    if lim > 0 {
        let t = unsafe { &mut *t_r328 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r328 = unsafe { (*t_r328).array.len() };
    p_r328 = unsafe { (*t_r328).array.as_mut_ptr() };
    i_r328 = 100;
    loop {
        b_r326 = i_r328 < 200;
        if b_r326 {
            i_r326 = i_r328 - 100;
            let k = i_r328;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = i_r326;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r328 = i_r328 + 1;
        } else {
            break;
        }
    }
    let lim = 350;
    if lim > 0 {
        let t = unsafe { &mut *t_r328 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r328 = unsafe { (*t_r328).array.len() };
    p_r328 = unsafe { (*t_r328).array.as_mut_ptr() };
    i_r326 = 200;
    loop {
        b_r326 = i_r326 < 350;
        if b_r326 {
            let k = i_r326;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = i_r326;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r326 = i_r326 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r327 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 150;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r328 };
    i_r326 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r327 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r326;
    }
    let k = 250;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r328 };
    i_r326 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r327 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r326;
    }
    let k = 349;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r328 };
    i_r326 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r327 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r326;
    }
    let k = 99;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r328 };
    i_r326 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r327 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r326;
    }
    let mut new_table = Box::new(Table::new());
    t_r327 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r326 = 0;
    loop {
        b_r326 = i_r326 < 2000;
        if b_r326 {
            i_r328 = 0;
            loop {
                b_r326 = i_r328 < 2000;
                if b_r326 {
                    let lim = 2000;
                    if lim > 0 {
                        let t = unsafe { &mut *t_r327 };
                        if (lim as usize) > t.array.len() {
                            t.array.resize(lim as usize, 0);
                        }
                    }
                    len_r327 = unsafe { (*t_r327).array.len() };
                    p_r327 = unsafe { (*t_r327).array.as_mut_ptr() };
                    i_r327 = 0;
                    loop {
                        b_r326 = i_r327 < 2000;
                        if b_r326 {
                            i_r330 = i_r327 + i_r328;
                            let k = i_r327;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r327 {
                                unsafe {
                                    *p_r327.add(k as usize) = i_r330;
                                }
                            } else {
                                panic!(
                                    "optimizer invariant violated: fast-path bounds check failed"
                                );
                            }
                            i_r327 = i_r327 + 1;
                        } else {
                            break;
                        }
                    }
                    i_r328 = i_r328 + 1;
                } else {
                    break;
                }
            }
            i_r326 = i_r326 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r328 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 8000;
    if lim > 0 {
        let t = unsafe { &mut *t_r328 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r328 = unsafe { (*t_r328).array.len() };
    p_r328 = unsafe { (*t_r328).array.as_mut_ptr() };
    i_r329 = 0;
    loop {
        b_r326 = i_r329 < 8000;
        if b_r326 {
            i_r330 = i_r329;
            let k = i_r330;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = 1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r327 = i_r330 + 0;
            let k = i_r327;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = 2;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r327 = i_r329;
            let k = i_r327;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = 3;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r329 = i_r329 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r328 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 2000000;
    if lim > 0 {
        let t = unsafe { &mut *t_r328 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r328 = unsafe { (*t_r328).array.len() };
    p_r328 = unsafe { (*t_r328).array.as_mut_ptr() };
    i_r327 = 0;
    loop {
        b_r326 = i_r327 < 2000000;
        if b_r326 {
            i_r329 = i_r327 + 1;
            let k = i_r327;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = i_r329;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r329 = i_r327 + 2;
            let k = i_r327;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = i_r329;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r327 = i_r327 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=17;fast_gets=3;dyn_sets=9;dyn_gets=5;hoists=16;hoist_ctx=0,1,0,0,0,0,1,0,0,0,0,0,0,0,2,0";
