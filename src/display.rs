use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 320;
const BOX_SIZE: u32 = 10;

pub struct Display {
    event_loop: EventLoop<()>,
    wininput: WinitInputHelper,
    window: WindowBuilder,
    pixels: Result<Pixels<Window>, Error>,
    gfx: [bool; 64 * 32], // 2048 pixels monochrone (1-on, 0-off)
}

impl Display {
    pub fn new() -> Display {
        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();
        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            WindowBuilder::new()
                .with_title("Hello Pixels")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture)
        };

        Display {
            event_loop: event_loop,
            wininput: input,
            window: window,
            pixels: pixels,
            gfx: [false; 64 * 32],
        }
    }
}
