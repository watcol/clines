#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PpuRegisters {
    pub ppu_ctrl: u8,
    pub ppu_mask: u8,
    pub ppu_status: u8,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub ppu_scroll: u8,
    pub ppu_addr: u8,
    pub ppu_data: u8,
}

impl Default for PpuRegisters {
    fn default() -> Self {
        Self {
            ppu_ctrl: 0,
            ppu_mask: 0,
            ppu_status: 0,
            oam_addr: 0,
            oam_data: 0,
            ppu_scroll: 0,
            ppu_addr: 0,
            ppu_data: 0,
        }
    }
}

impl PpuRegisters {
    pub fn read(&self, addr: u16) -> u8 {
        match addr % 0x8 {
            0x0 => {
                warn!("Writing to PPUCTRL is not allowed.");
                0
            }
            0x1 => {
                warn!("Writing to PPUMASK is not allowed.");
                0
            }
            0x2 => self.ppu_status,
            0x3 => {
                warn!("Writing to OAMADDR is not allowed.");
                0
            }
            0x4 => self.oam_data,
            0x5 => {
                warn!("Writing to PPUSCROLL is not allowed.");
                0
            }
            0x6 => {
                warn!("Writing to PPUADDR is not allowed.");
                0
            }
            0x7 => self.ppu_data,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr % 0x8 {
            0x0 => self.ppu_ctrl = value,
            0x1 => self.ppu_mask = value,
            0x2 => warn!("Writing to PPUSTATUS is not allowed."),
            0x3 => self.oam_addr = value,
            0x4 => self.oam_data = value,
            0x5 => self.ppu_scroll = value,
            0x6 => self.ppu_addr = value,
            0x7 => self.ppu_data = value,
            _ => unreachable!(),
        }
    }
}
