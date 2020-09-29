use crate::{cartridge, display};

#[derive(Copy, Clone, Debug)]
pub struct Cpu {
    opcode: u8,         // Chip8 has 35 opcodes
    memory: [u8; 4096], // 4096 locations 8bit each
    v: [u8; 16],        // 15 registers + 16th "carry flag" register
    i: u16,             // Index Register
    pc: u16,            // Program Counter 0x00f - 0xfff
    sp: u16,            // Stack Pointer
    stack: [u16; 12],   //Stack
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

    pub fn emulate_cycle(&self) {
        // fetch opcode
        // decode opcode
        // execute opcode

        // update timers
    }

    // Loads cartridge data starting from RAM address 0x200
    pub fn load_cartridge(&mut self, c: &cartridge::Cartridge) {
        for (i, &el) in c.rom.iter().enumerate() {
            self.memory[0x200 + i] = el;
        }
    }
}
