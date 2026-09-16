// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut i_r3 = 0i64;
    let mut i_r4 = 0i64;
    let mut i_r5 = 0i64;
    let mut b_r8 = false;
    let mut t_r12: *mut Table = std::ptr::null_mut();
    let mut p_r12: *mut i64 = std::ptr::null_mut();
    let mut len_r12 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r12 = &mut *new_table as *mut Table;
    tables.push(new_table);
    if true {
        b_r8 = true;
    } else {
        b_r8 = false;
    }
    if b_r8 {
        let k = 0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r12 };
        if idx >= t.array.len() {
            t.array.resize(idx + 1, 0);
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = 1;
        }
    } else {
        let k = 0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r12 };
        if idx >= t.array.len() {
            t.array.resize(idx + 1, 0);
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = 2;
        }
    }
    let lim = 5;
    if lim > 0 {
        let t = unsafe { &mut *t_r12 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r12 = unsafe { (*t_r12).array.len() };
    p_r12 = unsafe { (*t_r12).array.as_mut_ptr() };
    i_r0 = 0;
    loop {
        b_r8 = i_r0 < 4;
        if b_r8 {
            i_r1 = i_r0 % 2 + i64::from(i_r0 % 2 != 0 && (i_r0 % 2 < 0) != (2 < 0)) * 2;
            b_r8 = i_r1 == 0;
            if b_r8 {
                b_r8 = true;
            } else {
                b_r8 = false;
            }
            if b_r8 {
                i_r5 = i_r0 + 1;
                let k = i_r5;
                if k < 0 {
                    panic!("Runtime Error: Negative index in fast path");
                }
                if (k as usize) < len_r12 {
                    unsafe {
                        *p_r12.add(k as usize) = 1;
                    }
                } else {
                    panic!("optimizer invariant violated: fast-path bounds check failed");
                }
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r12 };
    i_r1 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r12 };
    i_r2 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r12 };
    i_r3 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r12 };
    i_r4 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r12 };
    i_r5 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    println!(
        "{}\t{}\t{}\t{}\t{}\t{}",
        "done", i_r1, i_r2, i_r3, i_r4, i_r5
    );
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=2;dyn_gets=5;hoists=1;hoist_ctx=0;consts_i=19;consts_b=4;consts_f=0;consts_s=0";
