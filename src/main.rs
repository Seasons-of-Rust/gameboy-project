mod cpu;
mod gameboy;

fn main() {
    let g: gameboy::Gameboy = gameboy::Gameboy::new();
    g.run();
}
