use Register;

pub struct Bus {
    reg: Register
}

impl Bus {
    pub fn new(length: usize) -> Bus {
        Bus {
            reg: Register::new(length)
        }
    }

    pub fn set(&mut self, value: usize) {
        self.reg.set(value);
    }

    pub fn get(&self) -> usize {
        self.reg.get()
    }

    pub fn reset(&mut self) {
        self.set(0);
    }
}