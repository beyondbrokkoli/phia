// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r78 = 0i64;
    let mut i_r79 = 0i64;
    let mut i_r80 = 0i64;
    let mut b_r78 = false;
    let mut b_r79 = false;
    let mut b_r80 = false;
    let mut b_r81 = false;
    let mut t_r78: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    b_r78 = true;
    b_r79 = false;
    i_r78 = 0;
    loop {
        b_r80 = i_r78 < 2;
        if b_r80 {
            b_r78 = !b_r78;
            b_r79 = b_r78;
            i_r78 = i_r78 + 1;
        } else {
            break;
        }
    }
    b_r80 = true == b_r78;
    if b_r80 {
        i_r78 = 1;
    } else {
        i_r78 = 0;
    }
    b_r80 = false == b_r79;
    if b_r80 {
        i_r79 = i_r78 + 10;
        i_r80 = i_r79;
    } else {
        i_r80 = i_r78;
    }
    b_r80 = b_r78 == true;
    if b_r80 {
        i_r79 = i_r80 + 100;
        i_r78 = i_r79;
    } else {
        i_r78 = i_r80;
    }
    b_r80 = b_r78 == b_r79;
    if b_r80 {
        i_r79 = i_r78 + 1000;
        i_r80 = i_r79;
    } else {
        i_r80 = i_r78;
    }
    b_r80 = b_r78 == b_r79;
    b_r81 = !b_r80;
    if b_r81 {
        i_r79 = i_r80 + 10000;
        i_r78 = i_r79;
    } else {
        i_r78 = i_r80;
    }
    b_r81 = b_r78 == false;
    b_r80 = !b_r81;
    if b_r80 {
        i_r79 = i_r78 + 100000;
        i_r80 = i_r79;
    } else {
        i_r80 = i_r78;
    }
    b_r80 = false == b_r79;
    b_r81 = !b_r80;
    if b_r81 {
        i_r79 = i_r80 + 1000000;
        i_r78 = i_r79;
    } else {
        i_r78 = i_r80;
    }
    b_r81 = !b_r78;
    b_r78 = b_r81 == b_r79;
    if b_r78 {
        i_r79 = i_r78 + 10000000;
        i_r80 = i_r79;
    } else {
        i_r80 = i_r78;
    }
    let mut new_table = Box::new(Table::new());
    t_r78 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r78 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r80;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=0;hoists=0;hoist_ctx=";
