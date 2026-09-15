// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
    let mut b_r7 = false;
    let mut t_r10: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r0 = 0;
    i_r1 = 0;
    loop {
        b_r7 = i_r1 < 11;
        if b_r7 {
            i_r2 = i_r1 % 2 + i64::from(i_r1 % 2 != 0 && (i_r1 % 2 < 0) != (2 < 0)) * 2;
            b_r7 = i_r2 == 0;
            if b_r7 {
                i_r2 = i_r1 * 10;
                i_r3 = i_r0 + i_r2;
                i_r0 = i_r3;
            } else {
                b_r7 = i_r1 == 5;
                if b_r7 {
                    i_r3 = i_r0 - 100;
                    i_r0 = i_r3;
                } else {
                    i_r3 = i_r0 + 1;
                    i_r0 = i_r3;
                }
            }
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r10 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r10 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r0;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=13;consts_b=0;consts_f=0;consts_s=0";
