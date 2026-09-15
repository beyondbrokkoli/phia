// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r35 = 0i64;
    let mut i_r36 = 0i64;
    let mut i_r37 = 0i64;
    let mut b_r40 = false;
    let mut t_r42 = 0i64;
    let mut t_r43 = 0i64;
    let mut p_r43: *mut i64 = std::ptr::null_mut();
    let mut len_r43 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r42 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r43 = tables.len() as i64;
    let lim = 8;
    if lim > 0 {
        if t_r43 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r43 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r43 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r43 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r43 = t.array.len();
    p_r43 = t.array.as_mut_ptr();
    i_r35 = 0;
    loop {
        b_r40 = i_r35 < 8;
        if b_r40 {
            i_r36 = i_r35 + 1;
            let k = i_r35;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r43 {
                unsafe {
                    *p_r43.add(k as usize) = i_r36;
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
    if t_r42 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r42 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r43;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r42 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r42 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r43 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 8;
    if lim > 0 {
        if t_r43 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r43 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r43 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r43 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r43 = t.array.len();
    p_r43 = t.array.as_mut_ptr();
    i_r36 = 0;
    i_r35 = 0;
    loop {
        b_r40 = i_r35 < 8;
        if b_r40 {
            let k = i_r35;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r43 {
                i_r37 = unsafe { *p_r43.add(k as usize) };
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

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=1;dyn_gets=1;hoists=2;hoist_ctx=0,0;consts_i=11;consts_b=0;consts_f=0;consts_s=0";
