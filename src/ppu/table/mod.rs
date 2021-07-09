mod attr_table;
mod mirroring;
mod name_table;

pub use mirroring::Mirroring;

use super::{RegisterIO, Registers};
use attr_table::AttrTable;
use name_table::NameTable;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Table {
    mirroring: Mirroring,
    w: bool,
    fine_x: u8,
    v_fine_y: u8,
    t_fine_y: u8,
    v_table: u8,
    v_x: u8,
    v_y: u8,
    t_table: u8,
    t_x: u8,
    t_y: u8,
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
            0x000..=0x3BF => self.name_tables[self.mirroring.mirror(0)].read(addr),
            0x3C0..=0x3FF => self.attr_tables[self.mirroring.mirror(0)].read(addr - 0x3C0),
            0x400..=0x7BF => self.name_tables[self.mirroring.mirror(1)].read(addr - 0x400),
            0x7C0..=0x7FF => self.attr_tables[self.mirroring.mirror(1)].read(addr - 0x7C0),
            0x800..=0xBBF => self.name_tables[self.mirroring.mirror(2)].read(addr - 0x800),
            0xBC0..=0xBFF => self.attr_tables[self.mirroring.mirror(2)].read(addr - 0xBC0),
            0xC00..=0xFBF => self.name_tables[self.mirroring.mirror(3)].read(addr - 0xC00),
            0xFC0..=0xFFF => self.attr_tables[self.mirroring.mirror(3)].read(addr - 0xFC0),
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x000..=0x3BF => self.name_tables[self.mirroring.mirror(0)].write(addr, value),
            0x3C0..=0x3FF => self.attr_tables[self.mirroring.mirror(0)].write(addr - 0x3C0, value),
            0x400..=0x7BF => self.name_tables[self.mirroring.mirror(1)].write(addr - 0x400, value),
            0x7C0..=0x7FF => self.attr_tables[self.mirroring.mirror(1)].write(addr - 0x7C0, value),
            0x800..=0xBBF => self.name_tables[self.mirroring.mirror(2)].write(addr - 0x800, value),
            0xBC0..=0xBFF => self.attr_tables[self.mirroring.mirror(2)].write(addr - 0xBC0, value),
            0xC00..=0xFBF => self.name_tables[self.mirroring.mirror(3)].write(addr - 0xC00, value),
            0xFC0..=0xFFF => self.attr_tables[self.mirroring.mirror(3)].write(addr - 0xFC0, value),
            _ => unreachable!(),
        }
    }

    pub fn sync_register(&mut self, registers: &Registers) {
        match registers.io {
            RegisterIO::WritePpuCtrl => {
                self.t_table = registers.ppu_ctrl.name_table;
            }
            RegisterIO::ReadPpuStatus => {
                self.w = false;
            }
            RegisterIO::WritePpuScroll if !self.w => {
                self.t_x = registers.ppu_scroll / 8;
                self.fine_x = registers.ppu_scroll % 8;
                self.w = true;
            }
            RegisterIO::WritePpuScroll => {
                self.t_y = registers.ppu_scroll / 8;
                self.t_fine_y = registers.ppu_scroll % 8;
                self.w = false;
            }
            RegisterIO::WritePpuAddr if !self.w => {
                self.t_fine_y = (registers.ppu_addr & 0x30) / 0x10;
                self.t_table = (registers.ppu_addr & 0x0c) / 0x04;
                self.t_y = (registers.ppu_addr & 0x03) * 0x08 + (self.t_y & 0x07);
                self.w = true;
            }
            RegisterIO::WritePpuAddr => {
                self.t_y = (self.t_y & 0x18) + ((registers.ppu_addr & 0xe0) / 0x20);
                self.t_x = registers.ppu_addr & 0x1f;
                self.v_fine_y = self.t_fine_y;
                self.v_table = self.t_table;
                self.v_y = self.t_y;
                self.v_x = self.t_x;
                self.w = false;
            }
            _ => {}
        }
    }

    pub fn sync_line(&mut self, line: u16) {
        if line == 0 {
            self.v_table = self.t_table;
            self.v_fine_y = self.t_fine_y;
            self.v_y = self.t_y;
        } else {
            self.v_table = (self.v_table & 0x02) + (self.t_table & 0x01);
        }
        self.v_x = self.t_x;
    }

    fn position(&self, x: u8, y: u8) -> (u8, u8, u8) {
        let (offset_x, offset_y) = match self.v_table {
            0 => (0, 0),
            1 => (256, 0),
            2 => (0, 240),
            3 => (256, 240),
            _ => unreachable!(),
        };
        let x = (offset_x + self.v_x as u16 * 8 + self.fine_x as u16 + x as u16) % 512;
        let y = (offset_y + self.v_y as u16 * 8 + self.v_fine_y as u16 + y as u16) % 480;
        let right = x >= 256;
        let bottom = y >= 240;
        let table = match (right, bottom) {
            (false, false) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (true, true) => 3,
        };
        (table, (x % 256) as u8, (y % 240) as u8)
    }

    pub fn pixel_position(&self, x: u8, y: u8) -> (u8, u8) {
        let (_, x, y) = self.position(x, y);
        (x % 8, y % 8)
    }

    pub fn get_character_id(&self, x: u8, y: u8) -> u8 {
        let (base_table, x, y) = self.position(x, y);
        self.name_tables[self.mirroring.mirror(base_table)].get_byte(x, y)
    }

    pub fn get_pallete_id(&self, x: u8, y: u8) -> u8 {
        let (base_table, x, y) = self.position(x, y);
        self.attr_tables[self.mirroring.mirror(base_table)].get_pallete_id(x, y)
    }
}
