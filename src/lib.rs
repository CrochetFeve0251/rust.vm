mod cpu;
use cpu::Cpu;
use cpu::stack::Stack;
use cpu::instruction::Instruction;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cannot_start_without_file() {
    }

    #[test]
    fn cannot_start_without_initial_state() {

    }

    #[test]
    fn test_cpu_initialize_register_with_initial_state() {
        let initial_state = [0i64; 16];
        let initial_ram = [0u8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram);
        assert_eq!(cpu.get_registers().to_vec(), initial_state.to_vec());
    }

    #[test]
    fn cpu_initialize_flags_all_false() {
        let initial_state = [0i64; 16];
        let initial_ram = [0u8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram);
        assert_eq!(cpu.get_flags().to_vec(), [false; 64].to_vec());
    }

    #[test]
    fn cpu_initialize_ip_equals_zero() {
        let initial_state = [0i64; 16];
        let initial_ram = [0u8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram);
        assert_eq!(*cpu.get_ip(), 0i64);
    }

    #[test]
    fn cpu_initialize_pc_equals_zero() {
        let initial_state = [0i64; 16];
        let initial_ram = [0u8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram);
        assert_eq!(*cpu.get_pc(), 0usize);
    }

    #[test]
    fn cpu_initialize_stack_empty() {

    }

    #[test]
    fn cpu_initialize_ram_empty() {
        let initial_state = [0i64; 16];
        let initial_ram = [0u8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram);
        assert_eq!(cpu.get_ram().to_vec(), [0; 0xFFFF].to_vec());
    }

    #[test]
    fn stack_initialize_index_at_zero() {
        let stack = Stack::new([0; 0xFFFF]);
    }

    #[test]
    fn stack_load_data_should_change_ram() {
        let initial_state = [0i64; 16];
        let initial_ram = [0u8; 0xFFFF];
        let data = [1u8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram);
        cpu.load_data(&data.to_vec());
        assert_eq!(cpu.get_ram().to_vec(), data.to_vec());
    }

    #[test]
    fn stack_load_data_too_big_should_change_ram() {
        let initial_state = [0i64; 16];
        let initial_ram = [0u8; 0xFFFF];
        let data = [1u8; 0xFFFFF].to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram);
        cpu.load_data(&data);
        assert_eq!(cpu.get_ram().to_vec(), initial_ram.to_vec());
    }


    #[test]
    fn test_random_instruction_convert(){
        let initial = 0x81122300;
        let mut result = Instruction::new(initial);
        assert!(*result.get_bcc() == cpu::instruction::branch_condition_code::BranchConditionCode::B);
        assert!(*result.get_iv_flag());
        assert!(*result.get_opcode() == cpu::instruction::opcode::Opcode::OR);
        assert_eq!(*result.get_op1(), 2);
        assert_eq!(*result.get_op2(), 2);
        assert_eq!(*result.get_dest(), 3);
        assert_eq!(*result.get_iv_value(), 0);
    }

    #[test]
        fn test_random_instruction_2_convert(){
            let initial = 0xe0a4537D;
            let mut result = Instruction::new(initial);
            assert!(*result.get_bcc() == cpu::instruction::branch_condition_code::BranchConditionCode::BG);
            assert!(! *result.get_iv_flag());
            assert!(*result.get_opcode() == cpu::instruction::opcode::Opcode::RSH);
            assert_eq!(*result.get_op1(), 4);
            assert_eq!(*result.get_op2(), 5);
            assert_eq!(*result.get_dest(), 3);
            assert_eq!(*result.get_iv_value(), 125);
        }

    #[test]
    fn test_invalid_instruction_convert_should_fail(){
        let initial = 0xe0a4537D;
        let mut result = Instruction::new(initial);
        assert!(*result.get_bcc() == cpu::instruction::branch_condition_code::BranchConditionCode::BG);
        assert!(! *result.get_iv_flag());
        assert!(*result.get_opcode() == cpu::instruction::opcode::Opcode::RSH);
        assert_eq!(*result.get_op1(), 4);
        assert_eq!(*result.get_op2(), 5);
        assert_eq!(*result.get_dest(), 3);
        assert_eq!(*result.get_iv_value(), 125);
    }

    #[test]
    fn test_random_addition_cpu_should_succeed() {
        let initial_state = [
            0i64,
            1i64,
            0i64,
            3i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
        ];
        let initial_ram = [0u8; 0xFFFF];
        //addition registre 1 et 3 dans le registre 1
        let initial_data: [u8; 4] = [
            0x80,
            0x31,
            0x31,
            0x00
        ];
        let data = initial_data.to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram);
        cpu.load_data(&data);
        cpu.run();
        assert_eq!(cpu.get_registers()[1], 3 + 1);
    }

    #[test]
    fn test_random_addition_direct_value_cpu_should_succeed() {
        let initial_state = [
            0i64,
            1i64,
            0i64,
            45i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
        ];
        let initial_ram = [0u8; 0xFFFF];
        //addition registre 1 et valeur 3 dans le registre 1
        let initial_data: [u8; 4] = [
            0x81,
            0x31,
            0x31,
            0x03
        ];
        let data = initial_data.to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram);
        cpu.load_data(&data);
        cpu.run();
        assert_eq!(cpu.get_registers()[1], 3 + 1);
    }

    #[test]
    fn test_overflow_addition_instruction_cpu_should_succeed() {
        let initial_state = [
            0i64,
            std::i64::MAX,
            0i64,
            std::i64::MAX,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
        ];
        let initial_ram = [0u8; 0xFFFF];
        let initial_data: [u8; 4] = [
            0x80,
            0x31,
            0x31,
            0x00
        ];
        let data = initial_data.to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram);
        cpu.load_data(&data);
        cpu.run();
        assert_eq!(cpu.get_registers()[1], std::i64::MAX - 1);
        assert_eq!(cpu.get_flags()[1], true);
    }

    #[test]
    fn test_random_substraction_cpu_should_succeed() {
        let initial_state = [
            0i64,
            0x2,
            0i64,
            0x3,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
        ];
        let initial_ram = [0u8; 0xFFFF];
        //substract the register 3 to the register 1 and store in the register 1
        let initial_data: [u8; 4] = [
            0x80,
            0x61,
            0x31,
            0x00
        ];
        let data = initial_data.to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram);
        cpu.load_data(&data);
        cpu.run();
        assert_eq!(cpu.get_registers()[1], -1);
    }

    #[test]
    fn test_overflow_substraction_cpu_should_succeed() {
        let initial_state = [
            0i64,
            std::i64::MIN,
            0i64,
            std::i64::MAX,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
            0i64,
        ];
        let initial_ram = [0u8; 0xFFFF];
        let initial_data: [u8; 4] = [
            0x80,
            0x61,
            0x31,
            0x00
        ];
        let data = initial_data.to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram);
        cpu.load_data(&data);
        cpu.run();
        assert_eq!(cpu.get_registers()[1], -9223372036854775806);
        assert_eq!(cpu.get_flags()[2], true);
    }
