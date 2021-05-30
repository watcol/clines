mod attr_table;
mod bus;
mod name_table;
mod pallete;
mod registers;

use crate::nes::Rom;
use crate::Display;

use attr_table::AttrTable;
use bus::PpuBus;
use name_table::NameTable;
use pallete::Pallete;
pub use registers::Registers;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Ppu {
    pub registers: Registers,
    cycle: u16,
    lines: u16,
    ppu_addr: u16,
    ppu_addr_tmp: Option<u8>,
    display: Display,
    name_table0: NameTable,
    attr_table0: AttrTable,
    pallete: Pallete,
}

impl Ppu {
    pub fn run(&mut self, rom: &Rom, cycle: u8) {
        PpuBus::new(self, rom).sync_registers();
        let _line = match self.add_cycle(cycle) {
            Some(line) => line,
            None => return,
        };
    }

    pub fn add_cycle(&mut self, cycle: u8) -> Option<u8> {
        self.cycle += cycle as u16;
        if self.cycle >= 314 {
            self.cycle -= 314;
            self.lines += 1;
        } else {
            return None;
        }

        if 10 <= self.lines && self.lines <= 242 && self.lines % 8 == 2 {
            Some(((self.lines - 2) / 8 - 1) as u8)
        } else if self.lines == 263 {
            self.lines = 0;
            None
        } else {
            None
        }
    }
}
