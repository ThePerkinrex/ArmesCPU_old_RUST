use std::fs;
use armes_cpu_lib::Memory;

pub fn load(file: &str) -> Memory {
    let data = fs::read(file).expect("Unable to read file");
    let mut addr_l: u32 = 4;
    let mut data_l: u32 = 8;
    let mut addr_b: u32 = 1;
    let mut data_b: u32 = 1;
    let mut status = 0;
    let mut c = String::new();
    let mut n1 = Vec::new();
    let mut n2 = Vec::new();
    let mut i = 0;
    let mut addresses = Vec::new();
    let mut values = Vec::new();
    for chr in data {
        if status == 0 {
            if (chr as char).is_ascii_digit() {
                c += &(chr as char).to_string()
            }else if (chr as char) == ':'{
                status+=1;
                addr_l = c.parse().unwrap();
                c = String::new();
            }
        } else if status == 1 {
            if (chr as char).is_ascii_digit() {
                c += &(chr as char).to_string()
            }else if (chr as char) == '!'{
                status+=1;
                data_l = c.parse().unwrap();
                c = String::new();
            }
        } else if status == 2 {
            if (chr as char).is_ascii_digit() {
                c += &(chr as char).to_string()
            }else if (chr as char) == ':'{
                status+=1;
                addr_b = c.parse().unwrap();
                c = String::new();
            }
        } else if status == 3 {
            if (chr as char).is_ascii_digit() {
                c += &(chr as char).to_string()
            }else if (chr as char) == '!'{
                status+=1;
                data_b = c.parse().unwrap();
                c = String::new();
            }
        } else if status == 4 {
            if i < addr_b{
                n1.push(chr);
                i+=1;
            } else {
                i = 1;
                status = 5;
                n2.push(chr);
            }
        }else if status == 5 {
            if i < data_b{
                n2.push(chr);
                i+=1;
            } else {
                i = 1;
                status = 4;
                addresses.push(n1);
                values.push(n2);
                n1 = Vec::new();
                n2 = Vec::new();
                n1.push(chr);
            }
        }
    }
    addresses.push(n1);
    values.push(n2);
    let mut mem = Memory::new(addr_l, data_l);
    let a: Vec<usize> = addresses.into_iter().map(|x| convert_to_bytes(x)).collect();
    let d: Vec<usize> = values.into_iter().map(|x| convert_to_bytes(x)).collect();
    
    for i in 0..a.len() {
        mem.set(a[i], d[i]);
    }

    println!("{}", mem);
    return mem;
}

fn convert_to_bytes(v: Vec<u8>) -> usize {
    let mut r: usize = 0;
    let mut i: usize = 0;
    
    for b in v.into_iter().rev() {
        r |= (b as usize)<<(8*i);
        i+=1;
    }
    return r;
}