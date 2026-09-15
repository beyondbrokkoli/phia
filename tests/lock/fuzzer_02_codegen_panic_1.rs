// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r0 = 0i64;
    let mut i_r1 = 0i64;
    let mut b_r8 = false;
    let mut b_r9 = false;
    let mut b_r10 = false;
    let mut b_r11 = false;
    let mut t_r24: *mut Table = std::ptr::null_mut();
    let mut t_r25: *mut Table = std::ptr::null_mut();
    let mut t_r26: *mut Table = std::ptr::null_mut();
    let mut s_r27 = String::new();
    let mut s_r28 = String::new();
    let mut s_r29 = String::new();
    let mut s_r30 = String::new();
    let mut f_r32 = 0f64;
    let mut f_r33 = 0f64;
    let mut t_r35: *mut Table = std::ptr::null_mut();
    let mut t_r36: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new_float());
    t_r35 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r35 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 15.28;
    }
    let mut new_table = Box::new(Table::new());
    t_r24 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r24 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 0;
    }
    let mut new_table = Box::new(Table::new());
    t_r25 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r25 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 0;
    }
    let mut new_table = Box::new(Table::new());
    t_r26 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r26 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = -4;
    }
    b_r8 = true;
    s_r27 = "alphadelta".to_string();
    f_r32 = -0.5700000000000001;
    i_r0 = 0;
    loop {
        b_r9 = i_r0 < 2;
        if b_r9 {
            if true {
                s_r28 = s_r27.clone();
                if false {
                    if false {
                        b_r9 = 16 >= i_r0;
                        if b_r9 {
                            i_r1 = 0;
                            loop {
                                b_r11 = i_r1 < 5;
                                if b_r11 {
                                    let k = 5;
                                    if k < 0 {
                                        panic!("Runtime Error: Negative table index");
                                    }
                                    let idx = k as usize;
                                    let t = unsafe { &mut *t_r26 };
                                    if idx >= t.array.len() {
                                        t.array.resize(idx + 1, 0);
                                    }
                                    unsafe {
                                        *t.array.get_unchecked_mut(idx) = 5;
                                    }
                                    i_r1 = i_r1 + 1;
                                } else {
                                    break;
                                }
                            }
                            b_r9 = b_r8;
                        } else {
                            b_r10 = true == b_r8;
                            b_r11 = !b_r10;
                            if b_r11 {
                                b_r11 = b_r8;
                                s_r29 = s_r27.clone();
                                f_r33 = f_r32;
                            } else {
                                let k = 5;
                                if k < 0 {
                                    panic!("Runtime Error: Negative table index");
                                }
                                let idx = k as usize;
                                let t = unsafe { &*t_r25 };
                                i_r1 = if idx < t.array.len() {
                                    unsafe { *t.array.get_unchecked(idx) }
                                } else {
                                    0
                                };
                                b_r10 = i_r1 < 16;
                                if b_r10 {
                                    s_r30 = format!("{}{}", "echo", s_r27);
                                    s_r29 = s_r30.clone();
                                    f_r33 = -3.416200000000001;
                                    b_r11 = true;
                                } else {
                                    s_r29 = s_r27.clone();
                                    f_r33 = f_r32;
                                    b_r11 = false;
                                }
                            }
                            b_r9 = b_r11;
                            s_r27 = s_r29.clone();
                            f_r32 = f_r33;
                        }
                    } else {
                        b_r9 = b_r8;
                    }
                    b_r10 = "echo" == s_r28;
                    if b_r10 {
                        b_r10 = !b_r9;
                        if b_r10 {
                            b_r8 = b_r9;
                        } else {
                            let k = 0;
                            if k < 0 {
                                panic!("Runtime Error: Negative table index");
                            }
                            let idx = k as usize;
                            let t = unsafe { &*t_r35 };
                            f_r33 = if idx < t.farray.len() {
                                unsafe { *t.farray.get_unchecked(idx) }
                            } else {
                                0.0
                            };
                            b_r10 = b_r9 == b_r9;
                            b_r9 = !b_r10;
                            b_r8 = b_r9;
                        }
                        let k = 4;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &*t_r25 };
                        i_r1 = if idx < t.array.len() {
                            unsafe { *t.array.get_unchecked(idx) }
                        } else {
                            0
                        };
                        let k = 5;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &mut *t_r26 };
                        if idx >= t.array.len() {
                            t.array.resize(idx + 1, 0);
                        }
                        unsafe {
                            *t.array.get_unchecked_mut(idx) = i_r1;
                        }
                        let k = 4;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &*t_r25 };
                        i_r1 = if idx < t.array.len() {
                            unsafe { *t.array.get_unchecked(idx) }
                        } else {
                            0
                        };
                        let k = 2;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &mut *t_r25 };
                        if idx >= t.array.len() {
                            t.array.resize(idx + 1, 0);
                        }
                        unsafe {
                            *t.array.get_unchecked_mut(idx) = i_r1;
                        }
                    } else {
                        b_r8 = b_r9;
                    }
                }
            }
            let k = 5;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &*t_r24 };
            i_r1 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let k = 2;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r24 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = i_r1;
            }
            i_r0 = i_r0 + 1;
        } else {
            break;
        }
    }
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r24 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = -2;
    }
    b_r9 = 3.12 == f_r32;
    if b_r9 {
        b_r9 = s_r27 == s_r27;
        if b_r9 {
            let k = 4;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r35 };
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
                let t = unsafe { &*t_r26 };
                i_r0 = if idx < t.array.len() {
                    unsafe { *t.array.get_unchecked(idx) }
                } else {
                    0
                };
                let k = 1;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r25 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = i_r0;
                }
                let k = 0;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r26 };
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
                let t = unsafe { &mut *t_r24 };
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
                let t = unsafe { &mut *t_r24 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = 4;
                }
            }
        }
        let mut new_table = Box::new(Table::new_string());
        t_r36 = &mut *new_table as *mut Table;
        tables.push(new_table);
        s_r30 = format!("{}{}", "beta", s_r27);
        let k = 0;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r36 };
        if idx >= t.sarray.len() {
            t.sarray.resize(idx + 1, String::new());
        }
        unsafe {
            *t.sarray.get_unchecked_mut(idx) = s_r30.clone();
        }
    }
    let k = 4;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r25 };
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
    let t = unsafe { &mut *t_r35 };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = 3.428;
    }
    i_r1 = 0;
    loop {
        b_r9 = i_r1 < 4;
        if b_r9 {
            let k = 5;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r35 };
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
            let t = unsafe { &mut *t_r24 };
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
            let t = unsafe { &mut *t_r26 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = 19;
            }
            i_r1 = i_r1 + 1;
        } else {
            break;
        }
    }
    println!(
        "{}\t{}\t{:?}\t{}\t{}\t{}\t{}\t{}\t{:?}\t{}\t{}",
        "final",
        16,
        f_r32,
        "betabeta",
        false,
        s_r27,
        -2,
        "betaalphadelta",
        0.19000000000000003,
        11,
        1
    );
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=20;dyn_gets=6;hoists=0;hoist_ctx=;consts_i=76;consts_b=17;consts_f=47;consts_s=15";
