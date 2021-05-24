use super::Registers;
use crate::nes::Nes;

pub struct CpuBus<'a> {
    pub registers: &'a mut Registers,
    unimplemented_region: [u8; 0x6000],
    wram: &'a mut [u8; 0x800],
    prg_rom: &'a [u8],
}

impl<'a> CpuBus<'a> {
    pub fn new(nes: &'a mut Nes) -> Self {
        Self {
            registers: &mut nes.cpu_registers,
            unimplemented_region: [0; 0x6000],
            wram: &mut nes.wram,
            prg_rom: &nes.rom.prg_rom,
        }
    }

    pub fn get_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.wram[(addr % 0x800) as usize],
            0x2000..=0x7FFF => self.unimplemented_region[(addr - 0x2000) as usize],
            _ => self.prg_rom[(addr - 0x8000) as usize],
        }
    }

    pub fn get_word(&self, addr: u16) -> u16 {
        let lower = self.get_byte(addr) as u16;
        let upper = self.get_byte(addr + 1) as u16;
        (upper << 8) + lower
    }

    pub fn set_byte(&mut self, addr: u16, value: u8) -> anyhow::Result<()> {
        match addr {
            0x0000..=0x1FFF => self.wram[(addr % 0x800) as usize] = value,
            0x2000..=0x7FFF => self.unimplemented_region[(addr - 0x2000) as usize] = value,
            _ => anyhow::bail!("Setting byte to rom is not allowed."),
        }
        Ok(())
    }

    pub fn set_word(&mut self, addr: u16, value: u16) -> anyhow::Result<()> {
        let lower = (value % 0x100) as u8;
        let upper = (value >> 8) as u8;
        self.set_byte(addr, lower)?;
        self.set_byte(addr, upper)
    }

    pub fn push_byte(&mut self, value: u8) -> anyhow::Result<()> {
        self.set_byte(0x100 + self.registers.S as u16, value)?;
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
