pub trait Logger {
    fn log(&self, s: &String);
}

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
