pub struct Apu {}

impl Apu {
    pub fn new() -> Apu {
        Apu {}
    }

    pub fn tick(self) {
        println!("Apu tick!");
    }
}
