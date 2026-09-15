// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut b_r16 = false;
    let mut b_r17 = false;
    let mut b_r18 = false;
    let mut b_r19 = false;
    let mut t_r31: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    b_r16 = true;
    b_r17 = false;
    i_r0 = 0;
    loop {
        b_r18 = i_r0 < 2;
        if b_r18 {
            b_r16 = !b_r16;
            b_r17 = b_r16;
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    b_r18 = true == b_r16;
    if b_r18 {
        i_r0 = 1;
    } else {
        i_r0 = 0;
    }
    b_r18 = false == b_r17;
    if b_r18 {
        i_r1 = i_r0 + 10;
        i_r2 = i_r1;
    } else {
        i_r2 = i_r0;
    }
    b_r18 = b_r16 == true;
    if b_r18 {
        i_r1 = i_r2 + 100;
        i_r0 = i_r1;
    } else {
        i_r0 = i_r2;
    }
    b_r18 = b_r16 == b_r17;
    if b_r18 {
        i_r1 = i_r0 + 1000;
        i_r2 = i_r1;
    } else {
        i_r2 = i_r0;
    }
    b_r18 = b_r16 == b_r17;
    b_r19 = !b_r18;
    if b_r19 {
        i_r1 = i_r2 + 10000;
        i_r0 = i_r1;
    } else {
        i_r0 = i_r2;
    }
    b_r19 = b_r16 == false;
    b_r18 = !b_r19;
    if b_r18 {
        i_r1 = i_r0 + 100000;
        i_r2 = i_r1;
    } else {
        i_r2 = i_r0;
    }
    b_r18 = false == b_r17;
    b_r19 = !b_r18;
    if b_r19 {
        i_r1 = i_r2 + 1000000;
        i_r0 = i_r1;
    } else {
        i_r0 = i_r2;
    }
    b_r19 = !b_r16;
    b_r16 = b_r19 == b_r17;
    if b_r16 {
        i_r1 = i_r0 + 10000000;
        i_r2 = i_r1;
    } else {
        i_r2 = i_r0;
    }
    let mut new_table = Box::new(Table::new());
    t_r31 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r31 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r2;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=14;consts_b=7;consts_f=0;consts_s=0";
