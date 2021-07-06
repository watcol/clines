#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Registers {
    pub(super) ppu_ctrl: PpuCtrl,
    pub(super) ppu_mask: PpuMask,
    pub(super) ppu_status: PpuStatus,
    pub(super) oam_addr: u8,
    pub(super) oam_addr_writed: bool,
    pub(super) oam_data: u8,
    pub(super) oam_data_writed: bool,
    pub(super) ppu_scroll: PpuScroll,
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
            0x2 => {
                let res = self.ppu_status.as_u8();
                self.ppu_status.vblank = false;
                res
            }
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
            0x0 => self.ppu_ctrl = PpuCtrl::from_u8(value),
            0x1 => self.ppu_mask = PpuMask::from_u8(value),
            0x2 => warn!("Writing to PPUSTATUS is not allowed."),
            0x3 => {
                self.oam_addr_writed = true;
                self.oam_addr = value;
            }
            0x4 => {
                self.oam_data_writed = true;
                self.oam_data = value;
            }
            0x5 => self.ppu_scroll.write(value),
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PpuCtrl {
    pub enable_nmi: bool,
    pub ppu_master_slave: bool,
    pub sprite_16: bool,
    pub bg_1000: bool,
    pub sprite_1000: bool,
    pub ppu_mem_32: bool,
    pub name_table: u8,
}

impl Default for PpuCtrl {
    fn default() -> Self {
        Self {
            enable_nmi: false,
            ppu_master_slave: true,
            sprite_16: false,
            bg_1000: false,
            sprite_1000: true,
            ppu_mem_32: false,
            name_table: 0,
        }
    }
}

impl PpuCtrl {
    pub fn from_u8(byte: u8) -> Self {
        Self {
            enable_nmi: byte & 0x80 == 0x80,
            ppu_master_slave: byte & 0x40 == 0x40,
            sprite_16: byte & 0x20 == 0x20,
            bg_1000: byte & 0x10 == 0x10,
            sprite_1000: byte & 0x08 == 0x08,
            ppu_mem_32: byte & 0x04 == 0x04,
            name_table: byte & 0x03,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PpuMask {
    pub emphasize_blue: bool,
    pub emphasize_green: bool,
    pub emphasize_red: bool,
    pub show_sprites: bool,
    pub show_bg: bool,
    pub show_left_sprite: bool,
    pub show_left_bg: bool,
    pub greyscale: bool,
}

impl PpuMask {
    pub fn from_u8(byte: u8) -> Self {
        Self {
            emphasize_blue: byte & 0x80 == 0x80,
            emphasize_green: byte & 0x40 == 0x40,
            emphasize_red: byte & 0x20 == 0x20,
            show_sprites: byte & 0x10 == 0x10,
            show_bg: byte & 0x08 == 0x08,
            show_left_sprite: byte & 0x04 == 0x04,
            show_left_bg: byte & 0x02 == 0x02,
            greyscale: byte & 0x01 == 0x01,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PpuStatus {
    pub vblank: bool,
    pub sprite_hit: bool,
    pub sprite_overflow: bool,
}

impl PpuStatus {
    pub fn as_u8(&self) -> u8 {
        self.vblank as u8 * 0x80 + self.sprite_hit as u8 * 0x40 + self.sprite_overflow as u8 * 0x20
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PpuScroll {
    pub is_y: bool,
    pub x: u8,
    pub y: u8,
}

impl PpuScroll {
    pub fn write(&mut self, value: u8) {
        if self.is_y {
            self.y = value;
            self.is_y = false;
        } else {
            self.x = value;
            self.is_y = true;
        }
    }
}
