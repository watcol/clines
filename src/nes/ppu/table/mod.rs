mod attr_table;
mod name_table;

use attr_table::AttrTable;
use name_table::NameTable;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mirroring {
    None,
    Horizontal,
    Vertical,
}

impl Default for Mirroring {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Table {
    mirroring: Mirroring,
    offset_x: u8,
    offset_y: u8,
    name_tables: [NameTable; 4],
    attr_tables: [AttrTable; 4],
}

impl Table {
    pub fn new(mirroring: Mirroring) -> Self {
        Self {
            mirroring,
            ..Default::default()
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x000..=0x3BF => self.name_tables[0].read(addr),
            0x3C0..=0x3FF => self.attr_tables[0].read(addr - 0x3C0),
            0x400..=0x7BF => self.name_tables[1].read(addr - 0x400),
            0x7C0..=0x7FF => self.attr_tables[1].read(addr - 0x7C0),
            0x800..=0xBBF => self.name_tables[2].read(addr - 0x800),
            0xBC0..=0xBFF => self.attr_tables[2].read(addr - 0xBC0),
            0xC00..=0xFBF => self.name_tables[3].read(addr - 0xC00),
            0xFC0..=0xFFF => self.attr_tables[3].read(addr - 0xFC0),
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x000..=0x3BF => self.name_tables[0].write(addr, value),
            0x3C0..=0x3FF => self.attr_tables[0].write(addr - 0x3C0, value),
            0x400..=0x7BF => self.name_tables[1].write(addr - 0x400, value),
            0x7C0..=0x7FF => self.attr_tables[1].write(addr - 0x7C0, value),
            0x800..=0xBBF => self.name_tables[2].write(addr - 0x800, value),
            0xBC0..=0xBFF => self.attr_tables[2].write(addr - 0xBC0, value),
            0xC00..=0xFBF => self.name_tables[3].write(addr - 0xC00, value),
            0xFC0..=0xFFF => self.attr_tables[3].write(addr - 0xFC0, value),
            _ => unreachable!(),
        }
    }

    pub fn get_background(&self, base_table: u8) -> &[[u8; 32]; 30] {
        &self.name_tables[base_table as usize].0
    }

    pub fn get_character_id(&self, base_table: u8, x: u8, y: u8) -> u8 {
        self.name_tables[base_table as usize].get_byte(x, y)
    }

    pub fn get_pallete_id(&self, base_table: u8, x: u8, y: u8) -> u8 {
        self.attr_tables[base_table as usize].get_pallete_id(x, y)
    }
}
