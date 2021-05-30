#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pallete {
    universal_bg: u8,
    unused_bg: [u8; 3],
    bg: [[u8; 3]; 4],
    sprite: [[u8; 3]; 4],
}

impl Default for Pallete {
    fn default() -> Self {
        Self {
            universal_bg: 0,
            unused_bg: [0; 3],
            bg: [[0; 3]; 4],
            sprite: [[0; 3]; 4],
        }
    }
}

impl Pallete {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x00 => self.universal_bg,
            0x04 | 0x08 | 0x0c => self.unused_bg[(addr / 4 - 1) as usize],
            0x01..=0x0F => self.bg[(addr / 4) as usize][(addr % 4 - 1) as usize],
            0x10 => self.universal_bg,
            0x14 | 0x18 | 0x1c => self.unused_bg[((addr - 0x10) / 4 - 1) as usize],
            0x11..=0x1F => {
                self.sprite[((addr - 0x10) / 4) as usize][((addr - 0x10) % 4 - 1) as usize]
            }
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x00 => self.universal_bg = value,
            0x04 | 0x08 | 0x0c => self.unused_bg[(addr / 4 - 1) as usize] = value,
            0x01..=0x0F => self.bg[(addr / 4) as usize][(addr % 4 - 1) as usize] = value,
            0x10 => self.universal_bg = value,
            0x14 | 0x18 | 0x1c => self.unused_bg[((addr - 0x10) / 4 - 1) as usize] = value,
            0x11..=0x1F => {
                self.sprite[((addr - 0x10) / 4) as usize][((addr - 0x10) % 4 - 1) as usize] = value;
            }
            _ => unreachable!(),
        }
    }

    pub fn get_bg_pallete(&self, id: u8) -> [u8; 4] {
        let bgs = self.bg[id as usize];
        [self.universal_bg, bgs[0], bgs[1], bgs[2]]
    }
}
