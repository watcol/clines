use super::{AttrTable, Context, NameTable, Pallete, Ppu, Registers};
use crate::nes::Rom;

pub struct PpuBus<'a> {
    pub registers: &'a mut Registers,
    pub ctx: &'a mut Context,
    pattern_table: &'a [u8],
    name_table0: &'a mut NameTable,
    attr_table0: &'a mut AttrTable,
    pallete: &'a mut Pallete,
}

impl<'a> PpuBus<'a> {
    pub fn new(ppu: &'a mut Ppu, rom: &'a Rom) -> Self {
        Self {
            registers: &mut ppu.registers,
            ctx: &mut ppu.ctx,
            pattern_table: &rom.chr_rom,
            name_table0: &mut ppu.name_table0,
            attr_table0: &mut ppu.attr_table0,
            pallete: &mut ppu.pallete,
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
            0x2400..=0x2FFF => warn!("Unimplemented memory region: {:#06x}", addr),
            0x3000..=0x33BF => self.name_table0.write(addr - 0x3000, value),
            0x33C0..=0x33FF => self.attr_table0.write(addr - 0x33C0, value),
            0x3400..=0x3EFF => {
                warn!("Unimplemented memory region: {:#06x}", addr);
            }
            0x3F00..=0x3FFF => self.pallete.write((addr - 0x3F00) % 0x20, value),
            _ => {
                warn!("Illegal memory region: {:#06x}", addr);
            }
        }
    }

    pub fn sync_registers(&mut self) {
        if self.registers.ppu_addr_writed {
            self.registers.ppu_addr_writed = false;
            match self.ctx.ppu_addr_tmp {
                None => {
                    self.ctx.ppu_addr_tmp = Some(self.registers.ppu_addr);
                }
                Some(tmp) => {
                    self.ctx.ppu_addr_tmp = None;
                    self.ctx.ppu_addr = (tmp as u16) * 0x100 + self.registers.ppu_addr as u16;
                    self.registers.ppu_data = self.get_byte(self.ctx.ppu_addr);
                }
            }
        }

        if self.registers.ppu_data_writed {
            self.registers.ppu_data_writed = false;
            self.set_byte(self.ctx.ppu_addr, self.registers.ppu_data);
            self.ctx.ppu_addr += 1;
            self.registers.ppu_data = self.get_byte(self.ctx.ppu_addr);
        }

        if self.registers.ppu_data_readed {
            self.registers.ppu_data_readed = false;
            self.ctx.ppu_addr += 1;
            self.registers.ppu_data = self.get_byte(self.ctx.ppu_addr);
        }
    }
}
