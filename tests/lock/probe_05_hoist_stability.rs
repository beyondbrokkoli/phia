// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r22 = 0i64;
    let mut i_r23 = 0i64;
    let mut b_r26 = false;
    let mut t_r27: *mut Table = std::ptr::null_mut();
    let mut p_r27: *mut i64 = std::ptr::null_mut();
    let mut len_r27 = 0usize;
    let mut t_r28: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r27 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r28 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 4;
    if lim > 0 {
        let t = unsafe { &mut *t_r27 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r27 = unsafe { (*t_r27).array.len() };
    p_r27 = unsafe { (*t_r27).array.as_mut_ptr() };
    i_r22 = 0;
    while i_r22 < 4 {
        i_r23 = i_r22 + 100;
        let k = i_r22;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r27 {
            unsafe {
                *p_r27.add(k as usize) = i_r23;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        i_r23 = i_r22 * 50;
        let k = i_r23;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r28 };
        if idx >= t.array.len() {
            t.array.resize(idx + 1, 0);
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = 1;
        }
        let k = i_r22;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r27 {
            i_r23 = unsafe { *p_r27.add(k as usize) };
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        println!(
            "{}\t{}\ttable(len={})\t{}\ttable(len={})",
            "iter",
            i_r22,
            unsafe { (*t_r27).array.len() },
            i_r23,
            unsafe { (*t_r28).array.len() }
        );
        i_r22 = i_r22 + 1;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=1;dyn_gets=0;hoists=1;hoist_ctx=0;consts_i=6;consts_b=0;consts_f=0;consts_s=0";
