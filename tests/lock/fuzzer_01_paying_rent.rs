// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r86 = 0i64;
    let mut i_r87 = 0i64;
    let mut i_r88 = 0i64;
    let mut b_r86 = false;
    let mut f_r90 = 0f64;
    let mut f_r91 = 0f64;
    let mut f_r92 = 0f64;
    let mut f_r93 = 0f64;
    let mut s_r86 = String::new();
    let mut t_r86: *mut Table = std::ptr::null_mut();
    let mut t_r111: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    f_r90 = 8.56;
    f_r91 = -f_r90;
    s_r86 = "beta".to_string();
    let mut new_table = Box::new(Table::new_float());
    t_r111 = &mut *new_table as *mut Table;
    tables.push(new_table);
    f_r90 = 1.87;
    f_r92 = -f_r90;
    f_r90 = 2.0;
    f_r93 = -f_r90;
    f_r90 = f_r92 / f_r93;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r111 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r90;
    }
    let mut new_table = Box::new(Table::new());
    t_r86 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r86 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 4;
    }
    i_r86 = 0;
    loop {
        b_r86 = i_r86 < 4;
        if b_r86 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r111 };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = f_r91;
            }
            f_r90 = 6.16;
            f_r93 = 3.0;
            f_r92 = -f_r93;
            f_r93 = f_r90 - (f_r90 / f_r92).floor() * f_r92;
            let k = 4;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r111 };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = f_r93;
            }
            let k = 5;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r86 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = 26;
            }
            i_r86 = i_r86 + 1;
        } else {
            break;
        }
    }
    f_r93 = 17.19;
    f_r92 = -f_r93;
    f_r93 = 3.0;
    f_r90 = -f_r93;
    f_r93 = (f_r92 / f_r90).floor();
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r111 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r93;
    }
    f_r93 = 16.65;
    f_r90 = -f_r93;
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r86 };
    i_r86 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r86 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 36;
    }
    let k = 5;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r86 };
    i_r87 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    b_r86 = f_r91 <= f_r91;
    let k = 5;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r111 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r91;
    }
    f_r90 = 4.43;
    f_r93 = -f_r90;
    f_r90 = f_r93 + f_r91;
    let mut new_table = Box::new(Table::new_float());
    t_r111 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r111 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r90;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r86 };
    i_r88 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let mut new_table = Box::new(Table::new_float());
    t_r111 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r111 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r91;
    }
    println!("PROBE final: i_r86={} f_r91={:?} s_r86={:?} b_r5={} i_r18={} b_r86={} i_r50={} i_r87={} f_r90={:?} i_r73={}", i_r86, f_r91, s_r86, false, -36, b_r86, 6, i_r87, f_r90, 0);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=10;dyn_gets=3;hoists=0;hoist_ctx=";
