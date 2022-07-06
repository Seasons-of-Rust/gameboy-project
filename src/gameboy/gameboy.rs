use crate::cpu::CPU;

pub struct Gameboy {
    cpu: CPU,
}

impl std::fmt::Display for Gameboy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CPU: ")
    }
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy { cpu: CPU::new() }
    }

    pub fn run(self) {
        self.cpu.run();
    }
}
