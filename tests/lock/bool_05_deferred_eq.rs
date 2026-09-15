// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut b_r12 = false;
    let mut b_r13 = false;
    let mut t_r16: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_bool());
    t_r16 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r16 };
    b_r12 = if idx < t.barray.len() {
        unsafe { *t.barray.get_unchecked(idx) }
    } else {
        false
    };
    b_r13 = b_r12 == true;
    if b_r13 {
        let k = 0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r16 };
        if idx >= t.barray.len() {
            t.barray.resize(idx + 1, false);
        }
        unsafe {
            *t.barray.get_unchecked_mut(idx) = false;
        }
        b_r13 = true;
    } else {
        b_r13 = false;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r16 };
    b_r12 = if idx < t.barray.len() {
        unsafe { *t.barray.get_unchecked(idx) }
    } else {
        false
    };
    println!("{}\t{}\t{}", "eq", b_r12, b_r13);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=2;hoists=0;hoist_ctx=";
