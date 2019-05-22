extern crate wasm_bindgen;
extern crate console_error_panic_hook;
use std::{panic, thread};
use wasm_bindgen::prelude::*;
use std::ops::Deref;

mod wasm_memory;
use wasm_memory::WasmMemory;
mod configloader;

extern crate armes_cpu_lib;
use armes_cpu_lib::{compile, Config, ConnectedRegister, Bus};
use std::collections::HashMap;



#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn printToCLI(s: &str);

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

struct Output {
    reg: armes_cpu_lib::ConnectedRegister
}

impl Output {
    pub fn new(length: usize) -> Output {
        Output {
            reg: ConnectedRegister::new(length)
        }
    }
}

impl armes_cpu_lib::Output for Output {
    fn in_from_bus(&mut self, bus: &Bus) {
        self.reg.get_from_bus(bus);
    }

    fn show(&self) {
        printToCLI(&format!("OUT >> {}", self.reg._get()));
    }
}



//let mut ram: armes_cpu_lib::Memory;
//#[wasm_bindgen]
//trait wasm_abi: wasm_bindgen::convert::IntoWasmAbi {}

//impl wasm_bindgen::convert::IntoWasmAbi for Config {}

#[wasm_bindgen]
pub fn compile_asm(s: &str, conf_s: &str) -> WasmMemory {
    let conf = configloader::load_cfg(conf_s);

    let mem = compile::asm(String::from(s), conf);
    wasm_memory::memory_to_wasm(mem)
}

#[wasm_bindgen]
pub fn compile_rom(s: &str, conf_s: &str) -> WasmMemory {
    let conf = configloader::load_cfg(conf_s);

    let mem = compile::rom(String::from(s), conf);
    wasm_memory::memory_to_wasm(mem)
}

#[wasm_bindgen]
pub fn load_instructions(cfg: &str) -> String {
    format!("{:?}",configloader::load_cfg(cfg).instructions)
}


#[wasm_bindgen]
pub fn print_mem(m: WasmMemory) {
    console_log!("{}", wasm_memory::wasm_to_memory(m));
}

#[wasm_bindgen]
pub fn init_wasm() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    //console_log!("Hello, wasm!\nFormat {} t #{}", "this", 2);
}

#[wasm_bindgen]
pub fn run_cpu(cfg: &str, ram: WasmMemory, rom: WasmMemory) {
    let conf = configloader::load_cfg(cfg);
    let mut out = Output::new(conf.data_length);
    armes_cpu_lib::run_cpu(conf, wasm_memory::wasm_to_memory(ram), wasm_memory::wasm_to_memory(rom), Logger{}, &mut out);
    //printToCLI("Thread started");
}
