use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Config {
    pub name: String,
    pub data_length: usize,
    pub ram_addr_length: usize,
    pub microinst_length: usize,
    pub flag_length: usize,

    pub instructions: HashMap<String, usize>,
    pub microinstructions: HashMap<String, usize>,

    pub ram_filename: Option<String>,
    pub rom_filename: Option<String>
}