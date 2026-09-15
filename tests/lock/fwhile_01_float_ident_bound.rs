// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut b_r0 = false;
    let mut f_r1 = 0f64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    f_r1 = 0.0;
    loop {
        b_r0 = f_r1 < 1.0;
        if b_r0 {
            f_r1 = f_r1 + 0.25;
        } else {
            break;
        }
    }
    println!("{}\t{:?}", "f", f_r1);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=0;consts_b=0;consts_f=3;consts_s=0";
