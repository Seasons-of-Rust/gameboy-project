pub struct Gameboy {
    cpu: bool,
}

impl std::fmt::Display for Gameboy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CPU: {}", self.cpu)
    }
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy { cpu: true }
    }

    pub fn run(self) {
        //TODO
    }
}
