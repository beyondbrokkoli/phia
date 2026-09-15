// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r18 = 0i64;
    let mut b_r18 = false;
    let mut t_r21: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_bool());
    t_r21 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r21 };
    if idx >= t.barray.len() {
        t.barray.resize(idx + 1, false);
    }
    unsafe {
        *t.barray.get_unchecked_mut(idx) = true;
    }
    i_r18 = 0;
    loop {
        let k = 0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &*t_r21 };
        b_r18 = if idx < t.barray.len() {
            unsafe { *t.barray.get_unchecked(idx) }
        } else {
            false
        };
        if b_r18 {
            i_r18 = i_r18 + 1;
            b_r18 = i_r18 == 2;
            if b_r18 {
                let k = 0;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r21 };
                if idx >= t.barray.len() {
                    t.barray.resize(idx + 1, false);
                }
                unsafe {
                    *t.barray.get_unchecked_mut(idx) = false;
                }
            }
        } else {
            break;
        }
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r21 };
    b_r18 = if idx < t.barray.len() {
        unsafe { *t.barray.get_unchecked(idx) }
    } else {
        false
    };
    println!("{}\t{}\t{}", "deferred", i_r18, b_r18);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=2;dyn_gets=2;hoists=0;hoist_ctx=;consts_i=7;consts_b=2;consts_f=0;consts_s=0";
