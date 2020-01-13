pub struct RegistersParser {
    content: String
}

impl RegistersParser {

    pub fn parse_register(content: String) -> [i64; 16] {
        let mut output = [0i64; 16];
        let lines: Vec<_> = content.split("\n").collect();
        for index in 0..lines.len() {
            let line = lines[index];
            let parts: Vec<_> = line.split('=').collect();
            let register = parts[0].chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
            output[register] = parts[1].parse().unwrap();
        }
        output
    }
}