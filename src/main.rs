extern crate armes_cpu_lib;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    armes_cpu_lib::run_cpu(armes_cpu_lib::Config {
        name: "Armes 8bit CPU".to_string(),
        data_length: 8,
        ram_addr_length: 4,
        microinst_length: 3,

        instructions: HashMap::new(),
        microinstructions: HashMap::new(),
    });
}
