// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut i_r2 = 0i64;
    let mut b_r4 = false;
    let mut b_r5 = false;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    b_r4 = true;
    i_r0 = 0;
    loop {
        if true {
            b_r5 = true;
        } else {
            b_r5 = false;
        }
        if b_r5 {
            b_r5 = b_r4;
        } else {
            b_r5 = false;
        }
        if b_r5 {
            i_r0 = i_r0 + 1;
            b_r5 = 2 < i_r0;
            if b_r5 {
                b_r4 = false;
            }
        } else {
            break;
        }
    }
    println!("{}\t{}", "nested", i_r0);
    i_r1 = 0;
    i_r2 = 0;
    loop {
        b_r5 = i_r2 < 3;
        if b_r5 {
            i_r0 = 0;
            loop {
                b_r5 = i_r0 < 2;
                if b_r5 {
                    b_r5 = i_r2 == 2;
                    b_r4 = !b_r5;
                    b_r5 = b_r4;
                } else {
                    b_r5 = false;
                }
                if b_r5 {
                    i_r1 = i_r1 + 1;
                    i_r0 = i_r0 + 1;
                } else {
                    break;
                }
            }
            i_r2 = i_r2 + 1;
        } else {
            break;
        }
    }
    println!("{}\t{}\t{}", "grid", i_r1, i_r2);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=12;consts_b=7;consts_f=0;consts_s=0";
