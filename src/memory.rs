// src/memory.rs

#[derive(Clone, Copy, PartialEq)]
pub struct Value(pub u64);

const QNAN: u64 = 0x7FFC_0000_0000_0000;
const TAG_NIL: u64 = 0x0000_0001_0000_0000;
const TAG_INT: u64 = 0x0000_0006_0000_0000;

impl Value {
    #[inline(always)]
    pub fn nil() -> Self {
        Value(QNAN | TAG_NIL)
    }

    // Notice we accept a 32-bit integer here to fit in the 51-bit NaN payload
    #[inline(always)]
    pub fn integer(i: i32) -> Self {
        Value(QNAN | TAG_INT | (i as u32 as u64))
    }
}

pub struct Table {
    pub array: Vec<Value>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            array: Vec::with_capacity(256),
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == (QNAN | TAG_NIL) {
            write!(f, "Nil")
        } else if (self.0 & 0xFFFF_FFFF_0000_0000) == (QNAN | TAG_INT) {
            let val = (self.0 & 0xFFFF_FFFF) as i32;
            write!(f, "Integer({})", val)
        } else {
            write!(f, "Raw({:#x})", self.0)
        }
    }
}
