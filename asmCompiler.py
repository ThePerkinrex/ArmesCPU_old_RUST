INST_BIT_LENGTH = 8
ADDR_BIT_LENGTH = 4

import sys, time
from instconfig import instconfig
f = open(sys.argv[1])
ramf = open('RAM.mmap', 'w')
ramf.write('!{}:{}\n'.format(ADDR_BIT_LENGTH, INST_BIT_LENGTH))
variables = {}
current_idx = 0
for line in f.readlines():
    line = line.strip()
    if ';' in line:
        line = line[:line.find(';')]
    line_items = line.split(' ')
    if line.startswith('$'):
        lastFreeSlot = 2**ADDR_BIT_LENGTH-1
        for name, addr in variables.items():
            addr = int(addr, 2)
            if addr <= lastFreeSlot:
                lastFreeSlot = addr-1
        variables[line_items[0]] = ('{:0'+str(ADDR_BIT_LENGTH)+'b}').format(lastFreeSlot)
        ramf.write(('{:0'+str(ADDR_BIT_LENGTH)+'b}:{:0'+str(INST_BIT_LENGTH)+'b}\n').format(lastFreeSlot, int(line_items[1])))
    else:
        if line.startswith('!'):
            variables[line_items[0]] = ('{:0'+str(ADDR_BIT_LENGTH)+'b}').format(current_idx)
            line_items = line_items[1:]
        inst = instconfig[line_items[0]]
        data = ('{:0'+str(ADDR_BIT_LENGTH)+'b}').format(0)
        if len(line_items)>1:
            if line_items[1].startswith('#'):
                line_items[1] = line_items[1][1:]
                data = ('{:0'+str(ADDR_BIT_LENGTH)+'b}').format(int(line_items[1]))
            else:
                data = variables[line_items[1]]
        ramf.write(('{:0'+str(ADDR_BIT_LENGTH)+'b}:{}{}\n').format(current_idx, inst, data))
        current_idx += 1
ramf.flush()
ramf.close()
print('> {} compiled in {}s!'.format(sys.argv[1], time.process_time()))