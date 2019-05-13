from utils.flag import OVERFLOW, UNDERFLOW, ZERO

class ALU:
    def __init__(self, a_reg, b_reg, f_reg):
        self._a_reg = a_reg
        self._b_reg = b_reg
        self._f_reg = f_reg
    
    def add(self):
        self._f_reg.reset()
        res = self._b_reg.set(self._a_reg.get()+self._b_reg.get())
        if res < 0:
            self._f_reg.setFlag(UNDERFLOW)
        elif res > 0:
            self._f_reg.setFlag(OVERFLOW)
        elif self._b_reg.get() == 0:
            self._f_reg.setFlag(ZERO)

    def sub(self):
        self._f_reg.reset()
        res = self._b_reg.set(self._a_reg.get()-self._b_reg.get())
        if res < 0:
            self._f_reg.setFlag(UNDERFLOW)
        elif res > 0:
            self._f_reg.setFlag(OVERFLOW)
        elif self._b_reg.get() == 0:
            self._f_reg.setFlag(ZERO)
