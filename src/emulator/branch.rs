use crate::emulator::State8080;
use crate::emulator::utils::combine;

pub fn conditional_jmp(condition: bool, state: &mut State8080) {
    if condition {
        jmp(state);
    } else {
        state.pc += 2;
    }
}

pub fn call(state: &mut State8080) {
    let return_address = state.pc + 2;
    state.memory[(state.sp - 1) as usize] = ((return_address >> 8) & 0xff) as u8;
    state.memory[(state.sp - 2) as usize] = (return_address & 0xff) as u8;
    state.sp -= 2;
    jmp(state);
}

pub fn ret(state: &mut State8080) {
    let upper = state.memory[state.sp as usize];
    let lower = state.memory[(state.sp + 1) as usize];
    state.pc = combine(upper, lower);
    state.sp += 2;
}

pub fn jmp(state: &mut State8080) {
    state.pc = state.get_double_at_pc();
}

#[cfg(test)]
mod tests {
    use crate::emulator::test_utils::*;

    #[test]
    fn test_jmp() {
        let mut state = setup_state();
        state.memory[0] = 0xc3; // JMP op code
        state.memory[1] = 0x11; // lower half of address
        state.memory[2] = 0x22; // upper half of address

        state.emulate_op();

        assert_eq!(state.pc, 0x2211);
    }

    #[test]
    fn test_call() {
        let mut state = setup_state();
        state.sp = 100;
        state.pc = 0x1234; // start somewhere interesting
        state.memory[0x1234] = 0xcd; // CALL op code
        state.memory[0x1235] = 0x11; // lower half of address
        state.memory[0x1236] = 0x22; // upper half of address

        state.emulate_op();

        assert_eq!(state.pc, 0x2211);
        assert_eq!(state.sp, 98);
        assert_eq!(state.memory[99], 0x12);
        assert_eq!(state.memory[98], 0x37); // 1234 + 1 for jump + 2 for jmp address
    }

    #[test]
    fn test_ret() {
        let mut state = setup_state();

        state.sp = 100;
        state.pc = 0x3456; // start somewhere interesting
        state.memory[0x3456] = 0xc9; // RET op code
        state.memory[100] = 0x11; // lower half of address
        state.memory[101] = 0x22; // upper half of address

        state.emulate_op();

        assert_eq!(state.pc, 0x1122);
        assert_eq!(state.sp, 102);
    }
}