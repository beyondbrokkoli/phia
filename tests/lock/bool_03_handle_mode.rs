// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut b_r17 = false;
    let mut t_r17 = 0i64;
    let mut t_r18 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new_bool()));
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
    if idx >= t.barray.len() {
        t.barray.resize(idx + 1, false);
    }
    unsafe {
        *t.barray.get_unchecked_mut(idx) = true;
    }
    let k = 1;
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
    if idx >= t.barray.len() {
        t.barray.resize(idx + 1, false);
    }
    unsafe {
        *t.barray.get_unchecked_mut(idx) = false;
    }
    tables.push(Box::new(Table::new()));
    t_r17 = tables.len() as i64;
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
        *t.array.get_unchecked_mut(idx) = t_r18;
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
    if t_r18 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r18 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    b_r17 = if idx < t.barray.len() {
        unsafe { *t.barray.get_unchecked(idx) }
    } else {
        false
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
    t_r18 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    println!(
        "{}\t{}\t{}",
        "grid",
        b_r17,
        match tables.get((t_r18 - 1) as usize) {
            Some(t) => format!("table#{}(len={})", t_r18, t.barray.len()),
            None => "nil".to_string(),
        }
    );
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=3;dyn_gets=3;hoists=0;hoist_ctx=";
