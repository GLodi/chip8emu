use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::env;
use std::time::Duration;

mod cartridge;
mod cpu;
mod display;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut cpu = cpu::Cpu::initialize();
    let mut d = display::Display::new();
    let c = cartridge::Cartridge::new(&args[1]);

    cpu.load_cartridge(&c);

    d.print_digit(8, 66, 10);

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

        // Do whatever, edit d.gfx to update the state of the next frame
        // and set_frame to reflect changes onto the screen

        d.set_frame();
        d.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
