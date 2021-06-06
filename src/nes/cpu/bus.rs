use super::Registers;
use crate::nes::{ppu::Registers as PpuRegisters, Cpu, Ppu, Rom};

pub struct CpuBus<'a> {
    pub registers: &'a mut Registers,
    ppu_registers: &'a mut PpuRegisters,
    wram: &'a mut [u8; 0x800],
    prg_rom: &'a [u8],
}

impl<'a> CpuBus<'a> {
    pub fn new(cpu: &'a mut Cpu, rom: &'a Rom, ppu: &'a mut Ppu) -> Self {
        Self {
            registers: &mut cpu.registers,
            ppu_registers: &mut ppu.registers,
            wram: &mut cpu.wram,
            prg_rom: &rom.prg_rom,
        }
    }

    pub fn get_byte(&mut self, addr: u16) -> u8 {
        let res = match addr {
            0x0000..=0x1FFF => self.wram[(addr % 0x800) as usize],
            0x2000..=0x3FFF => self.ppu_registers.read(addr),
            0x4000..=0x7FFF => {
                warn!("Address {:#06x} is unimplemented", addr);
                0
            }
            _ => self.prg_rom[((addr - 0x8000) as usize) % self.prg_rom.len()],
        };
        trace!("Get {:04X} (= {:02x})", addr, res);
        res
    }

    pub fn get_word(&mut self, addr: u16) -> u16 {
        let lower = self.get_byte(addr) as u16;
        let upper = self.get_byte(addr + 1) as u16;
        (upper << 8) + lower
    }

    pub fn get_word_page(&mut self, addr: u16) -> u16 {
        let lower = self.get_byte(addr) as u16;
        let upper_addr = if addr % 0x100 == 0xff {
            addr - 0xff
        } else {
            addr + 1
        };
        let upper = self.get_byte(upper_addr) as u16;
        (upper << 8) + lower
    }

    pub fn set_byte(&mut self, addr: u16, value: u8) {
        trace!("Set {:04X} = {:02x}", addr, value);
        match addr {
            0x0000..=0x1FFF => self.wram[(addr % 0x800) as usize] = value,
            0x2000..=0x3FFF => self.ppu_registers.write(addr, value),
            0x4000..=0x7FFF => warn!("Address {:#06x} is unimplemented", addr),
            _ => warn!("Writing to {:#06x} is not allowed.", addr),
        }
    }

    pub fn set_word(&mut self, addr: u16, value: u16) {
        let lower = (value % 0x100) as u8;
        let upper = (value >> 8) as u8;
        self.set_byte(addr, lower);
        self.set_byte(addr, upper);
    }

    pub fn push_byte(&mut self, value: u8) -> anyhow::Result<()> {
        self.set_byte(0x100 + self.registers.S as u16, value);
        self.registers.S -= 1;
        Ok(())
    }

    pub fn push_word(&mut self, value: u16) -> anyhow::Result<()> {
        let upper = (value >> 8) as u8;
        let lower = (value % 0x100) as u8;
        self.push_byte(upper)?;
        self.push_byte(lower)
    }

    pub fn pop_byte(&mut self) -> u8 {
        self.registers.S += 1;
        self.get_byte(0x100 + self.registers.S as u16)
    }

    pub fn pop_word(&mut self) -> u16 {
        let lower = self.pop_byte() as u16;
        let upper = self.pop_byte() as u16;
        (upper << 8) + lower
    }

    pub fn increment_byte(&mut self) -> u8 {
        let byte = self.get_byte(self.registers.PC);
        self.registers.PC += 1;
        byte
    }

    pub fn increment_word(&mut self) -> u16 {
        let word = self.get_word(self.registers.PC);
        self.registers.PC += 2;
        word
    }
}
