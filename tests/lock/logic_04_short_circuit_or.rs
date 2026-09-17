// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut b_r1 = false;
    let mut b_r2 = false;
    let mut b_r3 = false;
    let mut t_r5: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r5 = &mut *new_table as *mut Table;
    tables.push(new_table);
    if true {
        b_r1 = true;
    } else {
        let k = -1;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &*t_r5 };
        i_r0 = if idx < t.as_int().len() {
            unsafe { *t.as_int().get_unchecked(idx) }
        } else {
            0
        };
        b_r2 = 0 < i_r0;
        b_r1 = b_r2;
    }
    if false {
        b_r2 = true;
    } else {
        b_r2 = true;
    }
    if false {
        b_r3 = true;
    } else {
        b_r3 = false;
    }
    println!("{}\t{}\t{}\t{}", "or", b_r1, b_r2, b_r3);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=0;dyn_gets=1;hoists=0;hoist_ctx=;consts_i=11;consts_b=8;consts_f=0;consts_s=0";
