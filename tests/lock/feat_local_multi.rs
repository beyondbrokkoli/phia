// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r62 = 0i64;
    let mut b_r62 = false;
    let mut f_r66 = 0f64;
    let mut f_r67 = 0f64;
    let mut s_r62 = String::new();
    let mut s_r63 = String::new();
    let mut t_r62: *mut Table = std::ptr::null_mut();
    let mut t_r70: *mut Table = std::ptr::null_mut();
    let mut t_r71: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    println!("{}\t{}\t{}\t{}", "ints", 1, 2, 3);
    f_r66 = 1.5;
    f_r67 = 2.5;
    println!("{}\t{:?}\t{:?}", "floats", f_r66, f_r67);
    s_r62 = "left".to_string();
    s_r63 = "right".to_string();
    println!("{}\t{}\t{}", "strs", s_r62, s_r63);
    println!("{}\t{}\t{}", "bools", true, true);
    println!("{}\t{}\t{}", "swap", 2, 1);
    f_r67 = 0.25;
    s_r63 = "seven".to_string();
    println!("{}\t{}\t{:?}\t{}", "mixed", 7, f_r67, s_r63);
    let mut new_table = Box::new(Table::new());
    t_r62 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new_float());
    t_r70 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new_string());
    t_r71 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r62 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 1;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r70 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r66;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r71 };
    if idx >= t.sarray.len() {
        t.sarray.resize(idx + 1, String::new());
    }
    unsafe {
        *t.sarray.get_unchecked_mut(idx) = s_r62.clone();
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r62 };
    i_r62 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r70 };
    f_r67 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r71 };
    s_r63 = if idx < t.sarray.len() {
        unsafe { t.sarray.get_unchecked(idx).clone() }
    } else {
        String::new()
    };
    println!("{}\t{}\t{:?}\t{}", "tables", i_r62, f_r67, s_r63);
    i_r62 = 0;
    loop {
        b_r62 = i_r62 < 1;
        if b_r62 {
            println!("{}\t{}\t{}", "shadow", 0, 15);
            i_r62 = i_r62 + 1;
        } else {
            break;
        }
    }
    println!("{}\t{}", "outer", 10);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=3;dyn_gets=3;hoists=0;hoist_ctx=";
