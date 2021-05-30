mod cpu;
mod ppu;
mod rom;

use crate::ui::Ui;
use cpu::Cpu;
use ppu::Ppu;
pub use rom::Rom;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nes<U> {
    rom: Rom,
    cpu: Cpu,
    ppu: Ppu,
    ui: U,
}

impl<U: Ui> Nes<U> {
    pub fn from_path<P: AsRef<Path>>(path: P, ui: U) -> anyhow::Result<Self> {
        Ok(Self {
            rom: Rom::from_path(path)?,
            cpu: Cpu::default(),
            ppu: Ppu::default(),
            ui,
        })
    }

    pub fn run_loop(&mut self) {
        self.run_loop_inner().unwrap_or_else(|e| {
            println!("{}", e);
        })
    }

    pub fn run_loop_inner(&mut self) -> anyhow::Result<()> {
        loop {
            let cycle = self.cpu.run(&self.rom, &mut self.ppu)?;
            if let Some(display) = self.ppu.run(&self.rom, cycle * 3) {
                self.ui.flush(&display)?;
            }
        }
    }
}
