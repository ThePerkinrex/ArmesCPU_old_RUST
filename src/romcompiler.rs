extern crate armes_cpu_lib;
extern crate regex;
use armes_cpu_lib::compile;
use regex::Regex;

mod configloader;
mod memory;

use std::fs;

fn bin(n: usize, z: usize) -> String {
    let mut s = format!("{:b}", n);
    
    while s.len() < z {
        s = String::from("0") + &s;
    }
    s
}

fn main() {
    let c = configloader::load_cfg();
    let mut rommap = fs::read_to_string("uncompiled/rom.rommap").expect("Error reading rommap");
    // Replace the references to instructions with the instruction
    let re = Regex::new(r"\$\{(\w+)\}").unwrap();
    let rm_clone = rommap.clone();
    let captures = re.captures_iter(&rm_clone);
    let mut idx_change: isize = 0;
    for caps in captures {
        let cap = caps.get(1).unwrap();
        let big_cap = caps.get(0).unwrap();
        let to_replace = &bin(
            *c.instructions
                .get(cap.as_str())
                .expect(&format!("{} is not a instruction", cap.as_str())),
            c.ram_addr_length,
        );
        rommap.replace_range(
            (idx_change + big_cap.start() as isize) as usize
                ..(idx_change + big_cap.end() as isize) as usize,
            to_replace,
        );
        idx_change +=
            to_replace.len() as isize - (big_cap.end() as isize - big_cap.start() as isize);
    }
    //println!("{}", rommap);
    // Use the library compiler to compile the corrected string
    let mem = compile::rom(rommap, c);
    memory::store("memory/rom.mmap", mem);
}
