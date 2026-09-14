// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut f_r8 = 0f64;
    let mut s_r7 = String::new();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    f_r8 = 2.5;
    s_r7 = "val=C:\\tmp\\x".to_string();
    println!("{}\t{}", "a{b}c\\d", 1);
    println!("{}\t{:?}", "C:\\path", f_r8);
    println!("{}\t{}", s_r7, 7);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=0;hoist_ctx=";
