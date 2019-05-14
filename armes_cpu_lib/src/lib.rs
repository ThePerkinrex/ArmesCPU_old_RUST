extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod config;
pub use config::Config;

pub fn run_cpu(config: Config){
    println!("Initializing {}", config.name)
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet() {
    console_log!("Hello, wasm!\nFormat {} t #{}", "this", 2);
}