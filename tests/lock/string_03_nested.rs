// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut t_r0 = 0i64;
    let mut s_r1 = String::new();
    let mut t_r2 = 0i64;
    let mut t_r3 = 0i64;
    let mut t_r4 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r0 = tables.len() as i64;
    tables.push(Box::new(Table::new_string()));
    t_r2 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r2 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r2 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.as_string_mut().len() {
        t.as_string_mut().resize(idx + 1, String::new());
    }
    unsafe {
        *t.as_string_mut().get_unchecked_mut(idx) = "alpha".to_string();
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r2 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r2 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.as_string_mut().len() {
        t.as_string_mut().resize(idx + 1, String::new());
    }
    unsafe {
        *t.as_string_mut().get_unchecked_mut(idx) = "beta".to_string();
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r0 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r0 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = t_r2;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r0 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r0 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r3 = if idx < t.as_int().len() {
        unsafe { *t.as_int().get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r0 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r0 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r4 = if idx < t.as_int().len() {
        unsafe { *t.as_int().get_unchecked(idx) }
    } else {
        0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r4 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r4 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    s_r1 = if idx < t.as_string().len() {
        unsafe { t.as_string().get_unchecked(idx).clone() }
    } else {
        String::new()
    };
    println!(
        "{}\t{}\t{}\t{}\t{}",
        "nested",
        match tables.get((t_r0 - 1) as usize) {
            Some(t) => format!("table#{}(len={})", t_r0, t.as_int().len()),
            None => "nil".to_string(),
        },
        match tables.get((t_r3 - 1) as usize) {
            Some(t) => format!("table#{}(len={})", t_r3, t.as_string().len()),
            None => "nil".to_string(),
        },
        s_r1,
        match tables.get((t_r2 - 1) as usize) {
            Some(t) => format!("table#{}(len={})", t_r2, t.as_string().len()),
            None => "nil".to_string(),
        }
    );
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=3;dyn_gets=3;hoists=0;hoist_ctx=;consts_i=6;consts_b=0;consts_f=0;consts_s=2";
