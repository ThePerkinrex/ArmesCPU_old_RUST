extern crate armes_cpu_lib;
mod memory;
mod configloader;

fn main() {
    println!("Hello, world!");
    let conf = configloader::load_cfg();
    armes_cpu_lib::run_cpu(conf.clone(), armes_cpu_lib::DefaultLogger::new());
    let rom = memory::load(&conf.clone().rom_filename.unwrap());
    let ram = memory::load(&conf.clone().ram_filename.unwrap());
    println!("{}", rom);
    println!("{}", ram);
}
