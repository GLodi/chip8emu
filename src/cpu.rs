use crate::{cartridge, display};

#[derive(Copy, Clone, Debug)]
pub struct Cpu {
    opcode: u8,         // Chip8 has 35 opcodes
    memory: [u8; 4096], // 4096 locations 8bit each
    v: [u8; 16],        // 15 registers + 16th "carry flag" register
    i: u16,             // Index Register
    pc: usize,          // Program Counter 0x00f - 0xfff
    sp: usize,          // Stack Pointer
    stack: [u16; 12],   // Stack
    delay_timer: u8,    // Counters count at 60hz.
    sound_timer: u8,    // When set above zero, they will count down.
}

impl Cpu {
    // Initialize registers and memory once
    pub fn initialize() -> Cpu {
        let mut m: [u8; 4096] = [0; 4096];
        for i in 0..80 {
            m[i] = display::FONT_SET[i];
        }

        // Clear display
        // Clear stack
        // Clear registers
        // Clear memory

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
        }
    }

    // Reads memory from 0x200 until it finds an empty address
    pub fn dump_memory(&self) {
        for e in (0..(0x1000 - 0x200)).step_by(2) {
            let opcode: u16 =
                (self.memory[self.pc + e] as u16) << 8 | (self.memory[self.pc + e + 1] as u16);
            if opcode == 0 {
                break;
            }
            println!("{:#0x}", opcode);
        }
    }

    // Loads cartridge data starting from RAM address 0x200
    pub fn load_cartridge(&mut self, c: &cartridge::Cartridge) {
        for (i, &el) in c.rom.iter().enumerate() {
            self.memory[0x200 + i] = el;
        }
    }

    pub fn emulate_cycle(&mut self) {
        let opcode: u16 = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);

        // All opcodes use the first 4 bits to specify what command
        // it is. The remaining 12 bits are arguments.
        match opcode & 0xF000 {
            0x1000 => self.op_1(opcode),
            0x2000 => self.op_2(opcode),
            0x3000 => self.op_3(opcode),
            0x4000 => self.op_4(opcode),
            0x5000 => self.op_5(opcode),
            0x6000 => self.op_6(opcode),
            0x7000 => self.op_7(opcode),
            0x8000 => self.op_8(opcode),
            0x9000 => self.op_9(opcode),
            0xA000 => self.op_a(opcode),
            0xB000 => self.op_b(opcode),
            0xC000 => self.op_c(opcode),
            0xD000 => self.op_d(opcode),
            0xE000 => self.op_e(opcode),
            0xF000 => self.op_f(opcode),
            _ => println!("ERROR OPCODE NOT RECOGNIZED"),
        }

        // update timers
    }

    fn op_1(&mut self, opcode: u16) {
        println!("{:#0x}", opcode);
    }

    fn op_2(&mut self, opcode: u16) {
        println!("{:#0x}", opcode);
        self.stack[self.sp] = self.pc as u16;
        self.sp += 1;
        self.pc = (opcode & 0x0FFF) as usize;
    }

    fn op_3(&mut self, opcode: u16) {
        println!("{:#0x}", opcode);
    }

    fn op_4(&mut self, opcode: u16) {
        println!("{:#0x}", opcode);
    }

    fn op_5(&mut self, opcode: u16) {
        println!("{:#0x}", opcode);
    }

    fn op_6(&mut self, opcode: u16) {
        println!("{:#0x}", opcode);
        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
        let nn: u8 = (opcode & 0x00FF) as u8;
        self.v[x] = nn;
    }

    fn op_7(&mut self, opcode: u16) {
        println!("{:#0x}", opcode);
        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
        let nn: u8 = (opcode & 0x00FF) as u8;
        self.v[x] += nn;
    }

    fn op_8(&mut self, opcode: u16) {
        println!("{:#0x}", opcode);
    }

    fn op_9(&mut self, opcode: u16) {
        println!("{:#0x}", opcode);
    }

    fn op_a(&mut self, opcode: u16) {
        println!("{:#0x}", opcode);
        self.i = opcode;
        self.pc += 2;
    }

    fn op_b(&mut self, opcode: u16) {
        println!("{:#0x}", opcode);
    }

    fn op_c(&mut self, opcode: u16) {
        println!("{:#0x}", opcode);
    }

    fn op_d(&mut self, opcode: u16) {
        println!("{:#0x}", opcode);
    }

    fn op_e(&mut self, opcode: u16) {
        println!("{:#0x}", opcode);
    }

    fn op_f(&mut self, opcode: u16) {
        println!("{:#0x}", opcode);
    }
}
