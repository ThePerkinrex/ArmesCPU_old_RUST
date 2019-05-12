class ALU:
    def __init__(self, a_reg, b_reg):
        self._a_reg = a_reg
        self._b_reg = b_reg
    
    def add(self):
        self._b_reg.set(self._a_reg.get()+self._b_reg.get())