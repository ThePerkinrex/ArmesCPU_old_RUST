use std::collections::HashMap;

#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub data_length: usize,
    pub ram_addr_length: usize,
    pub microinst_length: usize,

    pub instructions: HashMap<String, usize>,
    pub microinstructions: HashMap<String, usize>,
}