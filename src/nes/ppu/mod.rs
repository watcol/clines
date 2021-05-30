mod attr_table;
mod bus;
mod context;
mod name_table;
mod pallete;
mod registers;

use crate::nes::Rom;

use attr_table::AttrTable;
use bus::PpuBus;
use context::Context;
use name_table::NameTable;
use pallete::Pallete;
pub use registers::Registers;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Ppu {
    pub registers: Registers,
    ctx: Context,
    name_table0: NameTable,
    attr_table0: AttrTable,
    pallete: Pallete,
}

impl Ppu {
    pub fn run(&mut self, rom: &Rom) {
        let mut bus = PpuBus::new(self, rom);
        bus.sync_registers();
    }
}
