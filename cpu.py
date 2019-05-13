from utils import *
from microinstconfig import microinst
from config import *

def checkInst(data, inst):
    return (data & inst)>0


def main():
    print('> Initializing '+NAME)
    bus = Bus(BUS_LENGTH)
    print('> Initialized bus')
    ram = loadRAM('memory/RAM.mmap', bus)#Memory(RAM_ADDR_LENGTH, RAM_LENGTH, bus)
    print('> Initialized RAM')
    a_register = ConnectedRegister(A_REG_LENGTH, bus)
    b_register = ConnectedRegister(B_REG_LENGTH, bus)
    inst_register = InstructionRegister(INST_REGISTER_LENGTH, bus)
    inst_counter = ConnectedRegister(RAM_ADDR_LENGTH, bus)
    flag_register = FlagRegister()
    print('> Initialized registers')
    alu = ALU(a_register, b_register, flag_register)
    out = Output(OUTPUT_REG_LENGTH, bus)
    rom = ROM('memory/ROM.mmap')
    print()
    print('> Initialized '+NAME+'!')
    while True:
        halting = False
        instruction = inst_register.get()
        #print()
        for i in range(2**MICROINST_BIT_LENGTH):
            instruction = inst_register.get()
            inst_data = instruction & 2**RAM_ADDR_LENGTH-1
            inst_real = instruction >> RAM_ADDR_LENGTH
            rom._setAddress(inst_real<<MICROINST_BIT_LENGTH|i)
            data = rom._getContents()
            #print('{:x}:{:x}: {:016b}'.format(inst_real, inst_data, data))
            #check conditionals
            execute = True
            if checkInst(data, microinst["FO"]) and flag_register.getFlag(OVERFLOW) != 1:
                execute = False
            if checkInst(data, microinst["FZ"]) and flag_register.getFlag(ZERO) != 1:
                execute = False
            if checkInst(data, microinst["FU"]) and flag_register.getFlag(UNDERFLOW) != 1:
                execute = False
            if execute:
                #check all the outs
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
                    halting = True
                    break
        if halting:
            break

if __name__ == "__main__":
    main()