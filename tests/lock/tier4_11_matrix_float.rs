// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r33 = 0i64;
    let mut i_r34 = 0i64;
    let mut b_r33 = false;
    let mut t_r33 = 0i64;
    let mut p_r33: *mut i64 = std::ptr::null_mut();
    let mut len_r33 = 0usize;
    let mut t_r35 = 0i64;
    let mut p_r35: *mut f64 = std::ptr::null_mut();
    let mut len_r35 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r33 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r35 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r35 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r35 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 0.5;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r33 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r33 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r35;
    }
    tables.push(Box::new(Table::new_float()));
    t_r35 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r35 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r35 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 0.5;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r33 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r33 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r35;
    }
    let lim = 2;
    if lim > 0 {
        if t_r33 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r33 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r33 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r33 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r33 = t.array.len();
    p_r33 = t.array.as_mut_ptr();
    i_r33 = 0;
    loop {
        b_r33 = i_r33 < 2;
        if b_r33 {
            let k = i_r33;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r33 {
                t_r35 = unsafe { *p_r33.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let lim = 4;
            if lim > 0 {
                if t_r35 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r35 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r35 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r35 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r35 = t.farray.len();
            p_r35 = t.farray.as_mut_ptr();
            i_r34 = 0;
            loop {
                b_r33 = i_r34 < 4;
                if b_r33 {
                    let k = i_r34;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r35 {
                        unsafe {
                            *p_r35.add(k as usize) = 0.25;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r34 = i_r34 + 1;
                } else {
                    break;
                }
            }
            i_r33 = i_r33 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=4;dyn_gets=0;hoists=2;hoist_ctx=0,1;consts_i=10;consts_b=0;consts_f=3;consts_s=0";
