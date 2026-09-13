// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r62 = 0i64;
    let mut i_r63 = 0i64;
    let mut i_r64 = 0i64;
    let mut i_r65 = 0i64;
    let mut b_r62 = false;
    let mut t_r62 = 0i64;
    let mut p_r62: *mut i64 = std::ptr::null_mut();
    let mut len_r62 = 0usize;
    let mut t_r63 = 0i64;
    let mut p_r63: *mut i64 = std::ptr::null_mut();
    let mut len_r63 = 0usize;
    let mut t_r64 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r62 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r63 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r63 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r63 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 0;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r62 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r62 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r63;
    }
    tables.push(Box::new(Table::new()));
    t_r63 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r63 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r63 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 0;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r62 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r62 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r63;
    }
    let lim = 2;
    if lim > 0 {
        if t_r62 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r62 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r62 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r62 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r62 = t.array.len();
    p_r62 = t.array.as_mut_ptr();
    i_r62 = 0;
    loop {
        b_r62 = i_r62 < 2;
        if b_r62 {
            let k = i_r62;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r62 {
                t_r63 = unsafe { *p_r62.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let lim = 4;
            if lim > 0 {
                if t_r63 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r63 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            if t_r63 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r63 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r63 = t.array.len();
            p_r63 = t.array.as_mut_ptr();
            i_r63 = 0;
            loop {
                b_r62 = i_r63 < 4;
                if b_r62 {
                    let k = i_r63;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r63 {
                        unsafe {
                            *p_r63.add(k as usize) = 1;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r63 = i_r63 + 1;
                } else {
                    break;
                }
            }
            i_r62 = i_r62 + 1;
        } else {
            break;
        }
    }
    let lim = 2;
    if lim > 0 {
        if t_r62 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r62 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r62 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r62 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r62 = t.array.len();
    p_r62 = t.array.as_mut_ptr();
    i_r64 = 0;
    i_r65 = 0;
    loop {
        b_r62 = i_r65 < 2;
        if b_r62 {
            let k = i_r65;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r62 {
                t_r63 = unsafe { *p_r62.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let lim = 4;
            if lim > 0 {
                if t_r63 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r63 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            if t_r63 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r63 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r63 = t.array.len();
            p_r63 = t.array.as_mut_ptr();
            i_r63 = 0;
            loop {
                b_r62 = i_r63 < 4;
                if b_r62 {
                    let k = i_r63;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r63 {
                        i_r62 = unsafe { *p_r63.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r64 = i_r64 + i_r62;
                    i_r63 = i_r63 + 1;
                } else {
                    break;
                }
            }
            i_r65 = i_r65 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r64 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r64 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r64 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r64;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=3;dyn_sets=5;dyn_gets=0;hoists=4;hoist_ctx=0,1,0,1";
