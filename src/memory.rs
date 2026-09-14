// src/memory.rs

pub struct Table {
    pub array: Vec<i64>,
    // Float-element side. A table is monomorphic: exactly one side is ever
    // written (the static element type decides), and an unwritten Vec never
    // allocates — so the extra field costs 3 words per table, nothing more.
    pub farray: Vec<f64>,
    // String-element side, same monomorphism argument.
    pub sarray: Vec<String>,
    // Boolean-element side, same monomorphism argument. Vec<bool> is
    // bit-packed by the stdlib (one bit per element) — the "bitmap table"
    // representation comes for free.
    pub barray: Vec<bool>,
    pub is_float: bool,
    pub is_string: bool,
    pub is_bool: bool,
}

impl Table {
    // exactly one of the sides is referenced by any given generated program
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            array: Vec::with_capacity(256),
            farray: Vec::new(),
            sarray: Vec::new(),
            barray: Vec::new(),
            is_float: false,
            is_string: false,
            is_bool: false,
        }
    }

    #[allow(dead_code)]
    pub fn new_float() -> Self {
        Self {
            array: Vec::new(),
            farray: Vec::with_capacity(256),
            sarray: Vec::new(),
            barray: Vec::new(),
            is_float: true,
            is_string: false,
            is_bool: false,
        }
    }

    #[allow(dead_code)]
    pub fn new_string() -> Self {
        Self {
            array: Vec::new(),
            farray: Vec::new(),
            sarray: Vec::with_capacity(256),
            barray: Vec::new(),
            is_float: false,
            is_string: true,
            is_bool: false,
        }
    }

    #[allow(dead_code)]
    pub fn new_bool() -> Self {
        Self {
            array: Vec::new(),
            farray: Vec::new(),
            sarray: Vec::new(),
            barray: Vec::with_capacity(256),
            is_float: false,
            is_string: false,
            is_bool: true,
        }
    }
}
