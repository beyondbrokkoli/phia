// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
    let mut b_r6 = false;
    let mut t_r10 = 0i64;
    let mut p_r10: *mut i64 = std::ptr::null_mut();
    let mut len_r10 = 0usize;
    let mut t_r11 = 0i64;
    let mut p_r11: *mut i64 = std::ptr::null_mut();
    let mut len_r11 = 0usize;
    let mut t_r12 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r10 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r11 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r11 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r11 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = 0;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r10 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r10 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = t_r11;
    }
    tables.push(Box::new(Table::new()));
    t_r11 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r11 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r11 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = 0;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r10 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r10 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = t_r11;
    }
    let lim = 2;
    if lim > 0 {
        if t_r10 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r10 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    if t_r10 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r10 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r10 = t.as_int_mut().len();
    p_r10 = t.as_int_mut().as_mut_ptr();
    i_r0 = 0;
    loop {
        b_r6 = i_r0 < 2;
        if b_r6 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r10 {
                t_r11 = unsafe { *p_r10.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let lim = 4;
            if lim > 0 {
                if t_r11 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r11 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.as_int_mut().len() {
                    t.as_int_mut().resize(lim as usize, 0);
                }
            }
            if t_r11 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r11 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r11 = t.as_int_mut().len();
            p_r11 = t.as_int_mut().as_mut_ptr();
            i_r1 = 0;
            loop {
                b_r6 = i_r1 < 4;
                if b_r6 {
                    let k = i_r1;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r11 {
                        unsafe {
                            *p_r11.add(k as usize) = 1;
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
    let lim = 2;
    if lim > 0 {
        if t_r10 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r10 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    if t_r10 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r10 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r10 = t.as_int_mut().len();
    p_r10 = t.as_int_mut().as_mut_ptr();
    i_r2 = 0;
    i_r3 = 0;
    loop {
        b_r6 = i_r3 < 2;
        if b_r6 {
            let k = i_r3;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r10 {
                t_r11 = unsafe { *p_r10.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let lim = 4;
            if lim > 0 {
                if t_r11 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r11 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.as_int_mut().len() {
                    t.as_int_mut().resize(lim as usize, 0);
                }
            }
            if t_r11 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r11 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r11 = t.as_int_mut().len();
            p_r11 = t.as_int_mut().as_mut_ptr();
            i_r1 = 0;
            loop {
                b_r6 = i_r1 < 4;
                if b_r6 {
                    let k = i_r1;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r11 {
                        i_r0 = unsafe { *p_r11.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r2 = i_r2 + i_r0;
                    i_r1 = i_r1 + 1;
                } else {
                    break;
                }
            }
            i_r3 = i_r3 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r12 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r12 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r12 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = i_r2;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=3;dyn_sets=5;dyn_gets=0;hoists=4;hoist_ctx=0,1,0,1;consts_i=21;consts_b=0;consts_f=0;consts_s=0";
