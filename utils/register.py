class Register:
    def __init__(self, length):
        self._value = 0
        self.length = length
    
    def set(self, value):
        if value < 2**self.length:
            self._value = value
        else:
            self._value = (2**self.length-1)&value
            print('Register overflow')
    
    def get(self):
        return self._value
    
    def add(self):
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
        self._bus.set(self._value&15)