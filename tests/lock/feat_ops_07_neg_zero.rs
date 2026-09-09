// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r44 = 0i64;
    let mut i_r45 = 0i64;
    let mut b_r44 = false;
    let mut f_r50 = 0f64;
    let mut f_r51 = 0f64;
    let mut f_r52 = 0f64;
    let mut t_r44: *mut Table = std::ptr::null_mut();
    let mut t_r63: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    f_r50 = 1.0;
    i_r44 = 0;
    f_r51 = f_r50;
    loop {
        b_r44 = i_r44 < 1;
        if b_r44 {
            f_r50 = 1.0;
            f_r51 = f_r51 - f_r50;
            i_r44 = i_r44 + 1;
        } else {
            break;
        }
    }
    i_r45 = 5 * i_r44;
    i_r44 = 0 - i_r45;
    let mut new_table = Box::new(Table::new());
    t_r44 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r45 = -i_r44;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r44 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r45;
    }
    i_r45 = -i_r44;
    i_r44 = -i_r45;
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r44 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r44;
    }
    f_r50 = 0.0;
    let mut new_table = Box::new(Table::new_float());
    t_r63 = &mut *new_table as *mut Table;
    tables.push(new_table);
    f_r52 = -f_r51;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r63 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r52;
    }
    f_r52 = -f_r51;
    f_r51 = -f_r52;
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r63 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r51;
    }
    f_r51 = 2.5;
    f_r52 = -f_r51;
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r63 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r52;
    }
    f_r52 = 2.5;
    f_r51 = -f_r52;
    f_r52 = -f_r51;
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r63 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r52;
    }
    f_r52 = -f_r50;
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r63 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r52;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=7;dyn_gets=0;hoists=0;hoist_ctx=";
