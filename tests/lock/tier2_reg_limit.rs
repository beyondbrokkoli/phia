// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r28 = 0i64;
    let mut i_r29 = 0i64;
    let mut i_r30 = 0i64;
    let mut i_r31 = 0i64;
    let mut b_r28 = false;
    let mut t_r28: *mut Table = std::ptr::null_mut();
    let mut p_r28: *mut i64 = std::ptr::null_mut();
    let mut len_r28 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r28 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r28 = 0;
    i_r29 = 0;
    loop {
        b_r28 = i_r29 < 2;
        if b_r28 {
            i_r30 = i_r28 + 5;
            let lim = i_r30;
            if lim > 0 {
                let t = unsafe { &mut *t_r28 };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            len_r28 = unsafe { (*t_r28).array.len() };
            p_r28 = unsafe { (*t_r28).array.as_mut_ptr() };
            i_r30 = 0;
            loop {
                b_r28 = i_r30 < i_r28;
                if b_r28 {
                    i_r31 = i_r30 + 5;
                    let k = i_r31;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r28 {
                        unsafe {
                            *p_r28.add(k as usize) = 1;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r30 = i_r30 + 1;
                } else {
                    break;
                }
            }
            i_r28 = i_r28 + 3;
            i_r29 = i_r29 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=1";
