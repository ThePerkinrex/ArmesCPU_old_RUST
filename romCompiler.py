from microinstconfig import microinst

romF = open('ROM.rommap')
romC = open('ROM.mmap', 'w')

romLines = romF.readlines()
first=True
for line in romLines:
    try:
        line = line.upper().strip().split(':')
        addr = line[0].strip()
        instructions = line[1].strip().split(' ')
        constructedFlag = 0;
        for ins in instructions:
            constructedFlag |= microinst[ins]
        if first:
            romC.write('!{}:{}\n'.format(len(addr),len(microinst)))
            first = False
        if 'X' in addr:
            for i in range(2**addr.count('X')):
                i_bin = ('{:0'+str(addr.count('X'))+'b}').format(i)
                to_write = ''
                j = 0
                for c in addr:
                    if c == 'X':
                        to_write += i_bin[j]
                        j+=1
                    else:
                        to_write += c
                romC.write(('{}:{:0'+str(len(microinst))+'b}\n').format(to_write, constructedFlag))
        else:
            romC.write(('{}:{:0'+str(len(microinst))+'b}\n').format(addr, constructedFlag))
    except:
        pass

romC.flush()
romC.close()