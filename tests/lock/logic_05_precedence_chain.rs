// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut b_r0 = false;
    let mut b_r1 = false;
    let mut b_r2 = false;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    if true {
        b_r0 = true;
    } else {
        if false {
            b_r0 = false;
        } else {
            b_r0 = false;
        }
    }
    if true {
        b_r1 = true;
    } else {
        b_r1 = false;
    }
    if b_r1 {
        b_r1 = false;
    } else {
        b_r1 = false;
    }
    if false {
        b_r2 = true;
    } else {
        b_r2 = true;
    }
    println!("{}\t{}\t{}\t{}", "prec", b_r0, b_r1, b_r2);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=0;consts_b=14;consts_f=0;consts_s=0";
