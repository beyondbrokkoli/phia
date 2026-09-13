// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r55 = 0i64;
    let mut i_r56 = 0i64;
    let mut i_r57 = 0i64;
    let mut b_r55 = false;
    let mut b_r56 = false;
    let mut f_r64 = 0f64;
    let mut f_r65 = 0f64;
    let mut f_r66 = 0f64;
    let mut f_r67 = 0f64;
    let mut t_r55: *mut Table = std::ptr::null_mut();
    let mut t_r69: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    f_r64 = 1.0;
    f_r65 = 0.0;
    f_r66 = f_r64 / f_r65;
    f_r67 = f_r65 / f_r65;
    b_r55 = f_r64 < f_r66;
    if b_r55 {
        i_r55 = 1;
    } else {
        i_r55 = 0;
    }
    b_r55 = f_r67 == f_r67;
    if b_r55 {
        i_r56 = i_r55 + 10;
        i_r57 = i_r56;
    } else {
        i_r57 = i_r55;
    }
    b_r55 = f_r67 == f_r67;
    b_r56 = !b_r55;
    if b_r56 {
        i_r56 = i_r57 + 100;
        i_r55 = i_r56;
    } else {
        i_r55 = i_r57;
    }
    b_r56 = f_r66 == f_r66;
    if b_r56 {
        i_r56 = i_r55 + 1000;
        i_r57 = i_r56;
    } else {
        i_r57 = i_r55;
    }
    b_r56 = f_r66 >= f_r66;
    if b_r56 {
        i_r56 = i_r57 + 10000;
        i_r55 = i_r56;
    } else {
        i_r55 = i_r57;
    }
    let mut new_table = Box::new(Table::new());
    t_r55 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r55 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r55;
    }
    let mut new_table = Box::new(Table::new_float());
    t_r69 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r69 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r66;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r69 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r67;
    }
    f_r67 = -f_r66;
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r69 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r67;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=4;dyn_gets=0;hoists=0;hoist_ctx=";
