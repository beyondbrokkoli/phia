// src/memory.rs

pub struct Table {
    pub array: Vec<i64>,
    // Float-element side. A table is monomorphic: exactly one side is ever
    // written (the static element type decides), and an unwritten Vec never
    // allocates — so the extra field costs 3 words per table, nothing more.
    pub farray: Vec<f64>,
    pub is_float: bool,
}

impl Table {
    // exactly one of the two is referenced by any given generated program
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            array: Vec::with_capacity(256),
            farray: Vec::new(),
            is_float: false,
        }
    }

    #[allow(dead_code)]
    pub fn new_float() -> Self {
        Self {
            array: Vec::new(),
            farray: Vec::with_capacity(256),
            is_float: true,
        }
    }
}
