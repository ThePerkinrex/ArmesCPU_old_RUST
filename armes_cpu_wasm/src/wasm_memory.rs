use armes_cpu_lib::Memory;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmMemory(u32, u32, Vec<usize>);

pub fn memory_to_wasm(mem: Memory) -> WasmMemory {
    let lengths = mem.get_lengths();
    WasmMemory(lengths.0, lengths.1, mem.values)
}

pub fn wasm_to_memory(mem: WasmMemory) -> Memory {
    let mut m = Memory::new(mem.0, mem.1);
    m.values = mem.2;
    m
}