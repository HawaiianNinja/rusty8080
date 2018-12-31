use crate::emulator::State8080;
use crate::emulator::utils::update_flags;
use crate::emulator::ConditionCodes;
use crate::emulator::utils::combine;
use crate::emulator::utils::split;

const MAX_U8: u8 = <u8>::max_value();
const MAX_U16: u16 = <u16>::max_value();

pub fn add(value: u8, state: &mut State8080) {
    add_core(value, state, false);
}

pub fn adc(value: u8, state: &mut State8080) {
    add_core(value, state, true);
}

fn add_core(value: u8, state: &mut State8080, use_carry: bool) {
    let answer :u16 = state.a as u16 + value as u16 + if use_carry && state.cc.cy {1} else {0};
    update_flags(answer, &mut state.cc);
    state.a = answer as u8;
}

pub fn dad(num: u16, state: &mut State8080) {
    let other = combine(state.h, state.l) as u32;
    let answer = num as u32 + other;
    state.cc.cy = answer > MAX_U16 as u32;
    let (upper, lower) = split(answer as u16);
    state.h = upper;
    state.l = lower;
}

// inx B -> BC + 1 add one to lower then carry to upper
pub fn inx(upper: &mut u8, lower: &mut u8) {
    let mut carry = false;
    if *lower < MAX_U8 {
        *lower += 1;
    } else {
        *lower = 0;
        carry = true;
    }

    if carry {
        if *upper < MAX_U8 {
            *upper += 1;
        } else {
            *upper = 0;
        }
    }
}

// inx B -> BC + 1 add one to lower then carry to upper
pub fn dcx(upper: &mut u8, lower: &mut u8) {
    let mut carry = false;
    if *lower > 0 {
        *lower -= 1;
    } else {
        carry = true;
    }

    if carry {
        if *upper > 0 {
            *upper -= 1;
        } else {
            *upper = 0xff;
        }
    }
}

pub fn inr(value: &mut u8, codes : &mut ConditionCodes) {
    let answer : u16 = *value as u16 + 1;
    update_flags(answer, codes);
    *value = answer as u8;
}

pub fn dcr(value: &mut u8, codes : &mut ConditionCodes) {
    if *value > 0 {
        let answer: u16 = *value as u16 - 1;
        update_flags(answer, codes);
        *value = answer as u8;
    } else {
        *value = 0xff;
        update_flags(*value as u16, codes);
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::test_utils::*;
    use crate::emulator::arithmetic::*;
    use crate::emulator::ConditionCodes;

    #[test]
    fn test_inx() {
        let mut lower: u8 = 0;
        let mut upper: u8 = 0;
        inx(&mut upper, &mut lower);
        assert_eq!(lower, 1);
        assert_eq!(upper, 0);

        lower = 255;
        inx(&mut upper, &mut lower);
        assert_eq!(lower, 0);
        assert_eq!(upper, 1);

        lower = 255;
        upper = 255;
        inx(&mut upper, &mut lower);
        assert_eq!(lower, 0);
        assert_eq!(upper, 0);
    }

    #[test]
    fn test_add() {
        let mut state = setup_state();
        add(0, &mut state);
        assert_eq!(0, state.a);
        assert_eq!(true, state.cc.z);
        assert_eq!(false, state.cc.s);
        assert_eq!(false, state.cc.cy);
        assert_eq!(true, state.cc.p); // 000


        add(1, &mut state);
        assert_eq!(1, state.a);
        assert_eq!(false, state.cc.z);
        assert_eq!(false, state.cc.s);
        assert_eq!(false, state.cc.cy);
        assert_eq!(false, state.cc.p); // 001

        add(1, &mut state);
        assert_eq!(2, state.a);
        assert_eq!(false, state.cc.z);
        assert_eq!(false, state.cc.s);
        assert_eq!(false, state.cc.cy);
        assert_eq!(false, state.cc.p); // 010

        add(200, &mut state);
        assert_eq!(202, state.a);
        assert_eq!(false, state.cc.z);
        assert_eq!(true, state.cc.s);
        assert_eq!(false, state.cc.cy);
        assert_eq!(true, state.cc.p); // 11001010


        add(202, &mut state);
        // 202 + 202 = 404 = 0x194, which gets truncated to 0x94 which is 148
        assert_eq!(148, state.a);
        assert_eq!(false, state.cc.z);
        assert_eq!(true, state.cc.s);
        assert_eq!(true, state.cc.cy);
        assert_eq!(false, state.cc.p); //10010100
    }

    #[test]
    fn test_dad() {
        let mut state = setup_state();
        dad(0x0001, &mut state);
        assert_eq!(state.h, 0x00);
        assert_eq!(state.l, 0x01);
        assert_eq!(state.cc.cy, false);
        dad(0x1001, &mut state);
        assert_eq!(state.h, 0x10);
        assert_eq!(state.l, 0x02);
        assert_eq!(state.cc.cy, false);
        //0x1002 + 0xfefe = 0x10f00 becomes 0x0f00
        dad(0xfefe, &mut state);
        assert_eq!(state.h, 0x0f);
        assert_eq!(state.l, 0x00);
        assert_eq!(state.cc.cy, true);
    }

    #[test]
    fn test_inr() {
        let mut codes = ConditionCodes {
            z: false,
            s: false,
            p: false,
            cy: false,
            ac: false,
            pad: false,
        };

        let mut test :u8 = 0;
        inr(&mut test, &mut codes);
        assert_eq!(test, 1);
        assert_eq!(false, codes.z);
        assert_eq!(false, codes.s);
        assert_eq!(false, codes.p);
        assert_eq!(false, codes.cy);

        inr(&mut test, &mut codes);
        assert_eq!(test, 2);
        assert_eq!(false, codes.z);
        assert_eq!(false, codes.s);
        assert_eq!(false, codes.p);
        assert_eq!(false, codes.cy);

        inr(&mut test, &mut codes);
        assert_eq!(test, 3);
        assert_eq!(false, codes.z);
        assert_eq!(false, codes.s);
        assert_eq!(true, codes.p);
        assert_eq!(false, codes.cy);

        test = 255;
        inr(&mut test, &mut codes);
        assert_eq!(test, 0);
        assert_eq!(true, codes.z);
        assert_eq!(false, codes.s);
        assert_eq!(true, codes.p);
        assert_eq!(true, codes.cy);
    }

    #[test]
    fn test_inr_m() {
        let mut state = setup_state();
        state.memory[0] = 0x34; // CMA op code
        state.memory[1] = 0x34; // CMA op code
        state.h = 0x12;
        state.l = 0xab;
        assert_eq!(state.memory[0x12ab], 0);
        state.emulate_op();
        assert_eq!(state.memory[0x12ab], 1);

        state.memory[0x12ab] = 0xff;
        state.emulate_op();
        assert_eq!(state.memory[0x12ab], 0);
        assert_eq!(state.cc.cy, true);
    }
}