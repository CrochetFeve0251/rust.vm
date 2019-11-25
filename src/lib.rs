mod cpu;
mod instruction;
use cpu::Cpu;
use cpu::stack::Stack;

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


}
