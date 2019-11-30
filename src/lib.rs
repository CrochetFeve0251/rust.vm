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
        let initial_ram = [0i8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram);
        assert_eq!(cpu.get_registers().to_vec(), initial_state.to_vec());
    }

    #[test]
    fn cpu_initialize_flags_all_false() {
        let initial_state = [0i64; 16];
        let initial_ram = [0i8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram);
        assert_eq!(cpu.get_flags().to_vec(), [false; 64].to_vec());
    }

    #[test]
    fn cpu_initialize_ip_equals_zero() {
        let initial_state = [0i64; 16];
        let initial_ram = [0i8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram);
        assert_eq!(*cpu.get_ip(), 0i64);
    }

    #[test]
    fn cpu_initialize_pc_equals_zero() {
        let initial_state = [0i64; 16];
        let initial_ram = [0i8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram);
        assert_eq!(*cpu.get_pc(), 0usize);
    }

    #[test]
    fn cpu_initialize_stack_empty() {

    }

    #[test]
    fn cpu_initialize_ram_empty() {
        let initial_state = [0i64; 16];
        let initial_ram = [0i8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram);
        assert_eq!(cpu.get_ram().to_vec(), [0; 0xFFFF].to_vec());
    }

    #[test]
    fn stack_initialize_index_at_zero() {
        let stack = Stack::new([0; 0xFFFF]);
    }

    #[test]
    fn stack_push_increment_index() {

    }

    #[test]
    fn stack_push_add_element() {

    }

    #[test]
    fn stack_pop_decrement_index() {

    }

    #[test]
    fn state_pop_remove_element() {

    }

    #[test]
    fn stack_cannot_pop_if_index_zero() {

    }

    #[test]
    fn stack_cannot_push_if_index_max() {

    }

    #[test]
    fn stack_load_data_should_change_ram() {
        let initial_state = [0i64; 16];
        let initial_ram = [0i8; 0xFFFF];
        let data = [1i8; 0xFFFF];
        let mut cpu = Cpu::new(initial_state, initial_ram);
        cpu.load_data(&data.to_vec());
        assert_eq!(cpu.get_ram().to_vec(), data.to_vec());
    }

    #[test]
    fn stack_load_data_too_big_should_change_ram() {
        let initial_state = [0i64; 16];
        let initial_ram = [0i8; 0xFFFF];
        let data = [1i8; 0xFFFFF].to_vec();
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

}
