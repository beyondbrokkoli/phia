// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r19 = 0i64;
    let mut b_r19 = false;
    let mut t_r19 = 0i64;
    let mut t_r20 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r19 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r20 = tables.len() as i64;
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
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 0;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r19 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r19 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r20;
    }
    i_r19 = 0;
    loop {
        b_r19 = i_r19 < 10;
        if b_r19 {
            let k = 0;
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
            t_r20 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let k = i_r19;
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
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = 1;
            }
            i_r19 = i_r19 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=3;dyn_gets=1;hoists=0;hoist_ctx=";
