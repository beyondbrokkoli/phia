// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r12 = 0i64;
    let mut b_r12 = false;
    let mut t_r12: *mut Table = std::ptr::null_mut();
    let mut t_r13: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r12 = &mut *new_table as *mut Table;
    tables.push(new_table);
    t_r13 = t_r12;
    i_r12 = 0;
    loop {
        b_r12 = i_r12 < 10;
        if b_r12 {
            let mut new_table = Box::new(Table::new());
            t_r13 = &mut *new_table as *mut Table;
            tables.push(new_table);
            let k = i_r12;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r13 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = 1;
            }
            i_r12 = i_r12 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=4;consts_b=0;consts_f=0;consts_s=0";
