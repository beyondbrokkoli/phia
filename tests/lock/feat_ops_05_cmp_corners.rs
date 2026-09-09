// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r72 = 0i64;
    let mut i_r73 = 0i64;
    let mut i_r74 = 0i64;
    let mut i_r75 = 0i64;
    let mut b_r72 = false;
    let mut f_r89 = 0f64;
    let mut f_r90 = 0f64;
    let mut f_r91 = 0f64;
    let mut t_r72: *mut Table = std::ptr::null_mut();
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    i_r72 = 0;
    loop {
        b_r72 = i_r72 < 1;
        if b_r72 {
            i_r72 = i_r72 + 1;
        } else {
            break;
        }
    }
    i_r73 = 9223372036854775806 + i_r72;
    if true {
        i_r72 = 1;
    } else {
        i_r72 = 0;
    }
    b_r72 = 0 <= i_r73;
    if b_r72 {
        i_r74 = i_r72 + 10;
        i_r75 = i_r74;
    } else {
        i_r75 = i_r72;
    }
    b_r72 = i_r73 >= i_r73;
    if b_r72 {
        i_r74 = i_r75 + 100;
        i_r72 = i_r74;
    } else {
        i_r72 = i_r75;
    }
    b_r72 = 9223372036854775806 < i_r73;
    if b_r72 {
        i_r74 = i_r72 + 1000;
        i_r75 = i_r74;
    } else {
        i_r75 = i_r72;
    }
    f_r89 = 9007199254740992.0;
    b_r72 = f_r89 <= f_r89;
    if b_r72 {
        i_r74 = i_r75 + 10000;
        i_r72 = i_r74;
    } else {
        i_r72 = i_r75;
    }
    b_r72 = f_r89 >= f_r89;
    if b_r72 {
        i_r74 = i_r72 + 100000;
        i_r75 = i_r74;
    } else {
        i_r75 = i_r72;
    }
    f_r90 = 9007199254740990.0;
    f_r91 = 9007199254740990.0;
    b_r72 = f_r90 <= f_r91;
    if b_r72 {
        i_r74 = i_r75 + 1000000;
        i_r72 = i_r74;
    } else {
        i_r72 = i_r75;
    }
    f_r91 = 9007199254740990.0;
    b_r72 = f_r91 < f_r89;
    if b_r72 {
        i_r74 = i_r72 + 10000000;
        i_r75 = i_r74;
    } else {
        i_r75 = i_r72;
    }
    let mut new_table = Box::new(Table::new());
    t_r72 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r72 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r75;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=0;fast_gets=0;dyn_sets=1;dyn_gets=0;hoists=0;hoist_ctx=";
