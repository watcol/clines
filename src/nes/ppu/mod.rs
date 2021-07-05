mod bus;
mod pallete;
mod registers;
mod sprite;
mod table;

use crate::nes::Rom;
pub use registers::Registers;
pub use sprite::ObjectAttributeMemory;
pub use table::Mirroring;

use bus::PpuBus;
use pallete::Pallete;
use sprite::Sprite;
use table::Table;

use once_cell::sync::Lazy;
use picto::color::Rgb;

pub type Display = picto::buffer::Rgb;

static COLORS: Lazy<[Rgb; 64]> = Lazy::new(|| {
    [
        Rgb::new_u8(0x80, 0x80, 0x80),
        Rgb::new_u8(0x00, 0x3D, 0xA6),
        Rgb::new_u8(0x00, 0x12, 0xB0),
        Rgb::new_u8(0x44, 0x00, 0x96),
        Rgb::new_u8(0xA1, 0x00, 0x5E),
        Rgb::new_u8(0xC7, 0x00, 0x28),
        Rgb::new_u8(0xBA, 0x06, 0x00),
        Rgb::new_u8(0x8C, 0x17, 0x00),
        Rgb::new_u8(0x5C, 0x2F, 0x00),
        Rgb::new_u8(0x10, 0x45, 0x00),
        Rgb::new_u8(0x05, 0x4A, 0x00),
        Rgb::new_u8(0x00, 0x47, 0x2E),
        Rgb::new_u8(0x00, 0x41, 0x66),
        Rgb::new_u8(0x00, 0x00, 0x00),
        Rgb::new_u8(0x05, 0x05, 0x05),
        Rgb::new_u8(0x05, 0x05, 0x05),
        Rgb::new_u8(0xC7, 0xC7, 0xC7),
        Rgb::new_u8(0x00, 0x77, 0xFF),
        Rgb::new_u8(0x21, 0x55, 0xFF),
        Rgb::new_u8(0x82, 0x37, 0xFA),
        Rgb::new_u8(0xEB, 0x2F, 0xB5),
        Rgb::new_u8(0xFF, 0x29, 0x50),
        Rgb::new_u8(0xFF, 0x22, 0x00),
        Rgb::new_u8(0xD6, 0x32, 0x00),
        Rgb::new_u8(0xC4, 0x62, 0x00),
        Rgb::new_u8(0x35, 0x80, 0x00),
        Rgb::new_u8(0x05, 0x8F, 0x00),
        Rgb::new_u8(0x00, 0x8A, 0x55),
        Rgb::new_u8(0x00, 0x99, 0xCC),
        Rgb::new_u8(0x21, 0x21, 0x21),
        Rgb::new_u8(0x09, 0x09, 0x09),
        Rgb::new_u8(0x09, 0x09, 0x09),
        Rgb::new_u8(0xFF, 0xFF, 0xFF),
        Rgb::new_u8(0x0F, 0xD7, 0xFF),
        Rgb::new_u8(0x69, 0xA2, 0xFF),
        Rgb::new_u8(0xD4, 0x80, 0xFF),
        Rgb::new_u8(0xFF, 0x45, 0xF3),
        Rgb::new_u8(0xFF, 0x61, 0x8B),
        Rgb::new_u8(0xFF, 0x88, 0x33),
        Rgb::new_u8(0xFF, 0x9C, 0x12),
        Rgb::new_u8(0xFA, 0xBC, 0x20),
        Rgb::new_u8(0x9F, 0xE3, 0x0E),
        Rgb::new_u8(0x2B, 0xF0, 0x35),
        Rgb::new_u8(0x0C, 0xF0, 0xA4),
        Rgb::new_u8(0x05, 0xFB, 0xFF),
        Rgb::new_u8(0x5E, 0x5E, 0x5E),
        Rgb::new_u8(0x0D, 0x0D, 0x0D),
        Rgb::new_u8(0x0D, 0x0D, 0x0D),
        Rgb::new_u8(0xFF, 0xFF, 0xFF),
        Rgb::new_u8(0xA6, 0xFC, 0xFF),
        Rgb::new_u8(0xB3, 0xEC, 0xFF),
        Rgb::new_u8(0xDA, 0xAB, 0xEB),
        Rgb::new_u8(0xFF, 0xA8, 0xF9),
        Rgb::new_u8(0xFF, 0xAB, 0xB3),
        Rgb::new_u8(0xFF, 0xD2, 0xB0),
        Rgb::new_u8(0xFF, 0xEF, 0xA6),
        Rgb::new_u8(0xFF, 0xF7, 0x9C),
        Rgb::new_u8(0xD7, 0xE8, 0x95),
        Rgb::new_u8(0xA6, 0xED, 0xAF),
        Rgb::new_u8(0xA2, 0xF2, 0xDA),
        Rgb::new_u8(0x99, 0xFF, 0xFC),
        Rgb::new_u8(0xDD, 0xDD, 0xDD),
        Rgb::new_u8(0x11, 0x11, 0x11),
        Rgb::new_u8(0x11, 0x11, 0x11),
    ]
});

