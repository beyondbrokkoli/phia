// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r28 = 0i64;
    let mut b_r28 = false;
    let mut t_r28 = 0i64;
    let mut p_r28: *mut i64 = std::ptr::null_mut();
    let mut len_r28 = 0usize;
    let mut t_r29 = 0i64;
    let mut t_r30 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r28 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r29 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r30 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r30 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r30 - 1) as usize) {
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
    if t_r29 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r29 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r30;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r28 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r28 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r29;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r28 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r28 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r30 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r30 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r30 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r28 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 8;
    if lim > 0 {
        if t_r28 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r28 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r28 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r28 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r28 = t.array.len();
    p_r28 = t.array.as_mut_ptr();
    i_r28 = 0;
    loop {
        b_r28 = i_r28 < 8;
        if b_r28 {
            let k = i_r28;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r28 {
                unsafe {
                    *p_r28.add(k as usize) = 1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r28 = i_r28 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=3;dyn_gets=2;hoists=1;hoist_ctx=0";
