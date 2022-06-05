use std::fmt;
use crate::bus::Bus;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    prev_pc: u16,
    i: u16,
    ret_stack: Vec<u16>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            prev_pc: 0,
            i: 0,
            ret_stack: Vec::<u16>::new(),
        }
    }

    pub fn run_instruction(&mut self, bus: &mut Bus) {
        let lo = bus.ram_read_byte(self.pc) as u16;
        let hi = bus.ram_read_byte(self.pc+1) as u16;
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
            0x0 => {
                match kk {
                    0xE0 => {
                        // Clear screen
                        bus.clear_screen();
                        self.increment_pc();
                    },
                    0xEE => {
                        // Return from subroutine
                        let addr = self.ret_stack.pop().unwrap();
                        self.pc = addr;
                    },
                    _ => panic!("Unrecognized instruction at {:#X}: {:#X}", self.pc, instruction)
                }
            },
            0x1 => {
                // jump to nnn
                self.pc = nnn;
            },
            0x2 => {
                // call subroutine at nnn
                self.ret_stack.push(self.pc + 2);
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
            0x8 => {
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                match n {
                    0 => {
                        // Vx = Vy
                        self.write_reg_vx(x, vy);
                    },
                    1 => {
                        // Vx = Vx OR Vy
                        self.write_reg_vx(x, vx | vy);
                    },
                    2 => {
                        // Vx = Vx AND Vy
                        self.write_reg_vx(x, vx & vy);
                    },
                    3 => {
                        // Vx = Vx XOR Vy
                        self.write_reg_vx(x, vx ^ vy);
                    },
                    4 => {
                        // Vx = Vx + Vy, set VF = carry
                        let sum: u16 = vx as u16 + vy as u16;
                        self.write_reg_vx(x, sum as u8);
                        if sum > 0xFF {
                            self.write_reg_vx(0xF, 1);
                        } else {
                            self.write_reg_vx(0xF, 0);
                        }
                    },
                    5 => {
                        // Vx = Vx - Vy, set VF = NOT borrow
                        if vx > vy {
                            self.write_reg_vx(0xF, 1);
                        } else {
                            self.write_reg_vx(0xF, 0);
                        }
                        self.write_reg_vx(x, vx - vy);
                    },
                    6 => {
                        // Vx = Vx SHR 1
                        self.write_reg_vx(0xF, vy & 0x1);
                        self.write_reg_vx(y, vy >> 1);
                        self.write_reg_vx(x, vy >> 1);
                    },
                    _ => panic!("Unrecognized instruction at {:#X}: {:#X}", self.pc, instruction)
                }
                self.increment_pc();
            },
            0xA => {
                // I = nnn
                self.write_reg_i(nnn);
                self.increment_pc();
            },
            0xD => {
                // Draw sprite
                self.debug_draw_sprite(bus, x, y, n);
                self.increment_pc();
            },
            0xE => {
                let key = self.read_reg_vx(x);
                match kk {
                    0x9E => {
                        // if (key() == Vx) then skips to the next instruction
                        if bus.key_pressed(key) {
                            self.increment_pc();
                        }
                    },
                    0xA1 => {
                        // if (key() != Vx) then skips to the next instruction
                        if !bus.key_pressed(key) {
                            self.increment_pc();
                        }
                    },
                    _ => panic!("Unrecognized instruction at {:#X}: {:#X}", self.pc, instruction)
                }
                self.increment_pc();
            },
            0xF => {
                match kk {
                    0x07 => {
                        // Vx = delay timer value
                        self.write_reg_vx(x, bus.get_delay_timer());
                        self.increment_pc();
                    },
                    0x15 => {
                        // delay timer = Vx
                        bus.set_delay_timer(self.read_reg_vx(x));
                        self.increment_pc();
                    },
                    0x1E => {
                        // I = I + Vx
                        let vx = self.read_reg_vx(x);
                        let new_i = self.i + vx as u16;
                        self.write_reg_i(new_i);
                        self.increment_pc();
                    },
                    0x65 => {
                        // load v0 to vx from memory starting at I
                        for reg in 0..x+1 {
                            let value = bus.ram_read_byte(self.i + reg as u16);
                            self.write_reg_vx(reg, value);
                            self.increment_pc();
                        }
                    },
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

    fn debug_draw_sprite(&mut self, bus: &mut Bus, x: u8, y: u8, height: u8) {
        println!("Drawing sprite at ({}, {})", x, y);
        let mut should_set_vf = false;
        for y in 0..height {
            let byte = bus.ram_read_byte(self.i + (y as u16));
            if bus.debug_draw_sprite(byte, x, y) {
                should_set_vf = true;
            }
        }
        if should_set_vf {
            self.write_reg_vx(0xF, 1);
        }
        else {
            self.write_reg_vx(0xF, 0);
        }
        bus.present_screen();
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
        write!(f, "\n\ti: {:#X}\n", self.i);
        write!(f, "\tret_stack: ");
        for item in self.ret_stack.iter() {
            write!(f, "{:#X}, ", *item);
        }
        write!(f, "\n")
    }
}