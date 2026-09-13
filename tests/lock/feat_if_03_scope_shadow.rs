// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r53 = 0i64;
    let mut i_r54 = 0i64;
    let mut i_r55 = 0i64;
    let mut i_r56 = 0i64;
    let mut t_r53: *mut Table = std::ptr::null_mut();
    let mut t_r54: *mut Table = std::ptr::null_mut();
    let mut t_r55: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    if true {
        i_r53 = 1;
    } else {
        i_r53 = 1;
    }
    if false {
        i_r54 = 50;
    } else {
        i_r54 = i_r53;
    }
    if true {
        i_r53 = i_r54 + 1;
        i_r55 = i_r53;
    } else {
        i_r55 = 0;
    }
    let mut new_table = Box::new(Table::new());
    t_r53 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r53 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 7;
    }
    if true {
        let mut new_table = Box::new(Table::new());
        t_r54 = &mut *new_table as *mut Table;
        tables.push(new_table);
        let k = 0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r54 };
        if idx >= t.array.len() {
            t.array.resize(idx + 1, 0);
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = 9;
        }
        t_r55 = t_r54;
    } else {
        t_r55 = t_r53;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r55 };
    i_r53 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    i_r56 = i_r53 + 1;
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r55 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r56;
    }
    let mut new_table = Box::new(Table::new());
    t_r54 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r54 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r54;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r54 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r55;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r55 };
    i_r56 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r54 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r56;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=6;dyn_gets=2;hoists=0;hoist_ctx=";
