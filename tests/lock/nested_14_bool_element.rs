// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut b_r0 = false;
    let mut t_r1: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_bool());
    t_r1 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r1 };
    if idx >= t.as_bool_mut().len() {
        t.as_bool_mut().resize(idx + 1, false);
    }
    unsafe {
        *t.as_bool_mut().get_unchecked_mut(idx) = true;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r1 };
    b_r0 = if idx < t.as_bool().len() {
        unsafe { *t.as_bool().get_unchecked(idx) }
    } else {
        false
    };
    println!("{}\t{}", "bool element", b_r0);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=1;hoists=0;hoist_ctx=;consts_i=4;consts_b=1;consts_f=0;consts_s=0";
