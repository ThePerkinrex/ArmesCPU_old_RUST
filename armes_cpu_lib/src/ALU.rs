pub struct ALU {}

use ConnectedRegister;
use flags;

impl ALU {
    pub fn add(a_reg: &ConnectedRegister, b_reg: &mut ConnectedRegister, f_reg: &mut flags::F_Register) {
        let b = b_reg._get();
        b_reg._set(a_reg._get() + b);
        if (b_reg._get() != a_reg._get() + b){
            f_reg.set(flags::F_OVERFLOW);
        }else{
            f_reg.unset(flags::F_OVERFLOW);
        }
    }

    pub fn sub(a_reg: &ConnectedRegister, b_reg: &mut ConnectedRegister, f_reg: &mut flags::F_Register) {
        let b = b_reg._get();
        b_reg._set(a_reg._get() - b);
        if (b_reg._get() == 0){
            f_reg.set(flags::F_ZERO);
        }else{
            f_reg.unset(flags::F_ZERO);
        }
    }
}