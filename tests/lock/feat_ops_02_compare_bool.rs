// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r44 = 0i64;
    let mut i_r45 = 0i64;
    let mut i_r46 = 0i64;
    let mut i_r47 = 0i64;
    let mut i_r48 = 0i64;
    let mut b_r52 = false;
    let mut t_r54: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r44 = 0;
    i_r45 = 0;
    loop {
        b_r52 = -4 < i_r44;
        if b_r52 {
            if true {
                i_r46 = i_r45 + 3;
                i_r47 = i_r46;
            } else {
                i_r47 = i_r45;
            }
            b_r52 = true == true;
            if b_r52 {
                i_r46 = i_r47 + 2;
                i_r48 = i_r46;
            } else {
                i_r48 = i_r47;
            }
            if false {
                i_r46 = i_r48 + 1000;
                i_r45 = i_r46;
            } else {
                i_r45 = i_r48;
            }
            i_r44 = i_r44 - 1;
        } else {
            break;
        }
    }
    if true {
        i_r46 = 5;
    } else {
        i_r46 = 0;
    }
    let mut new_table = Box::new(Table::new());
    t_r54 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r54 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r45;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r54 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r46;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=2;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=17;consts_b=7;consts_f=0;consts_s=0";
