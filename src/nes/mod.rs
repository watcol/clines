mod cpu;
mod rom;

use cpu::Registers;
pub use rom::Rom;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nes {
    rom: Rom,
    cpu_registers: Registers,
    wram: [u8; 0x800],
}

impl Nes {
    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        Ok(Self {
            rom: Rom::from_path(path)?,
            cpu_registers: Registers::default(),
            wram: [0; 0x800],
        })
    }

    pub fn run_loop(&mut self) {
        loop {
            cpu::run(self);
        }
    }
}
