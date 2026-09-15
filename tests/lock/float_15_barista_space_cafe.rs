// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
    let mut b_r26 = false;
    let mut t_r49 = 0i64;
    let mut p_r49: *mut i64 = std::ptr::null_mut();
    let mut len_r49 = 0usize;
    let mut t_r50 = 0i64;
    let mut p_r50: *mut i64 = std::ptr::null_mut();
    let mut len_r50 = 0usize;
    let mut t_r51 = 0i64;
    let mut p_r51: *mut i64 = std::ptr::null_mut();
    let mut len_r51 = 0usize;
    let mut f_r64 = 0f64;
    let mut f_r65 = 0f64;
    let mut f_r66 = 0f64;
    let mut t_r73 = 0i64;
    let mut p_r73: *mut f64 = std::ptr::null_mut();
    let mut len_r73 = 0usize;
    let mut t_r74 = 0i64;
    let mut p_r74: *mut f64 = std::ptr::null_mut();
    let mut len_r74 = 0usize;
    let mut t_r75 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new_float()));
    t_r73 = tables.len() as i64;
    let lim = 150000000;
    if lim > 0 {
        if t_r73 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r73 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r73 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r73 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r73 = t.farray.len();
    p_r73 = t.farray.as_mut_ptr();
    f_r64 = 0.5;
    i_r0 = 0;
    loop {
        b_r26 = i_r0 < 150000000;
        if b_r26 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r73 {
                unsafe {
                    *p_r73.add(k as usize) = f_r64;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            f_r64 = f_r64 + 0.25;
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r49 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r73 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r49 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r49 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r73;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r49 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r49 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r73 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 10000000;
    if lim > 0 {
        if t_r73 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r73 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r73 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r73 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r73 = t.farray.len();
    p_r73 = t.farray.as_mut_ptr();
    i_r0 = 0;
    loop {
        b_r26 = i_r0 < 10000000;
        if b_r26 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r73 {
                unsafe {
                    *p_r73.add(k as usize) = 1.75;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r49 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r50 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r73 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r73 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r73 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 0.0;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r50 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r50 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r73;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r49 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r49 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r50;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r49 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r49 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r50 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r50 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r50 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r73 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 25000000;
    if lim > 0 {
        if t_r73 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r73 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r73 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r73 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r73 = t.farray.len();
    p_r73 = t.farray.as_mut_ptr();
    i_r0 = 0;
    loop {
        b_r26 = i_r0 < 25000000;
        if b_r26 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r73 {
                unsafe {
                    *p_r73.add(k as usize) = 0.125;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r50 = tables.len() as i64;
    let lim = 5000;
    if lim > 0 {
        if t_r50 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r50 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r50 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r50 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r50 = t.array.len();
    p_r50 = t.array.as_mut_ptr();
    i_r0 = 0;
    loop {
        b_r26 = i_r0 < 5000;
        if b_r26 {
            tables.push(Box::new(Table::new_float()));
            t_r73 = tables.len() as i64;
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r50 {
                unsafe {
                    *p_r50.add(k as usize) = t_r73;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let lim = 5000;
    if lim > 0 {
        if t_r50 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r50 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r50 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r50 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r50 = t.array.len();
    p_r50 = t.array.as_mut_ptr();
    i_r0 = 0;
    loop {
        b_r26 = i_r0 < 5000;
        if b_r26 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r50 {
                t_r73 = unsafe { *p_r50.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let lim = 5000;
            if lim > 0 {
                if t_r73 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r73 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r73 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r73 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r73 = t.farray.len();
            p_r73 = t.farray.as_mut_ptr();
            i_r1 = 0;
            loop {
                b_r26 = i_r1 < 5000;
                if b_r26 {
                    let k = i_r1;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r73 {
                        unsafe {
                            *p_r73.add(k as usize) = 0.25;
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
    let lim = 5000;
    if lim > 0 {
        if t_r50 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r50 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r50 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r50 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r50 = t.array.len();
    p_r50 = t.array.as_mut_ptr();
    f_r64 = 0.0;
    i_r2 = 0;
    loop {
        b_r26 = i_r2 < 5000;
        if b_r26 {
            let k = i_r2;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r50 {
                t_r73 = unsafe { *p_r50.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let lim = 5000;
            if lim > 0 {
                if t_r73 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r73 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r73 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r73 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r73 = t.farray.len();
            p_r73 = t.farray.as_mut_ptr();
            i_r1 = 0;
            loop {
                b_r26 = i_r1 < 5000;
                if b_r26 {
                    let k = i_r1;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r73 {
                        f_r65 = unsafe { *p_r73.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    f_r64 = f_r64 + f_r65;
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
    tables.push(Box::new(Table::new_float()));
    t_r74 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r74 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r74 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r64;
    }
    tables.push(Box::new(Table::new()));
    t_r49 = tables.len() as i64;
    let lim = 15000000;
    if lim > 0 {
        if t_r49 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r49 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r49 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r49 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r49 = t.array.len();
    p_r49 = t.array.as_mut_ptr();
    i_r0 = 0;
    loop {
        b_r26 = i_r0 < 15000000;
        if b_r26 {
            i_r1 = i_r0 + i_r0;
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r49 {
                unsafe {
                    *p_r49.add(k as usize) = i_r1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new_float()));
    t_r74 = tables.len() as i64;
    let lim = 15000000;
    if lim > 0 {
        if t_r74 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r74 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r74 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r74 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r74 = t.farray.len();
    p_r74 = t.farray.as_mut_ptr();
    i_r1 = 0;
    loop {
        b_r26 = i_r1 < 15000000;
        if b_r26 {
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r74 {
                unsafe {
                    *p_r74.add(k as usize) = 0.5;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r49 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r49 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r49 - 1) as usize) {
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
    t_r51 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r74 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r74 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r74 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 0.0;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r51 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r51 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r74;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r49 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r49 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    i_r1 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    i_r0 = i_r1 + 1;
    i_r1 = 0;
    loop {
        b_r26 = i_r1 < i_r0;
        if b_r26 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r51 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r51 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r74 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r74 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r74 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = 0.75;
            }
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r51 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r74 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r74 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r74 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 0.0;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r51 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r51 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r74;
    }
    i_r1 = 100;
    loop {
        b_r26 = i_r1 < 8;
        if b_r26 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r51 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r51 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r74 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r74 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r74 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = 0.5;
            }
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r51 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r74 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r74 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r74 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 0.0;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r51 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r51 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r74;
    }
    tables.push(Box::new(Table::new_float()));
    t_r74 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r74 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r74 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 0.0;
    }
    i_r1 = 0;
    loop {
        b_r26 = i_r1 < 10000000;
        if b_r26 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r51 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r51 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r73 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r73 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r73 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = 0.625;
            }
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r51 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r51 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = t_r74;
            }
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r51 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r73 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r73 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r73 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 0.0;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r51 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r51 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r73;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r51 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r51 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r73 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 12000000;
    if lim > 0 {
        if t_r73 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r73 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r73 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r73 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r73 = t.farray.len();
    p_r73 = t.farray.as_mut_ptr();
    i_r1 = 0;
    loop {
        b_r26 = i_r1 < 12000000;
        if b_r26 {
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r73 {
                unsafe {
                    *p_r73.add(k as usize) = 0.375;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r73 {
                unsafe {
                    *p_r73.add(k as usize) = 0.875;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r51 = tables.len() as i64;
    let lim = 1024;
    if lim > 0 {
        if t_r51 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r51 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r51 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r51 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r51 = t.array.len();
    p_r51 = t.array.as_mut_ptr();
    i_r1 = 0;
    loop {
        b_r26 = i_r1 < 1024;
        if b_r26 {
            tables.push(Box::new(Table::new()));
            t_r49 = tables.len() as i64;
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r51 {
                unsafe {
                    *p_r51.add(k as usize) = t_r49;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    i_r1 = 0;
    loop {
        b_r26 = i_r1 < 1024;
        if b_r26 {
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r51 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r51 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r49 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            tables.push(Box::new(Table::new_float()));
            t_r73 = tables.len() as i64;
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r49 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r49 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = t_r73;
            }
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    i_r1 = 0;
    loop {
        b_r26 = i_r1 < 1024;
        if b_r26 {
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r51 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r51 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r49 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r49 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r49 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r73 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let lim = 8192;
            if lim > 0 {
                if t_r73 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r73 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r73 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r73 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r73 = t.farray.len();
            p_r73 = t.farray.as_mut_ptr();
            i_r0 = 0;
            loop {
                b_r26 = i_r0 < 8192;
                if b_r26 {
                    let k = i_r0;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r73 {
                        unsafe {
                            *p_r73.add(k as usize) = 0.0625;
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
    i_r2 = 0;
    loop {
        b_r26 = i_r2 < 12;
        if b_r26 {
            let lim = 5000;
            if lim > 0 {
                if t_r50 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r50 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            if t_r50 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r50 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r50 = t.array.len();
            p_r50 = t.array.as_mut_ptr();
            i_r0 = 0;
            loop {
                b_r26 = i_r0 < 5000;
                if b_r26 {
                    let k = i_r0;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r50 {
                        t_r73 = unsafe { *p_r50.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    let lim = 5000;
                    if lim > 0 {
                        if t_r73 == 0 {
                            panic!("Runtime Error: table is nil");
                        }
                        let t = match tables.get_mut((t_r73 - 1) as usize) {
                            Some(t) => &mut **t,
                            None => panic!("Runtime Error: table is nil"),
                        };
                        if (lim as usize) > t.farray.len() {
                            t.farray.resize(lim as usize, 0.0);
                        }
                    }
                    if t_r73 == 0 {
                        panic!("Runtime Error: table is nil");
                    }
                    let t = match tables.get_mut((t_r73 - 1) as usize) {
                        Some(t) => &mut **t,
                        None => panic!("Runtime Error: table is nil"),
                    };
                    len_r73 = t.farray.len();
                    p_r73 = t.farray.as_mut_ptr();
                    i_r3 = 0;
                    loop {
                        b_r26 = i_r3 < 5000;
                        if b_r26 {
                            let k = i_r3;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r73 {
                                f_r64 = unsafe { *p_r73.add(k as usize) };
                            } else {
                                panic!(
                                    "optimizer invariant violated: fast-path bounds check failed"
                                );
                            }
                            f_r66 = f_r64 + 0.25;
                            let k = i_r3;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r73 {
                                unsafe {
                                    *p_r73.add(k as usize) = f_r66;
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
                    i_r0 = i_r0 + 1;
                } else {
                    break;
                }
            }
            i_r2 = i_r2 + 1;
        } else {
            break;
        }
    }
    let lim = 5000;
    if lim > 0 {
        if t_r50 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r50 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r50 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r50 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r50 = t.array.len();
    p_r50 = t.array.as_mut_ptr();
    f_r65 = 0.0;
    i_r1 = 0;
    loop {
        b_r26 = i_r1 < 5000;
        if b_r26 {
            let k = i_r1;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r50 {
                t_r73 = unsafe { *p_r50.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let lim = 5000;
            if lim > 0 {
                if t_r73 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r73 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r73 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r73 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r73 = t.farray.len();
            p_r73 = t.farray.as_mut_ptr();
            i_r3 = 0;
            loop {
                b_r26 = i_r3 < 5000;
                if b_r26 {
                    let k = i_r3;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r73 {
                        f_r66 = unsafe { *p_r73.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    f_r65 = f_r65 + f_r66;
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
    tables.push(Box::new(Table::new_float()));
    t_r74 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r50 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r50 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r75 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r75 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r75 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    f_r66 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r74 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r74 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r66;
    }
    let k = 4999;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r50 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r50 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r75 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 4999;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r75 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r75 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    f_r66 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r74 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r74 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r66;
    }
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r74 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r74 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r65;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=12;fast_gets=7;dyn_sets=23;dyn_gets=15;hoists=17;hoist_ctx=0,0,0,0,0,1,0,1,0,0,0,0,1,1,0,2,1;consts_i=111;consts_b=0;consts_f=21;consts_s=0";
