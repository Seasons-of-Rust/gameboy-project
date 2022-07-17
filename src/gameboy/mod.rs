use crate::apu::Apu;
use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::ppu::Ppu;

#[allow(dead_code)]
pub struct Gameboy {
    apu: Apu,
    cpu: Cpu,
    mmu: Mmu,
    ppu: Ppu,
}

impl std::fmt::Display for Gameboy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Gameboy")
    }
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy {
            apu: Apu::new(),
            cpu: Cpu::new(),
            mmu: Mmu::new(),
            ppu: Ppu::new(),
        }
    }

    pub fn tick(&mut self) {
        self.apu.tick();
        self.cpu.tick();
        self.ppu.tick();
    }
}
