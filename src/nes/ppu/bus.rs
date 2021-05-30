use super::{AttrTable, NameTable, Ppu};
use crate::nes::Rom;

pub struct PpuBus<'a> {
    pattern_table: &'a [u8],
    name_table0: &'a mut NameTable,
    attr_table0: &'a mut AttrTable,
}

impl<'a> PpuBus<'a> {
    pub fn new(ppu: &'a mut Ppu, rom: &'a Rom) -> Self {
        Self {
            pattern_table: &rom.chr_rom,
            name_table0: &mut ppu.name_table0,
            attr_table0: &mut ppu.attr_table0,
        }
    }

    pub fn get_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.pattern_table[addr as usize],
            0x2000..=0x23BF => self.name_table0.read(addr - 0x2000),
            0x23C0..=0x23FF => self.attr_table0.read(addr - 0x23C0),
            0x2400..=0x2FFF => {
                warn!("Unimplemented memory region: {:#06x}", addr);
                0
            }
            0x3000..=0x33BF => self.name_table0.read(addr - 0x3000),
            0x33C0..=0x33FF => self.attr_table0.read(addr - 0x33C0),
            0x3400..=0x3EFF => {
                warn!("Unimplemented memory region: {:#06x}", addr);
                0
            }
            0x3F00..=0x3FFF => {
                warn!("Unimplemented memory region: {:#06x}", addr);
                0
            }
            _ => {
                warn!("Illegal memory region: {:#06x}", addr);
                0
            }
        }
    }

    pub fn set_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => warn!("CHR ROM is readonly."),
            0x2000..=0x23BF => self.name_table0.write(addr - 0x2000, value),
            0x23C0..=0x23FF => self.attr_table0.write(addr - 0x23C0, value),
            0x2400..=0x2FFF => warn!("Unimplemented memory region: {:#06x}", addr),
            0x3000..=0x33BF => self.name_table0.write(addr - 0x3000, value),
            0x33C0..=0x33FF => self.attr_table0.write(addr - 0x33C0, value),
            0x3400..=0x3EFF => {
                warn!("Unimplemented memory region: {:#06x}", addr);
            }
            0x3F00..=0x3FFF => {
                warn!("Unimplemented memory region: {:#06x}", addr);
            }
            _ => {
                warn!("Illegal memory region: {:#06x}", addr);
            }
        }
    }
}
