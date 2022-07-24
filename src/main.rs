mod apu;
mod cpu;
mod gameboy;
mod mmu;
mod ppu;
mod screen;

fn main() {
    pretty_env_logger::init();

    let mut test = screen::Screen::new("test", 5);
    test.render();

    let mut gameboy = gameboy::Gameboy::new();
    loop {
        gameboy.tick();
    }
}
