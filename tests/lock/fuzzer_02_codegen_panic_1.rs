// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r263 = 0i64;
    let mut i_r264 = 0i64;
    let mut b_r263 = false;
    let mut b_r264 = false;
    let mut b_r265 = false;
    let mut b_r266 = false;
    let mut f_r279 = 0f64;
    let mut f_r280 = 0f64;
    let mut s_r263 = String::new();
    let mut s_r264 = String::new();
    let mut s_r265 = String::new();
    let mut s_r266 = String::new();
    let mut t_r263: *mut Table = std::ptr::null_mut();
    let mut t_r264: *mut Table = std::ptr::null_mut();
    let mut t_r265: *mut Table = std::ptr::null_mut();
    let mut t_r282: *mut Table = std::ptr::null_mut();
    let mut t_r283: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_float());
    t_r282 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r282 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 15.28;
    }
    let mut new_table = Box::new(Table::new());
    t_r263 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r263 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 0;
    }
    let mut new_table = Box::new(Table::new());
    t_r264 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r264 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 0;
    }
    let mut new_table = Box::new(Table::new());
    t_r265 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r265 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = -4;
    }
    b_r263 = true;
    s_r263 = "alphadelta".to_string();
    f_r279 = -0.5700000000000001;
    i_r263 = 0;
    loop {
        b_r264 = i_r263 < 2;
        if b_r264 {
            if true {
                s_r264 = s_r263.clone();
                if false {
                    if false {
                        b_r264 = 16 >= i_r263;
                        if b_r264 {
                            i_r264 = 0;
                            loop {
                                b_r266 = i_r264 < 5;
                                if b_r266 {
                                    let k = 5;
                                    if k < 0 {
                                        panic!("Runtime Error: Negative table index");
                                    }
                                    let idx = k as usize;
                                    let t = unsafe { &mut *t_r265 };
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
                            b_r264 = b_r263;
                        } else {
                            b_r265 = true == b_r263;
                            b_r266 = !b_r265;
                            if b_r266 {
                                b_r266 = b_r263;
                                s_r265 = s_r263.clone();
                                f_r280 = f_r279;
                            } else {
                                let k = 5;
                                if k < 0 {
                                    panic!("Runtime Error: Negative table index");
                                }
                                let idx = k as usize;
                                let t = unsafe { &*t_r264 };
                                i_r264 = if idx < t.array.len() {
                                    unsafe { *t.array.get_unchecked(idx) }
                                } else {
                                    0
                                };
                                b_r265 = i_r264 < 16;
                                if b_r265 {
                                    s_r266 = format!("{}{}", "echo", s_r263);
                                    s_r265 = s_r266.clone();
                                    f_r280 = -3.416200000000001;
                                    b_r266 = true;
                                } else {
                                    s_r265 = s_r263.clone();
                                    f_r280 = f_r279;
                                    b_r266 = false;
                                }
                            }
                            b_r264 = b_r266;
                            s_r263 = s_r265.clone();
                            f_r279 = f_r280;
                        }
                    } else {
                        b_r264 = b_r263;
                    }
                    b_r265 = "echo" == s_r264;
                    if b_r265 {
                        b_r265 = !b_r264;
                        if b_r265 {
                            b_r263 = b_r264;
                        } else {
                            let k = 0;
                            if k < 0 {
                                panic!("Runtime Error: Negative table index");
                            }
                            let idx = k as usize;
                            let t = unsafe { &*t_r282 };
                            f_r280 = if idx < t.farray.len() {
                                unsafe { *t.farray.get_unchecked(idx) }
                            } else {
                                0.0
                            };
                            b_r265 = b_r264 == b_r264;
                            b_r264 = !b_r265;
                            b_r263 = b_r264;
                        }
                        let k = 4;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &*t_r264 };
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
                        let t = unsafe { &mut *t_r265 };
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
                        let t = unsafe { &*t_r264 };
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
                        let t = unsafe { &mut *t_r264 };
                        if idx >= t.array.len() {
                            t.array.resize(idx + 1, 0);
                        }
                        unsafe {
                            *t.array.get_unchecked_mut(idx) = i_r264;
                        }
                    } else {
                        b_r263 = b_r264;
                    }
                }
            }
            let k = 5;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &*t_r263 };
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
            let t = unsafe { &mut *t_r263 };
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
    let t = unsafe { &mut *t_r263 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = -2;
    }
    b_r264 = 3.12 == f_r279;
    if b_r264 {
        b_r264 = s_r263 == s_r263;
        if b_r264 {
            let k = 4;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r282 };
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
                let t = unsafe { &*t_r265 };
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
                let t = unsafe { &mut *t_r264 };
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
                let t = unsafe { &mut *t_r265 };
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
                let t = unsafe { &mut *t_r263 };
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
                let t = unsafe { &mut *t_r263 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = 4;
                }
            }
        }
        let mut new_table = Box::new(Table::new_string());
        t_r283 = &mut *new_table as *mut Table;
        tables.push(new_table);
        s_r266 = format!("{}{}", "beta", s_r263);
        let k = 0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r283 };
        if idx >= t.sarray.len() {
            t.sarray.resize(idx + 1, String::new());
        }
        unsafe {
            *t.sarray.get_unchecked_mut(idx) = s_r266.clone();
        }
    }
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r264 };
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
    let t = unsafe { &mut *t_r282 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 3.428;
    }
    i_r264 = 0;
    loop {
        b_r264 = i_r264 < 4;
        if b_r264 {
            let k = 5;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r282 };
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
            let t = unsafe { &mut *t_r263 };
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
            let t = unsafe { &mut *t_r265 };
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
        f_r279,
        "betabeta",
        false,
        s_r263,
        -2,
        "betaalphadelta",
        0.19000000000000003,
        11,
        1
    );
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=20;dyn_gets=6;hoists=0;hoist_ctx=;consts_i=76;consts_b=17;consts_f=47;consts_s=15";
