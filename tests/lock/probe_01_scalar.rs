// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r20 = 0i64;
    let mut i_r21 = 0i64;
    let mut f_r22 = 0f64;
    let mut t_r20: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    f_r22 = 2.5;
    let mut new_table = Box::new(Table::new());
    t_r20 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r20 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 7;
    }
    println!(
        "PROBE scalars: i_r0={} f_r22={:?} b_r2={} len_r20={}",
        5,
        f_r22,
        true,
        unsafe { (*t_r20).array.len() }
    );
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r20 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 6;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r20 };
    i_r20 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r20 };
    i_r21 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    println!("PROBE after: i_r20={} i_r21={}", i_r20, i_r21);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=2;dyn_gets=2;hoists=0;hoist_ctx=";
