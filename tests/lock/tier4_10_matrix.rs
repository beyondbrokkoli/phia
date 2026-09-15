// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut b_r2 = false;
    let mut t_r4 = 0i64;
    let mut p_r4: *mut i64 = std::ptr::null_mut();
    let mut len_r4 = 0usize;
    let mut t_r5 = 0i64;
    let mut p_r5: *mut i64 = std::ptr::null_mut();
    let mut len_r5 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r4 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r5 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r5 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r5 - 1) as usize) {
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
    if t_r4 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r4 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r5;
    }
    tables.push(Box::new(Table::new()));
    t_r5 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r5 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r5 - 1) as usize) {
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
    if t_r4 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r4 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r5;
    }
    let lim = 2;
    if lim > 0 {
        if t_r4 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r4 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r4 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r4 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r4 = t.array.len();
    p_r4 = t.array.as_mut_ptr();
    i_r0 = 0;
    loop {
        b_r2 = i_r0 < 2;
        if b_r2 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r4 {
                t_r5 = unsafe { *p_r4.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let lim = 4;
            if lim > 0 {
                if t_r5 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r5 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            if t_r5 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r5 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r5 = t.array.len();
            p_r5 = t.array.as_mut_ptr();
            i_r1 = 0;
            loop {
                b_r2 = i_r1 < 4;
                if b_r2 {
                    let k = i_r1;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r5 {
                        unsafe {
                            *p_r5.add(k as usize) = 1;
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
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=4;dyn_gets=0;hoists=2;hoist_ctx=0,1;consts_i=13;consts_b=0;consts_f=0;consts_s=0";
