from utils import Register
OUTPUT_FORMAT = 'd'
# 'x' > Hexadecimal
# 'd' > Decimal
# 'b' > Binary


class Output:
    def __init__(self, register_length, bus):
        self._register = Register(register_length)
        self._bus = bus
    
    def set(self):
        self._register.set(self._bus.get())
    
    def show(self):
        print(('OUT >> {:'+OUTPUT_FORMAT+'}').format(self._register.get()))