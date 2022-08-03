use crate::mmu::Mmu;

#[allow(dead_code)]
pub struct Registers {
    pub register_af: u16,
    pub register_bc: u16,
    pub register_de: u16,
    pub register_hl: u16,
    pub register_sp: u16,
    pub register_pc: u16,
    pub zero_flag: u8,
    pub subtract_flag: u8,
    pub half_carry_flag: u8,
    pub carry_flag: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            register_af: 0x0000,
            register_bc: 0x0000,
            register_de: 0x0000,
            register_hl: 0x0000,
            register_sp: 0x0000,
            register_pc: 0x0000,
            zero_flag: 7,
            subtract_flag: 6,
            half_carry_flag: 5,
            carry_flag: 4,
        }
    }

    pub fn accessing_register(&mut self, item: &str) -> u16 {
        match item {
            "A" => self.register_af >> 8,
            "B" => self.register_bc >> 8,
            "D" => self.register_de >> 8,
            "H" => self.register_hl >> 8,
            "F" => self.register_af & 0xFF,
            "C" => self.register_bc & 0xFF,
            "E" => self.register_de & 0xFF,
            "L" => self.register_hl & 0xFF,
            "c" => self.register_af >> self.carry_flag & 1,
            "h" => self.register_af >> self.half_carry_flag & 1,
            "n" => self.register_af >> self.subtract_flag & 1,
            "z" => self.register_af >> self.zero_flag & 1,
            _ => panic!("There does not exist a register called, {}", item),
        }
    }
}

pub struct Cpu {
    memory: Mmu,
    running: bool,
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { 
            running: false, 
            memory: Mmu::new(),
            registers: Registers::new(),
        }
    }

    pub fn tick(&mut self) {
        log::debug!("Cpu tick!");
    }
}
