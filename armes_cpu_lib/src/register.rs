use std::fmt::Display;
use std::fmt;

pub struct Register {
    value: usize,
    length: usize
}

impl Register {
    pub fn new(length: usize) -> Register {
        Register {
            value: 0,
            length: length
        }
    }

    pub fn set(&mut self, value: usize) {
        self.value = value & (2_usize.pow(self.length as u32)-1);
    }

    pub fn get(&self) -> usize {
        return self.value
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "R({})", self.get())
    }
}

use Bus;

pub struct ConnectedRegister {
    reg: Register,
}

impl ConnectedRegister {
    pub fn new(length: usize) -> ConnectedRegister {
        ConnectedRegister {
            reg: Register::new(length)
        }
    }

    pub fn get_from_bus(&mut self, bus: &Bus) {
        self.reg.set(bus.get());
    }

    pub fn put_on_bus(&mut self, bus: &mut Bus) {
        bus.set(self.reg.get())
    }

    pub fn _get(&self) -> usize {
        self.reg.get()
    }
}

impl Display for ConnectedRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CR({})", self.reg.get())
    }
}