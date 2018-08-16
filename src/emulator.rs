const MAX_U8: u8 = <u8>::max_value();

#[derive(Debug)]
struct ConditionCodes {
    z: bool,   // Zero, 1 when a is 0, else 0
    s: bool,   // Sign, 1 when bit 7 (MSB) is set in register a, else 0
    p: bool,   // Parity, 1 when the answer has even parity, else 0
    cy: bool,  // Carry, 1 when the previous instruction resulted in a carry, else 0
    ac: bool,  // Auxiliary carry
    pad: bool, // ?
}

#[derive(Debug)]
pub struct State8080 {
    a: u8,
    // a.k.a. accumulator register
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    memory: Vec<u8>,
    cc: ConditionCodes,
    int_enable: u8,
}

impl State8080 {
    pub fn new(game_data: Vec<u8>) -> State8080 {
        let codes = ConditionCodes {
            z: false,
            s: false,
            p: false,
            cy: false,
            ac: false,
            pad: false,
        };
        State8080 {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            memory: game_data.clone(),
            cc: codes,
            int_enable: 0,
        }
    }
    fn get_and_advance(&mut self) -> u8 {
        let value = self.memory.get(self.pc as usize)
            .expect(&format!("Failed to read memory at {}", self.pc));
        self.pc += 1;
        return *value;
    }
}

pub fn emulate_op(state: &mut State8080) {
    let code = state.get_and_advance();

    match code {
        0x00 => {} // NOP
        0x01 => {
            //LXI B, B <- byte 3, C <- byte 2
            state.c = state.get_and_advance();
            state.b = state.get_and_advance();
        }
        0x02 => {
            let mut destination: usize = state.b as usize;
            destination <<= 8;
            destination |= state.c as usize;
            state.memory[destination] = state.a;
        }
        0x03 => {
            inx(&mut state.b, &mut state.c)
        }
        0x81 => { // ADD C
            add(state.c, state);
        }
        _ => { println!("Unkown op code {:02x} ", code); }
    }
}

fn add(a: u8, state: &mut State8080) {
    let answer :u16 = state.a as u16 + a as u16;
    state.cc.z = answer as u8 & 0xff == 0;
    state.cc.s = answer as u8 & 0x80 > 0;
    state.cc.cy = answer > 0xff;
    // state->cc.p = Parity( answer & 0xff);
    state.a = answer as u8;
}

// inx B -> BC + 1 add one to lower then carry to upper
fn inx(upper: &mut u8, lower: &mut u8) {
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


#[cfg(test)]
mod tests {
    use emulator::State8080;
    use emulator::*;

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
        let contents = Vec::new();
        let mut state = State8080::new(contents);
        add(0, &mut state);
        assert_eq!(0, state.a);
        assert_eq!(true, state.cc.z);
        assert_eq!(false, state.cc.s);
        assert_eq!(false, state.cc.cy);

        add(1, &mut state);
        assert_eq!(1, state.a);
        assert_eq!(false, state.cc.z);
        assert_eq!(false, state.cc.s);
        assert_eq!(false, state.cc.cy);

        add(200, &mut state);
        assert_eq!(201, state.a);
        assert_eq!(false, state.cc.z);
        assert_eq!(true, state.cc.s);
        assert_eq!(false, state.cc.cy);

        add(200, &mut state);
        // 200 + 201 = 401 = 0x191, which gets truncated to 0x91 which is 145
        assert_eq!(145, state.a);
        assert_eq!(false, state.cc.z);
        assert_eq!(true, state.cc.s);
        assert_eq!(true, state.cc.cy);
    }
}