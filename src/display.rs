use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const BOX_SIZE: u32 = 10;

pub struct Display {
    pub event_pump: EventPump,
    pub canvas: WindowCanvas,
    pub gfx: [bool; (WIDTH as usize) * (HEIGHT as usize)], // 2048 pixels monochrone (1-on, 0-off)
}

impl Display {
    pub fn new() -> Display {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("chip8emu by glodi", WIDTH * 10, HEIGHT * 10)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        let event_pump = sdl_context.event_pump().unwrap();

        Display {
            event_pump: event_pump,
            canvas: canvas,
            gfx: [false; (WIDTH as usize) * (HEIGHT as usize)],
        }
    }

    pub fn get_frame(&mut self) {
        // edit self.canvas so that it reflects the current state of self.gfx
        for el in self.gfx.iter() {
            let color = if *el { Color::WHITE } else { Color::BLACK };
            self.canvas.set_draw_color(color);
            self.canvas.fill_rect(Rect::new(/*get coordinate*/));
        }
    }
}
