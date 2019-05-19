use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::FromStr;
use Config;
use Memory;

pub fn asm(s: String, c: Config) -> Memory {
    let mut mem = Memory::new(c.ram_addr_length as u32, c.data_length as u32);
    let mut last_mem = 2_usize.pow(c.ram_addr_length as u32) - 1;
    let mut variables: HashMap<&str, usize> = HashMap::new();
    let mut i = 0;
    let mut n_lines = Vec::new();
    // Find constants & labels
    for line in s.lines() {
        let idx = line.find(";").unwrap_or(line.len());
        let mut line_n = line.get(0..idx).unwrap().trim();
        if line_n != "" {
            if line_n.starts_with("$") {
                let line_s: Vec<&str> = line_n.split(" ").collect();
                //println!("{:?}", line_s);
                mem.set(
                    last_mem,
                    usize::from_str(line_s[1]).expect("Expected a number in constant definition"),
                );
                variables.insert(line_s[0], last_mem);
                last_mem -= 1;
                //print!("   ");
            } else {
                if line_n.starts_with("!") {
                    let line_s: Vec<&str> = line_n.split(" ").collect();
                    variables.insert(line_s[0], i);
                    let line_n_string = line_s.get(1..line_s.len()).unwrap().join(" ");
                    
                    n_lines.push(line_n_string);
                }else{
                    n_lines.push(line_n.to_string());
                }
                
                //print!(" {} ", i);
                i += 1;
            }
            //println!("'{}'", line_n);
        }
    }
    // Parse instructions
    i = 0;
    for line in &n_lines {
        let line_s: Vec<&str> = line.split(" ").collect();
        let inst = c.instructions.get(line_s[0]).expect(&format!("{} isnt a instruction", line_s[0]));
        let mut data = 0;
        if let Some(d) = line_s.get(1) {
            if d.starts_with("#") {
                data = usize::from_str(d.get(1..d.len()).unwrap()).unwrap();
            }else{
                data = *variables.get(d).expect("Variable not found");
            }
        }
        //println!("{}: {} {}", i, inst, data);
        mem.set(i, (inst << c.ram_addr_length) | (data & (2_usize.pow(c.ram_addr_length as u32)-1)));
        i += 1;
    }
    //println!("{:?}", variables);
    //println!("{}", n_lines.join("\n"));
    //println!("{}", mem);
    //////////////////////////////////////
    /*mem.set(0, 0b0000_1111); // LDA 15
    mem.set(1, 0b0100_1110); // ADD 14
    mem.set(2, 0b0110_0000); // OUTB
    mem.set(3, 0b0011_1110); // STB 14
    mem.set(4, 0b1000_0001); // JMP 1

    mem.set(14, 0b00000001); // 2
    mem.set(15, 0b00000001); // 6*/
    return mem;
}

pub fn rom(s: String, c: Config) -> Memory {
    let mut mem = Memory::new(
        (c.ram_addr_length + c.microinst_length + c.flag_length) as u32,
        c.microinstructions.len() as u32,
    );
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
            i += 1;
        }
        if status == 1 {
            cmds = cmds.trim().to_string();
            let microinst = convert_microinst(cmds.clone(), c.clone());
            //println!("{} | {} | {:020b}", addr, cmds, microinst);
            for i in 0..(2_usize.pow(x_count)) {
                let mut new_addr: Vec<char> = addr.clone().chars().collect();
                for j in 0..x_count {
                    new_addr[x_idxs[j as usize]] =
                        ((i & (1 << j)) >> j).to_string().chars().nth(0).unwrap();
                }
                let new_addr_as_number =
                    usize::from_str_radix(&String::from_iter(new_addr.iter()), 2).unwrap();
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

        r |= c
            .microinstructions
            .get(cmd)
            .expect(&format!("{} is not a valid command", cmd));
    }
    return r;
}
