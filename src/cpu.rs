use std::fmt;
use crate::ram::Ram;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
    prev_pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            prev_pc: 0,
        }
    }

    pub fn run_instruction(&mut self, ram: &mut Ram) {
        let lo = ram.read_byte(self.pc) as u16;
        let hi = ram.read_byte(self.pc+1) as u16;
        let instruction: u16 = (lo << 8) | hi;
        println!("lo: {:#X}  hi: {:#X} | instruction: {:#X}", lo, hi, instruction);

        let nnn = instruction & 0x0FFF;
        let kk  = (instruction & 0x00FF) as u8;
        let n   = (instruction & 0x000F) as u8;
        let x   = ((instruction & 0x0F00) >> 8) as u8;
        let y   = ((instruction & 0x00F0) >> 4) as u8;
        println!("nnn={:#X}, kk={:#X}, n={:#X}, x={:#X}, y={:#X}", nnn,kk,n,x,y);

        if self.pc == self.prev_pc {
            panic!("Please increase program counter");
        }
        self.prev_pc = self.pc;

        match (instruction & 0xF000) >> 12 {
            0x1 => {
                // jump to nnn
                self.pc = nnn;
            },
            0x3 => {
                // if Vx = kk
                let vx = self.read_reg_vx(x);
                if vx == kk {
                    self.increment_pc();
                }
                self.increment_pc();
            },
            0x6 => {
                // Vx = kk
                self.write_reg_vx(x, kk);
                self.increment_pc();
            },
            0x7 => {
                // Vx = Vx + kk
                let vx = self.read_reg_vx(x);
                self.write_reg_vx(x, vx.wrapping_add(kk));
                self.increment_pc();
            },
            0xA => {
                // I = nnn
                self.write_reg_i(nnn);
                self.increment_pc();
            },
            0xD => {
                // Draw sprite
                self.debug_draw_sprite(ram, x, y, n);
                self.increment_pc();
            },
            0xF => {
                match instruction & 0x00FF {
                    0x1E => {
                        // I = I + Vx
                        let new_i = self.i + self.read_reg_vx(x) as u16;
                        self.write_reg_i(new_i);
                        self.increment_pc();
                    }
                    _ => panic!("Unrecognized instruction at {:#X}: {:#X}", self.pc, instruction)
                }
            },
            _ => panic!("Unrecognized instruction at {:#X}: {:#X}", self.pc, instruction)
        }
    }

    fn write_reg_vx(&mut self, index: u8, value: u8) {
        self.vx[index as usize] = value;
    }

    fn read_reg_vx(&self, index: u8) -> u8 {
        self.vx[index as usize]
    }

    fn write_reg_i(&mut self, value: u16) {
        self.i = value;
    }

    fn debug_draw_sprite(&self, ram: &mut Ram, x: u8, y: u8, height: u8) {
        println!("Drawing sprite at ({}, {})", x, y);
        for h in 0..height {
            let mut byte = ram.read_byte(self.i + (h as u16));
            for _ in 0..8 {
                match (byte & 0b1000_0000) >> 7 {
                    0 => print!(" "),
                    1 => print!("#"),
                    _ => unreachable!()
                }
                byte = byte << 1;
            }
            print!("\n");
        }
        print!("\n");
    }

    fn increment_pc(&mut self) {
        self.pc += 2;
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n\tpc: {:#X}\n", self.pc);
        write!(f, "\tvx: ");
        for item in self.vx.iter() {
            write!(f, "{:#X}, ", *item);
        }
        write!(f, "\n\ti: {:#X}\n", self.i)
    }
}