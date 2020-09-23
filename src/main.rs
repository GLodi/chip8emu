mod cpu;
mod display;
mod input;

fn main() {
    println!("Hello, world!");
    let mut k = input::Keyboard::new();
    let c = cpu::Cpu::initialize();
    let d = display::Display::new();

    display::Display::bo();
    loop {
        k.get_active_keys();
    }
}
