pub trait Logger {
    fn log(&self, s: &String);
}

#[derive(Clone, Copy)]
pub struct DefaultLogger {}

impl DefaultLogger {
    pub fn new() -> DefaultLogger {
        DefaultLogger {}
    }
}

impl Logger for DefaultLogger {
    fn log(&self, s: &String) {
        println!("{}",s);
    }
}

use ConnectedRegister;
use Bus;

pub trait Output {
    fn in_from_bus(&mut self, bus: &Bus);
    fn show(&self);
}

pub struct DefaultOutput<'a, T: Logger> {
    logger: &'a T,
    reg: ConnectedRegister
}

impl<'a, T: Logger> DefaultOutput<'a, T> {
    pub fn new(logger: &'a T, length: usize) -> DefaultOutput<'a, T> {
        DefaultOutput {
            logger: logger,
            reg: ConnectedRegister::new(length)
        }
    }
}

impl<'a, T: Logger> Output for DefaultOutput<'a, T> {
    fn in_from_bus(&mut self, bus: &Bus) {
        self.reg.get_from_bus(bus);
    }

    fn show(&self) {
        self.logger.log(&format!("OUT >> {}", self.reg._get()));
    }
}