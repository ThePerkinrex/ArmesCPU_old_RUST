extern crate armes_cpu_lib;

mod memory;
mod configloader;
use armes_cpu_lib::Memory;

fn main() {
    let mut mem = Memory::new(7, 16);
    mem.set(1, 0x11ff);
    memory::store("memory/rom.mmap", mem);
    configloader::load_cfg();
}