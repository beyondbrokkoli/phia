// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r170 = 0i64;
    let mut i_r171 = 0i64;
    let mut i_r172 = 0i64;
    let mut b_r170 = false;
    let mut b_r171 = false;
    let mut f_r179 = 0f64;
    let mut f_r180 = 0f64;
    let mut f_r181 = 0f64;
    let mut f_r182 = 0f64;
    let mut f_r183 = 0f64;
    let mut s_r170 = String::new();
    let mut s_r171 = String::new();
    let mut s_r172 = String::new();
    let mut s_r173 = String::new();
    let mut t_r170: *mut Table = std::ptr::null_mut();
    let mut t_r171: *mut Table = std::ptr::null_mut();
    let mut t_r172: *mut Table = std::ptr::null_mut();
    let mut t_r204: *mut Table = std::ptr::null_mut();
    let mut t_r205: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    f_r179 = 0.36;
    f_r180 = -f_r179;
    let mut new_table = Box::new(Table::new());
    t_r170 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r170 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 38;
    }
    let mut new_table = Box::new(Table::new());
    t_r171 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r171 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 5;
    }
    s_r170 = "echo".to_string();
    f_r179 = f_r180;
    i_r170 = 0;
    loop {
        b_r170 = i_r170 < 4;
        if b_r170 {
            if false {
                let k = 1;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r171 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = 6;
                }
                s_r173 = "alpha".to_string();
                b_r170 = s_r173 == s_r170;
                b_r171 = !b_r170;
                if b_r171 {
                    let k = 5;
                    if k < 0 {
                        panic!("Runtime Error: Negative table index");
                    }
                    let idx = k as usize;
                    let t = unsafe { &mut *t_r170 };
                    if idx >= t.array.len() {
                        t.array.resize(idx + 1, 0);
                    }
                    unsafe {
                        *t.array.get_unchecked_mut(idx) = 2;
                    }
                    i_r171 = i_r170 - 5;
                    let k = 5;
                    if k < 0 {
                        panic!("Runtime Error: Negative table index");
                    }
                    let idx = k as usize;
                    let t = unsafe { &mut *t_r170 };
                    if idx >= t.array.len() {
                        t.array.resize(idx + 1, 0);
                    }
                    unsafe {
                        *t.array.get_unchecked_mut(idx) = i_r171;
                    }
                    let k = 0;
                    if k < 0 {
                        panic!("Runtime Error: Negative table index");
                    }
                    let idx = k as usize;
                    let t = unsafe { &mut *t_r171 };
                    if idx >= t.array.len() {
                        t.array.resize(idx + 1, 0);
                    }
                    unsafe {
                        *t.array.get_unchecked_mut(idx) = -1;
                    }
                }
                if true {
                    b_r171 = f_r179 <= f_r179;
                    if b_r171 {
                        i_r171 = i_r170 - i_r170;
                        f_r181 = 3.34;
                        f_r183 = -f_r181;
                        f_r181 = 5.79;
                        b_r171 = f_r183 <= f_r181;
                        if b_r171 {
                            s_r173 = "alpha".to_string();
                            b_r171 = s_r170 == s_r173;
                            if b_r171 {}
                        } else {
                            let mut new_table = Box::new(Table::new_string());
                            t_r205 = &mut *new_table as *mut Table;
                            tables.push(new_table);
                            s_r173 = "delta".to_string();
                            let k = 0;
                            if k < 0 {
                                panic!("Runtime Error: Negative table index");
                            }
                            let idx = k as usize;
                            let t = unsafe { &mut *t_r205 };
                            if idx >= t.sarray.len() {
                                t.sarray.resize(idx + 1, String::new());
                            }
                            unsafe {
                                *t.sarray.get_unchecked_mut(idx) = s_r173.clone();
                            }
                            i_r172 = 4 + i_r171;
                            i_r170 = i_r172;
                        }
                        f_r183 = 8.75;
                        f_r182 = -f_r183;
                        f_r181 = f_r179 - f_r182;
                    } else {
                        let k = 3;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &mut *t_r171 };
                        if idx >= t.array.len() {
                            t.array.resize(idx + 1, 0);
                        }
                        unsafe {
                            *t.array.get_unchecked_mut(idx) = 5;
                        }
                        f_r181 = f_r179;
                    }
                    f_r183 = 6.63;
                    b_r171 = f_r181 < f_r183;
                    if b_r171 {
                        let mut new_table = Box::new(Table::new());
                        t_r172 = &mut *new_table as *mut Table;
                        tables.push(new_table);
                        let k = 0;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &mut *t_r172 };
                        if idx >= t.array.len() {
                            t.array.resize(idx + 1, 0);
                        }
                        unsafe {
                            *t.array.get_unchecked_mut(idx) = 14;
                        }
                        b_r171 = 19 >= i_r170;
                        if b_r171 {
                            let mut new_table = Box::new(Table::new());
                            t_r172 = &mut *new_table as *mut Table;
                            tables.push(new_table);
                            let k = 0;
                            if k < 0 {
                                panic!("Runtime Error: Negative table index");
                            }
                            let idx = k as usize;
                            let t = unsafe { &mut *t_r172 };
                            if idx >= t.array.len() {
                                t.array.resize(idx + 1, 0);
                            }
                            unsafe {
                                *t.array.get_unchecked_mut(idx) = -4;
                            }
                            f_r182 = 10.1;
                            f_r183 = -f_r182;
                            f_r182 = 2.0;
                            f_r180 = -f_r182;
                            f_r182 = f_r183 / f_r180;
                            f_r179 = f_r182;
                        } else {
                            f_r179 = f_r181;
                        }
                        let mut new_table = Box::new(Table::new_string());
                        t_r205 = &mut *new_table as *mut Table;
                        tables.push(new_table);
                        s_r173 = "echo".to_string();
                        let k = 0;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &mut *t_r205 };
                        if idx >= t.sarray.len() {
                            t.sarray.resize(idx + 1, String::new());
                        }
                        unsafe {
                            *t.sarray.get_unchecked_mut(idx) = s_r173.clone();
                        }
                    } else {
                        f_r179 = f_r181;
                    }
                }
            } else {
                f_r183 = f_r179 - f_r179;
                let mut new_table = Box::new(Table::new_float());
                t_r204 = &mut *new_table as *mut Table;
                tables.push(new_table);
                f_r181 = f_r183 - f_r183;
                let k = 0;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r204 };
                if idx >= t.farray.len() {
                    t.farray.resize(idx + 1, 0.0);
                }
                unsafe {
                    *t.farray.get_unchecked_mut(idx) = f_r181;
                }
            }
            i_r170 = i_r170 + 1;
        } else {
            break;
        }
    }
    s_r171 = "beta".to_string();
    s_r172 = format!("{}{}", s_r171, s_r170);
    s_r171 = format!("{}{}", s_r172, s_r172);
    f_r180 = 4.28;
    f_r181 = -f_r180;
    f_r180 = 2.0;
    f_r182 = -f_r180;
    f_r180 = (f_r181 / f_r182).floor();
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r171 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 1;
    }
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r170 };
    i_r171 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 5;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r170 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r171;
    }
    f_r182 = 14.9;
    f_r181 = 3.0;
    f_r183 = (f_r182 / f_r181).floor();
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r171 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 19;
    }
    let mut new_table = Box::new(Table::new_string());
    t_r205 = &mut *new_table as *mut Table;
    tables.push(new_table);
    s_r173 = format!("{}{}", s_r172, s_r171);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r205 };
    if idx >= t.sarray.len() {
        t.sarray.resize(idx + 1, String::new());
    }
    unsafe {
        *t.sarray.get_unchecked_mut(idx) = s_r173.clone();
    }
    println!(
        "{}\t{}\t{:?}\t{}\t{}\t{}\t{}\t{}\t{}\t{:?}\t{}",
        "final", 19, f_r180, s_r170, true, false, s_r172, s_r171, 9, f_r183, -2
    );
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=16;dyn_gets=1;hoists=0;hoist_ctx=";
