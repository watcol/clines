use super::{AttrTable, NameTable, Pallete, Ppu, Registers};
use crate::nes::Rom;

pub struct PpuBus<'a> {
    pub registers: &'a mut Registers,
    ppu_addr: &'a mut u16,
    ppu_addr_tmp: &'a mut Option<u8>,
    pattern_table: &'a [u8],
    name_table0: &'a mut NameTable,
    attr_table0: &'a mut AttrTable,
    name_table1: &'a mut NameTable,
    attr_table1: &'a mut AttrTable,
    name_table2: &'a mut NameTable,
    attr_table2: &'a mut AttrTable,
    name_table3: &'a mut NameTable,
    attr_table3: &'a mut AttrTable,
    pallete: &'a mut Pallete,
}

impl<'a> PpuBus<'a> {
    pub fn new(ppu: &'a mut Ppu, rom: &'a Rom) -> Self {
        Self {
            registers: &mut ppu.registers,
            ppu_addr: &mut ppu.ppu_addr,
            ppu_addr_tmp: &mut ppu.ppu_addr_tmp,
            pattern_table: &rom.chr_rom,
            name_table0: &mut ppu.name_table0,
            attr_table0: &mut ppu.attr_table0,
            name_table1: &mut ppu.name_table1,
            attr_table1: &mut ppu.attr_table1,
            name_table2: &mut ppu.name_table2,
            attr_table2: &mut ppu.attr_table2,
            name_table3: &mut ppu.name_table3,
            attr_table3: &mut ppu.attr_table3,
            pallete: &mut ppu.pallete,
        }
    }

    pub fn get_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.pattern_table[addr as usize],
            0x2000..=0x23BF => self.name_table0.read(addr - 0x2000),
            0x23C0..=0x23FF => self.attr_table0.read(addr - 0x23C0),
            0x2400..=0x27BF => self.name_table1.read(addr - 0x2400),
            0x27C0..=0x27FF => self.attr_table1.read(addr - 0x27C0),
            0x2800..=0x2BBF => self.name_table2.read(addr - 0x2800),
            0x2BC0..=0x2BFF => self.attr_table2.read(addr - 0x2BC0),
            0x2C00..=0x2FBF => self.name_table3.read(addr - 0x2C00),
            0x2FC0..=0x2FFF => self.attr_table3.read(addr - 0x2FC0),
            0x3000..=0x33BF => self.name_table0.read(addr - 0x3000),
            0x33C0..=0x33FF => self.attr_table0.read(addr - 0x33C0),
            0x3400..=0x37BF => self.name_table1.read(addr - 0x3400),
            0x37C0..=0x37FF => self.attr_table1.read(addr - 0x37C0),
            0x3800..=0x3BBF => self.name_table2.read(addr - 0x3800),
            0x3BC0..=0x3BFF => self.attr_table2.read(addr - 0x3BC0),
            0x3C00..=0x3EFF => self.name_table3.read(addr - 0x3C00),
            0x3F00..=0x3FFF => self.pallete.read((addr - 0x3F00) % 0x20),
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
            0x2400..=0x27BF => self.name_table1.write(addr - 0x2400, value),
            0x27C0..=0x27FF => self.attr_table1.write(addr - 0x27C0, value),
            0x2800..=0x2BBF => self.name_table2.write(addr - 0x2800, value),
            0x2BC0..=0x2BFF => self.attr_table2.write(addr - 0x2BC0, value),
            0x2C00..=0x2FBF => self.name_table3.write(addr - 0x2C00, value),
            0x2FC0..=0x2FFF => self.attr_table3.write(addr - 0x2FC0, value),
            0x3000..=0x33BF => self.name_table0.write(addr - 0x3000, value),
            0x33C0..=0x33FF => self.attr_table0.write(addr - 0x33C0, value),
            0x3400..=0x37BF => self.name_table1.write(addr - 0x3400, value),
            0x37C0..=0x37FF => self.attr_table1.write(addr - 0x37C0, value),
            0x3800..=0x3BBF => self.name_table2.write(addr - 0x3800, value),
            0x3BC0..=0x3BFF => self.attr_table2.write(addr - 0x3BC0, value),
            0x3C00..=0x3EFF => self.name_table3.write(addr - 0x3C00, value),
            0x3F00..=0x3FFF => self.pallete.write((addr - 0x3F00) % 0x20, value),
            _ => {
                warn!("Illegal memory region: {:#06x}", addr);
            }
        }
    }

    pub fn sync_registers(&mut self) {
        if self.registers.ppu_addr_writed {
            self.registers.ppu_addr_writed = false;
            match self.ppu_addr_tmp {
                None => {
                    *self.ppu_addr_tmp = Some(self.registers.ppu_addr);
                }
                Some(tmp) => {
                    *self.ppu_addr = (*tmp as u16) * 0x100 + self.registers.ppu_addr as u16;
                    self.registers.ppu_data = self.get_byte(*self.ppu_addr);
                    *self.ppu_addr_tmp = None;
                }
            }
        }

        let offset = if self.registers.ppu_ctrl.ppu_mem_32 {
            0x20
        } else {
            0x01
        };

        if self.registers.ppu_data_writed {
            self.registers.ppu_data_writed = false;
            self.set_byte(*self.ppu_addr, self.registers.ppu_data);
            *self.ppu_addr += offset;
            self.registers.ppu_data = self.get_byte(*self.ppu_addr);
        }

        if self.registers.ppu_data_readed {
            self.registers.ppu_data_readed = false;
            *self.ppu_addr += offset;
            self.registers.ppu_data = self.get_byte(*self.ppu_addr);
        }
    }
}
