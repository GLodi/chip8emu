use rand::Rng;
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::display;

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
    wait_key: bool,     // CPU waiting for key press
    pub gfx: [[u8; (display::WIDTH as usize)]; (display::HEIGHT as usize)], // 2048 pixels monochrone (1-on, 0-off)
}

impl Cpu {
    // Initialize registers and memory once
    pub fn initialize(c: &Vec<u8>) -> Cpu {
        let mut m: [u8; 4096] = [0; 4096];
        for i in 0..80 {
            m[i] = display::FONT_SET[i];
        }

        // Loads cartridge data starting from RAM address 0x200
        let mut last = 0;
        for (i, &el) in c.iter().enumerate() {
            m[0x200 + i] = el;
            if 0x200 + i > last {
                last = 0x200 + i;
            }
        }
        println!("last: {:#0x}", last);

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
            wait_key: false,
            gfx: [[0; (display::WIDTH as usize)]; (display::HEIGHT as usize)],
        }
    }

    // Reads memory from 0x200 until it finds an empty address
    pub fn dump_memory(&self, to_file: bool) {
        for e in (0..(0x1000 - 0x200)).step_by(2) {
            let opcode: u16 = (self.memory[self.pc as usize + e] as u16) << 8
                | (self.memory[self.pc as usize + e + 1] as u16);
            if opcode == 0 {
                break;
            }

            if to_file {
                let a = format!("{:#0x}", opcode);
                write_to_file(&a[..]);
            } else {
                println!("{:#0x}", opcode);
            }
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
                    self.gfx[i as usize][e as usize] = 1;
                }
                temp = temp << 1;
            }
        }
    }

    pub fn print_state(&self, key: u8) {
        let flag = if self.wait_key { 1 } else { 0 };
        println!(
            "i: {:#0x}  pc: {:#0x}  sp:{:#0x}  flag:{:#0x}  key:{}",
            self.i, self.pc, self.sp, flag, key
        );
        print!("v: ");
        for i in 0..15 {
            print!(" {:#0x}", self.v[i]);
        }
        println!();
        print!("stack: ");
        for i in 0..12 {
            print!(" {:#0x}", self.stack[i]);
        }
        println!();
        print!("mem[i]: ");
        for i in 0..16 {
            print!(" {:#0x}", self.memory[self.i as usize + i]);
        }
        println!();
        println!();
    }

    pub fn emulate_instruction(&mut self, opcode: u16, key: u8) {
        let op1: u8 = (opcode >> 8) as u8;
        let op2: u8 = (opcode & 0x00FF) as u8;
        self.memory[self.pc as usize] = op1;
        self.memory[self.pc as usize + 1] = op2;
        self.emulate_cycle(key);
    }

    pub fn emulate_cycle(&mut self, key: u8) {
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[self.pc as usize + 1] as u16);
        let nnn: u16 = opcode & 0x0FFF;

        println!("opcode: {:#0x}", opcode);

        if !self.wait_key || (self.wait_key && key != 0) {
            // Update timers
            if !self.wait_key {
                if self.delay_timer > 0 {
                    self.delay_timer -= 1;
                }
                if self.sound_timer > 0 {
                    self.sound_timer -= 1;
                }
            }

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
                0xE000 => self.op_e(nnn, key),
                0xF000 => self.op_f(nnn, key),
                _ => panic!("ERROR OPCODE NOT RECOGNIZED"),
            }
        } else {
            return;
        }
    }

    fn op_0(&mut self, nnn: u16) {
        match nnn {
            // Clears the screen
            0x0E0 => {
                for i in 0..(display::HEIGHT) as usize {
                    for j in 0..(display::WIDTH) as usize {
                        self.gfx[i][j] = 0;
                    }
                }
                self.pc += 2;
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
        self.stack[self.sp as usize] = self.pc as u16 + 2;
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
        self.pc += 2;
    }

    // Skips the next instruction if VX doesn't equal NN.
    fn op_4(&mut self, nnn: u16) {
        let x: usize = ((nnn & 0x0F00) >> 8) as usize;
        let nn: u8 = (nnn & 0x00FF) as u8;
        if self.v[x] != nn {
            self.pc += 2;
        }
        self.pc += 2;
    }

    // Skips the next instruction if VX equals VY.
    fn op_5(&mut self, nnn: u16) {
        let x: usize = ((nnn & 0x0F00) >> 8) as usize;
        let y: usize = ((nnn & 0x00F0) >> 4) as usize;
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
        self.pc += 2;
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
        self.v[x] = self.v[x].wrapping_add(nn);
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
                    self.v[0xf] = 1;
                } else {
                    self.v[0xf] = 0;
                }
                self.v[x] = res as u8;
            }

            // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
            5 => {
                let res = self.v[x] as i16 - self.v[y] as i16;
                if res < 0x0 {
                    self.v[0xf] = 0;
                } else {
                    self.v[0xf] = 1;
                }
                self.v[x] = self.v[x].wrapping_sub(self.v[y]);
            }

            // Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
            6 => {
                self.v[0xf] = self.v[x] & 0b00000001;
                self.v[x] >>= 1;
            }

            // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
            7 => {
                let res = self.v[y] as i16 - self.v[x] as i16;
                if res < 0x0 {
                    self.v[0xf] = 0;
                } else {
                    self.v[0xf] = 1;
                }
                self.v[x] = self.v[y].wrapping_sub(self.v[x]);
            }

            // Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
            14 => {
                self.v[0xf] = (self.v[x] & 0b10000000) >> 7;
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
        self.pc += 2;
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

    // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N+1 pixels.
    fn op_d(&mut self, nnn: u16) {
        let x: usize = ((nnn & 0x0F00) >> 8) as usize;
        let y: usize = ((nnn & 0x00F0) >> 4) as usize;
        let n: usize = (nnn & 0x000F) as usize;

        self.v[0xf] = 0;

        for byte in 0..n {
            let y: u8 = (self.v[y] + byte as u8) % display::HEIGHT as u8;
            for b in 0..8 {
                let x: u8 = (self.v[x] + b as u8) % display::WIDTH as u8;
                let color: u8 = (self.memory[self.i as usize + byte] >> (7 - b)) & 1;
                self.v[0x0f] |= color & self.gfx[y as usize][x as usize];
                self.gfx[y as usize][x as usize] ^= color;
            }
        }

        self.pc += 2;
    }

    fn op_e(&mut self, nnn: u16, key: u8) {
        let x: usize = ((nnn & 0x0F00) >> 8) as usize;
        let y: usize = ((nnn & 0x00F0) >> 4) as usize;

        match y {
            // Skips the next instruction if the key stored in VX is pressed.
            9 => {
                if self.v[x] == key {
                    self.pc += 2;
                }
            }

            // Skips the next instruction if the key stored in VX isn't pressed.
            10 => {
                if self.v[x] != key {
                    self.pc += 2;
                }
            }

            _ => panic!("ERROR OP_E NOT RECOGNIZED"),
        }

        self.pc += 2;
    }

    fn op_f(&mut self, nnn: u16, key_pressed: u8) {
        let x: usize = ((nnn & 0x0F00) >> 8) as usize;
        let y: usize = ((nnn & 0x00F0) >> 4) as usize;
        let z: usize = (nnn & 0x000F) as usize;

        match y {
            0 => match z {
                // Sets VX to the value of the delay timer.
                7 => {
                    self.v[x] = self.delay_timer;
                    self.pc += 2;
                }

                // A key press is awaited, and then stored in VX.
                10 => {
                    println!("pressed: {}", key_pressed);
                    self.wait_key = true;
                    if key_pressed != 0 {
                        self.v[x] = key_pressed;
                        self.wait_key = false;
                        self.pc += 2;
                    }
                }

                _ => panic!("ERROR OP_F0 NOT RECOGNIZED"),
            },

            1 => match z {
                // Sets the delay timer to VX.
                5 => {
                    self.delay_timer = self.v[x];
                    self.pc += 2;
                }

                // Sets the sound timer to VX.
                8 => {
                    self.sound_timer = self.v[x];
                    self.pc += 2;
                }

                // Adds VX to I. VF is not affected.
                14 => {
                    self.i += self.v[x] as u16;
                    self.pc += 2;
                }

                _ => panic!("ERROR OP_F1 NOT RECOGNIZED"),
            },

            // Sets I to the location of the sprite for the character in VX.
            2 => {
                self.i = (self.v[x] as u16) * 5;
                self.pc += 2;
            }

            // Stores the binary-coded decimal representation of VX.
            3 => {
                self.memory[self.i as usize] = self.v[x] / 100;
                self.memory[self.i as usize + 1] = (self.v[x] % 100) / 10;
                self.memory[self.i as usize + 2] = self.v[x] % 10;
                self.pc += 2;
            }

            // Stores V0 to VX (including VX) in memory starting at address I.
            5 => {
                for i in 0..x + 1 {
                    self.memory[self.i as usize + i] = self.v[i];
                }
                self.pc += 2;
            }

            // Fills V0 to VX (including VX) with values from memory starting at address I.
            6 => {
                for i in 0..x + 1 {
                    self.v[i] = self.memory[self.i as usize + i];
                }
                self.pc += 2;
            }

            _ => panic!("ERROR OP_F NOT RECOGNIZED"),
        }
    }
}

fn write_to_file(s: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("instructions.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", s) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

#[cfg(test)]
#[path = "./cpu_tests.rs"]
mod cpu_tests;
