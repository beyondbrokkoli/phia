// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut b_r5 = false;
    let mut s_r10 = String::new();
    let mut f_r12 = 0f64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r0 = 16;
    i_r1 = 0;
    loop {
        b_r5 = i_r1 < 4;
        if b_r5 {
            i_r0 = 11;
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    println!("{}\t{}", "x", i_r0);
    i_r1 = 0;
    f_r12 = 18.38;
    loop {
        b_r5 = i_r1 < 0;
        if b_r5 {
            f_r12 = 6.34;
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    println!("{}\t{:?}", "z", f_r12);
    s_r10 = "init".to_string();
    i_r1 = 0;
    loop {
        b_r5 = i_r1 < 2;
        if b_r5 {
            s_r10 = "kept".to_string();
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    println!("{}\t{}", "s", s_r10);
    s_r10 = "init".to_string();
    i_r1 = 0;
    loop {
        b_r5 = i_r1 < 3;
        if b_r5 {
            b_r5 = i_r1 >= 0;
            if b_r5 {
                s_r10 = "taken".to_string();
            }
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    println!("{}\t{}", "r", s_r10);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=14;consts_b=0;consts_f=1;consts_s=3";
