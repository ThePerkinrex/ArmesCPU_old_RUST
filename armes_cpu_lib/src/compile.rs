use Memory;
use Config;
use std::iter::FromIterator;

pub fn asm(s: String, c: Config) -> Memory{
    let mut mem = Memory::new(c.ram_addr_length as u32, c.data_length as u32);
    let asm_arr: Vec<&str> = s.split("\n").collect();
    println!("{:?}", asm_arr);
    mem.set(0, 0b00001111); // LDA 15
    mem.set(1, 0b01001110); // ADD 14
    mem.set(2, 0b01110000); // OUT
    mem.set(3, 0b11110000); // HLT

    mem.set(14, 0b00000010); // 2
    mem.set(15, 0b00000110); // 6
    return mem;
}


pub fn rom(s: String, c: Config) -> Memory{
    let mut mem = Memory::new((c.ram_addr_length + c.microinst_length + c.flag_length) as u32, c.microinstructions.len() as u32);
    //let rommap = fs::read_to_string("uncompiled/rom.rommap").expect("Error reading rommap");
    let rommap_arr: Vec<&str> = s.split("\n").collect();
    //println!("{:?}", rommap_arr);
    for line in rommap_arr {
        let up_line = line.to_uppercase();
        let mut addr = String::new();
        let mut cmds = String::new();
        let mut status = 0;
        let mut x_count = 0;
        let mut x_idxs = Vec::new();
        let mut i: usize = 0;
        for chr in up_line.chars() {
            if status == 0 {
                if chr == ':' {
                    status = 1;
                } else {
                    if chr == 'X' {
                        x_count += 1;
                        x_idxs.push(i);
                    }
                    addr.push(chr);
                }
            } else if status == 1 {
                cmds.push(chr);
            }
            i+=1;
        }
        if status == 1 {
            cmds = cmds.trim().to_string();
            let microinst = convert_microinst(cmds.clone(), c.clone());
            //println!("{} | {} | {:020b}", addr, cmds, microinst);
            for i in 0..(2_usize.pow(x_count)) {
                let mut new_addr: Vec<char> = addr.clone().chars().collect();
                for j in 0..x_count {
                    new_addr[x_idxs[j as usize]] = ((i & (1<<j))>>j).to_string().chars().nth(0).unwrap();
                }
                let new_addr_as_number = usize::from_str_radix(&String::from_iter(new_addr.iter()), 2).unwrap();
                //println!("{}: ", new_addr_as_number);
                mem.set(new_addr_as_number, microinst);
            }
        }
    }
    return mem;
}

fn convert_microinst(cmds: String, c: Config) -> usize {
    let mut r: usize = 0;
    for cmd in cmds.split_ascii_whitespace() {
        //println!("{}: {}", cmd, c.microinstructions.get(cmd).unwrap());
        r |= c.microinstructions.get(cmd).unwrap();
    }
    return r;
}