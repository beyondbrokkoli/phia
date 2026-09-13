// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r21 = 0i64;
    let mut b_r21 = false;
    let mut f_r22 = 0f64;
    let mut t_r21 = 0i64;
    let mut t_r24 = 0i64;
    let mut p_r24: *mut f64 = std::ptr::null_mut();
    let mut len_r24 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r21 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r24 = tables.len() as i64;
    f_r22 = 0.5;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r24 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r24 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r22;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r21 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r21 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r24;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r21 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r21 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r24 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 8;
    if lim > 0 {
        if t_r24 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r24 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r24 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r24 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r24 = t.farray.len();
    p_r24 = t.farray.as_mut_ptr();
    i_r21 = 0;
    loop {
        b_r21 = i_r21 < 8;
        if b_r21 {
            f_r22 = 0.25;
            let k = i_r21;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r24 {
                unsafe {
                    *p_r24.add(k as usize) = f_r22;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r21 = i_r21 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=2;dyn_gets=1;hoists=1;hoist_ctx=0";
