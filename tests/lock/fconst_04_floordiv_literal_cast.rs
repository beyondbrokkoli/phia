// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r18 = 0i64;
    let mut b_r19 = false;
    let mut f_r20 = 0f64;
    let mut f_r21 = 0f64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    if true {
        i_r18 = 0;
        while i_r18 < 2 {
            i_r18 = i_r18 + 1;
        }
        f_r20 = ((-8.45 / 4.0) as f64).floor();
        f_r21 = -8.45 - ((-8.45 / 5.0) as f64).floor() * 5.0;
    } else {
        f_r20 = -8.45;
        f_r21 = -8.45;
    }
    println!("{}\t{:?}", "f", f_r20);
    println!("{}\t{:?}", "g", f_r21);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=3;consts_b=2;consts_f=6;consts_s=0";
