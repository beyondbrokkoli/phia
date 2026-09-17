// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut b_r1 = false;
    let mut s_r2 = String::new();
    let mut s_r3 = String::new();
    let mut t_r4: *mut Table = std::ptr::null_mut();
    let mut p_r4: *mut String = std::ptr::null_mut();
    let mut len_r4 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_string());
    t_r4 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 4;
    if lim > 0 {
        let t = unsafe { &mut *t_r4 };
        if (lim as usize) > t.as_string_mut().len() {
            t.as_string_mut().resize(lim as usize, String::new());
        }
    }
    len_r4 = unsafe { (*t_r4).as_string_mut().len() };
    p_r4 = unsafe { (*t_r4).as_string_mut().as_mut_ptr() };
    s_r2 = "x".to_string();
    i_r0 = 0;
    while i_r0 < 4 {
        s_r2 = format!("{}{}", s_r2, "o");
        let k = i_r0;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r4 {
            unsafe {
                *p_r4.add(k as usize) = s_r2.clone();
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        let k = i_r0;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r4 {
            s_r3 = unsafe { (*p_r4.add(k as usize)).clone() };
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        println!(
            "{}\t{}\t{}\ttable(len={})\t{}",
            "iter",
            i_r0,
            s_r2,
            unsafe { (*t_r4).as_string().len() },
            s_r3
        );
        i_r0 = i_r0 + 1;
    }
    println!("{}\t{}\ttable(len={})", "exit", s_r2, unsafe {
        (*t_r4).as_string().len()
    });
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=0;consts_i=3;consts_b=0;consts_f=0;consts_s=2";
