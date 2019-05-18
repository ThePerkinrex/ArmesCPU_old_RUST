extern crate armes_cpu_lib;
use armes_cpu_lib::compile;

mod memory;
mod configloader;

use std::fs;

fn main() {
    let c = configloader::load_cfg();
    let rommap = fs::read_to_string("uncompiled/rom.rommap").expect("Error reading rommap");
    let mem = compile::rom(rommap, c);
    memory::store("memory/rom.mmap", mem);
}