// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r60 = 0i64;
    let mut b_r60 = false;
    let mut f_r64 = 0f64;
    let mut f_r65 = 0f64;
    let mut s_r60 = String::new();
    let mut s_r61 = String::new();
    let mut t_r60: *mut Table = std::ptr::null_mut();
    let mut t_r68: *mut Table = std::ptr::null_mut();
    let mut t_r69: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    println!("{}\t{}\t{}\t{}", "ints", 1, 2, 3);
    f_r64 = 1.5;
    f_r65 = 2.5;
    println!("{}\t{:?}\t{:?}", "floats", f_r64, f_r65);
    s_r60 = "left".to_string();
    s_r61 = "right".to_string();
    println!("{}\t{}\t{}", "strs", s_r60, s_r61);
    println!("{}\t{}\t{}", "bools", true, true);
    println!("{}\t{}\t{}", "swap", 2, 1);
    f_r65 = 0.25;
    s_r61 = "seven".to_string();
    println!("{}\t{}\t{:?}\t{}", "mixed", 7, f_r65, s_r61);
    let mut new_table = Box::new(Table::new());
    t_r60 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new_float());
    t_r68 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new_string());
    t_r69 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r60 };
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
    let t = unsafe { &mut *t_r68 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r64;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r69 };
    if idx >= t.sarray.len() {
        t.sarray.resize(idx + 1, String::new());
    }
    unsafe {
        *t.sarray.get_unchecked_mut(idx) = s_r60.clone();
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r60 };
    i_r60 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r68 };
    f_r65 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r69 };
    s_r61 = if idx < t.sarray.len() {
        unsafe { t.sarray.get_unchecked(idx).clone() }
    } else {
        String::new()
    };
    println!("{}\t{}\t{:?}\t{}", "tables", i_r60, f_r65, s_r61);
    i_r60 = 0;
    loop {
        b_r60 = i_r60 < 1;
        if b_r60 {
            println!("{}\t{}\t{}", "shadow", 0, 15);
            i_r60 = i_r60 + 1;
        } else {
            break;
        }
    }
    println!("{}\t{}", "outer", 10);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=3;dyn_gets=3;hoists=0;hoist_ctx=";
