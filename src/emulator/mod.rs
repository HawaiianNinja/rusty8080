mod utils;
mod test_utils;
mod arithmetic;
mod branch;
mod logical;

use log::error;
use log::debug;
use crate::disassembler::disassemble_op;
use crate::emulator::utils::*;
use crate::emulator::branch::*;
use crate::emulator::arithmetic::*;
use crate::emulator::logical::*;

#[derive(Debug)]
pub struct ConditionCodes {
    z: bool,   // Zero, 1 when a is 0, else 0
    s: bool,   // Sign, 1 when bit 7 (MSB) is set in register a, else 0
    p: bool,   // Parity, 1 when the answer has an even number of 1 bits
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
            sp: 0xf000, // Must be initialized by the program to somewhere not used to store game data or heap
            pc: 0,
            memory: game_data.clone(),
            cc: codes,
            int_enable: 0,
        }
    }
    fn get_at_pc(&mut self) -> u8 {
        let value = self.memory[self.pc as usize];
        self.pc += 1;
        return value;
    }

    fn get_double_at_pc(&mut self) -> u16 {
        let lower = self.get_at_pc();
        let upper = self.get_at_pc();
        return combine(upper, lower);
    }

    fn m(&mut self) -> u16 {
        return combine(self.h, self.l);
    }

    pub fn emulate_op(&mut self) {
        let (op, _) = disassemble_op(&self.memory, self.pc as usize);
//    debug!("{:19} pc: {:4x} sp:{:4x} a:{:2x} b:{:2x} c:{:2x} d:{:2x} e:{:2x} h:{:2x} l:{:2x} {:?}", op, self.pc, self.sp, self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.cc);
        let z = if self.cc.z {"z"} else {"."};
        let s = if self.cc.z {"s"} else {"."};
        let p = if self.cc.z {"p"} else {"."};
        let cy = if self.cc.z {"cy"} else {"."};
        let ac = if self.cc.z {"ac"} else {"."};
        let pad = if self.cc.z {"pad"} else {"."};
        debug!("{:19} a:{:02x} bc:{:02x}{:02x} de:{:02x}{:02x} hl:{:02x}{:02x} pc:{:04x} sp:{:04x} {}{}{}{}{}{}", op, self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.pc, self.sp, z, s, p, cy, ac, pad);

        let code = self.get_at_pc();

        match code {
            0x00 => {} // NOP
            0x01 => { //LXI B, B <- byte 3, C <- byte 2
                self.c = self.get_at_pc();
                self.b = self.get_at_pc();
            }
            0x02 => { // STAX B
                let destination = combine(self.b, self.c) as usize;
                self.memory[destination] = self.a;
            }
            0x03 => { inx(&mut self.b, &mut self.c); }
            0x04 => { inr(&mut self.b, &mut self.cc); }
            0x05 => { dcr(&mut self.b, &mut self.cc); }
            0x06 => { self.b = self.get_at_pc(); }
            0x07 => { rlc(self) }
            0x08 => {} // NOP
            0x09 => { dad(combine(self.b, self.c), self); }
            0x0a => { // LDAX B
                let target = combine(self.b, self.c) as usize;
                self.a = self.memory[target];
            }
            0x0b => { dcx(&mut self.b, &mut self.c); }
            0x0c => { inr(&mut self.c, &mut self.cc); }
            0x0d => { dcr(&mut self.c, &mut self.cc); }
            0x0e => { self.c = self.get_at_pc();}
            0x0f => { rrc(self); }

            0x10 => {} // NOP
            0x11 => {
                self.e = self.get_at_pc();
                self.d = self.get_at_pc();
            }
            0x12 => {
                let destination = combine(self.d, self.e) as usize;
                self.memory[destination] = self.a;
            }
            0x13 => { inx(&mut self.d, &mut self.e); }
            0x14 => { inr(&mut self.d, &mut self.cc); }
            0x15 => { dcr(&mut self.d, &mut self.cc); }
            0x16 => { self.d = self.get_at_pc(); }
            0x17 => { ral(self); }
            0x18 => {} // NOP
            0x19 => { dad(combine(self.d, self.e), self); }
            0x1a => {
                let target = combine(self.d, self.e) as usize;
                self.a = self.memory[target];
            }
            0x1b => { dcx(&mut self.d, &mut self.e); }
            0x1c => { inr(&mut self.e, &mut self.cc); }
            0x1d => { dcr(&mut self.e, &mut self.cc); }
            0x1e => { self.e = self.get_at_pc(); }
            0x1f => { rar(self); }

            0x20 => {} //NOP
            0x21 => { // LXI H
                self.l = self.get_at_pc();
                self.h = self.get_at_pc();
            }
            0x22 => { // SHLD Store H and L Direct
                let address = self.get_double_at_pc() as usize;
                self.memory[address] = self.l;
                self.memory[address + 1] = self.h;
            }
            0x23 => { inx(&mut self.h, &mut self.l); }
            0x24 => { inr(&mut self.h, &mut self.cc); }
            0x25 => { dcr(&mut self.h, &mut self.cc); }
            0x26 => { self.h = self.get_at_pc(); }
            // 0x27 DAA
            0x28 => {} // NOP
            0x29 => {
                let num = combine(self.h, self.l);
                dad(num, self);
            }
            0x2a => {
                let address = self.get_double_at_pc() as usize;
                self.l = self.memory[address];
                self.h = self.memory[address + 1];
            }
            0x2b => { dcx(&mut self.h, &mut self.l); }
            0x2c => { inr(&mut self.l, &mut self.cc); }
            0x2d => { dcr(&mut self.l, &mut self.cc); }
            0x2e => { self.l = self.get_at_pc(); }
            0x2f => { self.a = !self.a; }

            0x30 => {} // NOP
            0x31 => { self.sp = self.get_double_at_pc(); }
            0x32 => {
                let m = self.m() as usize;
                self.memory[m] = self.a;
            }
            0x33 => { self.sp += 1; }
            0x34 => {
                let m = self.m() as usize;
                inr(&mut self.memory[m], &mut self.cc);
            }
            0x35 => {
                let m = self.m() as usize;
                dcr(&mut self.memory[m], &mut self.cc);
            }
            0x36 => {
                let m = self.m() as usize;
                self.memory[m] = self.get_at_pc(); }
            0x37 => { self.cc.cy = true; }
            0x38 => {} // NOP
            0x39 => { dad(self.sp, self); }
            0x3a => {
                let address = self.get_double_at_pc() as usize;
                self.a = self.memory[address];
            }

            0x77 => { self.memory[combine(self.h, self.l) as usize] = self.a; }

            0x80 => { add(self.b, self); }
            0x81 => { add(self.c, self); }
            0x82 => { add(self.d, self); }
            0x83 => { add(self.e, self); }
            0x84 => { add(self.h, self); }
            0x85 => { add(self.l, self); }
            0x86 => { // ADD M
                let address = combine(self.h, self.l) as usize;
                let val = self.memory[address];
                add(val, self);
            }
            0x87 => { adc(self.a, self); }
            0x88 => { adc(self.b, self); }
            0x89 => { adc(self.c, self); }
            0x8a => { adc(self.d, self); }
            0x8b => { adc(self.e, self); }
            0x8c => { adc(self.h, self); }
            0x8d => { adc(self.l, self); }
            0x8e => { //ADC M
                let address = combine(self.h, self.l) as usize;
                let val = self.memory[address];
                add(val, self);
            }
            0x8f => { adc(self.a, self); }

            0xc2 => { conditional_jmp(!self.cc.z, self); }
            0xc3 => { jmp(self); }
            0xca => { conditional_jmp(self.cc.z, self); }
            0xcd => { call(self); }
            0xc9 => { ret(self); }

            0xd2 => { conditional_jmp(!self.cc.cy, self); }
            0xda => { conditional_jmp(self.cc.cy, self); }

            0xe2 => { conditional_jmp(!self.cc.p, self); }
            0xea => { conditional_jmp(self.cc.p, self); }

            _ => { error!("Skipped {:2x}", code); }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::test_utils::*;

    #[test]
    fn test_shld() {
        let mut state = setup_state();
        state.memory[0] = 0x22; // SHLD op code
        state.memory[1] = 0x11; // Lower part of address
        state.memory[2] = 0x33; // Upper part of address
        state.h = 0xaa;
        state.l = 0xbb;

        state.emulate_op();

        assert_eq!(state.memory[0x3311], 0xbb);
        assert_eq!(state.memory[0x3312], 0xaa);
    }

    #[test]
    fn test_cma() {
        let mut state = setup_state();
        state.memory[0] = 0x2f; // CMA op code
        state.a = 0b01010001;

        state.emulate_op();

        assert_eq!(state.a, 0b10101110);
    }
}