from config import *

class Register:
    def __init__(self, length):
        self._value = 0
        self.length = length
    
    def set(self, value):
        
        if value < 2**self.length:
            self._value = value
            return 0
        else:
            if value < 0:
                self._value = -value
                return -1
            else:
                self._value = (2**self.length-1)&value
                return 1
    
    def get(self):
        return self._value
    
    def add(self):
        if self._value + 1 < 2**self.length:
            self._value += 1

    def __str__(self):
        return 'R'+repr(self)
    
    def __repr__(self):
        return '{0:08b}'.format(self._value)

class ConnectedRegister(Register):
    def __init__(self, length, bus):
        super().__init__(length)
        self._bus = bus
    
    def loadDataFromBus(self):
        self._value = self._bus.get()
    
    def putDataOnBus(self):
        self._bus.set(self._value)

class InstructionRegister(ConnectedRegister):
    def putDataOnBus(self):
        self._bus.set(self._value&(2**RAM_ADDR_LENGTH-1))