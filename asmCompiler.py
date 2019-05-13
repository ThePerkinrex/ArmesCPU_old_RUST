import sys, time
from instconfig import instconfig
from config import *

f = open(sys.argv[1])
ramf = open('memory/RAM.mmap', 'w')
ramf.write('!{}:{}\n'.format(RAM_ADDR_LENGTH, RAM_LENGTH))
variables = {}
current_idx = 0
lines = f.readlines()
for line in lines:
    if ';' in line:
        line = line[:line.find(';')]
    line = line.strip()
    if line != '':
        line_items = line.split(' ')
        if line.startswith('$'):
            lastFreeSlot = 2**RAM_ADDR_LENGTH-1
            for name, addr in variables.items():
                addr = int(addr, 2)
                if addr <= lastFreeSlot:
                    lastFreeSlot = addr-1
            variables[line_items[0]] = ('{:0'+str(RAM_ADDR_LENGTH)+'b}').format(lastFreeSlot)
            ramf.write(('{:0'+str(RAM_ADDR_LENGTH)+'b}:{:0'+str(RAM_LENGTH)+'b}\n').format(lastFreeSlot, int(line_items[1])))
        else:
            if line.startswith('!'):
                variables[line_items[0]] = ('{:0'+str(RAM_ADDR_LENGTH)+'b}').format(current_idx)
            current_idx += 1
current_idx = 0
for line in lines:
    if ';' in line:
        line = line[:line.find(';')]
    line = line.strip()
    if line != '':
        line_items = line.split(' ')
        if line.startswith('$'):
            pass
        else:
            if line.startswith('!'):
                line_items = line_items[1:]
            inst = instconfig[line_items[0]]
            data = ('{:0'+str(RAM_ADDR_LENGTH)+'b}').format(0)
            if len(line_items)>1:
                if line_items[1].startswith('#'):
                    line_items[1] = line_items[1][1:]
                    data = ('{:0'+str(RAM_ADDR_LENGTH)+'b}').format(int(line_items[1]))
                else:
                    data = variables[line_items[1]]
            ramf.write(('{:0'+str(RAM_ADDR_LENGTH)+'b}:{}{}\n').format(current_idx, inst, data))
            current_idx += 1
ramf.flush()
ramf.close()
print('> {} compiled in {}s!'.format(sys.argv[1], time.process_time()))