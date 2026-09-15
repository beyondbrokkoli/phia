// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut s_r14 = String::new();
    let mut t_r14 = 0i64;
    let mut t_r17 = 0i64;
    let mut t_r18 = 0i64;
    let mut t_r19 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r14 = tables.len() as i64;
    tables.push(Box::new(Table::new_string()));
    t_r17 = tables.len() as i64;
    s_r14 = "alpha".to_string();
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
    if idx >= t.sarray.len() {
        t.sarray.resize(idx + 1, String::new());
    }
    unsafe {
        *t.sarray.get_unchecked_mut(idx) = s_r14.clone();
    }
    s_r14 = "beta".to_string();
    let k = 1;
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
    if idx >= t.sarray.len() {
        t.sarray.resize(idx + 1, String::new());
    }
    unsafe {
        *t.sarray.get_unchecked_mut(idx) = s_r14.clone();
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r14 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r14 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r17;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r14 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r14 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r18 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r14 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r14 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r19 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r19 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r19 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    s_r14 = if idx < t.sarray.len() {
        unsafe { t.sarray.get_unchecked(idx).clone() }
    } else {
        String::new()
    };
    println!(
        "{}\t{}\t{}\t{}\t{}",
        "nested",
        match tables.get((t_r14 - 1) as usize) {
            Some(t) => format!("table#{}(len={})", t_r14, t.array.len()),
            None => "nil".to_string(),
        },
        match tables.get((t_r18 - 1) as usize) {
            Some(t) => format!("table#{}(len={})", t_r18, t.sarray.len()),
            None => "nil".to_string(),
        },
        s_r14,
        match tables.get((t_r17 - 1) as usize) {
            Some(t) => format!("table#{}(len={})", t_r17, t.sarray.len()),
            None => "nil".to_string(),
        }
    );
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=3;dyn_gets=3;hoists=0;hoist_ctx=";