/*
    #[test]
    fn test_random_multiplication_cpu_should_succeed() {
        let initial_state = [0i64; 16];
        let initial_ram = [0i8; 0xFFFF];
        let data = [1i8; 0xFFFFF].to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram);
        cpu.load_data(&data);
    }

    #[test]
    fn test_inf_overflow_multiplication_cpu_should_succeed() {
        let initial_state = [0i64; 16];
        let initial_ram = [0i8; 0xFFFF];
        let data = [1i8; 0xFFFFF].to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram);
        cpu.load_data(&data);
    }

    #[test]
    fn test_sup_overflow_multiplication_cpu_should_succeed() {
        let initial_state = [0i64; 16];
        let initial_ram = [0i8; 0xFFFF];
        let data = [1i8; 0xFFFFF].to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram);
        cpu.load_data(&data);
    }

    #[test]
    fn test_random_division_cpu_should_succeed() {
        let initial_state = [0i64; 16];
        let initial_ram = [0i8; 0xFFFF];
        let data = [1i8; 0xFFFFF].to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram);
        cpu.load_data(&data);
    }

    #[test]
    fn test_inf_overflow_division_cpu_should_succeed() {
        let initial_state = [0i64; 16];
        let initial_ram = [0i8; 0xFFFF];
        let data = [1i8; 0xFFFFF].to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram);
        cpu.load_data(&data);
    }

    #[test]
    fn test_sup_overflow_division_cpu_should_succeed() {
        let initial_state = [0i64; 16];
        let initial_ram = [0i8; 0xFFFF];
        let data = [1i8; 0xFFFFF].to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram);
        cpu.load_data(&data);
    }
    */
}
