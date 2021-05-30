mod bus;
mod instruction;
mod opecodes;
mod registers;

pub use instruction::{Addressing, Instruction, Opecode};
pub use opecodes::OPECODES;
pub use registers::{Registers, Status};

use super::{Ppu, Rom};
use bus::CpuBus;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cpu {
    registers: Registers,
    wram: [u8; 0x800],
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            registers: Registers::default(),
            wram: [0; 0x800],
        }
    }
}
impl Cpu {
    pub fn run(&mut self, rom: &Rom, ppu: &mut Ppu) -> anyhow::Result<u8> {
        let mut bus = CpuBus::new(self, rom, ppu);
        let opecode = OPECODES[bus.increment_byte() as usize];
        opecode.exec(&mut bus)
    }

    pub fn reset(&mut self, rom: &Rom, ppu: &mut Ppu) {
        let mut bus = CpuBus::new(self, rom, ppu);
        bus.registers.PC = bus.get_word(0xfffc);
    }
}
