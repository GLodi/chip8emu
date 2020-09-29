use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::env;
use std::time::Duration;

mod cartridge;
mod cpu;
mod display;
mod input;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("trying to open {:?}", &args[1]);

    let mut k = input::Keyboard::new();

    let cpu = cpu::Cpu::initialize();
    let mut d = display::Display::new();
    let c = cartridge::Cartridge::new(&args[1]);
    print!("{:?}", c.rom);

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
