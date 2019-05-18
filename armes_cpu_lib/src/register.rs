use std::fmt;
use std::fmt::Display;

pub struct Register {
    value: usize,
    length: usize,
}

impl Register {
    pub fn new(length: usize) -> Register {
        Register {
            value: 0,
            length: length,
        }
    }

    pub fn set(&mut self, value: usize) {
        self.value = value & (2_usize.pow(self.length as u32) - 1);
    }

    pub fn get(&self) -> usize {
        return self.value;
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
            reg: Register::new(length),
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

    pub fn _set(&mut self, value: usize) {
        self.reg.set(value);
    }
}

impl Display for ConnectedRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CR({})", self.reg.get())
    }
}

use Config;

pub struct InstRegister {
    reg: Register,
    c: Config,
}

impl InstRegister {
    pub fn new(c: Config) -> InstRegister {
        InstRegister {
            reg: Register::new(c.data_length),
            c: c,
        }
    }

    pub fn get_from_bus(&mut self, bus: &Bus) {
        self.reg.set(bus.get());
    }

    pub fn put_on_bus(&mut self, bus: &mut Bus) {
        bus.set(
            self.reg.get()
                & (2_usize.pow((self.c.data_length - self.c.ram_addr_length) as u32) - 1),
        )
    }

    pub fn get_inst(&self) -> usize {
        (self.reg.get()
            & ((2_usize.pow((self.c.data_length - self.c.ram_addr_length) as u32) - 1)
                << self.c.ram_addr_length))
            >> self.c.ram_addr_length
    }

    pub fn _get(&self) -> usize {
        self.reg.get()
    }

    pub fn _set(&mut self, value: usize) {
        self.reg.set(value);
    }
}

impl Display for InstRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IR({})", self.reg.get())
    }
}
