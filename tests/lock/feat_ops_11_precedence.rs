// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r86 = 0i64;
    let mut i_r87 = 0i64;
    let mut i_r88 = 0i64;
    let mut i_r89 = 0i64;
    let mut i_r90 = 0i64;
    let mut i_r91 = 0i64;
    let mut i_r92 = 0i64;
    let mut b_r86 = false;
    let mut b_r87 = false;
    let mut b_r88 = false;
    let mut t_r86: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r86 = 0;
    loop {
        b_r86 = i_r86 < 1;
        if b_r86 {
            i_r86 = i_r86 + 1;
        } else {
            break;
        }
    }
    i_r87 = i_r86 + 3;
    i_r88 = i_r86 + 1;
    i_r89 = i_r88 * 3;
    i_r88 = i_r86 * 2;
    i_r90 = 10 - i_r88;
    i_r88 = i_r90 - 3;
    i_r90 = -i_r86;
    i_r91 = i_r90 * 5;
    i_r90 = i_r86 + 5;
    i_r92 = -i_r90;
    b_r86 = 5 < i_r86;
    b_r87 = !b_r86;
    b_r86 = !b_r87;
    b_r88 = !b_r86;
    let mut new_table = Box::new(Table::new());
    t_r86 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r90 = i_r87 * 100000;
    i_r87 = i_r89 * 10000;
    i_r89 = i_r90 + i_r87;
    i_r87 = i_r88 * 1000;
    i_r88 = i_r89 + i_r87;
    i_r87 = i_r88 + 120;
    i_r88 = i_r87 + 4;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r86 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r88;
    }
    i_r88 = i_r91 * 100;
    i_r91 = i_r88 + i_r92;
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r86 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r91;
    }
    if b_r87 {
        i_r91 = 1;
    } else {
        i_r91 = 0;
    }
    if b_r88 {
        i_r88 = i_r91 + 2;
        i_r92 = i_r88;
    } else {
        i_r92 = i_r91;
    }
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r86 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r92;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=3;dyn_gets=0;hoists=0;hoist_ctx=";
