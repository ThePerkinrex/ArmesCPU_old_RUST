class Bus:
    def __init__(self, length):
        self._value = 0
        self.length = length
    
    def set(self, value):
        if value < 2**self.length:
            self._value = value
        else:
            self._value = (2**self.length-1)&value
            print('Bus overflow')
    
    def get(self):
        return self._value
    
    def reset(self):
        self._value = 0
    
    def __str__(self):
        return 'B'+repr(self)
    
    def __repr__(self):
        return ('{0:0'+str(self.length)+'b}').format(self._value)
