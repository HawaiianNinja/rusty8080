use crate::emulator::ConditionCodes;

pub fn combine(upper: u8, lower: u8) -> u16 {
    let mut destination = (upper as u16) << 8;
    destination |= lower as u16;
    return destination;
}

pub fn split(num: u16) -> (u8, u8) {
    let lower = (num & 0xff) as u8;
    let upper = (num >> 8) as u8;
    return (upper, lower);
}

pub fn update_flags(value: u16, codes: &mut ConditionCodes) {
    codes.z = value as u8 & 0xff == 0;
    codes.s = value as u8 & 0x80 > 0;
    codes.cy = value > 0xff;
    codes.p = parity(value as usize, 8);
}

fn parity(value_to_check: usize, size: usize) -> bool {
    let mut set_bits = 0;
    let mut mask: usize = 1;
    mask = mask << size; // 0xff
    mask -= 1; // 0xfe
    let mut temp = value_to_check & mask;
    for _number in 0..size {
        if temp & 0x1 == 0x1 {
            set_bits += 1;
        }
        temp = temp >> 1;
    }
    return 0 == (set_bits & 0x1);
}

#[cfg(test)]
mod tests {
    use crate::emulator::utils::combine;
    use crate::emulator::utils::parity;
    use crate::emulator::utils::split;

    #[test]
    fn test_parity() {
        assert_eq!(true, parity(0b0000, 4));
        assert_eq!(true, parity(0b00000, 5));
        assert_eq!(false, parity(0b00001, 5));
        assert_eq!(false, parity(0b10000, 5));
        assert_eq!(false, parity(0b01000, 5));
        assert_eq!(true, parity(0b01000100, 8));
        assert_eq!(true, parity(0b0100000, 2));
        assert_eq!(true, parity(0b011111111, 8));
        assert_eq!(true, parity(0xff, 8));
    }

    #[test]
    fn test_combine_and_split() {
        let upper1 = 0xab as u8;
        let lower1 = 0xcd as u8;
        let combined = combine(upper1, lower1);
        assert_eq!(combined, 0xabcd);
        let (upper2, lower2) = split(combined);
        assert_eq!(upper2, 0xab);
        assert_eq!(lower2, 0xcd);
    }
}
