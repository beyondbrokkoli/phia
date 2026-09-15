// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut b_r0 = false;
    let mut b_r1 = false;
    let mut t_r4: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_bool());
    t_r4 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r4 };
    b_r0 = if idx < t.barray.len() {
        unsafe { *t.barray.get_unchecked(idx) }
    } else {
        false
    };
    b_r1 = b_r0 == true;
    if b_r1 {
        let k = 0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r4 };
        if idx >= t.barray.len() {
            t.barray.resize(idx + 1, false);
        }
        unsafe {
            *t.barray.get_unchecked_mut(idx) = false;
        }
        b_r1 = true;
    } else {
        b_r1 = false;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r4 };
    b_r0 = if idx < t.barray.len() {
        unsafe { *t.barray.get_unchecked(idx) }
    } else {
        false
    };
    println!("{}\t{}\t{}", "eq", b_r0, b_r1);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=2;hoists=0;hoist_ctx=;consts_i=3;consts_b=4;consts_f=0;consts_s=0";
