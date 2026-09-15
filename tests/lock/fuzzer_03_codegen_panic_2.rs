// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments, clippy::eq_op)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r168 = 0i64;
    let mut i_r169 = 0i64;
    let mut i_r170 = 0i64;
    let mut b_r168 = false;
    let mut f_r173 = 0f64;
    let mut f_r174 = 0f64;
    let mut f_r175 = 0f64;
    let mut t_r168: *mut Table = std::ptr::null_mut();
    let mut t_r169: *mut Table = std::ptr::null_mut();
    let mut t_r170: *mut Table = std::ptr::null_mut();
    let mut t_r177: *mut Table = std::ptr::null_mut();
    let mut t_r178: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r168 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r168 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 38;
    }
    let mut new_table = Box::new(Table::new());
    t_r169 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r169 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 5;
    }
    f_r173 = -0.36;
    i_r168 = 0;
    loop {
        b_r168 = i_r168 < 4;
        if b_r168 {
            if false {
                let k = 1;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r169 };
                if idx >= t.array.len() {
                    t.array.resize(idx + 1, 0);
                }
                unsafe {
                    *t.array.get_unchecked_mut(idx) = 6;
                }
                if true {
                    let k = 5;
                    if k < 0 {
                        panic!("Runtime Error: Negative table index");
                    }
                    let idx = k as usize;
                    let t = unsafe { &mut *t_r168 };
                    if idx >= t.array.len() {
                        t.array.resize(idx + 1, 0);
                    }
                    unsafe {
                        *t.array.get_unchecked_mut(idx) = 2;
                    }
                    i_r169 = i_r168 - 5;
                    let k = 5;
                    if k < 0 {
                        panic!("Runtime Error: Negative table index");
                    }
                    let idx = k as usize;
                    let t = unsafe { &mut *t_r168 };
                    if idx >= t.array.len() {
                        t.array.resize(idx + 1, 0);
                    }
                    unsafe {
                        *t.array.get_unchecked_mut(idx) = i_r169;
                    }
                    let k = 0;
                    if k < 0 {
                        panic!("Runtime Error: Negative table index");
                    }
                    let idx = k as usize;
                    let t = unsafe { &mut *t_r169 };
                    if idx >= t.array.len() {
                        t.array.resize(idx + 1, 0);
                    }
                    unsafe {
                        *t.array.get_unchecked_mut(idx) = -1;
                    }
                }
                if true {
                    b_r168 = f_r173 <= f_r173;
                    if b_r168 {
                        i_r169 = i_r168 - i_r168;
                        if true {
                            if false {}
                        } else {
                            let mut new_table = Box::new(Table::new_string());
                            t_r178 = &mut *new_table as *mut Table;
                            tables.push(new_table);
                            let k = 0;
                            if k < 0 {
                                panic!("Runtime Error: Negative table index");
                            }
                            let idx = k as usize;
                            let t = unsafe { &mut *t_r178 };
                            if idx >= t.sarray.len() {
                                t.sarray.resize(idx + 1, String::new());
                            }
                            unsafe {
                                *t.sarray.get_unchecked_mut(idx) = "delta".to_string();
                            }
                            i_r170 = 4 + i_r169;
                            i_r168 = i_r170;
                        }
                        f_r175 = f_r173 - -8.75;
                    } else {
                        let k = 3;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &mut *t_r169 };
                        if idx >= t.array.len() {
                            t.array.resize(idx + 1, 0);
                        }
                        unsafe {
                            *t.array.get_unchecked_mut(idx) = 5;
                        }
                        f_r175 = f_r173;
                    }
                    b_r168 = f_r175 < 6.63;
                    if b_r168 {
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
                            *t.array.get_unchecked_mut(idx) = 14;
                        }
                        b_r168 = 19 >= i_r168;
                        if b_r168 {
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
                                *t.array.get_unchecked_mut(idx) = -4;
                            }
                            f_r173 = 5.05;
                        } else {
                            f_r173 = f_r175;
                        }
                        let mut new_table = Box::new(Table::new_string());
                        t_r178 = &mut *new_table as *mut Table;
                        tables.push(new_table);
                        let k = 0;
                        if k < 0 {
                            panic!("Runtime Error: Negative table index");
                        }
                        let idx = k as usize;
                        let t = unsafe { &mut *t_r178 };
                        if idx >= t.sarray.len() {
                            t.sarray.resize(idx + 1, String::new());
                        }
                        unsafe {
                            *t.sarray.get_unchecked_mut(idx) = "echo".to_string();
                        }
                    } else {
                        f_r173 = f_r175;
                    }
                }
            } else {
                f_r174 = f_r173 - f_r173;
                let mut new_table = Box::new(Table::new_float());
                t_r177 = &mut *new_table as *mut Table;
                tables.push(new_table);
                f_r175 = f_r174 - f_r174;
                let k = 0;
                if k < 0 {
                    panic!("Runtime Error: Negative table index");
                }
                let idx = k as usize;
                let t = unsafe { &mut *t_r177 };
                if idx >= t.farray.len() {
                    t.farray.resize(idx + 1, 0.0);
                }
                unsafe {
                    *t.farray.get_unchecked_mut(idx) = f_r175;
                }
            }
            i_r168 = i_r168 + 1;
        } else {
            break;
        }
    }
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r169 };
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
    let t = unsafe { &*t_r168 };
    i_r169 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 5;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r168 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r169;
    }
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r169 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 19;
    }
    let mut new_table = Box::new(Table::new_string());
    t_r178 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r178 };
    if idx >= t.sarray.len() {
        t.sarray.resize(idx + 1, String::new());
    }
    unsafe {
        *t.sarray.get_unchecked_mut(idx) = "betaechobetaechobetaecho".to_string();
    }
    println!(
        "{}\t{}\t{:?}\t{}\t{}\t{}\t{}\t{}\t{}\t{:?}\t{}",
        "final", 19, 2.0, "echo", true, false, "betaecho", "betaechobetaecho", 9, 4.0, -2
    );
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=16;dyn_gets=1;hoists=0;hoist_ctx=;consts_i=59;consts_b=8;consts_f=26;consts_s=14";
