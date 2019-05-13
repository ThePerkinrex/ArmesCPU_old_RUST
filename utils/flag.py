OVERFLOW = 0
UNDERFLOW = 1
ZERO = 2

class FlagRegister:
    def __init__(self):
        self._value = 0
    
    def setFlag(self, f_number):
        self._value |= 1<<f_number
    
    def unsetFlag(self, f_number):
        if self.getFlag(f_number) != 0:
            self._value ^= 1<<f_number
    
    def getFlag(self, f_number):
        return self._value & 1<<f_number
    
    def reset(self):
        self._value = 0
    
    def __repr__(self):
        return '{:b}'.format(self._value)
    
    def __str__(self):
        return 'F'+repr(self)