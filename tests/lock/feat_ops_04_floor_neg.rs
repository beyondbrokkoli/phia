// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r81 = 0i64;
    let mut i_r82 = 0i64;
    let mut i_r83 = 0i64;
    let mut i_r84 = 0i64;
    let mut i_r85 = 0i64;
    let mut i_r86 = 0i64;
    let mut b_r81 = false;
    let mut f_r101 = 0f64;
    let mut f_r102 = 0f64;
    let mut f_r103 = 0f64;
    let mut t_r81: *mut Table = std::ptr::null_mut();
    let mut t_r107: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r81 = 0;
    while i_r81 < 1 {
        i_r81 = i_r81 + 1;
    }
    i_r82 = 7 * i_r81;
    i_r83 = 0 - i_r82;
    i_r82 = 2 * i_r81;
    let mut new_table = Box::new(Table::new());
    t_r81 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r84 = i_r83 / i_r82 - i64::from(i_r83 % i_r82 != 0 && (i_r83 < 0) != (i_r82 < 0));
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r81 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r84;
    }
    i_r84 = 3 * i_r81;
    i_r85 =
        i_r83 % i_r84 + i64::from(i_r83 % i_r84 != 0 && (i_r83 % i_r84 < 0) != (i_r84 < 0)) * i_r84;
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r81 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r85;
    }
    i_r85 = 7 * i_r81;
    i_r84 = 2 * i_r81;
    i_r86 = 0 - i_r84;
    i_r84 = i_r85 / i_r86 - i64::from(i_r85 % i_r86 != 0 && (i_r85 < 0) != (i_r86 < 0));
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r81 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r84;
    }
    i_r84 = 7 * i_r81;
    i_r86 = 3 * i_r81;
    i_r85 = 0 - i_r86;
    i_r86 =
        i_r84 % i_r85 + i64::from(i_r84 % i_r85 != 0 && (i_r84 % i_r85 < 0) != (i_r85 < 0)) * i_r85;
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r81 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r86;
    }
    i_r86 = 7 * i_r81;
    i_r85 = 2 * i_r81;
    i_r81 = 0 - i_r85;
    i_r85 = i_r86 / i_r81;
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r81 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r85;
    }
    i_r85 = i_r83 / i_r82;
    let k = 5;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r81 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r85;
    }
    let mut new_table = Box::new(Table::new_float());
    t_r107 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r107 };
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
    let t = unsafe { &mut *t_r107 };
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
    let t = unsafe { &*t_r107 };
    f_r101 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r107 };
    f_r102 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    f_r103 = (f_r101 / f_r102).floor();
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r107 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r103;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r107 };
    f_r103 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r107 };
    f_r102 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    f_r101 = f_r103 - (f_r103 / f_r102).floor() * f_r102;
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r107 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r101;
    }
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r107 };
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
    let t = unsafe { &mut *t_r107 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 1.0;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=12;dyn_gets=4;hoists=0;hoist_ctx=;consts_i=32;consts_b=0;consts_f=9;consts_s=0";
