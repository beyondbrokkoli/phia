// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r34 = 0i64;
    let mut i_r35 = 0i64;
    let mut b_r34 = false;
    let mut t_r34 = 0i64;
    let mut t_r35 = 0i64;
    let mut t_r36 = 0i64;
    let mut t_r37 = 0i64;
    let mut p_r37: *mut i64 = std::ptr::null_mut();
    let mut len_r37 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r34 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
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
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r34;
    }
    tables.push(Box::new(Table::new()));
    t_r36 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r35 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r35 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r37 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r36 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r36 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r37;
    }
    i_r34 = 0;
    loop {
        b_r34 = i_r34 < 4;
        if b_r34 {
            let k = i_r34;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r34 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r34 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = 9;
            }
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r36 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r36 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r37 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let lim = 4;
            if lim > 0 {
                if t_r37 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r37 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            if t_r37 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r37 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r37 = t.array.len();
            p_r37 = t.array.as_mut_ptr();
            i_r35 = 0;
            loop {
                b_r34 = i_r35 < 4;
                if b_r34 {
                    let k = i_r35;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r37 {
                        unsafe {
                            *p_r37.add(k as usize) = 3;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r35 = i_r35 + 1;
                } else {
                    break;
                }
            }
            i_r34 = i_r34 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=3;dyn_gets=2;hoists=1;hoist_ctx=1";
