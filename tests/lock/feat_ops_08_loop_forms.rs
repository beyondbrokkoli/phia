// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r47 = 0i64;
    let mut i_r48 = 0i64;
    let mut i_r49 = 0i64;
    let mut i_r50 = 0i64;
    let mut i_r51 = 0i64;
    let mut b_r55 = false;
    let mut b_r56 = false;
    let mut t_r60: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r47 = 0;
    i_r48 = 0;
    loop {
        b_r55 = i_r47 <= 3;
        if b_r55 {
            i_r48 = i_r48 + i_r47;
            i_r47 = i_r47 + 1;
        } else {
            break;
        }
    }
    b_r55 = false;
    i_r47 = 0;
    loop {
        b_r56 = !b_r55;
        if b_r56 {
            i_r47 = i_r47 + 1;
            b_r56 = i_r47 >= 2;
            if b_r56 {
                b_r55 = true;
            }
        } else {
            break;
        }
    }
    i_r49 = 4;
    i_r50 = 0;
    loop {
        b_r56 = i_r49 >= 1;
        if b_r56 {
            i_r51 = i_r50 * 10;
            i_r50 = i_r51 + i_r49;
            i_r49 = i_r49 - 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r60 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r51 = i_r48 * 100;
    i_r48 = i_r51 + i_r47;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r60 };
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
    let t = unsafe { &mut *t_r60 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r50;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=2;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=15;consts_b=2;consts_f=0;consts_s=0";
