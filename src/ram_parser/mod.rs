use std::fs::File;
use std::io::Read;
use std::fs;

pub struct RamParser {}

impl RamParser {
    pub fn parse_ram(filename: String) -> Vec<u8> {
        RamParser::get_file_as_byte_vec(&filename)
    }

    fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        buffer
    }
}