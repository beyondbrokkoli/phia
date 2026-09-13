// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r32 = 0i64;
    let mut i_r33 = 0i64;
    let mut b_r32 = false;
    let mut t_r32 = 0i64;
    let mut t_r33 = 0i64;
    let mut p_r33: *mut i64 = std::ptr::null_mut();
    let mut len_r33 = 0usize;
    let mut t_r34 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r32 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r33 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r34 = tables.len() as i64;
    let k = 0;
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
        *t.array.get_unchecked_mut(idx) = t_r33;
    }
    let k = 1;
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
        *t.array.get_unchecked_mut(idx) = t_r32;
    }
    i_r32 = 0;
    loop {
        b_r32 = i_r32 < 4;
        if b_r32 {
            let k = i_r32;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r32 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r32 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = 7;
            }
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r34 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r34 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r33 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let lim = 4;
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
                b_r32 = i_r33 < 4;
                if b_r32 {
                    let k = i_r33;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r33 {
                        unsafe {
                            *p_r33.add(k as usize) = 3;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r33 = i_r33 + 1;
                } else {
                    break;
                }
            }
            i_r32 = i_r32 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=3;dyn_gets=1;hoists=1;hoist_ctx=1";
