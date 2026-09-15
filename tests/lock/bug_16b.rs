// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut b_r5 = false;
    let mut t_r6: *mut Table = std::ptr::null_mut();
    let mut t_r7: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r6 = &mut *new_table as *mut Table;
    tables.push(new_table);
    t_r7 = t_r6;
    i_r0 = 0;
    while i_r0 < 3 {
        let k = 0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &*t_r7 };
        i_r1 = if idx < t.array.len() {
            unsafe { *t.array.get_unchecked(idx) }
        } else {
            0
        };
        let mut new_table = Box::new(Table::new());
        t_r7 = &mut *new_table as *mut Table;
        tables.push(new_table);
        i_r2 = i_r1 + i_r0;
        i_r1 = i_r2 + 5;
        let k = i_r0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r7 };
        if idx >= t.array.len() {
            t.array.resize(idx + 1, 0);
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = i_r1;
        }
        i_r0 = i_r0 + 1;
    }
    let mut new_table = Box::new(Table::new());
    t_r6 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r7 };
    i_r1 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r6 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r1;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=2;dyn_gets=2;hoists=0;hoist_ctx=;consts_i=7;consts_b=0;consts_f=0;consts_s=0";
