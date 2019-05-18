extern crate armes_cpu_lib;
use armes_cpu_lib::Logger;
mod memory;
mod configloader;

fn main() {
    let logger: armes_cpu_lib::DefaultLogger = armes_cpu_lib::DefaultLogger::new();
    macro_rules! log {
        // Note that this is using the `log` function imported above during
        // `bare_bones`
        ($($t:tt)*) => (logger.log(&format_args!($($t)*).to_string()))
    }

    log!("CLI initialized");
    let conf = configloader::load_cfg();
    let rom = memory::load(&conf.clone().rom_filename.unwrap());
    //println!("---------------");
    let ram = memory::load(&conf.clone().ram_filename.unwrap());
    //println!("---------------");
    //println!("{}", &rom);
    //println!("{}", &ram);
    armes_cpu_lib::run_cpu(conf.clone(), ram, rom, logger);
    log!("CLI done");
}
