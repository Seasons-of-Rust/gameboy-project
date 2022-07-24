pub struct Mmu {
    memory: [u8; 0x10000],
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            memory: [0; 0x10000],
        }
    }

    #[allow(dead_code)]
    pub fn mem_read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    #[allow(dead_code)]
    pub fn mem_write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_mem_write() {
        let mut mmu = super::Mmu::new();
        mmu.mem_write(0x0000, 0xFF);
        assert_eq!(mmu.memory[0x0000], 0xFF);

        mmu.mem_write(0x8000, 0xCC);
        assert_eq!(mmu.memory[0x8000], 0xCC);
    }

    #[test]
    fn check_mem_read_and_write() {
        let mut mmu = super::Mmu::new();
        mmu.mem_write(0x8000, 0xCC);
        assert_eq!(mmu.mem_read(0x8000), 0xCC);
    }
}