#[derive(Clone, Debug)]
pub struct Ppu {
    pub registers: Registers,
    pub oam: ObjectAttributeMemory,
    pub nmi: bool,
    chr_ram: Option<[u8; 0x2000]>,
    display: Display,
    cycle: u16,
    lines: u16,
    ppu_addr: u16,
    ppu_addr_tmp: Option<u8>,
    table: Table,
    pallete: Pallete,
}

impl Ppu {
    pub fn new(rom: &Rom) -> Self {
        Self {
            registers: Registers::default(),
            oam: ObjectAttributeMemory::default(),
            nmi: false,
            chr_ram: if rom.chr_rom.is_empty() {
                Some([0; 0x2000])
            } else {
                None
            },
            display: Display::new(256, 240),
            cycle: 0,
            lines: 0,
            ppu_addr: 0,
            ppu_addr_tmp: None,
            table: Table::new(rom.mirroring),
            pallete: Pallete::default(),
        }
    }

    pub fn run(&mut self, rom: &Rom, cycle: u8) -> Option<Display> {
        PpuBus::new(self, rom).sync_registers();
        self.oam.sync_registers(&mut self.registers);
        if self.has_sprite_hit(rom) {
            self.registers.ppu_status.sprite_hit = true;
        }
        let updated = self.add_cycle(cycle);
        if !updated {
            return None;
        }

        if self.lines == 0 {
            self.registers.ppu_status.sprite_hit = false;
            None
        } else if 2 <= self.lines && self.lines < 242 {
            self.render_line((self.lines - 2) as u8, rom);
            None
        } else if self.lines == 242 {
            self.registers.ppu_status.vblank = true;
            if self.registers.ppu_ctrl.enable_nmi {
                self.nmi = true;
            }
            Some(self.display.clone())
        } else if self.lines == 262 {
            self.registers.ppu_status.vblank = false;
            None
        } else {
            None
        }
    }

    pub fn has_sprite_hit(&self, rom: &Rom) -> bool {
        let pattern: &[u8] = if let Some(ref ram) = self.chr_ram {
            ram
        } else {
            &rom.chr_rom
        };
        let sprite0 = self.oam.0[0];
        sprite0.y as u16 == self.lines
            && (self.registers.ppu_mask.show_bg && self.registers.ppu_mask.show_sprites)
            && !(sprite0.x < 8
                && (!self.registers.ppu_mask.show_left_bg
                    || !self.registers.ppu_mask.show_left_sprite))
            && !{
                let index = sprite0.tile as usize;
                let offset = if self.registers.ppu_ctrl.sprite_1000 {
                    0x100
                } else {
                    0x0
                };
                let chunk = pattern.chunks(0x10).nth(offset + index).unwrap();
                chunk.iter().all(|b| *b == 0)
            }
            && !{
                let name_table = self
                    .table
                    .get_background(self.registers.ppu_ctrl.name_table);
                name_table.iter().flatten().all(|&index| {
                    let offset = if self.registers.ppu_ctrl.sprite_1000 {
                        0x100
                    } else {
                        0x0
                    };
                    let chunk = pattern.chunks(0x10).nth(offset + index as usize).unwrap();
                    chunk.iter().all(|b| *b == 0)
                })
            }
    }

