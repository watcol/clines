mod bus;
mod pallete;
mod registers;
mod sprite;
mod table;

use crate::display::Display;
use crate::Rom;
pub use registers::Registers;
pub use sprite::ObjectAttributeMemory;
pub use table::Mirroring;

use bus::PpuBus;
use pallete::Pallete;
use registers::RegisterIO;
use sprite::Sprite;
use table::Table;

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
            display: Display::default(),
            cycle: 0,
            lines: 0,
            ppu_addr: 0,
            ppu_addr_tmp: None,
            table: Table::new(rom.mirroring),
            pallete: Pallete::default(),
        }
    }

    pub fn run(&mut self, rom: &Rom, cycle: u8) -> Option<Display> {
        self.table.sync_register(&self.registers);
        PpuBus::new(self, rom).sync_registers();
        self.oam.sync_registers(&mut self.registers);
        self.registers.io = RegisterIO::None;
        if !self.add_cycle(cycle) {
            return None;
        }

        if !self.registers.ppu_status.sprite_hit && self.has_sprite_hit(rom) {
            self.registers.ppu_status.sprite_hit = true;
        }

        let res = if self.lines == 0 {
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
        };
        self.table.sync_line(self.lines);
        res
    }

    pub fn has_sprite_hit(&self, rom: &Rom) -> bool {
        let pattern: &[u8] = if let Some(ref ram) = self.chr_ram {
            ram
        } else {
            &rom.chr_rom
        };
        let sprite0 = self.oam.0[0];
        sprite0.y == (self.lines - 8) as u8
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
            && !(0..32).all(|i| {
                let index = self.table.get_character_id(i * 8, (self.lines - 2) as u8);
                let offset = if self.registers.ppu_ctrl.sprite_1000 {
                    0x100
                } else {
                    0x0
                };
                let chunk = pattern.chunks(0x10).nth(offset + index as usize).unwrap();
                chunk.iter().all(|b| *b == 0)
            })
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
            let (hide, show): (Vec<&Sprite>, Vec<&Sprite>) = self
                .oam
                .0
                .iter()
                .rev()
                .partition(|&sprite| sprite.attr.hide);
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
            self.display.set(i as u8, line, *color);
        }
    }

    pub fn render_bg(&self, buf: &mut [u8; 256], line: u8, pattern: &[u8]) {
        for x in 0..=255 {
            if x < 8 && !self.registers.ppu_mask.show_left_bg {
                continue;
            }
            let (posx, posy) = self.table.pixel_position(x, line);
            let offset = if self.registers.ppu_ctrl.bg_1000 {
                0x100
            } else {
                0x0
            };
            let index = self.table.get_character_id(x, line) as usize + offset;
            let pallete_id = self.table.get_pallete_id(x, line);
            let pallete = self.pallete.get_bg_pallete(pallete_id);
            let byte1 = pattern[index * 0x10 + posy as usize];
            let byte2 = pattern[index * 0x10 + posy as usize + 8];
            let bit1 = byte1 & (0x80 >> posx) != 0;
            let bit2 = byte2 & (0x80 >> posx) != 0;
            let color = (bit2 as usize) * 2 + bit1 as usize;
            if color != 0 {
                buf[x as usize] = pallete[color];
            }
        }
    }

    pub fn render_sprite(&self, buf: &mut [u8; 256], sprite: &Sprite, line: u8, pattern: &[u8]) {
        let line = line - 1;
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
        let palette = self.pallete.get_sprite_pallete(sprite.attr.palette);
        for i in (0..8).rev() {
            let bit1 = byte1 % 2;
            let bit2 = byte2 % 2;
            byte1 /= 2;
            byte2 /= 2;
            let color = (bit2 * 2 + bit1) as usize;
            if color != 0 {
                let pixel = if sprite.attr.horizontal_flip {
                    8 - i
                } else {
                    i
                };
                let (x, overflow) = sprite.x.overflowing_add(pixel);
                if !overflow {
                    buf[x as usize] = palette[color];
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
