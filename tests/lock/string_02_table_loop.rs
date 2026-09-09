// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r22 = 0i64;
    let mut b_r22 = false;
    let mut s_r22 = String::new();
    let mut s_r23 = String::new();
    let mut t_r26: *mut Table = std::ptr::null_mut();
    let mut p_r26: *mut String = std::ptr::null_mut();
    let mut len_r26 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_string());
    t_r26 = &mut *new_table as *mut Table;
    tables.push(new_table);
    s_r22 = "x".to_string();
    let lim = 4;
    if lim > 0 {
        let t = unsafe { &mut *t_r26 };
        if (lim as usize) > t.sarray.len() {
            t.sarray.resize(lim as usize, String::new());
        }
    }
    len_r26 = unsafe { (*t_r26).sarray.len() };
    p_r26 = unsafe { (*t_r26).sarray.as_mut_ptr() };
    s_r23 = s_r22.clone();
    i_r22 = 0;
    loop {
        b_r22 = i_r22 < 4;
        if b_r22 {
            s_r22 = "o".to_string();
            s_r23 = format!("{}{}", s_r23, s_r22);
            let k = i_r22;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r26 {
                unsafe {
                    *p_r26.add(k as usize) = s_r23.clone();
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = i_r22;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r26 {
                s_r22 = unsafe { (*p_r26.add(k as usize)).clone() };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            println!(
                "PROBE iter: i_r22={} s_r23={:?} len_r26={} s_r22={:?}",
                i_r22,
                s_r23,
                unsafe { (*t_r26).sarray.len() },
                s_r22
            );
            i_r22 = i_r22 + 1;
        } else {
            break;
        }
    }
    println!("PROBE exit: s_r23={:?} len_r26={}", s_r23, unsafe {
        (*t_r26).sarray.len()
    });
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=0";
