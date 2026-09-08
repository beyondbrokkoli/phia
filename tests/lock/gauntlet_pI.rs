// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r16 = 0i64;
    let mut b_r16 = false;
    let mut t_r16: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r16 = &mut *new_table as *mut Table;
    tables.push(new_table);
    b_r16 = true;
    i_r16 = 0;
    while b_r16 {
        let k = i_r16;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r16 };
        if idx >= t.array.len() {
            t.array.resize(idx + 1, 0);
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = 100;
        }
        b_r16 = 0 < 0;
        i_r16 = i_r16 + 1;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=0;hoists=0;hoist_ctx=";
