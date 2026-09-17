// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut b_r2 = false;
    let mut t_r3: *mut Table = std::ptr::null_mut();
    let mut s_r4 = String::new();
    let mut f_r5 = 0f64;
    let mut t_r6: *mut Table = std::ptr::null_mut();
    let mut t_r7: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    println!("{}\t{}\t{}\t{}", "ints", 1, 2, 3);
    println!("{}\t{:?}\t{:?}", "floats", 1.5, 2.5);
    println!("{}\t{}\t{}", "strs", "left", "right");
    println!("{}\t{}\t{}", "bools", true, true);
    println!("{}\t{}\t{}", "swap", 2, 1);
    println!("{}\t{}\t{:?}\t{}", "mixed", 7, 0.25, "seven");
    let mut new_table = Box::new(Table::new());
    t_r3 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new_float());
    t_r6 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new_string());
    t_r7 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r3 };
    if idx >= t.as_int_mut().len() {
        t.as_int_mut().resize(idx + 1, 0);
    }
    unsafe {
        *t.as_int_mut().get_unchecked_mut(idx) = 1;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r6 };
    if idx >= t.as_float_mut().len() {
        t.as_float_mut().resize(idx + 1, 0.0);
    }
    unsafe {
        *t.as_float_mut().get_unchecked_mut(idx) = 1.5;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r7 };
    if idx >= t.as_string_mut().len() {
        t.as_string_mut().resize(idx + 1, String::new());
    }
    unsafe {
        *t.as_string_mut().get_unchecked_mut(idx) = "left".to_string();
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r3 };
    i_r0 = if idx < t.as_int().len() {
        unsafe { *t.as_int().get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r6 };
    f_r5 = if idx < t.as_float().len() {
        unsafe { *t.as_float().get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r7 };
    s_r4 = if idx < t.as_string().len() {
        unsafe { t.as_string().get_unchecked(idx).clone() }
    } else {
        String::new()
    };
    println!("{}\t{}\t{:?}\t{}", "tables", i_r0, f_r5, s_r4);
    i_r0 = 0;
    while i_r0 < 1 {
        println!("{}\t{}\t{}", "shadow", 0, 15);
        i_r0 = i_r0 + 1;
    }
    println!("{}\t{}", "outer", 10);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=3;dyn_gets=3;hoists=0;hoist_ctx=;consts_i=19;consts_b=3;consts_f=3;consts_s=3";
