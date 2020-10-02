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

        if !keys.is_empty() && keys[0] == Keycode::Num1 {
            cpu.print_digit(11, 12, 0);
        }

        cpu.emulate_cycle();

        d.set_frame(&cpu.gfx);

        d.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
