// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r26 = 0i64;
    let mut i_r27 = 0i64;
    let mut i_r28 = 0i64;
    let mut b_r26 = false;
    let mut t_r26: *mut Table = std::ptr::null_mut();
    let mut t_r27: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r26 = &mut *new_table as *mut Table;
    tables.push(new_table);
    t_r27 = t_r26;
    i_r26 = 0;
    loop {
        b_r26 = i_r26 < 3;
        if b_r26 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &*t_r27 };
            i_r27 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let mut new_table = Box::new(Table::new());
            t_r27 = &mut *new_table as *mut Table;
            tables.push(new_table);
            i_r28 = i_r27 + i_r26;
            i_r27 = i_r28 + 5;
            let k = i_r26;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r27 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = i_r27;
            }
            i_r26 = i_r26 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r26 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r27 };
    i_r27 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r26 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r27;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=2;dyn_gets=2;hoists=0;hoist_ctx=";
