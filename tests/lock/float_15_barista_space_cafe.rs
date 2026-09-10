// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r377 = 0i64;
    let mut i_r378 = 0i64;
    let mut i_r379 = 0i64;
    let mut i_r380 = 0i64;
    let mut b_r377 = false;
    let mut f_r403 = 0f64;
    let mut f_r404 = 0f64;
    let mut f_r405 = 0f64;
    let mut f_r406 = 0f64;
    let mut t_r377 = 0i64;
    let mut p_r377: *mut i64 = std::ptr::null_mut();
    let mut len_r377 = 0usize;
    let mut t_r378 = 0i64;
    let mut p_r378: *mut i64 = std::ptr::null_mut();
    let mut len_r378 = 0usize;
    let mut t_r379 = 0i64;
    let mut p_r379: *mut i64 = std::ptr::null_mut();
    let mut len_r379 = 0usize;
    let mut t_r433 = 0i64;
    let mut p_r433: *mut f64 = std::ptr::null_mut();
    let mut len_r433 = 0usize;
    let mut t_r434 = 0i64;
    let mut p_r434: *mut f64 = std::ptr::null_mut();
    let mut len_r434 = 0usize;
    let mut t_r435 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new_float()));
    t_r433 = tables.len() as i64;
    f_r403 = 0.5;
    let lim = 150000000;
    if lim > 0 {
        if t_r433 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r433 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r433 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r433 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r433 = t.farray.len();
    p_r433 = t.farray.as_mut_ptr();
    f_r404 = f_r403;
    i_r377 = 0;
    loop {
        b_r377 = i_r377 < 150000000;
        if b_r377 {
            let k = i_r377;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r433 {
                unsafe {
                    *p_r433.add(k as usize) = f_r404;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            f_r403 = 0.25;
            f_r404 = f_r404 + f_r403;
            i_r377 = i_r377 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r377 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r433 = tables.len() as i64;
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
        *t.array.get_unchecked_mut(idx) = t_r433;
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
    t_r433 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 10000000;
    if lim > 0 {
        if t_r433 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r433 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r433 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r433 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r433 = t.farray.len();
    p_r433 = t.farray.as_mut_ptr();
    i_r377 = 0;
    loop {
        b_r377 = i_r377 < 10000000;
        if b_r377 {
            f_r403 = 1.75;
            let k = i_r377;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r433 {
                unsafe {
                    *p_r433.add(k as usize) = f_r403;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r377 = i_r377 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r377 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r378 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r433 = tables.len() as i64;
    f_r403 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r433 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r433 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r403;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r378 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r378 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r433;
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
        *t.array.get_unchecked_mut(idx) = t_r378;
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
    t_r378 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r378 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r378 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r433 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 25000000;
    if lim > 0 {
        if t_r433 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r433 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r433 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r433 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r433 = t.farray.len();
    p_r433 = t.farray.as_mut_ptr();
    i_r377 = 0;
    loop {
        b_r377 = i_r377 < 25000000;
        if b_r377 {
            f_r403 = 0.125;
            let k = i_r377;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r433 {
                unsafe {
                    *p_r433.add(k as usize) = f_r403;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r377 = i_r377 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r378 = tables.len() as i64;
    let lim = 5000;
    if lim > 0 {
        if t_r378 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r378 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r378 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r378 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r378 = t.array.len();
    p_r378 = t.array.as_mut_ptr();
    i_r377 = 0;
    loop {
        b_r377 = i_r377 < 5000;
        if b_r377 {
            tables.push(Box::new(Table::new_float()));
            t_r433 = tables.len() as i64;
            let k = i_r377;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r378 {
                unsafe {
                    *p_r378.add(k as usize) = t_r433;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r377 = i_r377 + 1;
        } else {
            break;
        }
    }
    let lim = 5000;
    if lim > 0 {
        if t_r378 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r378 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r378 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r378 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r378 = t.array.len();
    p_r378 = t.array.as_mut_ptr();
    i_r377 = 0;
    loop {
        b_r377 = i_r377 < 5000;
        if b_r377 {
            let k = i_r377;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r378 {
                t_r433 = unsafe { *p_r378.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let lim = 5000;
            if lim > 0 {
                if t_r433 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r433 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r433 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r433 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r433 = t.farray.len();
            p_r433 = t.farray.as_mut_ptr();
            i_r378 = 0;
            loop {
                b_r377 = i_r378 < 5000;
                if b_r377 {
                    f_r403 = 0.25;
                    let k = i_r378;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r433 {
                        unsafe {
                            *p_r433.add(k as usize) = f_r403;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r378 = i_r378 + 1;
                } else {
                    break;
                }
            }
            i_r377 = i_r377 + 1;
        } else {
            break;
        }
    }
    f_r403 = 0.0;
    let lim = 5000;
    if lim > 0 {
        if t_r378 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r378 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r378 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r378 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r378 = t.array.len();
    p_r378 = t.array.as_mut_ptr();
    f_r404 = f_r403;
    i_r379 = 0;
    loop {
        b_r377 = i_r379 < 5000;
        if b_r377 {
            let k = i_r379;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r378 {
                t_r433 = unsafe { *p_r378.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let lim = 5000;
            if lim > 0 {
                if t_r433 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r433 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r433 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r433 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r433 = t.farray.len();
            p_r433 = t.farray.as_mut_ptr();
            i_r378 = 0;
            loop {
                b_r377 = i_r378 < 5000;
                if b_r377 {
                    let k = i_r378;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r433 {
                        f_r403 = unsafe { *p_r433.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    f_r404 = f_r404 + f_r403;
                    i_r378 = i_r378 + 1;
                } else {
                    break;
                }
            }
            i_r379 = i_r379 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new_float()));
    t_r434 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r434 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r434 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r404;
    }
    tables.push(Box::new(Table::new()));
    t_r377 = tables.len() as i64;
    let lim = 15000000;
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
    i_r377 = 0;
    loop {
        b_r377 = i_r377 < 15000000;
        if b_r377 {
            i_r378 = i_r377 + i_r377;
            let k = i_r377;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r377 {
                unsafe {
                    *p_r377.add(k as usize) = i_r378;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r377 = i_r377 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new_float()));
    t_r434 = tables.len() as i64;
    let lim = 15000000;
    if lim > 0 {
        if t_r434 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r434 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r434 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r434 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r434 = t.farray.len();
    p_r434 = t.farray.as_mut_ptr();
    i_r378 = 0;
    loop {
        b_r377 = i_r378 < 15000000;
        if b_r377 {
            f_r403 = 0.5;
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r434 {
                unsafe {
                    *p_r434.add(k as usize) = f_r403;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r378 = i_r378 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r377 = tables.len() as i64;
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
        *t.array.get_unchecked_mut(idx) = 11999999;
    }
    tables.push(Box::new(Table::new()));
    t_r379 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r434 = tables.len() as i64;
    f_r403 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r434 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r434 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r403;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r379 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r379 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r434;
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
    i_r378 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    i_r377 = i_r378 + 1;
    i_r378 = 0;
    loop {
        b_r377 = i_r378 < i_r377;
        if b_r377 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r379 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r379 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r434 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            f_r403 = 0.75;
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r434 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r434 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = f_r403;
            }
            i_r378 = i_r378 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r379 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r434 = tables.len() as i64;
    f_r403 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r434 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r434 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r403;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r379 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r379 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r434;
    }
    i_r378 = 100;
    loop {
        b_r377 = i_r378 < 8;
        if b_r377 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r379 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r379 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r434 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            f_r403 = 0.5;
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r434 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r434 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = f_r403;
            }
            i_r378 = i_r378 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r379 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r434 = tables.len() as i64;
    f_r403 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r434 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r434 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r403;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r379 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r379 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r434;
    }
    tables.push(Box::new(Table::new_float()));
    t_r434 = tables.len() as i64;
    f_r403 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r434 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r434 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r403;
    }
    i_r378 = 0;
    loop {
        b_r377 = i_r378 < 10000000;
        if b_r377 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r379 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r379 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r433 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            f_r403 = 0.625;
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r433 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r433 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = f_r403;
            }
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r379 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r379 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = t_r434;
            }
            i_r378 = i_r378 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r379 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r433 = tables.len() as i64;
    f_r403 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r433 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r433 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r403;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r379 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r379 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r433;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r379 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r379 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r433 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 12000000;
    if lim > 0 {
        if t_r433 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r433 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r433 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r433 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r433 = t.farray.len();
    p_r433 = t.farray.as_mut_ptr();
    i_r378 = 0;
    loop {
        b_r377 = i_r378 < 12000000;
        if b_r377 {
            f_r403 = 0.375;
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r433 {
                unsafe {
                    *p_r433.add(k as usize) = f_r403;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            f_r403 = 0.875;
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r433 {
                unsafe {
                    *p_r433.add(k as usize) = f_r403;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r378 = i_r378 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r379 = tables.len() as i64;
    let lim = 1024;
    if lim > 0 {
        if t_r379 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r379 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r379 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r379 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r379 = t.array.len();
    p_r379 = t.array.as_mut_ptr();
    i_r378 = 0;
    loop {
        b_r377 = i_r378 < 1024;
        if b_r377 {
            tables.push(Box::new(Table::new()));
            t_r377 = tables.len() as i64;
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r379 {
                unsafe {
                    *p_r379.add(k as usize) = t_r377;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r378 = i_r378 + 1;
        } else {
            break;
        }
    }
    i_r378 = 0;
    loop {
        b_r377 = i_r378 < 1024;
        if b_r377 {
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r379 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r379 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r377 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            tables.push(Box::new(Table::new_float()));
            t_r433 = tables.len() as i64;
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
                *t.array.get_unchecked_mut(idx) = t_r433;
            }
            i_r378 = i_r378 + 1;
        } else {
            break;
        }
    }
    i_r378 = 0;
    loop {
        b_r377 = i_r378 < 1024;
        if b_r377 {
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r379 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r379 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r377 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
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
            t_r433 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let lim = 8192;
            if lim > 0 {
                if t_r433 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r433 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r433 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r433 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r433 = t.farray.len();
            p_r433 = t.farray.as_mut_ptr();
            i_r377 = 0;
            loop {
                b_r377 = i_r377 < 8192;
                if b_r377 {
                    f_r403 = 0.0625;
                    let k = i_r377;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r433 {
                        unsafe {
                            *p_r433.add(k as usize) = f_r403;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r377 = i_r377 + 1;
                } else {
                    break;
                }
            }
            i_r378 = i_r378 + 1;
        } else {
            break;
        }
    }
    i_r379 = 0;
    loop {
        b_r377 = i_r379 < 12;
        if b_r377 {
            let lim = 5000;
            if lim > 0 {
                if t_r378 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r378 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            if t_r378 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r378 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r378 = t.array.len();
            p_r378 = t.array.as_mut_ptr();
            i_r377 = 0;
            loop {
                b_r377 = i_r377 < 5000;
                if b_r377 {
                    let k = i_r377;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r378 {
                        t_r433 = unsafe { *p_r378.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    let lim = 5000;
                    if lim > 0 {
                        if t_r433 == 0 {
                            panic!("Runtime Error: table is nil");
                        }
                        let t = match tables.get_mut((t_r433 - 1) as usize) {
                            Some(t) => &mut **t,
                            None => panic!("Runtime Error: table is nil"),
                        };
                        if (lim as usize) > t.farray.len() {
                            t.farray.resize(lim as usize, 0.0);
                        }
                    }
                    if t_r433 == 0 {
                        panic!("Runtime Error: table is nil");
                    }
                    let t = match tables.get_mut((t_r433 - 1) as usize) {
                        Some(t) => &mut **t,
                        None => panic!("Runtime Error: table is nil"),
                    };
                    len_r433 = t.farray.len();
                    p_r433 = t.farray.as_mut_ptr();
                    i_r380 = 0;
                    loop {
                        b_r377 = i_r380 < 5000;
                        if b_r377 {
                            let k = i_r380;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r433 {
                                f_r403 = unsafe { *p_r433.add(k as usize) };
                            } else {
                                panic!(
                                    "optimizer invariant violated: fast-path bounds check failed"
                                );
                            }
                            f_r405 = 0.25;
                            f_r406 = f_r403 + f_r405;
                            let k = i_r380;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r433 {
                                unsafe {
                                    *p_r433.add(k as usize) = f_r406;
                                }
                            } else {
                                panic!(
                                    "optimizer invariant violated: fast-path bounds check failed"
                                );
                            }
                            i_r380 = i_r380 + 1;
                        } else {
                            break;
                        }
                    }
                    i_r377 = i_r377 + 1;
                } else {
                    break;
                }
            }
            i_r379 = i_r379 + 1;
        } else {
            break;
        }
    }
    f_r403 = 0.0;
    let lim = 5000;
    if lim > 0 {
        if t_r378 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r378 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r378 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r378 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r378 = t.array.len();
    p_r378 = t.array.as_mut_ptr();
    f_r404 = f_r403;
    i_r378 = 0;
    loop {
        b_r377 = i_r378 < 5000;
        if b_r377 {
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r378 {
                t_r433 = unsafe { *p_r378.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let lim = 5000;
            if lim > 0 {
                if t_r433 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r433 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r433 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r433 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r433 = t.farray.len();
            p_r433 = t.farray.as_mut_ptr();
            i_r380 = 0;
            loop {
                b_r377 = i_r380 < 5000;
                if b_r377 {
                    let k = i_r380;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r433 {
                        f_r406 = unsafe { *p_r433.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    f_r404 = f_r404 + f_r406;
                    i_r380 = i_r380 + 1;
                } else {
                    break;
                }
            }
            i_r378 = i_r378 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new_float()));
    t_r434 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r378 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r378 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r435 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r435 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r435 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    f_r406 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r434 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r434 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r406;
    }
    let k = 4999;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r378 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r378 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r435 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 4999;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r435 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r435 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    f_r406 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r434 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r434 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r406;
    }
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r434 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r434 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r404;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=12;fast_gets=7;dyn_sets=23;dyn_gets=15;hoists=17;hoist_ctx=0,0,0,0,0,1,0,1,0,0,0,0,1,1,0,2,1";
