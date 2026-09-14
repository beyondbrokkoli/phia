// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r40 = 0i64;
    let mut i_r41 = 0i64;
    let mut i_r42 = 0i64;
    let mut b_r40 = false;
    let mut b_r41 = false;
    let mut t_r47: *mut Table = std::ptr::null_mut();
    let mut p_r47: *mut bool = std::ptr::null_mut();
    let mut len_r47 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_bool());
    t_r47 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 8;
    if lim > 0 {
        let t = unsafe { &mut *t_r47 };
        if (lim as usize) > t.barray.len() {
            t.barray.resize(lim as usize, false);
        }
    }
    len_r47 = unsafe { (*t_r47).barray.len() };
    p_r47 = unsafe { (*t_r47).barray.as_mut_ptr() };
    i_r40 = 0;
    loop {
        b_r40 = i_r40 < 8;
        if b_r40 {
            i_r41 = i_r40 % 2 + i64::from(i_r40 % 2 != 0 && (i_r40 % 2 < 0) != (2 < 0)) * 2;
            b_r40 = i_r41 == 0;
            let k = i_r40;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r47 {
                unsafe {
                    *p_r47.add(k as usize) = b_r40;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r40 = i_r40 + 1;
        } else {
            break;
        }
    }
    let lim = 8;
    if lim > 0 {
        let t = unsafe { &mut *t_r47 };
        if (lim as usize) > t.barray.len() {
            t.barray.resize(lim as usize, false);
        }
    }
    len_r47 = unsafe { (*t_r47).barray.len() };
    p_r47 = unsafe { (*t_r47).barray.as_mut_ptr() };
    i_r41 = 0;
    i_r40 = 0;
    loop {
        b_r40 = i_r40 < 8;
        if b_r40 {
            let k = i_r40;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r47 {
                b_r40 = unsafe { *p_r47.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            b_r41 = b_r40 == true;
            if b_r41 {
                i_r42 = i_r41 + 1;
                i_r41 = i_r42;
            }
            i_r40 = i_r40 + 1;
        } else {
            break;
        }
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r47 };
    b_r41 = if idx < t.barray.len() {
        unsafe { *t.barray.get_unchecked(idx) }
    } else {
        false
    };
    b_r40 = b_r41 == false;
    println!("{}\t{}\t{}", "witness", i_r41, b_r40);
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=0;dyn_gets=1;hoists=2;hoist_ctx=0,0";
