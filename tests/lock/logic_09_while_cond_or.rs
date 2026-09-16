// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut b_r1 = false;
    let mut b_r2 = false;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r0 = 0;
    b_r1 = true;
    loop {
        b_r2 = i_r0 < 2;
        if b_r2 {
            b_r2 = true;
        } else {
            b_r2 = b_r1;
        }
        if b_r2 {
            i_r0 = i_r0 + 1;
            b_r2 = 3 < i_r0;
            if b_r2 {
                b_r1 = false;
            }
        } else {
            break;
        }
    }
    println!("{}\t{}\t{}", "orloop", i_r0, b_r1);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=4;consts_b=3;consts_f=0;consts_s=0";
