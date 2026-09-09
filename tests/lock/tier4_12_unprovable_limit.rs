// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r27 = 0i64;
    let mut i_r28 = 0i64;
    let mut b_r27 = false;
    let mut t_r27 = 0i64;
    let mut t_r28 = 0i64;
    let mut t_r29 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r27 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r27 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r27 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 7;
    }
    tables.push(Box::new(Table::new()));
    t_r28 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r29 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r29 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r29 - 1) as usize) {
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
    if t_r28 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r28 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r29;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r27 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r27 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    i_r27 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    i_r28 = i_r27 + 1;
    i_r27 = 0;
    loop {
        b_r27 = i_r27 < i_r28;
        if b_r27 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r28 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r28 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r29 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let k = i_r27;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r29 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r29 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = 1;
            }
            i_r27 = i_r27 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=4;dyn_gets=2;hoists=0;hoist_ctx=";
