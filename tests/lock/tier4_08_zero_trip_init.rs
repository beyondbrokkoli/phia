// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r12 = 0i64;
    let mut b_r12 = false;
    let mut t_r12 = 0i64;
    let mut t_r13 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r12 = tables.len() as i64;
    i_r12 = 100;
    while i_r12 < 8 {
        let k = 0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        if t_r12 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get((t_r12 - 1) as usize) {
            Some(t) => &**t,
            None => panic!("Runtime Error: table is nil"),
        };
        t_r13 = if idx < t.array.len() {
            unsafe { *t.array.get_unchecked(idx) }
        } else {
            0
        };
        let k = i_r12;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        if t_r13 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r13 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if idx >= t.array.len() {
            t.array.resize(idx + 1, 0);
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = 1;
        }
        i_r12 = i_r12 + 1;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=1;hoists=0;hoist_ctx=;consts_i=5;consts_b=0;consts_f=0;consts_s=0";
