// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut b_r4 = false;
    let mut t_r5: *mut Table = std::ptr::null_mut();
    let mut p_r5: *mut i64 = std::ptr::null_mut();
    let mut len_r5 = 0usize;
    let mut t_r6: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r5 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r6 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 4;
    if lim > 0 {
        let t = unsafe { &mut *t_r5 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r5 = unsafe { (*t_r5).array.len() };
    p_r5 = unsafe { (*t_r5).array.as_mut_ptr() };
    i_r0 = 0;
    while i_r0 < 4 {
        i_r1 = i_r0 + 100;
        let k = i_r0;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r5 {
            unsafe {
                *p_r5.add(k as usize) = i_r1;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r1 = i_r0 * 50;
        let k = i_r1;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r6 };
        if idx >= t.array.len() {
            t.array.resize(idx + 1, 0);
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = 1;
        }
        let k = i_r0;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r5 {
            i_r1 = unsafe { *p_r5.add(k as usize) };
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        println!(
            "{}\t{}\ttable(len={})\t{}\ttable(len={})",
            "iter",
            i_r0,
            unsafe { (*t_r5).array.len() },
            i_r1,
            unsafe { (*t_r6).array.len() }
        );
        i_r0 = i_r0 + 1;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=1;dyn_gets=0;hoists=1;hoist_ctx=0;consts_i=6;consts_b=0;consts_f=0;consts_s=0";
