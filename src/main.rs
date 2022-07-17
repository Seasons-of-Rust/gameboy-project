mod apu;
mod cpu;
mod gameboy;
mod mmu;
mod ppu;

fn main() {
    pretty_env_logger::init();

    let mut gameboy = gameboy::Gameboy::new();
    loop {
        gameboy.tick();
    }
}
