// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r18 = 0i64;
    let mut b_r19 = false;
    let mut b_r20 = false;
    let mut t_r21: *mut Table = std::ptr::null_mut();
    let mut p_r21: *mut i64 = std::ptr::null_mut();
    let mut len_r21 = 0usize;
    let mut s_r22 = String::new();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r21 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 3;
    if lim > 0 {
        let t = unsafe { &mut *t_r21 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r21 = unsafe { (*t_r21).array.len() };
    p_r21 = unsafe { (*t_r21).array.as_mut_ptr() };
    i_r18 = 0;
    s_r22 = "x".to_string();
    b_r19 = true;
    while i_r18 < 3 {
        let k = i_r18;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r21 {
            unsafe {
                *p_r21.add(k as usize) = i_r18;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        s_r22 = format!("{}{}", s_r22, "y");
        b_r19 = !b_r19;
        i_r18 = i_r18 + 1;
    }
    println!("{}\t{}\t{}\t{}", "mix", i_r18, s_r22, b_r19);
    println!("{}\ttable(len={})", "tab", unsafe { (*t_r21).array.len() });
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=0;consts_i=3;consts_b=1;consts_f=0;consts_s=2";
