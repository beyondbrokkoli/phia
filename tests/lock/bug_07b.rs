// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut b_r1 = false;
    let mut t_r2: *mut Table = std::ptr::null_mut();
    let mut t_r3: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r2 = &mut *new_table as *mut Table;
    tables.push(new_table);
    t_r3 = t_r2;
    i_r0 = 0;
    while i_r0 < 10 {
        let mut new_table = Box::new(Table::new());
        t_r3 = &mut *new_table as *mut Table;
        tables.push(new_table);
        let k = i_r0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r3 };
        if idx >= t.array.len() {
            t.array.resize(idx + 1, 0);
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = 1;
        }
        i_r0 = i_r0 + 1;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=4;consts_b=0;consts_f=0;consts_s=0";
