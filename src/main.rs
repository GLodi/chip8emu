mod cpu;
mod input;

fn main() {
    println!("Hello, world!");
    let mut k = input::Keyboard::new();
    loop {
        k.get_key();
    }
}
