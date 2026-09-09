// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r35 = 0i64;
    let mut i_r36 = 0i64;
    let mut i_r37 = 0i64;
    let mut i_r38 = 0i64;
    let mut b_r35 = false;
    let mut t_r35: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r35 = 0;
    i_r36 = 0;
    loop {
        b_r35 = i_r36 < 11;
        if b_r35 {
            i_r37 = i_r36 % 2;
            b_r35 = i_r37 == 0;
            if b_r35 {
                i_r37 = i_r36 * 10;
                i_r38 = i_r35 + i_r37;
                i_r35 = i_r38;
            } else {
                b_r35 = i_r36 == 5;
                if b_r35 {
                    i_r38 = i_r35 - 100;
                    i_r35 = i_r38;
                } else {
                    i_r38 = i_r35 + 1;
                    i_r35 = i_r38;
                }
            }
            i_r36 = i_r36 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r35 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r35 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r35;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=0;hoists=0;hoist_ctx=";
