mod cpu;
mod ppu;
mod rom;

use cpu::Registers;
use ppu::PpuRegisters;
pub use rom::Rom;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nes {
    rom: Rom,
    cpu_registers: Registers,
    ppu_registers: PpuRegisters,
    wram: [u8; 0x800],
}

impl Nes {
    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        Ok(Self {
            rom: Rom::from_path(path)?,
            cpu_registers: Registers::default(),
            ppu_registers: PpuRegisters::default(),
            wram: [0; 0x800],
        })
    }

    pub fn run_loop(&mut self) {
        self.run_loop_inner().unwrap_or_else(|e| {
            println!("{}", e);
        })
    }

    pub fn run_loop_inner(&mut self) -> anyhow::Result<()> {
        loop {
            cpu::run(self)?;
        }
    }
}
