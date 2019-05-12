from utils import *
from microinstconfig import microinst

# ############ CONFIG ############ #
NAME = "ARMES 8 bit CPU"
BIT_LENGTH = 8
BUS_LENGTH = BIT_LENGTH
RAM_LENGTH = BIT_LENGTH
RAM_ADDR_LENGTH = 4
A_REG_LENGTH = BIT_LENGTH
B_REG_LENGTH = BIT_LENGTH
OUTPUT_REG_LENGTH = BIT_LENGTH

INST_REGISTER_LENGTH = RAM_LENGTH
MICROINST_BIT_LENGTH = 3
# ############ CONFIG ############ #

def checkInst(data, inst):
    return (data & inst)>0


def main():
    print('> Initializing '+NAME)
    bus = Bus(BUS_LENGTH)
    print('> Initialized bus')
    ram = loadRAM('RAM.mmap', bus)#Memory(RAM_ADDR_LENGTH, RAM_LENGTH, bus)
    print('> Initialized RAM')
    a_register = ConnectedRegister(A_REG_LENGTH, bus)
    b_register = ConnectedRegister(B_REG_LENGTH, bus)
    inst_register = InstructionRegister(INST_REGISTER_LENGTH, bus)
    inst_counter = ConnectedRegister(RAM_ADDR_LENGTH, bus)
    print('> Initialized registers')
    alu = ALU(a_register, b_register)
    out = Output(OUTPUT_REG_LENGTH, bus)
    rom = ROM('ROM.mmap')
    print()
    print('> Initialized '+NAME+'!')
    while True:
        halting = False
        instruction = inst_register.get()
        #print()
        for i in range(2**MICROINST_BIT_LENGTH):
            instruction = inst_register.get()
            inst_data = instruction & 15
            inst_real = instruction >> 4
            rom._setAddress(inst_real<<MICROINST_BIT_LENGTH|i)
            data = rom._getContents()
            #print('{:x}:{:x}: {:016b}'.format(inst_real, inst_data, data))
            #check all the outs first
            if checkInst(data, microinst["RCO"]):
                #print('RCO')
                ram.getData()
            if checkInst(data, microinst["AO"]):
                a_register.putDataOnBus()
            if checkInst(data, microinst["BO"]):
                b_register.putDataOnBus()
            if checkInst(data, microinst["ICO"]):
                inst_counter.putDataOnBus()
            if checkInst(data, microinst["IRO"]):
                inst_register.putDataOnBus()
                #print('IRO: {:04b}'.format(inst_register.get() & 15))
            # then check the rest
            if checkInst(data, microinst["RAI"]):
                ram.loadAddress()
            if checkInst(data, microinst["RCI"]):
                ram.setData()
            if checkInst(data, microinst["AI"]):
                a_register.loadDataFromBus()
                #print('A register loaded {}'.format(a_register.get()))
            if checkInst(data, microinst["BI"]):
                b_register.loadDataFromBus()
            if checkInst(data, microinst["ICA"]):
                inst_counter.add()
            if checkInst(data, microinst["ICI"]):
                inst_counter.loadDataFromBus()
            if checkInst(data, microinst["IRI"]):
                #print('IR loaded data from bus')
                inst_register.loadDataFromBus()
                #print('{:08b}'.format(inst_register.get()))
            if checkInst(data, microinst["OI"]):
                out.set()
            if checkInst(data, microinst["OSH"]):
                out.show()
            if checkInst(data, microinst["ADD"]):
                alu.add()
            if checkInst(data, microinst["HLT"]):
                #print('Halting')
                halting = True
                break
            #print(bus, inst_counter)
        #print(repr(ram))
        if halting:
            break
    #print(a_register, b_register)
    #ram._setAddress(0)
    #ram._setContents(0b00000001)
    #ram._setAddress(1)
    #ram._setContents(0b00000001)

    #print(repr(ram))
    #ram._setAddress(0)
    #ram.getData()
    #a_register.loadDataFromBus()

    #ram._setAddress(1)
    #ram.getData()
    #b_register.loadDataFromBus()
    #print(a_register, b_register)

    #alu.add()

    #print(a_register, b_register)
    



if __name__ == "__main__":
    main()