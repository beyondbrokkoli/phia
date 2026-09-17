// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut b_r5 = false;
    let mut b_r6 = false;
    let mut t_r12: *mut Table = std::ptr::null_mut();
    let mut p_r12: *mut bool = std::ptr::null_mut();
    let mut len_r12 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_bool());
    t_r12 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 8;
    if lim > 0 {
        let t = unsafe { &mut *t_r12 };
        if (lim as usize) > t.as_bool_mut().len() {
            t.as_bool_mut().resize(lim as usize, false);
        }
    }
    len_r12 = unsafe { (*t_r12).as_bool_mut().len() };
    p_r12 = unsafe { (*t_r12).as_bool_mut().as_mut_ptr() };
    i_r0 = 0;
    loop {
        b_r5 = i_r0 < 8;
        if b_r5 {
            i_r1 = i_r0 % 2 + i64::from(i_r0 % 2 != 0 && (i_r0 % 2 < 0) != (2 < 0)) * 2;
            b_r5 = i_r1 == 0;
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r12 {
                unsafe {
                    *p_r12.add(k as usize) = b_r5;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let lim = 8;
    if lim > 0 {
        let t = unsafe { &mut *t_r12 };
        if (lim as usize) > t.as_bool_mut().len() {
            t.as_bool_mut().resize(lim as usize, false);
        }
    }
    len_r12 = unsafe { (*t_r12).as_bool_mut().len() };
    p_r12 = unsafe { (*t_r12).as_bool_mut().as_mut_ptr() };
    i_r1 = 0;
    i_r0 = 0;
    loop {
        b_r5 = i_r0 < 8;
        if b_r5 {
            let k = i_r0;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r12 {
                b_r5 = unsafe { *p_r12.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            b_r6 = b_r5 == true;
            if b_r6 {
                i_r2 = i_r1 + 1;
                i_r1 = i_r2;
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r12 };
    b_r6 = if idx < t.as_bool().len() {
        unsafe { *t.as_bool().get_unchecked(idx) }
    } else {
        false
    };
    b_r5 = b_r6 == false;
    println!("{}\t{}\t{}", "witness", i_r1, b_r5);
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=1;dyn_sets=0;dyn_gets=1;hoists=2;hoist_ctx=0,0;consts_i=11;consts_b=2;consts_f=0;consts_s=0";
