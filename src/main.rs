use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod cpu;
mod display;
mod input;

fn main() {
    let mut k = input::Keyboard::new();

    let c = cpu::Cpu::initialize();
    let mut d = display::Display::new();

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
        // Do whatever and edit d.gfx to update the state of the next frame
        d.set_frame();

        d.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
