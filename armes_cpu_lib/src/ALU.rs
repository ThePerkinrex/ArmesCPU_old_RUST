pub struct ALU {}

use {Bus, ConnectedRegister};

impl ALU {
    pub fn add(a_reg: &ConnectedRegister, b_reg: &mut ConnectedRegister) {
        let b = b_reg._get();
        b_reg._set(a_reg._get() + b);
    }

    pub fn sub(a_reg: &ConnectedRegister, b_reg: &mut ConnectedRegister) {
        let b = b_reg._get();
        b_reg._set(a_reg._get() - b);
    }
}