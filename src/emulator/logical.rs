use crate::emulator::State8080;

pub fn rlc(state: &mut State8080) {
    state.cc.cy = state.a >> 7 == 0b1;
    state.a = state.a << 1;
    if state.cc.cy {
        state.a |= 0b1;
    }
}

pub fn rrc(state: &mut State8080) {
    state.cc.cy = state.a & 0b1 == 0b1;
    state.a = state.a >> 1;
    if state.cc.cy {
        state.a |= 0b1 << 7;
    }
}

pub fn ral(state: &mut State8080) {
    let old_carry = state.cc.cy;
    state.cc.cy = state.a >> 7 == 0b1;
    state.a = state.a << 1;
    if old_carry {
        state.a |= 0b1;
    }
}

pub fn rar(state: &mut State8080) {
    let old_carry = state.cc.cy;
    state.cc.cy = state.a & 0b1 == 0b1;
    state.a = state.a >> 1;
    if old_carry {
        state.a |= 0b1 << 7;
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::test_utils::*;

    #[test]
    fn test_rlc_with_carry() {
        let mut state = setup_state();
        state.memory[0] = 0x07; // RLC op code
        state.a = 0b11110010;
        state.cc.cy = false;

        state.emulate_op();

        assert_eq!(state.a, 0b11100101);
        assert_eq!(state.cc.cy, true);
    }

    #[test]
    fn test_rlc_without_carry() {
        let mut state = setup_state();
        state.memory[0] = 0x07; // RLC op code
        state.a = 0b01110010;
        state.cc.cy = true;

        state.emulate_op();

        assert_eq!(state.a, 0b11100100);
        assert_eq!(state.cc.cy, false);
    }

    #[test]
    fn test_rrc_without_carry() {
        let mut state = setup_state();
        state.memory[0] = 0x0f; // RRC op code
        state.a = 0b11110010;
        state.cc.cy = true;

        state.emulate_op();

        assert_eq!(state.a, 0b01111001);
        assert_eq!(state.cc.cy, false);
    }

    #[test]
    fn test_rrc_with_carry() {
        let mut state = setup_state();
        state.memory[0] = 0x0f; // RRC op code
        state.a = 0b11110011;
        state.cc.cy = false;

        state.emulate_op();

        assert_eq!(state.a, 0b11111001);
        assert_eq!(state.cc.cy, true);
    }

    #[test]
    fn test_ral_with_carry() {
        let mut state = setup_state();
        state.memory[0] = 0x17; // RRC op code
        state.a = 0b10110101;
        state.cc.cy = false;

        state.emulate_op();

        assert_eq!(state.a, 0b01101010);
        assert_eq!(state.cc.cy, true);
    }

    #[test]
    fn test_ral_without_carry() {
        let mut state = setup_state();
        state.memory[0] = 0x17; // RRC op code
        state.a = 0b00110101;
        state.cc.cy = true;

        state.emulate_op();

        assert_eq!(state.a, 0b01101011);
        assert_eq!(state.cc.cy, false);
    }

    #[test]
    fn test_rar_with_carry() {
        let mut state = setup_state();
        state.memory[0] = 0x1f; // rar op code
        state.a = 0b01101010;
        state.cc.cy = true;

        state.emulate_op();

        assert_eq!(state.a, 0b10110101);
        assert_eq!(state.cc.cy, false);
    }

    #[test]
    fn test_rar_without_carry() {
        let mut state = setup_state();
        state.memory[0] = 0x1f; // rar op code
        state.a = 0b01101011;
        state.cc.cy = false;

        state.emulate_op();

        assert_eq!(state.a, 0b00110101);
        assert_eq!(state.cc.cy, true);
    }
}