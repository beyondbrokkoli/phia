// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r51 = 0i64;
    let mut i_r52 = 0i64;
    let mut i_r53 = 0i64;
    let mut i_r54 = 0i64;
    let mut i_r55 = 0i64;
    let mut b_r51 = false;
    let mut b_r52 = false;
    let mut t_r51: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r51 = 0;
    i_r52 = 0;
    loop {
        b_r51 = i_r51 <= 3;
        if b_r51 {
            i_r52 = i_r52 + i_r51;
            i_r51 = i_r51 + 1;
        } else {
            break;
        }
    }
    b_r51 = false;
    i_r51 = 0;
    loop {
        b_r52 = !b_r51;
        if b_r52 {
            i_r51 = i_r51 + 1;
            b_r52 = i_r51 >= 2;
            if b_r52 {
                b_r51 = true;
            }
        } else {
            break;
        }
    }
    i_r53 = 4;
    i_r54 = 0;
    loop {
        b_r52 = i_r53 >= 1;
        if b_r52 {
            i_r55 = i_r54 * 10;
            i_r54 = i_r55 + i_r53;
            i_r53 = i_r53 - 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r51 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r55 = i_r52 * 100;
    i_r52 = i_r55 + i_r51;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r51 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r52;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r51 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r54;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=2;dyn_gets=0;hoists=0;hoist_ctx=";
