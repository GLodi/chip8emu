use super::*;

#[test]
fn test_init() {
    let cpu = Cpu::initialize(&vec![1, 2, 3]);
    assert_eq!(cpu.opcode, 0);
    assert_eq!(cpu.memory[0x200], 1);
    assert_eq!(cpu.memory[0x201], 2);
    assert_eq!(cpu.memory[0x202], 3);
    assert_eq!(cpu.i, 0);
    assert_eq!(cpu.pc, 0x200);
    assert_eq!(cpu.sp, 0);
    assert_eq!(cpu.v[0], 0);
}

#[test]
fn op_00e0() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.emulate_instruction(0x00e0, 0);
    for i in 0..(display::HEIGHT) as usize {
        for j in 0..(display::WIDTH) as usize {
            assert_eq!(cpu.gfx[i][j], 0);
        }
    }
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_00ee() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.sp = 3;
    cpu.stack[3] = 0x2222;
    cpu.emulate_instruction(0x00ee, 0);
    assert_eq!(cpu.pc, cpu.stack[cpu.sp as usize]);
    assert_eq!(cpu.sp, 2);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_1nnn() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.emulate_instruction(0x1234, 0);
    assert_eq!(cpu.pc, 0x0234);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_2nnn() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.emulate_instruction(0x2345, 0);
    assert_eq!(cpu.stack[0], 0x200);
    assert_eq!(cpu.pc, 0x0345);
    assert_eq!(cpu.sp, 1);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_3xnn() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[1] = 1;
    cpu.emulate_instruction(0x3101, 0);
    assert_eq!(cpu.pc, 0x204);
    assert_eq!(cpu.v[0xf], 0);
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.emulate_instruction(0x3101, 0);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_4xnn() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[1] = 1;
    cpu.emulate_instruction(0x4101, 0);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 0);
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.emulate_instruction(0x4101, 0);
    assert_eq!(cpu.pc, 0x204);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_5xy0() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[2] = 3;
    cpu.v[1] = 1;
    cpu.emulate_instruction(0x5210, 0);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 0);
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.emulate_instruction(0x5100, 0);
    assert_eq!(cpu.pc, 0x204);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_6xn0() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.emulate_instruction(0x6201, 0);
    assert_eq!(cpu.v[2], 1);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_7xnn() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.emulate_instruction(0x7344, 0);
    assert_eq!(cpu.v[3], 0x44);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_8xy0() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[4] = 3;
    cpu.emulate_instruction(0x8340, 0);
    assert_eq!(cpu.v[3], 0x3);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_8xy1() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[3] = 0b00000100;
    cpu.v[4] = 0b00000001;
    cpu.emulate_instruction(0x8341, 0);
    assert_eq!(cpu.v[3], 0b00000101);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_8xy2() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[3] = 0b00000100;
    cpu.v[4] = 0b00000101;
    cpu.emulate_instruction(0x8342, 0);
    assert_eq!(cpu.v[3], 0b00000100);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_8xy3() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[3] = 0b00000100;
    cpu.v[4] = 0b00000101;
    cpu.emulate_instruction(0x8343, 0);
    assert_eq!(cpu.v[3], 0b00000001);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_8xy4() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[3] = 0x12;
    cpu.v[4] = 0x5;
    cpu.emulate_instruction(0x8344, 0);
    assert_eq!(cpu.v[3], 0x17);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 0);
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[3] = 0xff;
    cpu.v[4] = 0x5;
    cpu.emulate_instruction(0x8344, 0);
    assert_eq!(cpu.v[3], 0x4);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 1);
}

#[test]
fn op_8xy5() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[3] = 0x12;
    cpu.v[4] = 0x5;
    cpu.emulate_instruction(0x8345, 0);
    assert_eq!(cpu.v[3], 0xd);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 1);
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[3] = 0x2;
    cpu.v[4] = 0x3;
    cpu.emulate_instruction(0x8345, 0);
    assert_eq!(cpu.v[3], 0xff);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_8x06() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[6] = 0b00000101;
    cpu.emulate_instruction(0x8606, 0);
    assert_eq!(cpu.v[6], 0b00000010);
    assert_eq!(cpu.v[0xf], 1);
    assert_eq!(cpu.pc, 0x202);
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[6] = 0b00000100;
    cpu.emulate_instruction(0x8606, 0);
    assert_eq!(cpu.v[6], 0b00000010);
    assert_eq!(cpu.v[0xf], 0);
    assert_eq!(cpu.pc, 0x202);
}

#[test]
fn op_8xy7() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[3] = 0x5;
    cpu.v[4] = 0x12;
    cpu.emulate_instruction(0x8347, 0);
    assert_eq!(cpu.v[3], 0xd);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 1);
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[3] = 0x3;
    cpu.v[4] = 0x2;
    cpu.emulate_instruction(0x8347, 0);
    assert_eq!(cpu.v[3], 0xff);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_8x0e() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[6] = 0b10000101;
    cpu.emulate_instruction(0x860e, 0);
    assert_eq!(cpu.v[6], 0b00001010);
    assert_eq!(cpu.v[0xf], 1);
    assert_eq!(cpu.pc, 0x202);
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[6] = 0b00000100;
    cpu.emulate_instruction(0x860e, 0);
    assert_eq!(cpu.v[6], 0b00001000);
    assert_eq!(cpu.v[0xf], 0);
    assert_eq!(cpu.pc, 0x202);
}