    pub fn render_line(&mut self, line: u8, rom: &Rom) {
        let pattern: &[u8] = if let Some(ref ram) = self.chr_ram {
            ram
        } else {
            &rom.chr_rom
        };
        let universal_bg = self.pallete.get_universal_bg();
        let mut buf = [universal_bg; 256];
        if self.registers.ppu_mask.show_sprites {
            let (hide, show): (Vec<&Sprite>, Vec<&Sprite>) =
                self.oam.0.iter().partition(|&sprite| sprite.attr.hide);
            for sprite in hide {
                self.render_sprite(&mut buf, sprite, line, pattern);
            }
            if self.registers.ppu_mask.show_bg {
                self.render_bg(&mut buf, line, pattern);
            }
            for sprite in show {
                self.render_sprite(&mut buf, sprite, line, pattern);
            }
        } else if self.registers.ppu_mask.show_bg {
            self.render_bg(&mut buf, line, pattern);
        }
        for (i, color) in buf.iter().enumerate() {
            self.display
                .set(i as u32, line as u32, &COLORS[*color as usize]);
        }
    }

    pub fn render_bg(&self, buf: &mut [u8; 256], line: u8, pattern: &[u8]) {
        for x in 0..32 {
            if x == 0 && !self.registers.ppu_mask.show_left_bg {
                continue;
            }
            let base_table = self.registers.ppu_ctrl.name_table;
            let line_div = line / 8;
            let line_mod = line % 8;
            let index = self.table.get_character_id(base_table, x, line_div);
            let offset = if self.registers.ppu_ctrl.bg_1000 {
                0x100
            } else {
                0x0
            };
            let index = index as usize + offset;
            let mut byte1 = pattern[index * 0x10 + line_mod as usize];
            let mut byte2 = pattern[index * 0x10 + line_mod as usize + 8];
            let pallete_id = self.table.get_pallete_id(base_table, x, line_div);
            let pallete = self.pallete.get_bg_pallete(pallete_id);
            for i in (0..8).rev() {
                let bit1 = byte1 % 2;
                let bit2 = byte2 % 2;
                byte1 /= 2;
                byte2 /= 2;
                let color = (bit2 * 2 + bit1) as usize;
                if color != 0 {
                    buf[(x * 8 + i) as usize] = pallete[color];
                }
            }
        }
    }

    pub fn render_sprite(&self, buf: &mut [u8; 256], sprite: &Sprite, line: u8, pattern: &[u8]) {
        if sprite.y >= 231
            || (!self.registers.ppu_mask.show_left_sprite && sprite.x < 8)
            || (line < sprite.y || sprite.y + 8 <= line)
        {
            return;
        }
        let pos = if sprite.attr.vertical_flip {
            8 - (line - sprite.y)
        } else {
            line - sprite.y
        };
        let offset = if self.registers.ppu_ctrl.sprite_1000 {
            0x100
        } else {
            0x0
        };
        let index = sprite.tile as usize + offset;
        let mut byte1 = pattern[index * 0x10 + pos as usize];
        let mut byte2 = pattern[index * 0x10 + pos as usize + 8];
        let pallete = self.pallete.get_sprite_pallete(sprite.attr.pallete);
        for i in (0..8).rev() {
            let bit1 = byte1 % 2;
            let bit2 = byte2 % 2;
            byte1 /= 2;
            byte2 /= 2;
            let color = (bit2 * 2 + bit1) as usize;
            if color != 0 {
                if sprite.attr.horizontal_flip {
                    buf[(sprite.x + (8 - i)) as usize] = pallete[color];
                } else {
                    buf[(sprite.x + i) as usize] = pallete[color];
                }
            }
        }
    }

    pub fn add_cycle(&mut self, cycle: u8) -> bool {
        self.cycle += cycle as u16;
        if self.cycle >= 314 {
            self.cycle -= 314;
            self.lines += 1;
            if self.lines == 263 {
                self.lines = 0;
            }
            true
        } else {
            false
        }
    }
}
