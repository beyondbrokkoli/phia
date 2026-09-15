// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r15 = 0i64;
    let mut b_r15 = false;
    let mut t_r15 = 0i64;
    let mut t_r16 = 0i64;
    let mut p_r16: *mut f64 = std::ptr::null_mut();
    let mut len_r16 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r15 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r16 = tables.len() as i64;
    let lim = 8;
    if lim > 0 {
        if t_r16 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r16 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r16 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r16 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r16 = t.farray.len();
    p_r16 = t.farray.as_mut_ptr();
    i_r15 = 0;
    while i_r15 < 8 {
        let k = i_r15;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r16 {
            unsafe {
                *p_r16.add(k as usize) = 0.5;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r15 = i_r15 + 1;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r15 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r15 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r16;
    }
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r15 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r15 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r16 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=1;dyn_gets=1;hoists=1;hoist_ctx=0;consts_i=5;consts_b=0;consts_f=1;consts_s=0";
