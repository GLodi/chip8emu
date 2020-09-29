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
            gfx: [false; (WIDTH as usize) * (HEIGHT as usize)],
        }
    }

    // edit self.canvas so that it reflects the current state of self.gfx
    pub fn set_frame(&mut self) {
        for (n, el) in self.gfx.to_vec().iter().enumerate() {
            let color = if *el { Color::WHITE } else { Color::BLACK };
            self.canvas.set_draw_color(color);
            let _ = self.canvas.fill_rect(Rect::new(
                ((n as i32) % WIDTH as i32) * BOX_SIZE as i32,
                ((n as i32) / WIDTH as i32) * BOX_SIZE as i32,
                BOX_SIZE,
                BOX_SIZE,
            ));
        }
    }

    pub fn print_digit(&mut self, digit: u8, offset_x: u8, offset_y: u8) {
        let a: &[u8] = &FONT_SET[(digit * 5) as usize..(digit * 5 + 5) as usize];
        let and_mask: u8 = 128;

        let mut x = 0;
        for i in offset_y..(offset_y + 5) {
            let mut temp = a[x as usize];
            x += 1;
            // println!("{:#01x}", a[i]);
            for e in offset_x..(offset_x + 8) {
                let last_bit: u8 = temp & and_mask;
                // println!("{}", last_bit);
                if last_bit == 128 {
                    self.gfx[i as usize * 64 + e as usize] = true;
                }
                temp = temp << 1;
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
