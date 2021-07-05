use super::{Pallete, Ppu, Registers, Table};
use crate::nes::Rom;

pub struct PpuBus<'a> {
    pub registers: &'a mut Registers,
    ppu_addr: &'a mut u16,
    ppu_addr_tmp: &'a mut Option<u8>,
    pattern_table: PatternTable<'a>,
    table: &'a mut Table,
    pallete: &'a mut Pallete,
}

enum PatternTable<'a> {
    Rom(&'a [u8]),
    Ram(&'a mut [u8]),
}

impl<'a> PatternTable<'a> {
    fn read(&self, addr: u16) -> u8 {
        match self {
            PatternTable::Rom(rom) => rom[addr as usize],
            PatternTable::Ram(ram) => ram[addr as usize],
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match self {
            PatternTable::Rom(_) => warn!("CHR ROM is readonly."),
            PatternTable::Ram(ram) => ram[addr as usize] = value,
        }
    }
}

impl<'a> PpuBus<'a> {
    pub fn new(ppu: &'a mut Ppu, rom: &'a Rom) -> Self {
        Self {
            registers: &mut ppu.registers,
            ppu_addr: &mut ppu.ppu_addr,
            ppu_addr_tmp: &mut ppu.ppu_addr_tmp,
            pattern_table: if let Some(ref mut ram) = ppu.chr_ram {
                PatternTable::Ram(ram)
            } else {
                PatternTable::Rom(&rom.chr_rom)
            },
            table: &mut ppu.table,
            pallete: &mut ppu.pallete,
        }
    }

    pub fn get_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.pattern_table.read(addr),
            0x2000..=0x2FFF => self.table.read(addr - 0x2000),
            0x3000..=0x3EFF => self.table.read(addr - 0x3000),
            0x3F00..=0x3FFF => self.pallete.read((addr - 0x3F00) % 0x20),
            _ => {
                warn!("Illegal memory region: {:#06x}", addr);
                0
            }
        }
    }

    pub fn set_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.pattern_table.write(addr, value),
            0x2000..=0x2FFF => self.table.write(addr - 0x2000, value),
            0x3000..=0x3EFF => self.table.write(addr - 0x3000, value),
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
