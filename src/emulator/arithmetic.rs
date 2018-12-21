use crate::emulator::State8080;
use crate::emulator::utils::update_flags;
use crate::emulator::ConditionCodes;

const MAX_U8: u8 = <u8>::max_value();

pub fn add(value: u8, state: &mut State8080) {
    add_core(value, state, false);
}

pub fn adc(value: u8, state: &mut State8080) {
    add_core(value, state, true);
}

pub fn add_core(value: u8, state: &mut State8080, use_carry: bool) {
    let answer :u16 = state.a as u16 + value as u16 + if use_carry && state.cc.cy {1} else {0};
    update_flags(answer, &mut state.cc);
    state.a = answer as u8;
}

// dad B = h,l = b,c + h,l
pub fn dad(upper_save: &mut u8, lower_save: &mut u8, upper2: &mut u8, lower2: &mut u8, cc: &mut ConditionCodes) {
    let temp_lower: u16 = *lower_save as u16 + *lower2 as u16;
    let mut temp_upper : u16 = *upper_save as u16 + *upper2 as u16;
    if temp_lower > MAX_U8 as u16 {
        *lower_save = 0;
        temp_upper += 1;
    } else {
        *lower_save = temp_lower as u8;
    }
    if temp_upper > MAX_U8 as u16{
        cc.cy = true;
        *upper_save = 0;
    } else {
        cc.cy = false;
        *upper_save = temp_upper as u8;
    }
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
}