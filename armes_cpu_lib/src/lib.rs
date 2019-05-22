mod config;
pub use config::Config;
mod register;
pub use register::{ConnectedRegister, InstRegister, Register};
mod memory;
pub use memory::Memory;
mod output;
pub use output::{DefaultLogger, DefaultOutput, Logger, Output};

pub mod compile;

mod bus;
pub use bus::Bus;

mod alu;
use alu::ALU;

mod flags;

#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn run_cpu<L: Logger, O: Output>(config: Config, mut ram: Memory, rom: Memory, logger: L, out: &mut O) {
    macro_rules! log {
        ($($t:tt)*) => (logger.log(&format_args!($($t)*).to_string()))
    }

    log!("Initializing {}", config.name);

    let mut bus = Bus::new(config.data_length);
    let mut a_reg = ConnectedRegister::new(config.data_length);
    let mut ram_addr_reg = ConnectedRegister::new(config.ram_addr_length);
    let mut b_reg = ConnectedRegister::new(config.data_length);
    let mut flags_reg = flags::F_Register::new();
    let mut inst_reg = InstRegister::new(config.clone());
    let mut inst_counter = ConnectedRegister::new(config.ram_addr_length);
    //let mut out = DefaultOutput::new(&logger, config.data_length);
    log!("Initialized!");
    loop {
        let mut halting = false;
        
        for i in 0..2_usize.pow(config.microinst_length as u32) {
            let inst = inst_reg.get_inst();
            
            let minst = rom.get(
                inst << (config.microinst_length + flags::number)
                    | i << flags::number
                    | flags_reg.get(), /*For now no flags*/
            );
            if minst == 0 {
                continue; // Skip empty microinstruction sets
            }
            log!("{:04b}{:03b}{:02b}: {:019b}", inst, i, flags_reg.get(), minst);
            
            //log!("{:04b} {:03b}: {:016b}",inst, i, minst); 
            // First the outs
            if check_minst(minst, *config.microinstructions.get("RCO").unwrap()) {
                bus.set(ram.get(ram_addr_reg._get()));
                //log!("{:04b} {:03b}: RCO", inst, i);
            }
            if check_minst(minst, *config.microinstructions.get("AO").unwrap()) {
                a_reg.put_on_bus(&mut bus);
            }
            if check_minst(minst, *config.microinstructions.get("BO").unwrap()) {
                b_reg.put_on_bus(&mut bus);
            }
            if check_minst(minst, *config.microinstructions.get("ICO").unwrap()) {
                inst_counter.put_on_bus(&mut bus);
            }
            if check_minst(minst, *config.microinstructions.get("IRO").unwrap()) {
                inst_reg.put_on_bus(&mut bus);
            }
            // Then the rest
            if check_minst(minst, *config.microinstructions.get("RAI").unwrap()) {
                ram_addr_reg.get_from_bus(&bus);
            }
            if check_minst(minst, *config.microinstructions.get("RCI").unwrap()) {
                ram.set(ram_addr_reg._get(), bus.get());
            }
            if check_minst(minst, *config.microinstructions.get("AI").unwrap()) {
                a_reg.get_from_bus(&bus);
                //log!("{:04b} {:03b}: AI", inst, i);
            }
            if check_minst(minst, *config.microinstructions.get("BI").unwrap()) {
                b_reg.get_from_bus(&bus);
                //log!("{:04b} {:03b}: BI", inst, i);
            }
            if check_minst(minst, *config.microinstructions.get("ICI").unwrap()) {
                inst_counter.get_from_bus(&bus);
            }
            if check_minst(minst, *config.microinstructions.get("ICA").unwrap()) {
                let contents = inst_counter._get();
                inst_counter._set(contents + 1);
            }
            if check_minst(minst, *config.microinstructions.get("IRI").unwrap()) {
                inst_reg.get_from_bus(&bus);
            }
            if check_minst(minst, *config.microinstructions.get("OI").unwrap()) {
                out.in_from_bus(&bus);
            }
            if check_minst(minst, *config.microinstructions.get("OSH").unwrap()) {
                out.show();
            }
            if check_minst(minst, *config.microinstructions.get("ADD").unwrap()) {
                ALU::add(&a_reg, &mut b_reg, &mut flags_reg);
            }
            if check_minst(minst, *config.microinstructions.get("SUB").unwrap()) {
                ALU::sub(&a_reg, &mut b_reg, &mut flags_reg);
            }
            if check_minst(minst, *config.microinstructions.get("HLT").unwrap()) {
                halting = true;
            }
            bus.reset();
        }
        if halting {
            break;
        }
    }
    log!("Done!");
    log!("A_{}", a_reg);
    log!("B_{}", b_reg);
    /*let lda_f = rom.get(0b0000_010_00);
    log!("ROM: {:016b}, {}, {}", lda_f, check_minst(lda_f, *config.microinstructions.get("RAI").unwrap()), check_minst(lda_f, *config.microinstructions.get("IRO").unwrap()));
    let lda_s = rom.get(0b0000_011_00);
    log!("ROM: {:016b}, {}, {}", lda_s, check_minst(lda_s, *config.microinstructions.get("AI").unwrap()), check_minst(lda_s, *config.microinstructions.get("RCO").unwrap()));
    */
    /*
    log!("A_{}", a_reg);
    bus.set(0xf);
    a_reg.get_from_bus(&bus);
    log!("A_{}", a_reg);
    bus.reset();
    a_reg.put_on_bus(&mut bus);
    out.in_from_bus(&bus);
    out.show();*/
}

fn check_minst(data: usize, minst: usize) -> bool {
    (data & minst) > 0
}
