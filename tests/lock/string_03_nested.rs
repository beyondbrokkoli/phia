// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut s_r17 = String::new();
    let mut t_r17 = 0i64;
    let mut t_r20 = 0i64;
    let mut t_r21 = 0i64;
    let mut t_r22 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r17 = tables.len() as i64;
    tables.push(Box::new(Table::new_string()));
    t_r20 = tables.len() as i64;
    s_r17 = "alpha".to_string();
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r20 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r20 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.sarray.len() {
        t.sarray.resize(idx + 1, String::new());
    }
    unsafe {
        *t.sarray.get_unchecked_mut(idx) = s_r17.clone();
    }
    s_r17 = "beta".to_string();
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r20 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r20 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.sarray.len() {
        t.sarray.resize(idx + 1, String::new());
    }
    unsafe {
        *t.sarray.get_unchecked_mut(idx) = s_r17.clone();
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r17 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r17 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r20;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r17 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r17 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r21 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r17 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r17 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r22 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r22 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r22 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    s_r17 = if idx < t.sarray.len() {
        unsafe { t.sarray.get_unchecked(idx).clone() }
    } else {
        String::new()
    };
    println!(
        "PROBE nested: t_r17={} len_r17={} t_r21={} len_r21={} s_r17={:?} t_r20={} len_r20={}",
        t_r17,
        match tables.get((t_r17 - 1) as usize) {
            Some(t) => t.array.len(),
            None => usize::MAX,
        },
        t_r21,
        match tables.get((t_r21 - 1) as usize) {
            Some(t) => t.sarray.len(),
            None => usize::MAX,
        },
        s_r17,
        t_r20,
        match tables.get((t_r20 - 1) as usize) {
            Some(t) => t.sarray.len(),
            None => usize::MAX,
        }
    );
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=3;dyn_gets=3;hoists=0;hoist_ctx=";
