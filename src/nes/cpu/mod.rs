mod bus;
mod instruction;
mod opecodes;
mod registers;

pub use instruction::{Addressing, Instruction, Opecode};
pub use opecodes::OPECODES;
pub use registers::{Registers, Status};

use super::{Pad, Ppu, Rom, Ui};
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
    pub fn run<U: Ui>(
        &mut self,
        rom: &Rom,
        ppu: &mut Ppu,
        pad: &mut Pad,
        ui: &U,
    ) -> anyhow::Result<u8> {
        let nmi_flag = if ppu.nmi {
            ppu.nmi = false;
            true
        } else {
            false
        };

        let mut bus = CpuBus::new(self, rom, ppu, pad, ui);
        if nmi_flag {
            bus.registers.P.break_mode = false;
            bus.push_word(bus.registers.PC)?;
            bus.push_byte(bus.registers.P.as_u8())?;
            bus.registers.P.interrupt = true;
            bus.registers.PC = bus.get_word(0xfffa);
        }
        let opecode = OPECODES[bus.increment_byte() as usize];
        opecode.exec(&mut bus)
    }

    pub fn reset<U: Ui>(&mut self, rom: &Rom, ppu: &mut Ppu, pad: &mut Pad, ui: &U) {
        let mut bus = CpuBus::new(self, rom, ppu, pad, ui);
        bus.registers.PC = bus.get_word(0xfffc);
    }
}
