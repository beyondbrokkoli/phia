// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r17 = 0i64;
    let mut i_r18 = 0i64;
    let mut b_r17 = false;
    let mut t_r17: *mut Table = std::ptr::null_mut();
    let mut p_r17: *mut i64 = std::ptr::null_mut();
    let mut len_r17 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r17 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 8;
    if lim > 0 {
        let t = unsafe { &mut *t_r17 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r17 = unsafe { (*t_r17).array.len() };
    p_r17 = unsafe { (*t_r17).array.as_mut_ptr() };
    i_r17 = 0;
    loop {
        b_r17 = i_r17 < 8;
        if b_r17 {
            i_r18 = i_r17 * i_r17;
            let k = i_r17;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r17 {
                unsafe {
                    *p_r17.add(k as usize) = i_r18;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            println!("PROBE iter: i_r17={} len_r17={}", i_r17, unsafe {
                (*t_r17).array.len()
            });
            i_r17 = i_r17 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=1;hoist_ctx=0";
