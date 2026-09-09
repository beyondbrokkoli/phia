// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut f_r36 = 0f64;
    let mut f_r37 = 0f64;
    let mut f_r38 = 0f64;
    let mut f_r39 = 0f64;
    let mut t_r35: *mut Table = std::ptr::null_mut();
    let mut t_r47: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    f_r36 = 2.5;
    f_r37 = 2.0;
    f_r38 = f_r36 * f_r37;
    f_r37 = 0.5;
    f_r36 = -f_r37;
    f_r37 = f_r38 - f_r36;
    f_r36 = 5.0;
    f_r38 = f_r37 / f_r36;
    let mut new_table = Box::new(Table::new());
    t_r35 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r35 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 5;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r35 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 15;
    }
    let mut new_table = Box::new(Table::new_float());
    t_r47 = &mut *new_table as *mut Table;
    tables.push(new_table);
    f_r36 = 1000.0;
    f_r39 = f_r38 * f_r36;
    f_r36 = f_r39 + f_r37;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r47 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r36;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=3;dyn_gets=0;hoists=0;hoist_ctx=";
