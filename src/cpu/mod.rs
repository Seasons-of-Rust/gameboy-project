use crate::mmu::Mmu;

#[allow(dead_code)]
pub struct Registers {
    register_AF: u16,
    register_BC: u16,
    register_DE: u16,
    register_HL: u16,
    register_SP: u16,
    register_PC: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            register_AF: 0x0000,
            register_BC: 0x0000,
            register_DE: 0x0000,
            register_HL: 0x0000,
            register_SP: 0x0000,
            register_PC: 0x0000,
        }
    }
}

impl Flags {
    
}

pub struct Flags {
    zero_flag: u8,
    subtract_flag: u8,
    half_carry_flag: u8,
    carry_flag: u8,
}

pub struct Cpu {
    memory: Mmu,
    running: bool,
    registers: Registers,
    flags: Flags,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { running: false, 
              memory: Mmu::new(),
        }
    }

    pub fn tick(&mut self) {
        log::debug!("Cpu tick!");
    }
}
