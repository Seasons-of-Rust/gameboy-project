mod apu;
mod cpu;
mod gameboy;
mod mmu;
mod ppu;

fn main() {
    let gameboy = gameboy::Gameboy::new();
    gameboy.tick();
}
