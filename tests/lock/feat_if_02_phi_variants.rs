// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r89 = 0i64;
    let mut i_r90 = 0i64;
    let mut i_r91 = 0i64;
    let mut i_r92 = 0i64;
    let mut i_r93 = 0i64;
    let mut i_r94 = 0i64;
    let mut i_r95 = 0i64;
    let mut i_r96 = 0i64;
    let mut b_r89 = false;
    let mut t_r89: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    if true {
        i_r89 = 2;
    } else {
        i_r89 = 3;
    }
    if true {
        i_r90 = 15;
    } else {
        i_r90 = 10;
    }
    if false {
        i_r91 = 101;
    } else {
        i_r91 = 102;
    }
    if true {
        if false {
            i_r92 = 1;
        } else {
            i_r92 = 2;
        }
    } else {
        i_r92 = 3;
    }
    if false {
        i_r93 = 10;
    } else {
        if false {
            i_r93 = 20;
        } else {
            if true {
                i_r93 = 30;
            } else {
                i_r93 = 40;
            }
        }
    }
    i_r94 = 0;
    i_r95 = 0;
    loop {
        b_r89 = i_r95 < 6;
        if b_r89 {
            b_r89 = i_r95 >= 3;
            if b_r89 {
                i_r96 = i_r94 + i_r95;
                i_r94 = i_r96;
            }
            i_r95 = i_r95 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r89 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r89 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r89;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r89 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r90;
    }
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r89 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r91;
    }
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r89 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r92;
    }
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r89 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r93;
    }
    let k = 5;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r89 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r94;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=6;dyn_gets=0;hoists=0;hoist_ctx=";
