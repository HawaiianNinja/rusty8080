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
    fn get_and_advance(&mut self) -> u8 {
        let value = self.memory.get(self.pc as usize)
            .expect(&format!("Failed to read memory at {}", self.pc));
        self.pc += 1;
        return *value;
    }

    fn get_double(&mut self) -> u16 {
        let lower = self.get_and_advance();
        let upper = self.get_and_advance();
        return combine_registers(upper, lower);
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

        let code = self.get_and_advance();

        match code {
            0x00 => {} // NOP
            0x01 => { //LXI B, B <- byte 3, C <- byte 2
                self.c = self.get_and_advance();
                self.b = self.get_and_advance();
            }
            0x02 => { // STAX B
                let destination = combine_registers(self.b, self.c) as usize;
                self.memory[destination] = self.a;
            }
            0x03 => { inx(&mut self.b, &mut self.c); }
            0x04 => { inr(&mut self.b, &mut self.cc); }
            0x05 => { dcr(&mut self.b, &mut self.cc); }
            0x06 => { //MVI B
                self.b = self.get_and_advance();
            }
            0x07 => { // RLC with carry
                let acc = self.a;
                self.cc.cy = acc >> 7 == 1;
                self.a = acc << 1;
                if self.cc.cy {
                    self.a += 1;
                }
            }
            0x08 => {} // NOP
            0x09 => { dad(&mut self.h, &mut self.l, &mut self.b, &mut self.c, &mut self.cc); }
            0x0a => { // LDAX B
                let target = combine_registers(self.b, self.c) as usize;
                self.a = self.memory[target];
            }
            0x0b => { dcx(&mut self.b, &mut self.c); }

            0x11 => {
                self.e = self.get_and_advance();
                self.d = self.get_and_advance();
            }
            0x13 => { inx(&mut self.d, &mut self.e); }
            0x1a => {
                let target = combine_registers(self.d, self.e) as usize;
                self.a = self.memory[target];
            }
            0x1b => { dcx(&mut self.d, &mut self.e); }

            0x21 => {
                self.l = self.get_and_advance();
                self.h = self.get_and_advance();
            }
            0x23 => { inx(&mut self.h, &mut self.l); }
            0x2b => { dcx(&mut self.h, &mut self.l); }

            0x31 => { self.sp = self.get_double(); }
            0x33 => { self.sp += 1; }

            0x77 => { self.memory[combine_registers(self.h, self.l) as usize] = self.a; }

            0x80 => { add(self.b, self); }
            0x81 => { add(self.c, self); }
            0x82 => { add(self.d, self); }
            0x83 => { add(self.e, self); }
            0x84 => { add(self.h, self); }
            0x85 => { add(self.l, self); }
            0x86 => { // ADD M
                let address = combine_registers(self.h, self.l) as usize;
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
                let address = combine_registers(self.h, self.l) as usize;
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
