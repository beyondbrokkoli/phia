// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
    let mut i_r4 = 0i64;
    let mut i_r5 = 0i64;
    let mut b_r20 = false;
    let mut t_r21: *mut Table = std::ptr::null_mut();
    let mut f_r22 = 0f64;
    let mut f_r23 = 0f64;
    let mut f_r24 = 0f64;
    let mut t_r28: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r0 = 0;
    while i_r0 < 1 {
        i_r0 = i_r0 + 1;
    }
    i_r1 = 7 * i_r0;
    i_r2 = 0 - i_r1;
    i_r1 = 2 * i_r0;
    let mut new_table = Box::new(Table::new());
    t_r21 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r3 = i_r2 / i_r1 - i64::from(i_r2 % i_r1 != 0 && (i_r2 < 0) != (i_r1 < 0));
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r21 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r3;
    }
    i_r3 = 3 * i_r0;
    i_r4 = i_r2 % i_r3 + i64::from(i_r2 % i_r3 != 0 && (i_r2 % i_r3 < 0) != (i_r3 < 0)) * i_r3;
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r21 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r4;
    }
    i_r4 = 7 * i_r0;
    i_r3 = 2 * i_r0;
    i_r5 = 0 - i_r3;
    i_r3 = i_r4 / i_r5 - i64::from(i_r4 % i_r5 != 0 && (i_r4 < 0) != (i_r5 < 0));
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r21 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r3;
    }
    i_r3 = 7 * i_r0;
    i_r5 = 3 * i_r0;
    i_r4 = 0 - i_r5;
    i_r5 = i_r3 % i_r4 + i64::from(i_r3 % i_r4 != 0 && (i_r3 % i_r4 < 0) != (i_r4 < 0)) * i_r4;
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r21 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r5;
    }
    i_r5 = 7 * i_r0;
    i_r4 = 2 * i_r0;
    i_r0 = 0 - i_r4;
    i_r4 = i_r5 / i_r0;
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r21 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r4;
    }
    i_r4 = i_r2 / i_r1;
    let k = 5;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r21 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r4;
    }
    let mut new_table = Box::new(Table::new_float());
    t_r28 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r28 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = -7.5;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r28 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 2.0;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r28 };
    f_r22 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r28 };
    f_r23 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    f_r24 = (f_r22 / f_r23).floor();
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r28 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r24;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r28 };
    f_r24 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r28 };
    f_r23 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    f_r22 = f_r24 - (f_r24 / f_r23).floor() * f_r23;
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r28 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r22;
    }
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r28 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 3.0;
    }
    let k = 5;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r28 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 1.0;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=12;dyn_gets=4;hoists=0;hoist_ctx=;consts_i=32;consts_b=0;consts_f=9;consts_s=0";
