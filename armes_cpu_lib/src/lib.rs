mod config;
pub use config::Config;
mod register;
pub use register::Register;
mod memory;
pub use memory::Memory;
mod output;
pub use output::{DefaultLogger, Logger};

pub fn run_cpu<T: Logger>(config: Config, logger: T){
    macro_rules! log {
        // Note that this is using the `log` function imported above during
        // `bare_bones`
        ($($t:tt)*) => (logger.log(&format_args!($($t)*).to_string()))
    }


    log!("Initializing {}", config.name);
    let mut ram = Memory::new(4, 8);
    ram.set(0xf, 0xf);
    log!("{:08b}", ram.get(0xf));
}