// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r20 = 0i64;
    let mut b_r20 = false;
    let mut s_r20 = String::new();
    let mut s_r21 = String::new();
    let mut t_r24: *mut Table = std::ptr::null_mut();
    let mut p_r24: *mut String = std::ptr::null_mut();
    let mut len_r24 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_string());
    t_r24 = &mut *new_table as *mut Table;
    tables.push(new_table);
    s_r20 = "x".to_string();
    let lim = 4;
    if lim > 0 {
        let t = unsafe { &mut *t_r24 };
        if (lim as usize) > t.sarray.len() {
            t.sarray.resize(lim as usize, String::new());
        }
    }
    len_r24 = unsafe { (*t_r24).sarray.len() };
    p_r24 = unsafe { (*t_r24).sarray.as_mut_ptr() };
    s_r21 = s_r20.clone();
    i_r20 = 0;
    loop {
        b_r20 = i_r20 < 4;
        if b_r20 {
            s_r20 = "o".to_string();
            s_r21 = format!("{}{}", s_r21, s_r20);
            let k = i_r20;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r24 {
                unsafe {
                    *p_r24.add(k as usize) = s_r21.clone();
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = i_r20;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r24 {
                s_r20 = unsafe { (*p_r24.add(k as usize)).clone() };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            println!(
                "{}\t{}\t{}\ttable(len={})\t{}",
                "iter",
                i_r20,
                s_r21,
                unsafe { (*t_r24).sarray.len() },
                s_r20
            );
            i_r20 = i_r20 + 1;
        } else {
            break;
        }
    }
    println!("{}\t{}\ttable(len={})", "exit", s_r21, unsafe {
        (*t_r24).sarray.len()
    });
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=0";
