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
use std::sync::mpsc::{channel, TryRecvError};
use std::thread;

#[derive(Clone, Debug)]
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
        let (tx_time, rx_time) = channel();
        let (tx_stop, rx_stop) = channel();
        thread::spawn(move || {
            let time = std::time::Duration::from_micros(1_000_000 / 60);
            loop {
                spin_sleep::sleep(time);
                let _ = tx_time.send(());
                if matches!(rx_stop.try_recv(), Ok(()) | Err(TryRecvError::Disconnected)) {
                    break;
                }
            }
        });

        loop {
            let cycle = self
                .cpu
                .run(&self.rom, &mut self.ppu, &mut self.pad, &self.ui)?;
            if let Some(display) = self.ppu.run(&self.rom, cycle * 3) {
                rx_time.recv()?;
                self.ui.flush(&display)?;
            }

            if self.ui.button_pressed(Button::Reset) {
                self.reset();
            } else if self.ui.button_pressed(Button::Quit) {
                break;
            }
        }
        tx_stop.send(())?;
        Ok(())
    }
}
