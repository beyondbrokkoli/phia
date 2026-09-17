// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut b_r5 = false;
    let mut t_r7 = 0i64;
    let mut t_r8 = 0i64;
    let mut p_r8: *mut i64 = std::ptr::null_mut();
    let mut len_r8 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r7 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r8 = tables.len() as i64;
    let lim = 8;
    if lim > 0 {
        if t_r8 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r8 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    if t_r8 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r8 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r8 = t.as_int_mut().len();
    p_r8 = t.as_int_mut().as_mut_ptr();
    i_r0 = 0;
    loop {
        b_r5 = i_r0 < 8;
        if b_r5 {
            i_r1 = i_r0 + 1;
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r8 {
                unsafe {
                    *p_r8.add(k as usize) = i_r1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r7 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r7 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = t_r8;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r7 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r7 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r8 = if idx < t.as_int().len() {
        unsafe { *t.as_int().get_unchecked(idx) }
    } else {
        0
    };
    let lim = 8;
    if lim > 0 {
        if t_r8 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r8 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    if t_r8 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r8 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r8 = t.as_int_mut().len();
    p_r8 = t.as_int_mut().as_mut_ptr();
    i_r1 = 0;
    i_r0 = 0;
    loop {
        b_r5 = i_r0 < 8;
        if b_r5 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r8 {
                i_r2 = unsafe { *p_r8.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r1 = i_r1 + i_r2;
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=1;dyn_gets=1;hoists=2;hoist_ctx=0,0;consts_i=11;consts_b=0;consts_f=0;consts_s=0";
