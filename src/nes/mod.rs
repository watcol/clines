mod cpu;
mod ppu;
mod rom;

use cpu::Cpu;
use ppu::Ppu;
pub use rom::Rom;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nes {
    rom: Rom,
    cpu: Cpu,
    ppu: Ppu,
}

impl Nes {
    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        Ok(Self {
            rom: Rom::from_path(path)?,
            cpu: Cpu::default(),
            ppu: Ppu::default(),
        })
    }

    pub fn run_loop(&mut self) {
        self.run_loop_inner().unwrap_or_else(|e| {
            println!("{}", e);
        })
    }

    pub fn run_loop_inner(&mut self) -> anyhow::Result<()> {
        loop {
            self.cpu.run(&self.rom, &mut self.ppu)?;
        }
    }
}
