// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r21 = 0i64;
    let mut i_r22 = 0i64;
    let mut i_r23 = 0i64;
    let mut i_r24 = 0i64;
    let mut b_r21 = false;
    let mut t_r21: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r21 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r21 = 1;
    i_r22 = 2;
    i_r23 = 0;
    loop {
        b_r21 = i_r23 < 4;
        if b_r21 {
            i_r24 = i_r22;
            i_r22 = i_r21 + 10;
            i_r21 = i_r24;
            i_r23 = i_r23 + 1;
        } else {
            break;
        }
    }
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
        *t.array.get_unchecked_mut(idx) = i_r21;
    }
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
        *t.array.get_unchecked_mut(idx) = i_r22;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=2;dyn_gets=0;hoists=0;hoist_ctx=";
