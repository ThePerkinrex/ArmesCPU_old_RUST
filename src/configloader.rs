extern crate toml;
use std::collections::HashMap;
use std::iter::FromIterator;
use armes_cpu_lib::Config;
use std::fs;

pub fn load_cfg() -> Config {
    let t: toml::Value = toml::from_str(&String::from_utf8(fs::read("config.toml").expect("Error reading config")).expect("Couldn't load as utf8")).expect("Error parsing TOML");
    let inst = t.get("instructions").expect("COnfig format WrOnG");
    let microinst = t.get("microinstructions").expect("COnfig format WrOnG");

    let inst_hashmap: HashMap<String, usize> = HashMap::from_iter(
        inst.as_table().expect("Table")
        .into_iter().map(|(x, y)|{
            (x.clone(), y.as_integer().unwrap() as usize)
        }));
    let microinst_hashmap: HashMap<String, usize> = HashMap::from_iter(
        microinst.as_table().expect("Table")
        .into_iter().map(|(x, y)|{
            (x.clone(), 1<<y.as_integer().unwrap() as usize)
        }));

    Config {
        name: String::from(t.get("name").expect("COnfig format WrOnG").as_str().unwrap()),
        data_length: t.get("data_length").expect("COnfig format WrOnG").as_integer().unwrap() as usize,
        ram_addr_length: t.get("RAM_addr_length").expect("COnfig format WrOnG").as_integer().unwrap() as usize,
        microinst_length: t.get("microinst_length").expect("COnfig format WrOnG").as_integer().unwrap() as usize,
        flag_length: t.get("flag_length").expect("COnfig format WrOnG").as_integer().unwrap() as usize,

        microinstructions: microinst_hashmap,
        instructions: inst_hashmap,

        ram_filename: Some(String::from(t.get("RAM_filename").expect("COnfig format WrOnG").as_str().unwrap())),
        rom_filename: Some(String::from(t.get("ROM_filename").expect("COnfig format WrOnG").as_str().unwrap())),
    }
}