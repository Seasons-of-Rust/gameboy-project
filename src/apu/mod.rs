pub struct Apu {}

impl Apu {
    pub fn new() -> Apu {
        Apu {}
    }

    pub fn tick(&mut self) {
        log::debug!("Apu tick!");
    }
}
