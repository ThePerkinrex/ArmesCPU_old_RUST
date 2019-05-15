extern crate armes_cpu_lib;
mod memory;

fn main() {
    println!("RAM");
    let ram = memory::load("memory/ram.mmap");
    println!("{}", ram);
    println!("ROM");
    let rom = memory::load("memory/rom.mmap");
    println!("{}", rom);
}