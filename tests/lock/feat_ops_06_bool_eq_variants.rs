// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r77 = 0i64;
    let mut i_r78 = 0i64;
    let mut i_r79 = 0i64;
    let mut b_r93 = false;
    let mut b_r94 = false;
    let mut b_r95 = false;
    let mut b_r96 = false;
    let mut t_r108: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    b_r93 = true;
    b_r94 = false;
    i_r77 = 0;
    loop {
        b_r95 = i_r77 < 2;
        if b_r95 {
            b_r93 = !b_r93;
            b_r94 = b_r93;
            i_r77 = i_r77 + 1;
        } else {
            break;
        }
    }
    b_r95 = true == b_r93;
    if b_r95 {
        i_r77 = 1;
    } else {
        i_r77 = 0;
    }
    b_r95 = false == b_r94;
    if b_r95 {
        i_r78 = i_r77 + 10;
        i_r79 = i_r78;
    } else {
        i_r79 = i_r77;
    }
    b_r95 = b_r93 == true;
    if b_r95 {
        i_r78 = i_r79 + 100;
        i_r77 = i_r78;
    } else {
        i_r77 = i_r79;
    }
    b_r95 = b_r93 == b_r94;
    if b_r95 {
        i_r78 = i_r77 + 1000;
        i_r79 = i_r78;
    } else {
        i_r79 = i_r77;
    }
    b_r95 = b_r93 == b_r94;
    b_r96 = !b_r95;
    if b_r96 {
        i_r78 = i_r79 + 10000;
        i_r77 = i_r78;
    } else {
        i_r77 = i_r79;
    }
    b_r96 = b_r93 == false;
    b_r95 = !b_r96;
    if b_r95 {
        i_r78 = i_r77 + 100000;
        i_r79 = i_r78;
    } else {
        i_r79 = i_r77;
    }
    b_r95 = false == b_r94;
    b_r96 = !b_r95;
    if b_r96 {
        i_r78 = i_r79 + 1000000;
        i_r77 = i_r78;
    } else {
        i_r77 = i_r79;
    }
    b_r96 = !b_r93;
    b_r93 = b_r96 == b_r94;
    if b_r93 {
        i_r78 = i_r77 + 10000000;
        i_r79 = i_r78;
    } else {
        i_r79 = i_r77;
    }
    let mut new_table = Box::new(Table::new());
    t_r108 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r108 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r79;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=14;consts_b=7;consts_f=0;consts_s=0";
