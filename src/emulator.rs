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
    a: u8, // a.k.a. accumulator register
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
            sp: 0, // Must be initialized by the program to somewhere not used to store game data or heap
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
            let destination = combine_registers(state.b, state.c) as usize;
            state.memory[destination] = state.a;
        }
        0x03 => { inx(&mut state.b, &mut state.c); }
        0x13 => { inx(&mut state.d, &mut state.e); }
        0x23 => { inx(&mut state.h, &mut state.l); }
        0x33 => { state.sp += 1; }
        0x80 => { add(state.b, state); }
        0x81 => { add(state.c, state); }
        0x82 => { add(state.d, state); }
        0x83 => { add(state.e, state); }
        0x84 => { add(state.h, state); }
        0x85 => { add(state.l, state); }
        0x86 => {
            let address = combine_registers(state.h, state.l) as usize;
            let val = state.memory[address];
            add(val, state);
        }
        0x87 => { adc(state.a, state); }
        0x88 => { adc(state.b, state); }
        0x89 => { adc(state.c, state); }
        0x8a => { adc(state.d, state); }
        0x8b => { adc(state.e, state); }
        0x8c => { adc(state.h, state); }
        0x8d => { adc(state.l, state); }
        0x8e => {
            let address = combine_registers(state.h, state.l) as usize;
            let val = state.memory[address];
            add(val, state);
        }
        0x8f => { adc(state.a, state); }
        0xc2 => {
            if state.cc.z {
                state.pc += 2;
            } else {
                jmp(state);
            }
        }
        0xc3 => {
            jmp(state);
        }
        0xcd => { // CALL
            let return_address = state.pc + 2;
            state.memory[(state.sp - 1) as usize] = ((return_address >> 8) & 0xff) as u8;
            state.memory[(state.sp - 2) as usize] = (return_address & 0xff) as u8;
            state.sp -= 2;
            jmp(state);
        }
        0xc9 => { // RET
            let upper = state.memory[state.sp as usize];
            let lower = state.memory[(state.sp + 1) as usize];
            state.pc = combine_registers(upper, lower);
            state.sp += 2;
        }

        _ => { println!("Unkown op code {:02x} ", code); }
    }
}

fn jmp(state: &mut State8080) {
    let lower = state.get_and_advance();
    let upper = state.get_and_advance();
    state.pc = combine_registers(upper, lower);
}

fn combine_registers(upper: u8, lower: u8) -> u16 {
    let mut destination: u16 = upper as u16;
    destination <<= 8;
    destination |= lower as u16;
    return destination;
}

fn add(value: u8, state: &mut State8080) {
    add_core(value, state, false);
}

fn adc(value: u8, state: &mut State8080) {
    add_core(value, state, true);
}

fn add_core(value: u8, state: &mut State8080, use_carry: bool) {
    let answer :u16 = state.a as u16 + value as u16 + if use_carry && state.cc.cy {1} else {0};
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

    fn setup_state() -> State8080 {
        let contents = vec![0; 64_000];
        let mut state =  State8080::new(contents);
        state.sp = 100;
        return state;
    }

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

    #[test]
    fn test_jmp() {
        let mut state = setup_state();
        state.memory[0] = 0xc3; // JMP op code
        state.memory[1] = 0x11; // lower half of address
        state.memory[2] = 0x22; // upper half of address

        emulate_op(&mut state);

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

        emulate_op(&mut state);

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

        emulate_op(&mut state);

        assert_eq!(state.pc, 0x1122);
        assert_eq!(state.sp, 102);
    }
}