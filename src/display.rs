use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;
const BOX_SIZE: u32 = 10;

pub struct Display {
    pub event_pump: EventPump,
    pub canvas: WindowCanvas,
}

impl Display {
    pub fn new() -> Display {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("chip8emu by glodi", WIDTH * BOX_SIZE, HEIGHT * BOX_SIZE)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let event_pump = sdl_context.event_pump().unwrap();

        Display {
            event_pump: event_pump,
            canvas: canvas,
        }
    }

    // edit self.canvas so that it reflects the current state of gfx
    pub fn set_frame(&mut self, &gfx: &[[u8; (WIDTH as usize)]; (HEIGHT as usize)]) {
        for (n, el) in gfx.to_vec().iter().enumerate() {
            for (s, &c) in el.iter().enumerate() {
                let x = (s as u32) * BOX_SIZE;
                let y = (n as u32) * BOX_SIZE;

                if c == 0 {
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                } else {
                    self.canvas.set_draw_color(Color::RGB(0, 250, 0));
                }

                let _ = self
                    .canvas
                    .fill_rect(Rect::new(x as i32, y as i32, BOX_SIZE, BOX_SIZE));
            }
        }
    }
}

// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#font
//
// Font set defines how to draw each digit.
pub const FONT_SET: [u8; 80] = [
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
