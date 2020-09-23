mod cpu;
mod display;
mod input;

fn main() {
    let mut k = input::Keyboard::new();

    let c = cpu::Cpu::initialize();
    display::Display::new();

    // loop {
    //     k.get_active_keys();
    // }
}
