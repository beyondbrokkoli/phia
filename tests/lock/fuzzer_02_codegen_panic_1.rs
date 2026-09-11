// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r268 = 0i64;
    let mut i_r269 = 0i64;
    let mut b_r268 = false;
    let mut b_r269 = false;
    let mut b_r270 = false;
    let mut b_r271 = false;
    let mut b_r272 = false;
    let mut b_r273 = false;
    let mut f_r290 = 0f64;
    let mut f_r291 = 0f64;
    let mut f_r292 = 0f64;
    let mut f_r293 = 0f64;
    let mut f_r294 = 0f64;
    let mut f_r295 = 0f64;
    let mut s_r268 = String::new();
    let mut s_r269 = String::new();
    let mut s_r270 = String::new();
    let mut s_r271 = String::new();
    let mut s_r272 = String::new();
    let mut s_r273 = String::new();
    let mut t_r268: *mut Table = std::ptr::null_mut();
    let mut t_r269: *mut Table = std::ptr::null_mut();
    let mut t_r270: *mut Table = std::ptr::null_mut();
    let mut t_r336: *mut Table = std::ptr::null_mut();
    let mut t_r337: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    f_r290 = 2.85;
    f_r291 = -f_r290;
    s_r268 = "alpha".to_string();
    let mut new_table = Box::new(Table::new_float());
    t_r336 = &mut *new_table as *mut Table;
    tables.push(new_table);
    f_r290 = 18.13;
    f_r292 = -f_r290;
    f_r290 = f_r291 - f_r292;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r336 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r290;
    }
    let mut new_table = Box::new(Table::new());
    t_r268 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r268 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 0;
    }
    let mut new_table = Box::new(Table::new());
    t_r269 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r269 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 0;
    }
    s_r269 = "delta".to_string();
    s_r270 = format!("{}{}", s_r268, s_r269);
    let mut new_table = Box::new(Table::new());
    t_r270 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r270 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = -4;
    }
    s_r269 = "beta".to_string();
    s_r268 = format!("{}{}", s_r269, s_r270);
    f_r290 = 5.0;
    f_r292 = f_r291 / f_r290;
    f_r290 = 3.0;
    f_r291 = -f_r290;
    f_r290 = f_r292 / f_r291;
    b_r268 = true;
    s_r269 = s_r270.clone();
    f_r291 = f_r292;
    i_r268 = 0;
    loop {
        b_r269 = i_r268 < 2;
        if b_r269 {
            if true {
                s_r270 = s_r269.clone();
                f_r292 = 3.57;
                f_r293 = -f_r292;
                f_r292 = 9.19;
                b_r269 = f_r292 < f_r293;
                if b_r269 {
                    if false {
                        b_r269 = 16 >= i_r268;
                        if b_r269 {
                            s_r271 = "beta".to_string();
                            s_r271 = "echo".to_string();
                            i_r269 = 0;
                            loop {
                                b_r271 = i_r269 < 5;
                                if b_r271 {
                                    let k = 5;
                                    if k < 0 {
                                        panic!("Runtime Error: Negative table index");
                                    }
                                    let idx = k as usize;
                                    let t = unsafe { &mut *t_r270 };
                                    if idx >= t.array.len() {
                                        t.array.resize(idx + 1, 0);
                                    }
                                    unsafe {
                                        *t.array.get_unchecked_mut(idx) = 5;
                                    }
                                    i_r269 = i_r269 + 1;
                                } else {
                                    break;
                                }
                            }
                            f_r293 = 1.86;
                            f_r294 = -f_r293;
                            b_r269 = b_r268;
                        } else {
                            f_r292 = f_r290 + f_r290;
                            b_r270 = true == b_r268;
                            b_r271 = !b_r270;
                            if b_r271 {
                                b_r271 = b_r268;
                                s_r271 = s_r269.clone();
                                f_r294 = f_r291;
                            } else {
                                let k = 5;
                                if k < 0 {
                                    panic!("Runtime Error: Negative table index");
                                }
                                let idx = k as usize;
                                let t = unsafe { &*t_r269 };
                                i_r269 = if idx < t.array.len() {
                                    unsafe { *t.array.get_unchecked(idx) }
                                } else {
                                    0
                                };
                                f_r293 = 16.98;
                                b_r270 = f_r293 < f_r292;
                                b_r272 = i_r269 < 16;
                                if b_r272 {
                                    f_r293 = 19.61;
                                    f_r295 = -f_r293;
                                    b_r272 = f_r292 == f_r295;
                                    b_r273 = !b_r272;
                                    f_r295 = 17.98;
                                    f_r293 = -f_r295;
                                    f_r295 = f_r293 * f_r290;
                                    s_r272 = "echo".to_string();
                                    s_r273 = format!("{}{}", s_r272, s_r269);
                                    s_r271 = s_r273.clone();
                                    f_r294 = f_r295;
                                    b_r271 = b_r273;
                                } else {
                                    s_r271 = s_r269.clone();
                                    f_r294 = f_r291;
                                    b_r271 = b_r270;
                                }
                            }
                            b_r269 = b_r271;
                            s_r269 = s_r271.clone();
                            f_r291 = f_r294;
                        }
                    } else {
                        b_r269 = b_r268;
                    }
                    s_r271 = "echo".to_string();
                    b_r270 = s_r271 == s_r270;
                    if b_r270 {
                        b_r273 = !b_r269;
                        if b_r273 {
                            f_r295 = 19.59;
                            f_r293 = -f_r295;
                            f_r293 = 3.0;
                            b_r268 = b_r269;
                        } else {
                            let k = 0;
                            if k < 0 {
                                panic!("Runtime Error: Negative table index");
                            }
                            let idx = k as usize;
                            let t = unsafe { &*t_r336 };
                            f_r293 = if idx < t.farray.len() {
                                unsafe { *t.farray.get_unchecked(idx) }
                            } else {
                                0.0
                            };
                            f_r293 = 9.42;
                            f_r295 = -f_r293;
                            f_r295 = 2.0;
                            b_r273 = b_r269 == b_r269;
                            b_r269 = !b_r273;
                            b_r268 = b_r269;
                        }
                        let k = 4;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &*t_r269 };
                        i_r269 = if idx < t.array.len() {
                            unsafe { *t.array.get_unchecked(idx) }
                        } else {
                            0
                        };
                        let k = 5;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &mut *t_r270 };
                        if idx >= t.array.len() {
                            t.array.resize(idx + 1, 0);
                        }
                        unsafe {
                            *t.array.get_unchecked_mut(idx) = i_r269;
                        }
                        let k = 4;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &*t_r269 };
                        i_r269 = if idx < t.array.len() {
                            unsafe { *t.array.get_unchecked(idx) }
                        } else {
                            0
                        };
                        let k = 2;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &mut *t_r269 };
                        if idx >= t.array.len() {
                            t.array.resize(idx + 1, 0);
                        }
                        unsafe {
                            *t.array.get_unchecked_mut(idx) = i_r269;
                        }
                    } else {
                        b_r268 = b_r269;
                    }
                } else {
                    f_r292 = 1.9;
                    f_r292 = 9.67;
                }
            }
            let k = 5;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &*t_r268 };
            i_r269 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let k = 2;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r268 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = i_r269;
            }
            i_r268 = i_r268 + 1;
        } else {
            break;
        }
    }
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r268 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = -2;
    }
    f_r292 = 3.12;
    b_r269 = f_r292 == f_r291;
    if b_r269 {
        b_r269 = s_r269 == s_r269;
        if b_r269 {
            f_r294 = 16.98;
            f_r293 = f_r290 + f_r294;
            f_r294 = 3.42;
            f_r295 = -f_r294;
            f_r294 = f_r293 + f_r295;
            let k = 4;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r336 };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = f_r294;
            }
            if false {
                let k = 0;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &*t_r270 };
                i_r268 = if idx < t.array.len() {
                    unsafe { *t.array.get_unchecked(idx) }
                } else {
                    0
                };
                let k = 1;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r269 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r268;
                }
                let k = 0;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r270 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = 16;
                }
                let k = 3;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r268 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = -2;
                }
            } else {
                let k = 0;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r268 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = 4;
                }
            }
        }
        let mut new_table = Box::new(Table::new_string());
        t_r337 = &mut *new_table as *mut Table;
        tables.push(new_table);
        s_r272 = "beta".to_string();
        s_r273 = format!("{}{}", s_r272, s_r269);
        let k = 0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r337 };
        if idx >= t.sarray.len() {
            t.sarray.resize(idx + 1, String::new());
        }
        unsafe {
            *t.sarray.get_unchecked_mut(idx) = s_r273.clone();
        }
    }
    s_r273 = "alpha".to_string();
    s_r272 = "alpha".to_string();
    b_r269 = s_r273 == s_r272;
    b_r273 = !b_r269;
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r269 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = -18;
    }
    f_r295 = 17.14;
    f_r293 = 5.0;
    f_r294 = f_r295 / f_r293;
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r336 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r294;
    }
    i_r269 = 0;
    loop {
        b_r269 = i_r269 < 4;
        if b_r269 {
            f_r294 = 11.74;
            f_r295 = -f_r294;
            f_r294 = 3.0;
            f_r293 = f_r295 / f_r294;
            let k = 5;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r336 };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = f_r293;
            }
            let k = 4;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r268 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = 1;
            }
            let k = 2;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r270 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = 19;
            }
            i_r269 = i_r269 + 1;
        } else {
            break;
        }
    }
    s_r273 = "beta".to_string();
    s_r272 = "beta".to_string();
    s_r271 = format!("{}{}", s_r273, s_r272);
    println!("PROBE final: i_r0={} f_r291={:?} s_r271={:?} b_r273={} s_r269={:?} i_r26={} s_r268={:?} f_r290={:?} i_r185={} i_r190={}", 16, f_r291, s_r271, b_r273, s_r269, -2, s_r268, f_r290, 11, 1);
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=20;dyn_gets=6;hoists=0;hoist_ctx=";
