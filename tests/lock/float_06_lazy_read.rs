// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut f_r6 = 0f64;
    let mut t_r8: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_float());
    t_r8 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r8 };
    f_r6 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    f_r6 = 1.5;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r8 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r6;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=1;hoists=0;hoist_ctx=";
