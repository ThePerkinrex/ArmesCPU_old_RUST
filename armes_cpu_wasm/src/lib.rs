extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate armes_cpu_lib;
use std::collections::HashMap;

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

struct Logger {}
impl armes_cpu_lib::Logger for Logger {
    fn log(&self, s: &String) {
        log(s);
    }
}

#[wasm_bindgen]
pub fn greet() {
    console_log!("Hello, wasm!\nFormat {} t #{}", "this", 2);
    armes_cpu_lib::run_cpu(armes_cpu_lib::Config {
        name: "Armes 8bit CPU".to_string(),
        data_length: 8,
        ram_addr_length: 4,
        microinst_length: 3,

        instructions: HashMap::new(),
        microinstructions: HashMap::new(),
    }, Logger{});
}