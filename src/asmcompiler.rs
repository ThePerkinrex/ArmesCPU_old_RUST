extern crate armes_cpu_lib;

mod memory;
use armes_cpu_lib::Memory;

fn main() {
    let mut mem = Memory::new(4, 8);
    mem.set(0, 0b00001111); // LDA 15
    mem.set(1, 0b01001110); // ADD 14
    mem.set(2, 0b01110000); // OUT
    mem.set(3, 0b11110000); // HLT

    mem.set(14, 0b00000010); // 2
    mem.set(15, 0b00000110); // 6
    memory::store("memory/ram.mmap", mem);
}