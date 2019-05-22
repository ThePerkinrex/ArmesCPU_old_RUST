
use std::fmt;
use std::fmt::Display;

pub const number: usize = 2;
pub const F_OVERFLOW: usize = 0;
pub const F_ZERO: usize     = 1;

pub struct F_Register {
    value: usize,
    length: usize,
}

impl F_Register {
    pub fn new() -> F_Register {
        F_Register {
            value: 0,
            length: number,
        }
    }

    pub fn set(&mut self, idx: usize) {
        self.value |= (1<<idx) & (2_usize.pow(self.length as u32) - 1);
    }

    pub fn unset(&mut self, idx: usize) {
        self.value &= !((1<<idx) & (2_usize.pow(self.length as u32) - 1))
    }

    pub fn get(&self) -> usize {
        return self.value;
    }

    pub fn and(&mut self, value: usize) -> usize {
        self.value & (value & (2_usize.pow(self.length as u32) - 1))
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }
}

impl Display for F_Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FR({})", self.get())
    }
}
