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

    pub fn get(&mut self) -> usize {
        return self.value
    }
}