use armes_cpu_lib::Memory;
use std::io::Write;
use std::fs;

#[allow(dead_code)]
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
                //println!("1 {:?} -> {}", n1.clone(), convert_to_bytes(n1.clone()));
            }
            //println!("{}", chr as u8);
        }else if status == 5 {
            if i < data_b{
                n2.push(chr);
                i+=1;
            } else {
                i = 1;
                status = 4;
                addresses.push(n1.clone());
                values.push(n2.clone());
                //println!("1 {:?} -> {}", n1.clone(), convert_to_bytes(n1.clone()));
                //println!("2 {:?} -> {}", n2.clone(), convert_to_bytes(n2.clone()));
                n1 = Vec::new();
                n2 = Vec::new();
                n1.push(chr);
            }
            //println!("{}", chr);
        }
    }
    addresses.push(n1);
    values.push(n2);
    //println!("{:?}:{:?}", addresses, values);
    let mut mem = Memory::new(addr_l, data_l);
    let a: Vec<usize> = addresses.into_iter().map(|x| convert_to_bytes(x)).collect();
    let d: Vec<usize> = values.into_iter().map(|x| convert_to_bytes(x)).collect();
    //println!("{:?}:{:?}", a, d);
    for i in 0..a.len() {
        mem.set(a[i], d[i]);
        //println!("{}:{}", a[i], d[i]);
    }
    return mem;
}

#[allow(dead_code)]
fn convert_to_bytes(v: Vec<u8>) -> usize {
    let mut r: usize = 0;
    let mut i: usize = 0;
    
    for b in v.into_iter() {
        r |= (b as usize)<<(8*i);
        i+=1;
    }
    return r;
}

#[allow(dead_code)]
pub fn store(fname: &str, mem: Memory){
    let mut f = fs::File::create(fname).expect("Could not open file");
    let (addr_l, data_l) = mem.get_lengths();
    let (addr_b, data_b) = ((addr_l as f64/8.0).ceil() as usize, (data_l as f64/8.0).ceil() as usize);

    //println!("!{}:{}!{}:{}!", addr_l, data_l, addr_b, data_b);
    f.write_all(format!("!{}:{}!{}:{}!", addr_l, data_l, addr_b, data_b).as_bytes()).expect("Unable to write file");
    let mut n_d = Vec::new();
    for i in 0..2_usize.pow(addr_l){
        for j in 0..addr_b {
            //println!(">{}:{}", i, ((i>>j*8)&255) as u8);
            n_d.push(((i>>j*8)&255) as u8);

            //f.write(&vec![((i>>j*8)&255) as u8]).expect("Unable to write file");
        }
        for j in 0..data_b {
            let d = mem.get(i);
            //println!("-{}:{}", d, ((d>>j*8)&255) as u8);
            n_d.push(((d>>j*8)&255) as u8);
            //f.write(&vec![((d>>j*8)&255) as u8]).expect("Unable to write file");
        }
    }
    f.write_all(&n_d).expect("Unable to write file");
    f.flush().expect("Unable to flush file");
}