use crate::emulator::ConditionCodes;

pub fn rotate_left_with_carry(register: &mut u8, cc: &mut ConditionCodes) {
    cc.cy = *register >> 7 == 0b1;
    *register = *register << 1;
    if cc.cy {
        *register |= 0b1;
    }
}

pub fn rotate_right_with_carry(register: &mut u8, cc: &mut ConditionCodes) {
    cc.cy = *register & 0b1 == 0b1;
    *register = *register << 1;
    if cc.cy {
        *register |= 0b1;
    }
}