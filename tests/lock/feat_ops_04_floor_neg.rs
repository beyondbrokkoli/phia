// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r91 = 0i64;
    let mut i_r92 = 0i64;
    let mut i_r93 = 0i64;
    let mut i_r94 = 0i64;
    let mut i_r95 = 0i64;
    let mut i_r96 = 0i64;
    let mut b_r91 = false;
    let mut f_r111 = 0f64;
    let mut f_r112 = 0f64;
    let mut f_r113 = 0f64;
    let mut t_r91: *mut Table = std::ptr::null_mut();
    let mut t_r126: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r91 = 0;
    loop {
        b_r91 = i_r91 < 1;
        if b_r91 {
            i_r91 = i_r91 + 1;
        } else {
            break;
        }
    }
    i_r92 = 7 * i_r91;
    i_r93 = 0 - i_r92;
    i_r92 = 2 * i_r91;
    let mut new_table = Box::new(Table::new());
    t_r91 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r94 = i_r93 / i_r92 - i64::from(i_r93 % i_r92 != 0 && (i_r93 < 0) != (i_r92 < 0));
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r91 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r94;
    }
    i_r94 = 3 * i_r91;
    i_r95 =
        i_r93 % i_r94 + i64::from(i_r93 % i_r94 != 0 && (i_r93 % i_r94 < 0) != (i_r94 < 0)) * i_r94;
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r91 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r95;
    }
    i_r95 = 7 * i_r91;
    i_r94 = 2 * i_r91;
    i_r96 = 0 - i_r94;
    i_r94 = i_r95 / i_r96 - i64::from(i_r95 % i_r96 != 0 && (i_r95 < 0) != (i_r96 < 0));
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r91 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r94;
    }
    i_r94 = 7 * i_r91;
    i_r96 = 3 * i_r91;
    i_r95 = 0 - i_r96;
    i_r96 =
        i_r94 % i_r95 + i64::from(i_r94 % i_r95 != 0 && (i_r94 % i_r95 < 0) != (i_r95 < 0)) * i_r95;
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r91 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r96;
    }
    i_r96 = 7 * i_r91;
    i_r95 = 2 * i_r91;
    i_r91 = 0 - i_r95;
    i_r95 = i_r96 / i_r91;
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r91 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r95;
    }
    i_r95 = i_r93 / i_r92;
    let k = 5;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r91 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r95;
    }
    let mut new_table = Box::new(Table::new_float());
    t_r126 = &mut *new_table as *mut Table;
    tables.push(new_table);
    f_r111 = 7.5;
    f_r112 = -f_r111;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r126 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r112;
    }
    f_r112 = 2.0;
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r126 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r112;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r126 };
    f_r112 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r126 };
    f_r111 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    f_r113 = (f_r112 / f_r111).floor();
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r126 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r113;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r126 };
    f_r113 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r126 };
    f_r111 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    f_r112 = f_r113 - (f_r113 / f_r111).floor() * f_r111;
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r126 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r112;
    }
    f_r112 = 7.0;
    f_r111 = 2.0;
    f_r113 = (f_r112 / f_r111).floor();
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r126 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r113;
    }
    f_r113 = 7.0;
    f_r111 = 3.0;
    f_r112 = f_r113 - (f_r113 / f_r111).floor() * f_r111;
    let k = 5;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r126 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r112;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=12;dyn_gets=4;hoists=0;hoist_ctx=";
