// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r37 = 0i64;
    let mut i_r38 = 0i64;
    let mut i_r39 = 0i64;
    let mut b_r37 = false;
    let mut b_r38 = false;
    let mut t_r44: *mut Table = std::ptr::null_mut();
    let mut p_r44: *mut bool = std::ptr::null_mut();
    let mut len_r44 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_bool());
    t_r44 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 8;
    if lim > 0 {
        let t = unsafe { &mut *t_r44 };
        if (lim as usize) > t.barray.len() {
            t.barray.resize(lim as usize, false);
        }
    }
    len_r44 = unsafe { (*t_r44).barray.len() };
    p_r44 = unsafe { (*t_r44).barray.as_mut_ptr() };
    i_r37 = 0;
    loop {
        b_r37 = i_r37 < 8;
        if b_r37 {
            i_r38 = i_r37 % 2 + i64::from(i_r37 % 2 != 0 && (i_r37 % 2 < 0) != (2 < 0)) * 2;
            b_r37 = i_r38 == 0;
            let k = i_r37;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r44 {
                unsafe {
                    *p_r44.add(k as usize) = b_r37;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r37 = i_r37 + 1;
        } else {
            break;
        }
    }
    let lim = 8;
    if lim > 0 {
        let t = unsafe { &mut *t_r44 };
        if (lim as usize) > t.barray.len() {
            t.barray.resize(lim as usize, false);
        }
    }
    len_r44 = unsafe { (*t_r44).barray.len() };
    p_r44 = unsafe { (*t_r44).barray.as_mut_ptr() };
    i_r38 = 0;
    i_r37 = 0;
    loop {
        b_r37 = i_r37 < 8;
        if b_r37 {
            let k = i_r37;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r44 {
                b_r37 = unsafe { *p_r44.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            b_r38 = b_r37 == true;
            if b_r38 {
                i_r39 = i_r38 + 1;
                i_r38 = i_r39;
            }
            i_r37 = i_r37 + 1;
        } else {
            break;
        }
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r44 };
    b_r38 = if idx < t.barray.len() {
        unsafe { *t.barray.get_unchecked(idx) }
    } else {
        false
    };
    b_r37 = b_r38 == false;
    println!("{}\t{}\t{}", "witness", i_r38, b_r37);
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=0;dyn_gets=1;hoists=2;hoist_ctx=0,0";
