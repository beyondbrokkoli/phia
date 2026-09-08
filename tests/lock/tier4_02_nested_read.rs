// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r35 = 0i64;
    let mut i_r36 = 0i64;
    let mut i_r37 = 0i64;
    let mut b_r35 = false;
    let mut t_r35 = 0i64;
    let mut t_r36 = 0i64;
    let mut p_r36: *mut i64 = std::ptr::null_mut();
    let mut len_r36 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r35 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r36 = tables.len() as i64;
    let lim = 8;
    if lim > 0 {
        if t_r36 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r36 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r36 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r36 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r36 = t.array.len();
    p_r36 = t.array.as_mut_ptr();
    i_r35 = 0;
    loop {
        b_r35 = i_r35 < 8;
        if b_r35 {
            i_r36 = i_r35 + 1;
            let k = i_r35;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r36 {
                unsafe {
                    *p_r36.add(k as usize) = i_r36;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r35 = i_r35 + 1;
        } else {
            break;
        }
    }
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
        *t.array.get_unchecked_mut(idx) = t_r36;
    }
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
    t_r36 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 8;
    if lim > 0 {
        if t_r36 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r36 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r36 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r36 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r36 = t.array.len();
    p_r36 = t.array.as_mut_ptr();
    i_r36 = 0;
    i_r35 = 0;
    loop {
        b_r35 = i_r35 < 8;
        if b_r35 {
            let k = i_r35;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r36 {
                i_r37 = unsafe { *p_r36.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r36 = i_r36 + i_r37;
            i_r35 = i_r35 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=1;dyn_gets=1;hoists=2;hoist_ctx=0,0";
