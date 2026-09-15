// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r37 = 0i64;
    let mut i_r38 = 0i64;
    let mut i_r39 = 0i64;
    let mut b_r42 = false;
    let mut b_r43 = false;
    let mut t_r49: *mut Table = std::ptr::null_mut();
    let mut p_r49: *mut bool = std::ptr::null_mut();
    let mut len_r49 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_bool());
    t_r49 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 8;
    if lim > 0 {
        let t = unsafe { &mut *t_r49 };
        if (lim as usize) > t.barray.len() {
            t.barray.resize(lim as usize, false);
        }
    }
    len_r49 = unsafe { (*t_r49).barray.len() };
    p_r49 = unsafe { (*t_r49).barray.as_mut_ptr() };
    i_r37 = 0;
    loop {
        b_r42 = i_r37 < 8;
        if b_r42 {
            i_r38 = i_r37 % 2 + i64::from(i_r37 % 2 != 0 && (i_r37 % 2 < 0) != (2 < 0)) * 2;
            b_r42 = i_r38 == 0;
            let k = i_r37;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r49 {
                unsafe {
                    *p_r49.add(k as usize) = b_r42;
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
        let t = unsafe { &mut *t_r49 };
        if (lim as usize) > t.barray.len() {
            t.barray.resize(lim as usize, false);
        }
    }
    len_r49 = unsafe { (*t_r49).barray.len() };
    p_r49 = unsafe { (*t_r49).barray.as_mut_ptr() };
    i_r38 = 0;
    i_r37 = 0;
    loop {
        b_r42 = i_r37 < 8;
        if b_r42 {
            let k = i_r37;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r49 {
                b_r42 = unsafe { *p_r49.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            b_r43 = b_r42 == true;
            if b_r43 {
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
    let t = unsafe { &*t_r49 };
    b_r43 = if idx < t.barray.len() {
        unsafe { *t.barray.get_unchecked(idx) }
    } else {
        false
    };
    b_r42 = b_r43 == false;
    println!("{}\t{}\t{}", "witness", i_r38, b_r42);
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=0;dyn_gets=1;hoists=2;hoist_ctx=0,0;consts_i=11;consts_b=2;consts_f=0;consts_s=0";
