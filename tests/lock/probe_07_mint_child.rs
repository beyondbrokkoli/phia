// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r26 = 0i64;
    let mut i_r27 = 0i64;
    let mut b_r26 = false;
    let mut t_r26 = 0i64;
    let mut t_r27 = 0i64;
    let mut p_r27: *mut i64 = std::ptr::null_mut();
    let mut len_r27 = 0usize;
    let mut t_r28 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r26 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r27 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r26 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r26 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r27;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r26 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r26 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r27 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 3;
    if lim > 0 {
        if t_r27 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r27 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r27 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r27 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r27 = t.array.len();
    p_r27 = t.array.as_mut_ptr();
    i_r26 = 0;
    loop {
        b_r26 = i_r26 < 3;
        if b_r26 {
            i_r27 = i_r26 * 7;
            let k = i_r26;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r27 {
                unsafe {
                    *p_r27.add(k as usize) = i_r27;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r26 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r26 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r28 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            println!(
                "PROBE iter: i_r26={} t_r26={} len_r26={} t_r28={} len_r28={}",
                i_r26,
                t_r26,
                match tables.get((t_r26 - 1) as usize) {
                    Some(t) => t.array.len(),
                    None => usize::MAX,
                },
                t_r28,
                match tables.get((t_r28 - 1) as usize) {
                    Some(t) => t.array.len(),
                    None => usize::MAX,
                }
            );
            i_r26 = i_r26 + 1;
        } else {
            break;
        }
    }
    println!(
        "PROBE exit: t_r26={} len_r26={}",
        t_r26,
        match tables.get((t_r26 - 1) as usize) {
            Some(t) => t.array.len(),
            None => usize::MAX,
        }
    );
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=1;dyn_gets=2;hoists=1;hoist_ctx=0";
