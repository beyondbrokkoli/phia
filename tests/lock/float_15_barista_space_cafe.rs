// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r377 = 0i64;
    let mut i_r378 = 0i64;
    let mut i_r379 = 0i64;
    let mut i_r380 = 0i64;
    let mut b_r403 = false;
    let mut t_r426 = 0i64;
    let mut p_r426: *mut i64 = std::ptr::null_mut();
    let mut len_r426 = 0usize;
    let mut t_r427 = 0i64;
    let mut p_r427: *mut i64 = std::ptr::null_mut();
    let mut len_r427 = 0usize;
    let mut t_r428 = 0i64;
    let mut p_r428: *mut i64 = std::ptr::null_mut();
    let mut len_r428 = 0usize;
    let mut f_r441 = 0f64;
    let mut f_r442 = 0f64;
    let mut f_r443 = 0f64;
    let mut t_r450 = 0i64;
    let mut p_r450: *mut f64 = std::ptr::null_mut();
    let mut len_r450 = 0usize;
    let mut t_r451 = 0i64;
    let mut p_r451: *mut f64 = std::ptr::null_mut();
    let mut len_r451 = 0usize;
    let mut t_r452 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new_float()));
    t_r450 = tables.len() as i64;
    let lim = 150000000;
    if lim > 0 {
        if t_r450 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r450 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r450 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r450 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r450 = t.farray.len();
    p_r450 = t.farray.as_mut_ptr();
    f_r441 = 0.5;
    i_r377 = 0;
    loop {
        b_r403 = i_r377 < 150000000;
        if b_r403 {
            let k = i_r377;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r450 {
                unsafe {
                    *p_r450.add(k as usize) = f_r441;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            f_r441 = f_r441 + 0.25;
            i_r377 = i_r377 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r426 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r450 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r426 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r426 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r450;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r426 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r426 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r450 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 10000000;
    if lim > 0 {
        if t_r450 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r450 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r450 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r450 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r450 = t.farray.len();
    p_r450 = t.farray.as_mut_ptr();
    i_r377 = 0;
    loop {
        b_r403 = i_r377 < 10000000;
        if b_r403 {
            let k = i_r377;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r450 {
                unsafe {
                    *p_r450.add(k as usize) = 1.75;
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
    t_r426 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r427 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r450 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r450 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r450 - 1) as usize) {
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
    if t_r427 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r427 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r450;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r426 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r426 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r427;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r426 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r426 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r427 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r427 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r427 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r450 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 25000000;
    if lim > 0 {
        if t_r450 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r450 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r450 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r450 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r450 = t.farray.len();
    p_r450 = t.farray.as_mut_ptr();
    i_r377 = 0;
    loop {
        b_r403 = i_r377 < 25000000;
        if b_r403 {
            let k = i_r377;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r450 {
                unsafe {
                    *p_r450.add(k as usize) = 0.125;
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
    t_r427 = tables.len() as i64;
    let lim = 5000;
    if lim > 0 {
        if t_r427 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r427 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r427 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r427 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r427 = t.array.len();
    p_r427 = t.array.as_mut_ptr();
    i_r377 = 0;
    loop {
        b_r403 = i_r377 < 5000;
        if b_r403 {
            tables.push(Box::new(Table::new_float()));
            t_r450 = tables.len() as i64;
            let k = i_r377;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r427 {
                unsafe {
                    *p_r427.add(k as usize) = t_r450;
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
        if t_r427 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r427 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r427 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r427 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r427 = t.array.len();
    p_r427 = t.array.as_mut_ptr();
    i_r377 = 0;
    loop {
        b_r403 = i_r377 < 5000;
        if b_r403 {
            let k = i_r377;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r427 {
                t_r450 = unsafe { *p_r427.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let lim = 5000;
            if lim > 0 {
                if t_r450 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r450 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r450 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r450 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r450 = t.farray.len();
            p_r450 = t.farray.as_mut_ptr();
            i_r378 = 0;
            loop {
                b_r403 = i_r378 < 5000;
                if b_r403 {
                    let k = i_r378;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r450 {
                        unsafe {
                            *p_r450.add(k as usize) = 0.25;
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
    let lim = 5000;
    if lim > 0 {
        if t_r427 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r427 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r427 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r427 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r427 = t.array.len();
    p_r427 = t.array.as_mut_ptr();
    f_r441 = 0.0;
    i_r379 = 0;
    loop {
        b_r403 = i_r379 < 5000;
        if b_r403 {
            let k = i_r379;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r427 {
                t_r450 = unsafe { *p_r427.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let lim = 5000;
            if lim > 0 {
                if t_r450 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r450 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r450 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r450 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r450 = t.farray.len();
            p_r450 = t.farray.as_mut_ptr();
            i_r378 = 0;
            loop {
                b_r403 = i_r378 < 5000;
                if b_r403 {
                    let k = i_r378;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r450 {
                        f_r442 = unsafe { *p_r450.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    f_r441 = f_r441 + f_r442;
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
    t_r451 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r451 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r451 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r441;
    }
    tables.push(Box::new(Table::new()));
    t_r426 = tables.len() as i64;
    let lim = 15000000;
    if lim > 0 {
        if t_r426 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r426 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r426 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r426 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r426 = t.array.len();
    p_r426 = t.array.as_mut_ptr();
    i_r377 = 0;
    loop {
        b_r403 = i_r377 < 15000000;
        if b_r403 {
            i_r378 = i_r377 + i_r377;
            let k = i_r377;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r426 {
                unsafe {
                    *p_r426.add(k as usize) = i_r378;
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
    t_r451 = tables.len() as i64;
    let lim = 15000000;
    if lim > 0 {
        if t_r451 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r451 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r451 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r451 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r451 = t.farray.len();
    p_r451 = t.farray.as_mut_ptr();
    i_r378 = 0;
    loop {
        b_r403 = i_r378 < 15000000;
        if b_r403 {
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r451 {
                unsafe {
                    *p_r451.add(k as usize) = 0.5;
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
    t_r426 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r426 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r426 - 1) as usize) {
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
    t_r428 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r451 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r451 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r451 - 1) as usize) {
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
    if t_r428 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r428 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r451;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r426 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r426 - 1) as usize) {
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
        b_r403 = i_r378 < i_r377;
        if b_r403 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r428 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r428 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r451 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r451 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r451 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = 0.75;
            }
            i_r378 = i_r378 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r428 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r451 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r451 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r451 - 1) as usize) {
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
    if t_r428 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r428 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r451;
    }
    i_r378 = 100;
    loop {
        b_r403 = i_r378 < 8;
        if b_r403 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r428 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r428 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r451 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r451 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r451 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = 0.5;
            }
            i_r378 = i_r378 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r428 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r451 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r451 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r451 - 1) as usize) {
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
    if t_r428 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r428 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r451;
    }
    tables.push(Box::new(Table::new_float()));
    t_r451 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r451 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r451 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 0.0;
    }
    i_r378 = 0;
    loop {
        b_r403 = i_r378 < 10000000;
        if b_r403 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r428 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r428 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r450 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r450 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r450 - 1) as usize) {
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
            if t_r428 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r428 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = t_r451;
            }
            i_r378 = i_r378 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r428 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r450 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r450 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r450 - 1) as usize) {
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
    if t_r428 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r428 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r450;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r428 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r428 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r450 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 12000000;
    if lim > 0 {
        if t_r450 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r450 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r450 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r450 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r450 = t.farray.len();
    p_r450 = t.farray.as_mut_ptr();
    i_r378 = 0;
    loop {
        b_r403 = i_r378 < 12000000;
        if b_r403 {
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r450 {
                unsafe {
                    *p_r450.add(k as usize) = 0.375;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r450 {
                unsafe {
                    *p_r450.add(k as usize) = 0.875;
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
    t_r428 = tables.len() as i64;
    let lim = 1024;
    if lim > 0 {
        if t_r428 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r428 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r428 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r428 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r428 = t.array.len();
    p_r428 = t.array.as_mut_ptr();
    i_r378 = 0;
    loop {
        b_r403 = i_r378 < 1024;
        if b_r403 {
            tables.push(Box::new(Table::new()));
            t_r426 = tables.len() as i64;
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r428 {
                unsafe {
                    *p_r428.add(k as usize) = t_r426;
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
        b_r403 = i_r378 < 1024;
        if b_r403 {
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r428 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r428 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r426 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            tables.push(Box::new(Table::new_float()));
            t_r450 = tables.len() as i64;
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r426 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r426 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = t_r450;
            }
            i_r378 = i_r378 + 1;
        } else {
            break;
        }
    }
    i_r378 = 0;
    loop {
        b_r403 = i_r378 < 1024;
        if b_r403 {
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r428 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r428 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r426 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r426 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r426 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r450 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let lim = 8192;
            if lim > 0 {
                if t_r450 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r450 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r450 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r450 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r450 = t.farray.len();
            p_r450 = t.farray.as_mut_ptr();
            i_r377 = 0;
            loop {
                b_r403 = i_r377 < 8192;
                if b_r403 {
                    let k = i_r377;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r450 {
                        unsafe {
                            *p_r450.add(k as usize) = 0.0625;
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
        b_r403 = i_r379 < 12;
        if b_r403 {
            let lim = 5000;
            if lim > 0 {
                if t_r427 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r427 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            if t_r427 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r427 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r427 = t.array.len();
            p_r427 = t.array.as_mut_ptr();
            i_r377 = 0;
            loop {
                b_r403 = i_r377 < 5000;
                if b_r403 {
                    let k = i_r377;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r427 {
                        t_r450 = unsafe { *p_r427.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    let lim = 5000;
                    if lim > 0 {
                        if t_r450 == 0 {
                            panic!("Runtime Error: table is nil");
                        }
                        let t = match tables.get_mut((t_r450 - 1) as usize) {
                            Some(t) => &mut **t,
                            None => panic!("Runtime Error: table is nil"),
                        };
                        if (lim as usize) > t.farray.len() {
                            t.farray.resize(lim as usize, 0.0);
                        }
                    }
                    if t_r450 == 0 {
                        panic!("Runtime Error: table is nil");
                    }
                    let t = match tables.get_mut((t_r450 - 1) as usize) {
                        Some(t) => &mut **t,
                        None => panic!("Runtime Error: table is nil"),
                    };
                    len_r450 = t.farray.len();
                    p_r450 = t.farray.as_mut_ptr();
                    i_r380 = 0;
                    loop {
                        b_r403 = i_r380 < 5000;
                        if b_r403 {
                            let k = i_r380;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r450 {
                                f_r441 = unsafe { *p_r450.add(k as usize) };
                            } else {
                                panic!(
                                    "optimizer invariant violated: fast-path bounds check failed"
                                );
                            }
                            f_r443 = f_r441 + 0.25;
                            let k = i_r380;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r450 {
                                unsafe {
                                    *p_r450.add(k as usize) = f_r443;
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
    let lim = 5000;
    if lim > 0 {
        if t_r427 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r427 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r427 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r427 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r427 = t.array.len();
    p_r427 = t.array.as_mut_ptr();
    f_r442 = 0.0;
    i_r378 = 0;
    loop {
        b_r403 = i_r378 < 5000;
        if b_r403 {
            let k = i_r378;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r427 {
                t_r450 = unsafe { *p_r427.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let lim = 5000;
            if lim > 0 {
                if t_r450 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r450 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r450 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r450 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r450 = t.farray.len();
            p_r450 = t.farray.as_mut_ptr();
            i_r380 = 0;
            loop {
                b_r403 = i_r380 < 5000;
                if b_r403 {
                    let k = i_r380;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r450 {
                        f_r443 = unsafe { *p_r450.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    f_r442 = f_r442 + f_r443;
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
    t_r451 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r427 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r427 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r452 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r452 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r452 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    f_r443 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r451 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r451 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r443;
    }
    let k = 4999;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r427 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r427 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r452 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 4999;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r452 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r452 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    f_r443 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r451 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r451 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r443;
    }
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r451 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r451 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r442;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=12;fast_gets=7;dyn_sets=23;dyn_gets=15;hoists=17;hoist_ctx=0,0,0,0,0,1,0,1,0,0,0,0,1,1,0,2,1;consts_i=111;consts_b=0;consts_f=21;consts_s=0";
