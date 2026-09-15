// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut b_r2 = false;
    let mut t_r3: *mut Table = std::ptr::null_mut();
    let mut p_r3: *mut i64 = std::ptr::null_mut();
    let mut len_r3 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r3 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 8;
    if lim > 0 {
        let t = unsafe { &mut *t_r3 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r3 = unsafe { (*t_r3).array.len() };
    p_r3 = unsafe { (*t_r3).array.as_mut_ptr() };
    i_r0 = 0;
    while i_r0 < 8 {
        i_r1 = i_r0 * i_r0;
        let k = i_r0;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r3 {
            unsafe {
                *p_r3.add(k as usize) = i_r1;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        println!("{}\t{}\ttable(len={})", "iter", i_r0, unsafe {
            (*t_r3).array.len()
        });
        i_r0 = i_r0 + 1;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=0;consts_i=5;consts_b=0;consts_f=0;consts_s=0";
