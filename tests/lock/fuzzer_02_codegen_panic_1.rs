// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r263 = 0i64;
    let mut i_r264 = 0i64;
    let mut b_r271 = false;
    let mut b_r272 = false;
    let mut b_r273 = false;
    let mut b_r274 = false;
    let mut t_r287: *mut Table = std::ptr::null_mut();
    let mut t_r288: *mut Table = std::ptr::null_mut();
    let mut t_r289: *mut Table = std::ptr::null_mut();
    let mut s_r290 = String::new();
    let mut s_r291 = String::new();
    let mut s_r292 = String::new();
    let mut s_r293 = String::new();
    let mut f_r295 = 0f64;
    let mut f_r296 = 0f64;
    let mut t_r298: *mut Table = std::ptr::null_mut();
    let mut t_r299: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_float());
    t_r298 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r298 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 15.28;
    }
    let mut new_table = Box::new(Table::new());
    t_r287 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r287 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 0;
    }
    let mut new_table = Box::new(Table::new());
    t_r288 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r288 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 0;
    }
    let mut new_table = Box::new(Table::new());
    t_r289 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r289 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = -4;
    }
    b_r271 = true;
    s_r290 = "alphadelta".to_string();
    f_r295 = -0.5700000000000001;
    i_r263 = 0;
    loop {
        b_r272 = i_r263 < 2;
        if b_r272 {
            if true {
                s_r291 = s_r290.clone();
                if false {
                    if false {
                        b_r272 = 16 >= i_r263;
                        if b_r272 {
                            i_r264 = 0;
                            loop {
                                b_r274 = i_r264 < 5;
                                if b_r274 {
                                    let k = 5;
                                    if k < 0 {
                                        panic!("Runtime Error: Negative table index");
                                    }
                                    let idx = k as usize;
                                    let t = unsafe { &mut *t_r289 };
                                    if idx >= t.array.len() {
                                        t.array.resize(idx + 1, 0);
                                    }
                                    unsafe {
                                        *t.array.get_unchecked_mut(idx) = 5;
                                    }
                                    i_r264 = i_r264 + 1;
                                } else {
                                    break;
                                }
                            }
                            b_r272 = b_r271;
                        } else {
                            b_r273 = true == b_r271;
                            b_r274 = !b_r273;
                            if b_r274 {
                                b_r274 = b_r271;
                                s_r292 = s_r290.clone();
                                f_r296 = f_r295;
                            } else {
                                let k = 5;
                                if k < 0 {
                                    panic!("Runtime Error: Negative table index");
                                }
                                let idx = k as usize;
                                let t = unsafe { &*t_r288 };
                                i_r264 = if idx < t.array.len() {
                                    unsafe { *t.array.get_unchecked(idx) }
                                } else {
                                    0
                                };
                                b_r273 = i_r264 < 16;
                                if b_r273 {
                                    s_r293 = format!("{}{}", "echo", s_r290);
                                    s_r292 = s_r293.clone();
                                    f_r296 = -3.416200000000001;
                                    b_r274 = true;
                                } else {
                                    s_r292 = s_r290.clone();
                                    f_r296 = f_r295;
                                    b_r274 = false;
                                }
                            }
                            b_r272 = b_r274;
                            s_r290 = s_r292.clone();
                            f_r295 = f_r296;
                        }
                    } else {
                        b_r272 = b_r271;
                    }
                    b_r273 = "echo" == s_r291;
                    if b_r273 {
                        b_r273 = !b_r272;
                        if b_r273 {
                            b_r271 = b_r272;
                        } else {
                            let k = 0;
                            if k < 0 {
                                panic!("Runtime Error: Negative table index");
                            }
                            let idx = k as usize;
                            let t = unsafe { &*t_r298 };
                            f_r296 = if idx < t.farray.len() {
                                unsafe { *t.farray.get_unchecked(idx) }
                            } else {
                                0.0
                            };
                            b_r273 = b_r272 == b_r272;
                            b_r272 = !b_r273;
                            b_r271 = b_r272;
                        }
                        let k = 4;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &*t_r288 };
                        i_r264 = if idx < t.array.len() {
                            unsafe { *t.array.get_unchecked(idx) }
                        } else {
                            0
                        };
                        let k = 5;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &mut *t_r289 };
                        if idx >= t.array.len() {
                            t.array.resize(idx + 1, 0);
                        }
                        unsafe {
                            *t.array.get_unchecked_mut(idx) = i_r264;
                        }
                        let k = 4;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &*t_r288 };
                        i_r264 = if idx < t.array.len() {
                            unsafe { *t.array.get_unchecked(idx) }
                        } else {
                            0
                        };
                        let k = 2;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &mut *t_r288 };
                        if idx >= t.array.len() {
                            t.array.resize(idx + 1, 0);
                        }
                        unsafe {
                            *t.array.get_unchecked_mut(idx) = i_r264;
                        }
                    } else {
                        b_r271 = b_r272;
                    }
                }
            }
            let k = 5;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &*t_r287 };
            i_r264 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let k = 2;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r287 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = i_r264;
            }
            i_r263 = i_r263 + 1;
        } else {
            break;
        }
    }
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r287 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = -2;
    }
    b_r272 = 3.12 == f_r295;
    if b_r272 {
        b_r272 = s_r290 == s_r290;
        if b_r272 {
            let k = 4;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r298 };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = 13.750000000000002;
            }
            if false {
                let k = 0;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &*t_r289 };
                i_r263 = if idx < t.array.len() {
                    unsafe { *t.array.get_unchecked(idx) }
                } else {
                    0
                };
                let k = 1;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r288 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r263;
                }
                let k = 0;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r289 };
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
                let t = unsafe { &mut *t_r287 };
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
                let t = unsafe { &mut *t_r287 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = 4;
                }
            }
        }
        let mut new_table = Box::new(Table::new_string());
        t_r299 = &mut *new_table as *mut Table;
        tables.push(new_table);
        s_r293 = format!("{}{}", "beta", s_r290);
        let k = 0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r299 };
        if idx >= t.sarray.len() {
            t.sarray.resize(idx + 1, String::new());
        }
        unsafe {
            *t.sarray.get_unchecked_mut(idx) = s_r293.clone();
        }
    }
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r288 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = -18;
    }
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r298 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 3.428;
    }
    i_r264 = 0;
    loop {
        b_r272 = i_r264 < 4;
        if b_r272 {
            let k = 5;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r298 };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = -3.9133333333333336;
            }
            let k = 4;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r287 };
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
            let t = unsafe { &mut *t_r289 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = 19;
            }
            i_r264 = i_r264 + 1;
        } else {
            break;
        }
    }
    println!(
        "{}\t{}\t{:?}\t{}\t{}\t{}\t{}\t{}\t{:?}\t{}\t{}",
        "final",
        16,
        f_r295,
        "betabeta",
        false,
        s_r290,
        -2,
        "betaalphadelta",
        0.19000000000000003,
        11,
        1
    );
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=20;dyn_gets=6;hoists=0;hoist_ctx=;consts_i=76;consts_b=17;consts_f=47;consts_s=15";
