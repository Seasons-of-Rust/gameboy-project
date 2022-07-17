pub struct Apu {}

impl Apu {
    pub fn new() -> Apu {
        Apu {}
    }

    pub fn tick(&mut self) {
        println!("Apu tick!");
    }
}
