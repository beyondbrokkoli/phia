// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut b_r1 = false;
    let mut t_r2 = 0i64;
    let mut p_r2: *mut i64 = std::ptr::null_mut();
    let mut len_r2 = 0usize;
    let mut t_r3 = 0i64;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new()));
    t_r2 = tables.len() as i64;
    let lim = 3;
    if lim > 0 {
        if t_r2 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r2 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.as_int_mut().len() {
            t.as_int_mut().resize(lim as usize, 0);
        }
    }
    if t_r2 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r2 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r2 = t.as_int_mut().len();
    p_r2 = t.as_int_mut().as_mut_ptr();
    i_r0 = 0;
    while i_r0 < 3 {
        tables.push(Box::new(Table::new()));
        t_r3 = tables.len() as i64;
        let k = 0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        if t_r3 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r3 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if idx >= t.as_int_mut().len() {
            t.as_int_mut().resize(idx + 1, 0);
        }
        unsafe {
            *t.as_int_mut().get_unchecked_mut(idx) = i_r0;
        }
        let k = i_r0;
        if k < 0 {
            panic!("Runtime Error: Negative index in fast path");
        }
        if (k as usize) < len_r2 {
            unsafe {
                *p_r2.add(k as usize) = t_r3;
            }
        } else {
            panic!("optimizer invariant violated: fast-path bounds check failed");
        }
        println!(
            "{}\t{}\t{}\t{}",
            "iter",
            i_r0,
            match tables.get((t_r2 - 1) as usize) {
                Some(t) => format!("table#{}(len={})", t_r2, t.as_int().len()),
                None => "nil".to_string(),
            },
            match tables.get((t_r3 - 1) as usize) {
                Some(t) => format!("table#{}(len={})", t_r3, t.as_int().len()),
                None => "nil".to_string(),
            }
        );
        i_r0 = i_r0 + 1;
    }
    println!(
        "{}\t{}\t{}",
        "exit",
        i_r0,
        match tables.get((t_r2 - 1) as usize) {
            Some(t) => format!("table#{}(len={})", t_r2, t.as_int().len()),
            None => "nil".to_string(),
        }
    );
    return tables;
}

pub const STATS: &str = "fast_sets=1;fast_gets=0;dyn_sets=1;dyn_gets=0;hoists=1;hoist_ctx=0;consts_i=4;consts_b=0;consts_f=0;consts_s=0";
