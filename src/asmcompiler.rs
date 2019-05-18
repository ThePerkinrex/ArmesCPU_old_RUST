extern crate armes_cpu_lib;

mod memory;
use armes_cpu_lib::compile;
mod configloader;
use std::fs;

fn main() {
    let c = configloader::load_cfg();
    let asm = fs::read_to_string("uncompiled/asm.armes.asm").expect("Error reading asm");
    let mem = compile::asm(asm, c);
    memory::store("memory/ram.mmap", mem);
}