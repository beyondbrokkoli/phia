// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r71 = 0i64;
    let mut i_r72 = 0i64;
    let mut i_r73 = 0i64;
    let mut i_r74 = 0i64;
    let mut b_r88 = false;
    let mut t_r92: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r71 = 0;
    loop {
        b_r88 = i_r71 < 1;
        if b_r88 {
            i_r71 = i_r71 + 1;
        } else {
            break;
        }
    }
    i_r72 = 9223372036854775806 + i_r71;
    if true {
        i_r71 = 1;
    } else {
        i_r71 = 0;
    }
    b_r88 = 0 <= i_r72;
    if b_r88 {
        i_r73 = i_r71 + 10;
        i_r74 = i_r73;
    } else {
        i_r74 = i_r71;
    }
    b_r88 = i_r72 >= i_r72;
    if b_r88 {
        i_r73 = i_r74 + 100;
        i_r71 = i_r73;
    } else {
        i_r71 = i_r74;
    }
    b_r88 = 9223372036854775806 < i_r72;
    if b_r88 {
        i_r73 = i_r71 + 1000;
        i_r74 = i_r73;
    } else {
        i_r74 = i_r71;
    }
    if true {
        i_r73 = i_r74 + 10000;
        i_r71 = i_r73;
    } else {
        i_r71 = i_r74;
    }
    if true {
        i_r73 = i_r71 + 100000;
        i_r74 = i_r73;
    } else {
        i_r74 = i_r71;
    }
    if true {
        i_r73 = i_r74 + 1000000;
        i_r71 = i_r73;
    } else {
        i_r71 = i_r74;
    }
    if true {
        i_r73 = i_r71 + 10000000;
        i_r74 = i_r73;
    } else {
        i_r74 = i_r71;
    }
    let mut new_table = Box::new(Table::new());
    t_r92 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r92 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r74;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=18;consts_b=5;consts_f=4;consts_s=0";
