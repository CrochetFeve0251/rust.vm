use std::env;
use crate::file_opener::FileOpener;
use crate::registers_parser::RegistersParser;
use crate::ram_parser::RamParser;

pub mod cpu;
pub mod file_opener;
pub mod registers_parser;
pub mod ram_parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_registers_opener = FileOpener::new(args[1].clone());
    let registers = RegistersParser::parse_register(file_registers_opener.get_content());
    let ram = RamParser::parse_ram(args[2].clone());
    println!("{:?}", registers);
    println!("{:?}", args);
    let initial_ram = [0u8; 0xFFFF];
    let mut cpu = cpu::Cpu::new(registers, initial_ram, args.len() > 3 &&  args[3] == "true");
    println!("{:?}", ram);
    cpu.load_data(&ram);
    cpu.run();
}