mod config;
pub use config::Config;
mod register;
use register::{Register, ConnectedRegister};
mod memory;
pub use memory::Memory;
mod output;
pub use output::{DefaultLogger, Logger, Output, DefaultOutput};

pub mod compile;

mod bus;
use bus::Bus;

#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn run_cpu<T: Logger>(config: Config, mut ram: Memory, rom: Memory,logger: T){
    macro_rules! log {
        // Note that this is using the `log` function imported above during
        // `bare_bones`
        ($($t:tt)*) => (logger.log(&format_args!($($t)*).to_string()))
    }
    log!("Initializing {}", config.name);

    let mut bus = Bus::new(config.data_length);
    let mut a_reg = ConnectedRegister::new(config.data_length);
    let mut out = DefaultOutput::new(&logger, config.data_length);
    log!("A_{}", a_reg);
    bus.set(0xf);
    a_reg.get_from_bus(&bus);
    log!("A_{}", a_reg);
    bus.reset();
    a_reg.put_on_bus(&mut bus);
    out.in_from_bus(&bus);
    out.show();
}