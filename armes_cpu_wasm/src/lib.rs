extern crate wasm_bindgen;
extern crate console_error_panic_hook;
use std::panic;
use wasm_bindgen::prelude::*;

mod wasm_memory;
mod configloader;

extern crate armes_cpu_lib;
use armes_cpu_lib::{compile, Config};
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

//let mut ram: armes_cpu_lib::Memory;

#[wasm_bindgen]
pub fn compile_asm(s: &str, conf_s: &str) -> wasm_memory::WasmMemory {
    let conf = configloader::load_cfg(conf_s);

    let mem = compile::asm(String::from(s), conf);
    wasm_memory::memory_to_wasm(mem)
}

#[wasm_bindgen]
pub fn print_mem(m: wasm_memory::WasmMemory) {
    console_log!("{}", wasm_memory::wasm_to_memory(m));
}

#[wasm_bindgen]
pub fn init_wasm() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    //console_log!("Hello, wasm!\nFormat {} t #{}", "this", 2);
    /*armes_cpu_lib::run_cpu(armes_cpu_lib::Config {
        name: "Armes 8bit CPU".to_string(),
        data_length: 8,
        ram_addr_length: 4,
        microinst_length: 3,
        flag_length: 2,
        
        ram_filename: None,
        rom_filename: None,

        instructions: HashMap::new(),
        microinstructions: HashMap::new(),
    }, Logger{});*/
}