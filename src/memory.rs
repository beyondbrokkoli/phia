// src/memory.rs

pub struct Table {
    pub array: Vec<i64>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            array: Vec::with_capacity(256),
        }
    }
}
