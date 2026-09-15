// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut f_r0 = 0f64;
    let mut f_r1 = 0f64;
    let mut f_r2 = 0f64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    f_r0 = 1.0 / 0.0;
    f_r1 = -1.0 / 0.0;
    f_r2 = 0.0 / 0.0;
    println!("{}\t{:?}", "pinf", f_r0);
    println!("{}\t{:?}", "ninf", f_r1);
    println!("{}\t{:?}", "nan", f_r2);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=0;consts_b=0;consts_f=7;consts_s=0";
