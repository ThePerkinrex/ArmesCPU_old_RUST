from utils.register import Register
DUMP_STYLE = 'hex'#'hex','dec','bin'


DUMP_STYLE_F = 'b' if DUMP_STYLE == 'bin' else ('x' if DUMP_STYLE == 'hex' else 'd')

class Memory:
    def __init__(self, address_bit_length, content_bit_length, bus):
        self._addr_l = address_bit_length
        self._cont_l = content_bit_length
        self._values = []
        for v in range(2**address_bit_length):
            self._values.append(Register(content_bit_length))
        self._addr_register = Register(address_bit_length)
        self._DUMP_STYLE_L_ADDR = len(('{:'+DUMP_STYLE_F+'}').format(2**address_bit_length-1))
        self._DUMP_STYLE_L_CONT = len(('{:'+DUMP_STYLE_F+'}').format(2**content_bit_length-1))
        self._bus = bus
    #Utility. Not to be used
    def _setAddress(self, address):
        self._addr_register.set(address)
    
    def _setContents(self, contents):
        self._values[self._addr_register.get()].set(contents)
    
    def _getContents(self):
        return self._values[self._addr_register.get()].get()
    
    def loadAddress(self):
        self._addr_register.set(self._bus.get())

    # Get from bus
    def setData(self):
        self._values[self._addr_register.get()].set(self._bus.get())

    def getData(self):
        self._bus.set(self._values[self._addr_register.get()].get())
    
    def __repr__(self):
        # Like a memory dump
        r = ''
        
        addr_l_max = 2**self._addr_l
        for addr in range(addr_l_max):
            r+= '> ' if self._addr_register.get()==addr else '  '
            r+= ('{0:0'+str(self._DUMP_STYLE_L_ADDR)+DUMP_STYLE_F+'}: {1:0'+str(self._DUMP_STYLE_L_CONT)+DUMP_STYLE_F+'}').format(addr, self._values[addr].get())
            if addr < addr_l_max-1:
                r+='\n'
        return r

class ROM(Memory):
    def __init__(self, fileToLoad):
        f = open(fileToLoad)
        for l in f.readlines():
            if l.startswith('!'):
                ls = l[1:].split(':')
                addr_l = int(ls[0])
                cont_l = int(ls[1])
                super().__init__(addr_l, cont_l, None)
            else:
                ls = l.split(':')
                self._values[int(ls[0], 2)].set(int(ls[1], 2))

    def setData(self):
        raise Exception("Read-only memory, cant set values and not connected to bus")
    
    def getData(self):
        raise Exception("Read-only memory, not connected to bus")
    
    def loadAddress(self):
        raise Exception("Read-only memory, not connected to bus")
    
    def _setContents(self, contents):
        raise Exception("Read-only memory, cant set values")

def loadRAM(fileToLoad, bus):
    f = open(fileToLoad)
    mem = None
    for l in f.readlines():
        if l.startswith('!'):
            ls = l[1:].split(':')
            addr_l = int(ls[0])
            cont_l = int(ls[1])
            mem = Memory(addr_l, cont_l, bus)
        else:
            ls = l.split(':')
            mem._values[int(ls[0], 2)].set(int(ls[1], 2))
    return mem