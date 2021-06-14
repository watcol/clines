mod cpu;
mod pad;
mod ppu;
mod rom;

pub use pad::Button;
pub use ppu::Display;

use crate::ui::Ui;
use cpu::Cpu;
use pad::Pad;
use ppu::Ppu;
use rom::Rom;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nes<U> {
    rom: Rom,
    pad: Pad,
    cpu: Cpu,
    ppu: Ppu,
    ui: U,
}

impl<U: Ui> Nes<U> {
    pub fn from_path<P: AsRef<Path>>(path: P, ui: U) -> anyhow::Result<Self> {
        let mut res = Self {
            rom: Rom::from_path(path)?,
            cpu: Cpu::default(),
            pad: Pad::default(),
            ppu: Ppu::default(),
            ui,
        };
        res.reset();
        Ok(res)
    }

    pub fn reset(&mut self) {
        self.cpu
            .reset(&self.rom, &mut self.ppu, &mut self.pad, &self.ui);
    }

    pub fn run_loop(&mut self) {
        self.run_loop_inner().unwrap_or_else(|e| {
            println!("{}", e);
        })
    }

    pub fn run_loop_inner(&mut self) -> anyhow::Result<()> {
        loop {
            let cycle = self
                .cpu
                .run(&self.rom, &mut self.ppu, &mut self.pad, &self.ui)?;
            if let Some(display) = self.ppu.run(&self.rom, cycle * 3) {
                self.ui.flush(&display)?;
            }

            if self.ui.button_pressed(Button::Reset) {
                self.reset();
            } else if self.ui.button_pressed(Button::Quit) {
                break;
            }
        }
        Ok(())
    }
}
