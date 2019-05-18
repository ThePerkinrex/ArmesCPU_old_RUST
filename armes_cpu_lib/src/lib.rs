mod config;
pub use config::Config;
mod register;
pub use register::Register;
mod memory;
pub use memory::Memory;
mod output;
pub use output::{DefaultLogger, Logger};

pub mod compile;

pub fn run_cpu<T: Logger>(config: Config, mut ram: Memory, rom: Memory,logger: T){
    macro_rules! log {
        // Note that this is using the `log` function imported above during
        // `bare_bones`
        ($($t:tt)*) => (logger.log(&format_args!($($t)*).to_string()))
    }


    log!("Initializing {}", config.name);
    log!("{}", rom.get(0b000000000));
    log!("{}", rom.get(0b000000100));
}