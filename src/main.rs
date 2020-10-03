use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::env;
use std::time::Duration;

mod cartridge;
mod cpu;
mod display;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("You need to provide the rom filename as argument!!!");
        panic!();
    }

    let c = cartridge::Cartridge::new(&args[1]);
    let mut cpu = cpu::Cpu::initialize(&c);
    let mut d = display::Display::new();

    cpu.dump_memory();

    cpu.print_digit(8, 32, 10);

    'gameloop: loop {
        d.canvas.clear();

        for event in d.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'gameloop,
                _ => {}
            }
        }
        let keys: Vec<Keycode> = d
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut key_pressed: u8 = 0;

        if !keys.is_empty() && keys[0] == Keycode::Num1 {
            cpu.print_digit(11, 12, 0);
            match keys[0] {
                Keycode::Num1 => key_pressed = 1,
                Keycode::Num2 => key_pressed = 2,
                Keycode::Num3 => key_pressed = 3,
                Keycode::Num4 => key_pressed = 4,
                Keycode::Q => key_pressed = 5,
                Keycode::W => key_pressed = 6,
                Keycode::E => key_pressed = 7,
                Keycode::R => key_pressed = 8,
                Keycode::A => key_pressed = 9,
                Keycode::S => key_pressed = 10,
                Keycode::D => key_pressed = 11,
                Keycode::F => key_pressed = 12,
                Keycode::Z => key_pressed = 13,
                Keycode::X => key_pressed = 14,
                Keycode::C => key_pressed = 15,
                Keycode::V => key_pressed = 16,
                _ => println!("ERROR KEYCODE NOT RECOGNIZED"),
            }
        }

        cpu.emulate_cycle(key_pressed);

        d.set_frame(&cpu.gfx);

        d.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
