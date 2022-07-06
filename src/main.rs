mod cpu;
mod gameboy;

fn main() {
    let g: gameboy::Gameboy = gameboy::Gameboy::new();
    println!("{}", g);
}
