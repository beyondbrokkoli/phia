// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r17 = 0i64;
    let mut b_r17 = false;
    let mut t_r17 = 0i64;
    let mut p_r17: *mut i64 = std::ptr::null_mut();
    let mut len_r17 = 0usize;
    let mut t_r18 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r17 = tables.len() as i64;
    let lim = 3;
    if lim > 0 {
        if t_r17 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r17 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r17 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r17 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r17 = t.array.len();
    p_r17 = t.array.as_mut_ptr();
    i_r17 = 0;
    loop {
        b_r17 = i_r17 < 3;
        if b_r17 {
            tables.push(Box::new(Table::new()));
            t_r18 = tables.len() as i64;
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r18 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r18 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = i_r17;
            }
            let k = i_r17;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r17 {
                unsafe {
                    *p_r17.add(k as usize) = t_r18;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            println!(
                "PROBE iter: i_r17={} t_r17={} len_r17={} t_r18={} len_r18={}",
                i_r17,
                t_r17,
                match tables.get((t_r17 - 1) as usize) {
                    Some(t) => t.array.len(),
                    None => usize::MAX,
                },
                t_r18,
                match tables.get((t_r18 - 1) as usize) {
                    Some(t) => t.array.len(),
                    None => usize::MAX,
                }
            );
            i_r17 = i_r17 + 1;
        } else {
            break;
        }
    }
    println!(
        "PROBE exit: i_r17={} t_r17={} len_r17={}",
        i_r17,
        t_r17,
        match tables.get((t_r17 - 1) as usize) {
            Some(t) => t.array.len(),
            None => usize::MAX,
        }
    );
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=1;dyn_gets=0;hoists=1;hoist_ctx=0";
