// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r6 = 0i64;
    let mut b_r7 = false;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r6 = 0;
    while i_r6 < 3 {
        i_r6 = i_r6 + 1;
    }
    println!("{}\t{}", "i", i_r6);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=3;consts_b=0;consts_f=0;consts_s=0";
