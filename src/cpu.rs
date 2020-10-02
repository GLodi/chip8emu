use crate::{cartridge, display};
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Cpu {
    opcode: u8,         // Chip8 has 35 opcodes
    memory: [u8; 4096], // 4096 locations 8bit each
    v: [u8; 16],        // 15 registers + 16th "carry flag" register
    i: u16,             // Index Register
    pc: u16,            // Program Counter 0x00f - 0xfff
    sp: u16,            // Stack Pointer
    stack: [u16; 12],   // Stack
    delay_timer: u8,    // Counters count at 60hz.
    sound_timer: u8,    // When set above zero, they will count down.
    pub gfx: [bool; (display::WIDTH as usize) * (display::HEIGHT as usize)], // 2048 pixels monochrone (1-on, 0-off)
}

impl Cpu {
    // Initialize registers and memory once
    pub fn initialize(c: &cartridge::Cartridge) -> Cpu {
        let mut m: [u8; 4096] = [0; 4096];
        for i in 0..80 {
            m[i] = display::FONT_SET[i];
        }

        // Loads cartridge data starting from RAM address 0x200
        for (i, &el) in c.rom.iter().enumerate() {
            m[0x200 + i] = el;
        }

        Cpu {
            opcode: 0,
            memory: m,
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 12],
            delay_timer: 0,
            sound_timer: 0,
            gfx: [false; (display::WIDTH as usize) * (display::HEIGHT as usize)],
        }
    }

    // Reads memory from 0x200 until it finds an empty address
    pub fn dump_memory(&self) {
        for e in (0..(0x1000 - 0x200)).step_by(2) {
            let opcode: u16 = (self.memory[self.pc as usize + e] as u16) << 8
                | (self.memory[self.pc as usize + e + 1] as u16);
            if opcode == 0 {
                break;
            }
            println!("{:#0x}", opcode);
        }
    }

    pub fn print_digit(&mut self, digit: u8, offset_x: u8, offset_y: u8) {
        let a: &[u8] = &display::FONT_SET[(digit * 5) as usize..(digit * 5 + 5) as usize];
        let and_mask: u8 = 128;

        let mut x = 0;
        for i in offset_y..(offset_y + 5) {
            let mut temp = a[x as usize];
            x += 1;
            // println!("{:#01x}", a[i]);
            for e in offset_x..(offset_x + 8) {
                let last_bit: u8 = temp & and_mask;
                // println!("{}", last_bit);
                if last_bit == 128 {
                    self.gfx[i as usize * 64 + e as usize] = true;
                }
                temp = temp << 1;
            }
        }
    }

    pub fn emulate_cycle(&mut self) {
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[self.pc as usize + 1] as u16);
        let nnn: u16 = opcode & 0x0FFF;

        println!("{:#0x}", opcode);

        // All opcodes use the first 4 bits to specify what command
        // it is. The remaining 12 bits are arguments.
        match opcode & 0xF000 {
            0x0000 => self.op_0(nnn),
            0x1000 => self.op_1(nnn),
            0x2000 => self.op_2(nnn),
            0x3000 => self.op_3(nnn),
            0x4000 => self.op_4(nnn),
            0x5000 => self.op_5(nnn),
            0x6000 => self.op_6(nnn),
            0x7000 => self.op_7(nnn),
            0x8000 => self.op_8(nnn),
            0x9000 => self.op_9(nnn),
            0xA000 => self.op_a(nnn),
            0xB000 => self.op_b(nnn),
            0xC000 => self.op_c(nnn),
            0xD000 => self.op_d(nnn),
            0xE000 => self.op_e(nnn),
            0xF000 => self.op_f(nnn),
            _ => println!("ERROR OPCODE NOT RECOGNIZED"),
        }

        // update timers
    }

    fn op_0(&mut self, nnn: u16) {
        match nnn {
            // Clears the screen
            0x0E0 => {
                for i in 0..(display::HEIGHT + display::WIDTH) as usize {
                    self.gfx[i] = false;
                }
            }

            // Returns from a subroutine
            0x0EE => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }

            _ => panic!("ERROR OP_0 NOT RECOGNIZED"),
        }
    }

    // Jump to address NNN
    fn op_1(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    // Calls subroutine at NNN
    fn op_2(&mut self, nnn: u16) {
        self.stack[self.sp as usize] = self.pc as u16;
        self.sp += 1;
        self.pc = nnn;
    }

    // Skips the next instruction if VX equals NN.
    fn op_3(&mut self, nnn: u16) {
        let x: usize = ((nnn & 0x0F00) >> 8) as usize;
        let nn: u8 = (nnn & 0x00FF) as u8;
        if self.v[x] == nn {
            self.pc += 2;
        }
    }

    // Skips the next instruction if VX doesn't equal NN.
    fn op_4(&mut self, nnn: u16) {
        let x: usize = ((nnn & 0x0F00) >> 8) as usize;
        let nn: u8 = (nnn & 0x00FF) as u8;
        if self.v[x] != nn {
            self.pc += 2;
        }
    }

    // Skips the next instruction if VX equals VY.
    fn op_5(&mut self, nnn: u16) {
        let x: usize = ((nnn & 0x0F00) >> 8) as usize;
        let y: usize = ((nnn & 0x00F0) >> 4) as usize;
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }

    // Sets VX to NN.
    fn op_6(&mut self, nnn: u16) {
        let x: usize = ((nnn & 0x0F00) >> 8) as usize;
        let nn: u8 = (nnn & 0x00FF) as u8;
        self.v[x] = nn;
        self.pc += 2;
    }

    // Adds NN to VX. (Carry flag is not changed)
    fn op_7(&mut self, nnn: u16) {
        let x: usize = ((nnn & 0x0F00) >> 8) as usize;
        let nn: u8 = (nnn & 0x00FF) as u8;
        self.v[x] += nn;
        self.pc += 2;
    }

    fn op_8(&mut self, nnn: u16) {
        let x: usize = ((nnn & 0x0F00) >> 8) as usize;
        let y: usize = ((nnn & 0x00F0) >> 4) as usize;
        let z: usize = (nnn & 0x000F) as usize;
        match z {
            // Sets VX to the value of VY.
            0 => self.v[x] = self.v[y],

            // Sets VX to VX or VY. (Bitwise OR operation)
            1 => self.v[x] |= self.v[y],

            // Sets VX to VX and VY. (Bitwise AND operation)
            2 => self.v[x] &= self.v[y],

            // Sets VX to VX xor VY.
            3 => self.v[x] ^= self.v[y],

            // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
            4 => {
                let res = self.v[x] as u16 + self.v[y] as u16;
                if res > 0xFF {
                    self.v[15] = 1;
                } else {
                    self.v[15] = 0;
                }
                self.v[x] = res as u8;
            }

            // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
            5 => {
                let res = self.v[x] as i16 - self.v[y] as i16;
                if res < 0x0 {
                    self.v[15] = 0;
                } else {
                    self.v[15] = 1;
                }
                self.v[x] = self.v[x].wrapping_sub(self.v[y]);
            }

            // Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
            6 => {
                self.v[15] = self.v[x] & 0b00000001;
                self.v[x] >>= 1;
            }

            // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
            7 => {
                let res = self.v[y] as i16 - self.v[x] as i16;
                if res < 0x0 {
                    self.v[15] = 0;
                } else {
                    self.v[15] = 1;
                }
                self.v[x] = self.v[y].wrapping_sub(self.v[x]);
            }

            // Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
            14 => {
                self.v[15] = (self.v[x] & 0b10000000) >> 7;
                self.v[x] <<= 1;
            }

            _ => panic!("ERROR OP_8 NOT RECOGNIZED"),
        }
        self.pc += 2;
    }

    // Skips the next instruction if VX doesn't equal VY.
    fn op_9(&mut self, nnn: u16) {
        let x: usize = ((nnn & 0x0F00) >> 8) as usize;
        let y: usize = ((nnn & 0x00F0) >> 4) as usize;
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }

    // Sets I to the address NNN.
    fn op_a(&mut self, nnn: u16) {
        self.i = nnn;
        self.pc += 2;
    }

    // Jumps to the address NNN plus V0.
    fn op_b(&mut self, nnn: u16) {
        self.pc = self.v[0] as u16 + nnn;
    }

    // Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
    fn op_c(&mut self, nnn: u16) {
        let x: usize = ((nnn & 0x0F00) >> 8) as usize;
        let nn: u8 = (nnn & 0x00FF) as u8;

        let mut rng = rand::thread_rng();
        let r: u8 = rng.gen::<u8>();
        self.v[x] = r & nn;

        self.pc += 2;
    }

    // TODO:
    fn op_d(&mut self, nnn: u16) {}

    fn op_e(&mut self, nnn: u16) {
        let x: usize = ((nnn & 0x0F00) >> 8) as usize;
        let y: usize = ((nnn & 0x00F0) >> 4) as usize;
        let z: usize = (nnn & 0x000F) as usize;

        match y {
            // TODO:
            9 => {}

            // TODO:
            10 => {}

            _ => panic!("ERROR OP_E NOT RECOGNIZED"),
        }
    }

    fn op_f(&mut self, nnn: u16) {
        let x: usize = ((nnn & 0x0F00) >> 8) as usize;
        let y: usize = ((nnn & 0x00F0) >> 4) as usize;
        let z: usize = (nnn & 0x000F) as usize;

        match y {
            0 => match z {
                // Sets VX to the value of the delay timer.
                7 => self.v[x] = self.delay_timer,

                // TODO:
                10 => self.v[x] = self.delay_timer,

                _ => panic!("ERROR OP_F0 NOT RECOGNIZED"),
            },

            1 => match z {
                // Sets the delay timer to VX.
                5 => self.delay_timer = self.v[x],

                // Sets the sound timer to VX.
                8 => self.sound_timer = self.v[x],

                // Adds VX to I. VF is not affected.
                15 => self.i += self.v[x] as u16,

                _ => panic!("ERROR OP_F1 NOT RECOGNIZED"),
            },

            // TODO:
            2 => {}

            // TODO:
            3 => {}

            // TODO:
            5 => {}

            // TODO:
            6 => {}

            _ => panic!("ERROR OP_F NOT RECOGNIZED"),
        }
    }
}
