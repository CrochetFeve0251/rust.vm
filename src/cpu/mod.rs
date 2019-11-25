pub mod stack;
pub mod instruction;

pub struct Cpu {
    registers: [i64; 16],
    ram: [i8; 0xFFFF],
    pc: usize,
    ip: i64,
    cir: i32,
    size: usize,
    flags: [bool; 64],
    running: bool,
}

impl Cpu{
    pub fn new(registers: [i64; 16], ram: [i8; 0xFFFF]) -> Cpu {
        Cpu {
            registers,
            ram,
            flags: [false; 64],
            pc: 0,
            ip: 0,
            size: 0,
            cir: 0,
            running: false,
        }
    }

    pub fn load_data(&mut self, data: &Vec<i8>) {
        if data.len() <= 0xFFFF {
            let data_tmp = data.to_vec();
            for index in 0..data_tmp.len() {
                self.ram[index] = data_tmp[index]
            }
        }
    }

    pub fn run(&mut self) {
        self.running = true;
        loop {
            let instruction = self.fetch();
            self.decode();
            self.execute();
            self.memory_access();
            self.write()
        }
    }

    fn fetch(&mut self) {
        let mar = &self.pc;
        let mdr : &mut i32 = &mut 0i32;
        for index in 0..4 {
             mdr += &mut self.ram[mar + index] << index * 8;
        }
        &mut self.pc += 4;
        &mut self.cir = mdr;
    }

    fn decode() {

    }

    fn execute() {

    }

    fn memory_access() {

    }

    fn write() {

    }

    pub fn get_registers(&mut self) -> &[i64; 16] {
        &self.registers
    }

    pub fn get_ram(&mut self) -> &[i8; 0xFFFF] {
        &self.ram
    }

    pub fn get_ip(&mut self) -> &i64 {
        &mut self.ip
    }

    pub fn get_pc(&mut self) -> &usize {
        &mut self.pc
    }

    pub fn get_size(&mut self) -> &usize {
        &mut self.size
    }

    pub fn get_flags(&mut self) -> &[bool; 64] {
        &mut self.flags
    }
}