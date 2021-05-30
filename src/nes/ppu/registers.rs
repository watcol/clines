#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Registers {
    pub(super) ppu_ctrl: u8,
    pub(super) ppu_mask: u8,
    pub(super) ppu_status: u8,
    pub(super) oam_addr: u8,
    pub(super) oam_data: u8,
    pub(super) ppu_scroll: u8,
    pub(super) ppu_addr: u8,
    pub(super) ppu_addr_writed: bool,
    pub(super) ppu_data: u8,
    pub(super) ppu_data_writed: bool,
    pub(super) ppu_data_readed: bool,
}

impl Registers {
    pub fn read(&mut self, addr: u16) -> u8 {
        match addr % 0x8 {
            0x0 => {
                warn!("Reading to PPUCTRL is not allowed.");
                0
            }
            0x1 => {
                warn!("Reading to PPUMASK is not allowed.");
                0
            }
            0x2 => self.ppu_status,
            0x3 => {
                warn!("Reading to OAMADDR is not allowed.");
                0
            }
            0x4 => self.oam_data,
            0x5 => {
                warn!("Reading to PPUSCROLL is not allowed.");
                0
            }
            0x6 => {
                warn!("Reading to PPUADDR is not allowed.");
                0
            }
            0x7 => {
                self.ppu_data_readed = true;
                self.ppu_data
            }
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
            0x6 => {
                self.ppu_addr_writed = true;
                self.ppu_addr = value;
            }
            0x7 => {
                self.ppu_data_writed = true;
                self.ppu_data = value;
            }
            _ => unreachable!(),
        }
    }
}
