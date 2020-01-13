use std::env;
use crate::file_opener::FileOpener;
use crate::registers_parser::RegistersParser;

pub mod cpu;
pub mod file_opener;
pub mod registers_parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_opener = FileOpener::new(args[1].clone());
    let mut file_opener = FileOpener::new(args[2].clone());
    let registers = RegistersParser::parse_register(file_opener.get_content());
    println!("{:?}", registers);
    println!("{:?}", args);
}