// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r14 = 0i64;
    let mut b_r14 = false;
    let mut t_r14: *mut Table = std::ptr::null_mut();
    let mut t_r15: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r14 = &mut *new_table as *mut Table;
    tables.push(new_table);
    t_r15 = t_r14;
    i_r14 = 0;
    loop {
        b_r14 = i_r14 < 10;
        if b_r14 {
            let mut new_table = Box::new(Table::new());
            t_r15 = &mut *new_table as *mut Table;
            tables.push(new_table);
            let k = i_r14;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r15 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = 1;
            }
            i_r14 = i_r14 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=0;hoists=0;hoist_ctx=";
