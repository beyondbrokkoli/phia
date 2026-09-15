// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut b_r1 = false;
    let mut t_r2 = 0i64;
    let mut p_r2: *mut i64 = std::ptr::null_mut();
    let mut len_r2 = 0usize;
    let mut t_r3 = 0i64;
    let mut t_r4 = 0i64;
    let mut p_r4: *mut i64 = std::ptr::null_mut();
    let mut len_r4 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r2 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r2 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r2 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 0;
    }
    tables.push(Box::new(Table::new()));
    t_r3 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r4 = tables.len() as i64;
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
        *t.array.get_unchecked_mut(idx) = 0;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r3 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r3 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r4;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r3 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r3 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r4 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 8;
    if lim > 0 {
        if t_r2 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r2 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r2 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r2 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r2 = t.array.len();
    p_r2 = t.array.as_mut_ptr();
    let lim = 8;
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
    while i_r0 < 8 {
        let k = i_r0;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r2 {
            unsafe {
                *p_r2.add(k as usize) = i_r0;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        let k = i_r0;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r4 {
            unsafe {
                *p_r4.add(k as usize) = 1;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r0 = i_r0 + 1;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=2;fast_gets=0;dyn_sets=3;dyn_gets=1;hoists=2;hoist_ctx=0,0;consts_i=11;consts_b=0;consts_f=0;consts_s=0";
