// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r375 = 0i64;
    let mut i_r376 = 0i64;
    let mut i_r377 = 0i64;
    let mut i_r378 = 0i64;
    let mut b_r375 = false;
    let mut f_r401 = 0f64;
    let mut f_r402 = 0f64;
    let mut f_r403 = 0f64;
    let mut f_r404 = 0f64;
    let mut t_r375 = 0i64;
    let mut p_r375: *mut i64 = std::ptr::null_mut();
    let mut len_r375 = 0usize;
    let mut t_r376 = 0i64;
    let mut p_r376: *mut i64 = std::ptr::null_mut();
    let mut len_r376 = 0usize;
    let mut t_r377 = 0i64;
    let mut p_r377: *mut i64 = std::ptr::null_mut();
    let mut len_r377 = 0usize;
    let mut t_r431 = 0i64;
    let mut p_r431: *mut f64 = std::ptr::null_mut();
    let mut len_r431 = 0usize;
    let mut t_r432 = 0i64;
    let mut p_r432: *mut f64 = std::ptr::null_mut();
    let mut len_r432 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    f_r401 = 0.5;
    let lim = 65536;
    if lim > 0 {
        if t_r431 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r431 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r431 = t.farray.len();
    p_r431 = t.farray.as_mut_ptr();
    f_r402 = f_r401;
    i_r375 = 0;
    loop {
        b_r375 = i_r375 < 65536;
        if b_r375 {
            let k = i_r375;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r431 {
                unsafe {
                    *p_r431.add(k as usize) = f_r402;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            f_r401 = 0.25;
            f_r402 = f_r402 + f_r401;
            i_r375 = i_r375 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r375 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r375 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r375 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r431;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r375 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r375 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r431 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 8;
    if lim > 0 {
        if t_r431 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r431 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r431 = t.farray.len();
    p_r431 = t.farray.as_mut_ptr();
    i_r375 = 0;
    loop {
        b_r375 = i_r375 < 8;
        if b_r375 {
            f_r401 = 1.75;
            let k = i_r375;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r431 {
                unsafe {
                    *p_r431.add(k as usize) = f_r401;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r375 = i_r375 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r375 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r376 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    f_r401 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r401;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r376 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r376 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r431;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r375 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r375 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r376;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r375 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r375 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r376 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r376 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r376 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r431 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 16;
    if lim > 0 {
        if t_r431 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r431 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r431 = t.farray.len();
    p_r431 = t.farray.as_mut_ptr();
    i_r375 = 0;
    loop {
        b_r375 = i_r375 < 16;
        if b_r375 {
            f_r401 = 0.125;
            let k = i_r375;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r431 {
                unsafe {
                    *p_r431.add(k as usize) = f_r401;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r375 = i_r375 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r376 = tables.len() as i64;
    let lim = 32;
    if lim > 0 {
        if t_r376 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r376 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r376 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r376 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r376 = t.array.len();
    p_r376 = t.array.as_mut_ptr();
    i_r375 = 0;
    loop {
        b_r375 = i_r375 < 32;
        if b_r375 {
            tables.push(Box::new(Table::new_float()));
            t_r431 = tables.len() as i64;
            let k = i_r375;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r376 {
                unsafe {
                    *p_r376.add(k as usize) = t_r431;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r375 = i_r375 + 1;
        } else {
            break;
        }
    }
    i_r375 = 0;
    loop {
        b_r375 = i_r375 < 32;
        if b_r375 {
            let k = i_r375;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r376 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r376 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r431 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let lim = 1024;
            if lim > 0 {
                if t_r431 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r431 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r431 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r431 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r431 = t.farray.len();
            p_r431 = t.farray.as_mut_ptr();
            i_r376 = 0;
            loop {
                b_r375 = i_r376 < 1024;
                if b_r375 {
                    f_r401 = 0.25;
                    let k = i_r376;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r431 {
                        unsafe {
                            *p_r431.add(k as usize) = f_r401;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r376 = i_r376 + 1;
                } else {
                    break;
                }
            }
            i_r375 = i_r375 + 1;
        } else {
            break;
        }
    }
    f_r401 = 0.0;
    let lim = 32;
    if lim > 0 {
        if t_r376 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r376 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r376 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r376 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r376 = t.array.len();
    p_r376 = t.array.as_mut_ptr();
    f_r402 = f_r401;
    i_r377 = 0;
    loop {
        b_r375 = i_r377 < 32;
        if b_r375 {
            i_r376 = 0;
            loop {
                b_r375 = i_r376 < 1024;
                if b_r375 {
                    let k = i_r377;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r376 {
                        t_r431 = unsafe { *p_r376.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    let k = i_r376;
                    if k < 0 {
                        panic!("Runtime Error: Negative table index");
                    }
                    let idx = k as usize;
                    if t_r431 == 0 {
                        panic!("Runtime Error: table is nil");
                    }
                    let t = match tables.get((t_r431 - 1) as usize) {
                        Some(t) => &**t,
                        None => panic!("Runtime Error: table is nil"),
                    };
                    f_r401 = if idx < t.farray.len() {
                        unsafe { *t.farray.get_unchecked(idx) }
                    } else {
                        0.0
                    };
                    f_r402 = f_r402 + f_r401;
                    i_r376 = i_r376 + 1;
                } else {
                    break;
                }
            }
            i_r377 = i_r377 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r402;
    }
    tables.push(Box::new(Table::new()));
    t_r375 = tables.len() as i64;
    let lim = 48;
    if lim > 0 {
        if t_r375 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r375 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r375 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r375 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r375 = t.array.len();
    p_r375 = t.array.as_mut_ptr();
    i_r375 = 0;
    loop {
        b_r375 = i_r375 < 48;
        if b_r375 {
            i_r376 = i_r375 + i_r375;
            let k = i_r375;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r375 {
                unsafe {
                    *p_r375.add(k as usize) = i_r376;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r375 = i_r375 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    let lim = 48;
    if lim > 0 {
        if t_r431 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r431 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r431 = t.farray.len();
    p_r431 = t.farray.as_mut_ptr();
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < 48;
        if b_r375 {
            f_r401 = 0.5;
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r431 {
                unsafe {
                    *p_r431.add(k as usize) = f_r401;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r375 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r375 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r375 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 7;
    }
    tables.push(Box::new(Table::new()));
    t_r377 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    f_r401 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r401;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r377 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r377 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r431;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r375 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r375 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    i_r376 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    i_r375 = i_r376 + 1;
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < i_r375;
        if b_r375 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r377 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r377 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r431 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            f_r401 = 0.75;
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r431 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r431 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = f_r401;
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r377 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    f_r401 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r401;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r377 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r377 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r431;
    }
    i_r376 = 100;
    loop {
        b_r375 = i_r376 < 8;
        if b_r375 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r377 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r377 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r431 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            f_r401 = 0.5;
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r431 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r431 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = f_r401;
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r377 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    f_r401 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r401;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r377 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r377 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r431;
    }
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    f_r401 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r401;
    }
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < 8;
        if b_r375 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r377 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r377 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r432 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            f_r401 = 0.625;
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r432 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r432 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = f_r401;
            }
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r377 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r377 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = t_r431;
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r377 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r432 = tables.len() as i64;
    f_r401 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r432 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r432 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r401;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r377 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r377 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r432;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r377 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r377 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r432 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 8;
    if lim > 0 {
        if t_r432 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r432 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r432 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r432 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r432 = t.farray.len();
    p_r432 = t.farray.as_mut_ptr();
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < 8;
        if b_r375 {
            f_r401 = 0.375;
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r432 {
                unsafe {
                    *p_r432.add(k as usize) = f_r401;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            f_r401 = 0.875;
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r432 {
                unsafe {
                    *p_r432.add(k as usize) = f_r401;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r377 = tables.len() as i64;
    let lim = 4;
    if lim > 0 {
        if t_r377 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r377 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r377 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r377 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r377 = t.array.len();
    p_r377 = t.array.as_mut_ptr();
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < 4;
        if b_r375 {
            tables.push(Box::new(Table::new()));
            t_r375 = tables.len() as i64;
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r377 {
                unsafe {
                    *p_r377.add(k as usize) = t_r375;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < 4;
        if b_r375 {
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r377 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r377 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r375 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            tables.push(Box::new(Table::new_float()));
            t_r432 = tables.len() as i64;
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r375 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r375 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = t_r432;
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < 4;
        if b_r375 {
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r377 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r377 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r375 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r375 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r375 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r432 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let lim = 8;
            if lim > 0 {
                if t_r432 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r432 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r432 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r432 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r432 = t.farray.len();
            p_r432 = t.farray.as_mut_ptr();
            i_r375 = 0;
            loop {
                b_r375 = i_r375 < 8;
                if b_r375 {
                    f_r401 = 0.0625;
                    let k = i_r375;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r432 {
                        unsafe {
                            *p_r432.add(k as usize) = f_r401;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r375 = i_r375 + 1;
                } else {
                    break;
                }
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    i_r377 = 0;
    loop {
        b_r375 = i_r377 < 2;
        if b_r375 {
            i_r375 = 0;
            loop {
                b_r375 = i_r375 < 32;
                if b_r375 {
                    let k = i_r375;
                    if k < 0 {
                        panic!("Runtime Error: Negative table index");
                    }
                    let idx = k as usize;
                    if t_r376 == 0 {
                        panic!("Runtime Error: table is nil");
                    }
                    let t = match tables.get((t_r376 - 1) as usize) {
                        Some(t) => &**t,
                        None => panic!("Runtime Error: table is nil"),
                    };
                    t_r432 = if idx < t.array.len() {
                        unsafe { *t.array.get_unchecked(idx) }
                    } else {
                        0
                    };
                    let lim = 1024;
                    if lim > 0 {
                        if t_r432 == 0 {
                            panic!("Runtime Error: table is nil");
                        }
                        let t = match tables.get_mut((t_r432 - 1) as usize) {
                            Some(t) => &mut **t,
                            None => panic!("Runtime Error: table is nil"),
                        };
                        if (lim as usize) > t.farray.len() {
                            t.farray.resize(lim as usize, 0.0);
                        }
                    }
                    if t_r432 == 0 {
                        panic!("Runtime Error: table is nil");
                    }
                    let t = match tables.get_mut((t_r432 - 1) as usize) {
                        Some(t) => &mut **t,
                        None => panic!("Runtime Error: table is nil"),
                    };
                    len_r432 = t.farray.len();
                    p_r432 = t.farray.as_mut_ptr();
                    i_r378 = 0;
                    loop {
                        b_r375 = i_r378 < 1024;
                        if b_r375 {
                            let k = i_r378;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r432 {
                                f_r401 = unsafe { *p_r432.add(k as usize) };
                            } else {
                                panic!(
                                    "optimizer invariant violated: fast-path bounds check failed"
                                );
                            }
                            f_r403 = 0.25;
                            f_r404 = f_r401 + f_r403;
                            let k = i_r378;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r432 {
                                unsafe {
                                    *p_r432.add(k as usize) = f_r404;
                                }
                            } else {
                                panic!(
                                    "optimizer invariant violated: fast-path bounds check failed"
                                );
                            }
                            i_r378 = i_r378 + 1;
                        } else {
                            break;
                        }
                    }
                    i_r375 = i_r375 + 1;
                } else {
                    break;
                }
            }
            i_r377 = i_r377 + 1;
        } else {
            break;
        }
    }
    f_r401 = 0.0;
    let lim = 32;
    if lim > 0 {
        if t_r376 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r376 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r376 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r376 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r376 = t.array.len();
    p_r376 = t.array.as_mut_ptr();
    f_r402 = f_r401;
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < 32;
        if b_r375 {
            i_r378 = 0;
            loop {
                b_r375 = i_r378 < 1024;
                if b_r375 {
                    let k = i_r376;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r376 {
                        t_r431 = unsafe { *p_r376.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    let k = i_r378;
                    if k < 0 {
                        panic!("Runtime Error: Negative table index");
                    }
                    let idx = k as usize;
                    if t_r431 == 0 {
                        panic!("Runtime Error: table is nil");
                    }
                    let t = match tables.get((t_r431 - 1) as usize) {
                        Some(t) => &**t,
                        None => panic!("Runtime Error: table is nil"),
                    };
                    f_r404 = if idx < t.farray.len() {
                        unsafe { *t.farray.get_unchecked(idx) }
                    } else {
                        0.0
                    };
                    f_r402 = f_r402 + f_r404;
                    i_r378 = i_r378 + 1;
                } else {
                    break;
                }
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new_float()));
    t_r432 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r376 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r376 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r431 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r431 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    f_r404 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r432 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r432 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r404;
    }
    let k = 31;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r376 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r376 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r431 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 1023;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r431 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    f_r404 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r432 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r432 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r404;
    }
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r432 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r432 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r402;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=12;fast_gets=3;dyn_sets=23;dyn_gets=19;hoists=13;hoist_ctx=0,0,0,0,1,0,0,0,0,0,1,0,2";
