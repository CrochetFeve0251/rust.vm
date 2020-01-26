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
        let initial_state = [0u64; 16];
        let initial_ram = [0u8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram, false);
        assert_eq!(cpu.get_registers().to_vec(), initial_state.to_vec());
    }

    #[test]
    fn cpu_initialize_flags_all_false() {
        let initial_state = [0u64; 16];
        let initial_ram = [0u8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram, false);
        assert_eq!(cpu.get_flags().to_vec(), [false; 64].to_vec());
    }

    #[test]
    fn cpu_initialize_ip_equals_zero() {
        let initial_state = [0u64; 16];
        let initial_ram = [0u8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram, false);
        assert_eq!(*cpu.get_ip(), 0i64);
    }

    #[test]
    fn cpu_initialize_pc_equals_zero() {
        let initial_state = [0u64; 16];
        let initial_ram = [0u8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram, false);
        assert_eq!(*cpu.get_pc(), 0usize);
    }

    #[test]
    fn cpu_initialize_stack_empty() {

    }

    #[test]
    fn cpu_initialize_ram_empty() {
        let initial_state = [0u64; 16];
        let initial_ram = [0u8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram, false);
        assert_eq!(cpu.get_ram().to_vec(), [0; 0xFFFF].to_vec());
    }

    #[test]
    fn stack_initialize_index_at_zero() {
        let stack = Stack::new([0; 0xFFFF]);
    }

    #[test]
    fn stack_load_data_should_change_ram() {
        let initial_state = [0u64; 16];
        let initial_ram = [0u8; 0xFFFF];
        let data = [1u8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram, false);
        cpu.load_data(&data.to_vec());
        assert_eq!(cpu.get_ram().to_vec(), data.to_vec());
    }

    #[test]
    fn stack_load_data_too_big_should_change_ram() {
        let initial_state = [0u64; 16];
        let initial_ram = [0u8; 0xFFFF];
        let data = [1u8; 0xFFFFF].to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram, false);
        cpu.load_data(&data);
        assert_eq!(cpu.get_ram().to_vec(), initial_ram.to_vec());
    }


    #[test]
    fn test_random_instruction_convert(){
        let initial = 0x01122300;
        let result = Instruction::new(initial);
        match result {
            Instruction::OperationInstruction { opcode, iv_flag, ope1, ope2, dest, iv_value} => {
                assert!(iv_flag);
                assert!(opcode == cpu::instruction::opcode::Opcode::OR);
                assert_eq!(ope1, 2);
                assert_eq!(ope2, 2);
                assert_eq!(dest, 3);
                assert_eq!(iv_value, 0);
            },
            Instruction::BranchInstruction { bcc, is_positive, offset } => assert!(false),
            Instruction::ErrorInstruction => assert!(false)
        }
    }

    #[test]
        fn test_random_instruction_2_convert() {
        let initial = 0xe0a4537D;
        let result = Instruction::new(initial);
        match result {
            Instruction::OperationInstruction { opcode, iv_flag, ope1, ope2, dest, iv_value } => assert!(false),
            Instruction::BranchInstruction { bcc, is_positive, offset } => {
                assert!(bcc == cpu::instruction::branch_condition_code::BranchConditionCode::BG);
                assert_eq!(is_positive, false);
                assert_eq!(offset, 8213636);
                assert!(true)
            },
            Instruction::ErrorInstruction => assert!(false)
        }
    }

        #[test]
        fn test_random_instruction_flag_should_success(){
            let initial = 0xe7a4537D;
            let result = Instruction::new(initial);
            match result {

                Instruction::OperationInstruction { opcode, iv_flag, ope1, ope2, dest, iv_value} => assert!(false),
                Instruction::BranchInstruction { bcc, is_positive, offset } => {
                    assert!(bcc == cpu::instruction::branch_condition_code::BranchConditionCode::BG);
                    assert_eq!(is_positive, true);
                    println!("{:x}\n", offset);
                    assert_eq!(offset, 8213642);
                    assert!(true)
                },
                Instruction::ErrorInstruction => assert!(false)
            }
        }

    #[test]
    fn test_random_addition_cpu_should_succeed() {
        let initial_state = [
            0u64,
            1u64,
            0u64,
            3u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
        ];
        let initial_ram = [0u8; 0xFFFF];
        //addition registre 1 et 3 dans le registre 1
        let initial_data: [u8; 4] = [
            0x00,
            0x31,
            0x31,
            0x00
        ];
        let data = initial_data.to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram, false);
        cpu.load_data(&data);
        cpu.run();
        assert_eq!(cpu.get_registers()[1], 3 + 1);
    }

    #[test]
    fn test_random_addition_direct_value_cpu_should_succeed() {
        let initial_state = [
            0u64,
            1u64,
            0u64,
            45u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
        ];
        let initial_ram = [0u8; 0xFFFF];
        //addition registre 1 et valeur 3 dans le registre 1
        let initial_data: [u8; 4] = [
            0x01,
            0x31,
            0x31,
            0x03
        ];
        let data = initial_data.to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram, false);
        cpu.load_data(&data);
        cpu.run();
        assert_eq!(cpu.get_registers()[1], 3 + 1);
    }

    #[test]
    fn test_overflow_addition_instruction_cpu_should_succeed() {
        let initial_state = [
            0u64,
            std::u64::MAX,
            0u64,
            std::u64::MAX,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
        ];
        let initial_ram = [0u8; 0xFFFF];
        let initial_data: [u8; 4] = [
            0x00,
            0x31,
            0x31,
            0x00
        ];
        let data = initial_data.to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram, false);
        cpu.load_data(&data);
        cpu.run();
        assert_eq!(cpu.get_registers()[1], std::u64::MAX - 1);
        assert_eq!(cpu.get_flags()[1], true);
    }

    #[test]
    fn test_random_substraction_cpu_should_succeed() {
        let initial_state = [
            0u64,
            0x2,
            0u64,
            0x3,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
        ];
        let initial_ram = [0u8; 0xFFFF];
        //substract the register 3 to the register 1 and store in the register 1
        let initial_data: [u8; 4] = [
            0x00,
            0x61,
            0x31,
            0x00
        ];
        let data = initial_data.to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram, false);
        cpu.load_data(&data);
        cpu.run();
        assert_eq!(cpu.get_registers()[1], -1);
    }

    #[test]
    fn test_overflow_substraction_cpu_should_succeed() {
        let initial_state = [
            0u64,
            std::u64::MIN,
            0u64,
            std::u64::MAX,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
        ];
        let initial_ram = [0u8; 0xFFFF];
        let initial_data: [u8; 4] = [
            0x00,
            0x61,
            0x31,
            0x00
        ];
        let data = initial_data.to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram, false);
        cpu.load_data(&data);
        cpu.run();
        assert_eq!(cpu.get_registers()[1], -9223372036854775806);
        assert_eq!(cpu.get_flags()[2], true);
    }

    #[test]
    fn test_branch_with_equal_test_should_succeed() {
        let initial_state = [
            0u64,
            std::u64::MAX,
            0u64,
            std::u64::MAX,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
        ];
        let initial_ram = [0u8; 0xFFFF];
        let initial_data: [u8; 4] = [
            0x00,
            0x51,
            0x31,
            0x00
        ];
        let data = initial_data.to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram, false);
        cpu.load_data(&data);
        cpu.run();
        assert_eq!(cpu.get_flags()[3], true);
        assert_eq!(cpu.get_flags()[4], false);
        assert_eq!(cpu.get_flags()[5], true);
        assert_eq!(cpu.get_flags()[6], true);
        assert_eq!(cpu.get_flags()[7], false);
        assert_eq!(cpu.get_flags()[8], false);
    }

    #[test]
    fn test_branch_with_min_test_should_succeed() {
        let initial_state = [
            0u64,
            std::u64::MIN,
            0u64,
            std::u64::MAX,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
        ];
        let initial_ram = [0u8; 0xFFFF];
        let initial_data: [u8; 4] = [
            0x00,
            0x51,
            0x31,
            0x00
        ];
        let data = initial_data.to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram, false);
        cpu.load_data(&data);
        cpu.run();
        assert_eq!(cpu.get_flags()[3], false);
        assert_eq!(cpu.get_flags()[4], true);
        assert_eq!(cpu.get_flags()[5], true);
        assert_eq!(cpu.get_flags()[6], false);
        assert_eq!(cpu.get_flags()[7], true);
        assert_eq!(cpu.get_flags()[8], false);
    }

    #[test]
    fn test_branch_with_max_test_should_succeed() {
        let initial_state = [
            0u64,
            std::u64::MAX,
            0u64,
            std::u64::MIN,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
            0u64,
        ];
        let initial_ram = [0u8; 0xFFFF];
        let initial_data: [u8; 4] = [
            0x00,
            0x51,
            0x31,
            0x00
        ];
        let data = initial_data.to_vec();
        let mut cpu = Cpu::new(initial_state, initial_ram, false);
        cpu.load_data(&data);
        cpu.run();
        assert_eq!(cpu.get_flags()[3], false);
        assert_eq!(cpu.get_flags()[4], true);
        assert_eq!(cpu.get_flags()[5], false);
        assert_eq!(cpu.get_flags()[6], true);
        assert_eq!(cpu.get_flags()[7], false);
        assert_eq!(cpu.get_flags()[8], true);
    }
}


