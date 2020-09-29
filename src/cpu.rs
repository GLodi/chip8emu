use crate::cartridge;

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
            m[i] = FONT_SET[i];
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

// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#font
//
// Font set defines how to draw each digit.
const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
