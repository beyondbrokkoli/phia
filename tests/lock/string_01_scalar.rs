// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut b_r30 = false;
    let mut b_r31 = false;
    let mut b_r32 = false;
    let mut s_r30 = String::new();
    let mut s_r31 = String::new();
    let mut s_r32 = String::new();
    let mut s_r33 = String::new();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    s_r30 = "pha".to_string();
    s_r31 = "ia".to_string();
    s_r32 = format!("{}{}", s_r30, s_r31);
    println!(
        "PROBE concat: s_r30={:?} s_r31={:?} s_r32={:?}",
        s_r30, s_r31, s_r32
    );
    s_r32 = "-".to_string();
    s_r33 = format!("{}{}", s_r30, s_r32);
    s_r32 = format!("{}{}", s_r33, s_r31);
    println!("PROBE chain: s_r32={:?}", s_r32);
    s_r32 = "pha".to_string();
    b_r30 = s_r30 == s_r32;
    b_r31 = s_r30 == s_r31;
    b_r32 = !b_r31;
    println!("PROBE eq: b_r30={} b_r32={}", b_r30, b_r32);
    s_r32 = "x".to_string();
    s_r33 = format!("{}{}", s_r30, s_r32);
    s_r32 = "phax".to_string();
    b_r32 = s_r33 == s_r32;
    b_r31 = s_r30 == s_r31;
    b_r30 = !b_r31;
    println!("PROBE prec: b_r32={} b_r30={}", b_r32, b_r30);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=0;hoist_ctx=";
