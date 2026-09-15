// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r20 = 0i64;
    let mut b_r21 = false;
    let mut s_r22 = String::new();
    let mut s_r23 = String::new();
    let mut t_r24: *mut Table = std::ptr::null_mut();
    let mut p_r24: *mut String = std::ptr::null_mut();
    let mut len_r24 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_string());
    t_r24 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 4;
    if lim > 0 {
        let t = unsafe { &mut *t_r24 };
        if (lim as usize) > t.sarray.len() {
            t.sarray.resize(lim as usize, String::new());
        }
    }
    len_r24 = unsafe { (*t_r24).sarray.len() };
    p_r24 = unsafe { (*t_r24).sarray.as_mut_ptr() };
    s_r22 = "x".to_string();
    i_r20 = 0;
    while i_r20 < 4 {
        s_r22 = format!("{}{}", s_r22, "o");
        let k = i_r20;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r24 {
            unsafe {
                *p_r24.add(k as usize) = s_r22.clone();
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        let k = i_r20;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r24 {
            s_r23 = unsafe { (*p_r24.add(k as usize)).clone() };
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        println!(
            "{}\t{}\t{}\ttable(len={})\t{}",
            "iter",
            i_r20,
            s_r22,
            unsafe { (*t_r24).sarray.len() },
            s_r23
        );
        i_r20 = i_r20 + 1;
    }
    println!("{}\t{}\ttable(len={})", "exit", s_r22, unsafe {
        (*t_r24).sarray.len()
    });
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=0;consts_i=3;consts_b=0;consts_f=0;consts_s=2";