#[test]
fn op_9xy0() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[3] = 1;
    cpu.v[4] = 5;
    cpu.emulate_instruction(0x9340, 0);
    assert_eq!(cpu.pc, 0x204);
    assert_eq!(cpu.v[0xf], 0);
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[3] = 1;
    cpu.v[4] = 1;
    cpu.emulate_instruction(0x9340, 0);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_annn() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.emulate_instruction(0xa234, 0);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.i, 0x234);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_bnnn() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[0] = 0x4;
    cpu.emulate_instruction(0xb234, 0);
    assert_eq!(cpu.pc, 0x238);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_cxnn() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[0] = 0x4;
    cpu.emulate_instruction(0xc000, 0);
    assert_eq!(cpu.v[0], 0);
    assert_eq!(cpu.v[0xf], 0);
    cpu.emulate_instruction(0xc00f, 0);
    assert_eq!(cpu.v[0] & 0xf0, 0);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_dxyn() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.i = 0;
    cpu.memory[0] = 0b11111111;
    cpu.memory[1] = 0b00000000;
    cpu.gfx[0][0] = 1;
    cpu.gfx[0][1] = 0;
    cpu.gfx[1][0] = 1;
    cpu.gfx[1][1] = 0;
    cpu.v[0] = 0;
    cpu.emulate_instruction(0xd002, 0);

    assert_eq!(cpu.gfx[0][0], 0);
    assert_eq!(cpu.gfx[0][1], 1);
    assert_eq!(cpu.gfx[1][0], 1);
    assert_eq!(cpu.gfx[1][1], 0);
    assert_eq!(cpu.v[0x0f], 1);
    assert_eq!(cpu.pc, 0x202);
}

#[test]
fn op_ex9e() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[4] = 0x4;
    cpu.emulate_instruction(0xe49e, 6);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.wait_key, false);
    assert_eq!(cpu.v[0xf], 0);
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[4] = 0x6;
    cpu.emulate_instruction(0xe49e, 6);
    assert_eq!(cpu.pc, 0x204);
    assert_eq!(cpu.wait_key, false);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_exa1() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[2] = 0x4;
    cpu.emulate_instruction(0xe2a1, 6);
    assert_eq!(cpu.pc, 0x204);
    assert_eq!(cpu.wait_key, false);
    assert_eq!(cpu.v[0xf], 0);
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[2] = 0x6;
    cpu.emulate_instruction(0xe2a1, 6);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.wait_key, false);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_fx07() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.delay_timer = 0x14;
    cpu.emulate_instruction(0xf407, 0);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v[4], 0x13);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_fx0a() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.emulate_instruction(0xf40a, 0);
    assert_eq!(cpu.pc, 0x200);
    assert_eq!(cpu.wait_key, true);
    assert_eq!(cpu.v[0xf], 0);

    // check that it does nothing if it doesn't receive a key
    cpu.emulate_instruction(0x1fff, 0);
    assert_eq!(cpu.pc, 0x200);
    assert_eq!(cpu.wait_key, true);

    cpu.emulate_instruction(0xf40a, 3);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.wait_key, false);
    assert_eq!(cpu.v[4], 3);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_fx15() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[4] = 20;
    cpu.emulate_instruction(0xf415, 0);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.delay_timer, 20);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_fx18() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[4] = 20;
    cpu.emulate_instruction(0xf418, 0);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.sound_timer, 20);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_fx1e() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[4] = 20;
    cpu.i = 4;
    cpu.emulate_instruction(0xf41e, 0);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.i, 24);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_fx29() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[4] = 2;
    cpu.emulate_instruction(0xf429, 0);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.i, 5 * 2);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_fx33() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.v[5] = 123;
    cpu.i = 1000;
    cpu.emulate_instruction(0xf533, 0);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.i, 1000);
    assert_eq!(cpu.memory[1000], 1);
    assert_eq!(cpu.memory[1001], 2);
    assert_eq!(cpu.memory[1002], 3);
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_fx55() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.i = 1000;
    cpu.emulate_instruction(0xf553, 0);
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.i, 1000);
    for i in 0..16 {
        assert_eq!(cpu.memory[1000 + i as usize], cpu.v[i]);
    }
    assert_eq!(cpu.v[0xf], 0);
}

#[test]
fn op_fx65() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    for i in 0..16 {
        cpu.memory[1000 + i as usize] = i as u8;
    }
    cpu.i = 1000;
    cpu.emulate_instruction(0xff65, 0);
    assert_eq!(cpu.i, 1000);
    for i in 0..16 {
        assert_eq!(cpu.v[i as usize], cpu.memory[1000 + i as usize]);
    }
    assert_eq!(cpu.pc, 0x202);
}

#[test]
fn test_timers() {
    let mut cpu = Cpu::initialize(&vec![1, 1, 1]);
    cpu.delay_timer = 200;
    cpu.sound_timer = 100;
    cpu.emulate_instruction(0x1234, 0);
    assert_eq!(cpu.delay_timer, 199);
    assert_eq!(cpu.sound_timer, 99);
}
