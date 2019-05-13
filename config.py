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
RAM_FILENAME = 'memory/RAM.mmap'
ROM_FILENAME = 'memory/ROM.mmap'

ramfile = open(RAM_FILENAME)
ramline = ramfile.readline().strip()

if ramline.startswith('!'):
    ramline = ramline[1:]
    ram_structure = ramline.split(':')
    if int(ram_structure[0]) == RAM_ADDR_LENGTH and int(ram_structure[1]) == RAM_LENGTH:
        pass # OK
    else:
        raise Exception('RAM file does not follow length set in config')
else:
    raise Exception('RAM file does not follow format')