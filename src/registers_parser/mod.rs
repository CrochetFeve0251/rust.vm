
pub struct RegistersParser {
}

impl RegistersParser {

    pub fn parse_register(content: String) -> [u64; 16] {
        let mut output = [0u64; 16];
        let lines: Vec<_> = content.split("\n").collect();
        for index in 0..lines.len() {
            let line = lines[index];
            let parts: Vec<_> = line.split('=').collect();
            if parts[0].len() > 0 {
                let register : usize = parts[0][1..].parse().unwrap();
                output[register] = u64::from_str_radix(parts[1], 16).unwrap();
            }
        }
        output
    }
}