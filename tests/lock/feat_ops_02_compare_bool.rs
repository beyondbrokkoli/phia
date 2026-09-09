// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r47 = 0i64;
    let mut i_r48 = 0i64;
    let mut i_r49 = 0i64;
    let mut i_r50 = 0i64;
    let mut i_r51 = 0i64;
    let mut b_r47 = false;
    let mut t_r47: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r47 = 0;
    i_r48 = 0;
    loop {
        b_r47 = -4 < i_r47;
        if b_r47 {
            if true {
                i_r49 = i_r48 + 3;
                i_r50 = i_r49;
            } else {
                i_r50 = i_r48;
            }
            b_r47 = true == true;
            if b_r47 {
                i_r49 = i_r50 + 2;
                i_r51 = i_r49;
            } else {
                i_r51 = i_r50;
            }
            if false {
                i_r49 = i_r51 + 1000;
                i_r48 = i_r49;
            } else {
                i_r48 = i_r51;
            }
            i_r47 = i_r47 - 1;
        } else {
            break;
        }
    }
    if true {
        i_r49 = 5;
    } else {
        i_r49 = 0;
    }
    let mut new_table = Box::new(Table::new());
    t_r47 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r47 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r48;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r47 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r49;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=2;dyn_gets=0;hoists=0;hoist_ctx=";
