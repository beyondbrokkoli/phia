// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    println!("{}\t{}", "a{b}c\\d", 1);
    println!("{}\t{:?}", "C:\\path", 2.5);
    println!("{}\t{}", "val=C:\\tmp\\x", 7);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=0;dyn_gets=0;hoists=0;hoist_ctx=;consts_i=2;consts_b=0;consts_f=1;consts_s=1";
