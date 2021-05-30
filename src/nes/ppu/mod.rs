mod attr_table;
mod bus;
mod name_table;
mod pallete;
mod registers;
mod renderer;
mod sprite;

use crate::nes::Rom;
use crate::Display;

pub use registers::Registers;

use attr_table::AttrTable;
use bus::PpuBus;
use name_table::NameTable;
use pallete::Pallete;
use sprite::{ColoredSprite, Sprite};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ppu {
    pub registers: Registers,
    display: Display,
    cycle: u16,
    lines: u16,
    ppu_addr: u16,
    ppu_addr_tmp: Option<u8>,
    name_table0: NameTable,
    attr_table0: AttrTable,
    pallete: Pallete,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            registers: Registers::default(),
            display: Display::new(256, 240),
            cycle: 0,
            lines: 0,
            ppu_addr: 0,
            ppu_addr_tmp: None,
            name_table0: NameTable::default(),
            attr_table0: AttrTable::default(),
            pallete: Pallete::default(),
        }
    }
}

impl Ppu {
    pub fn run(&mut self, rom: &Rom, cycle: u8) -> Option<Display> {
        PpuBus::new(self, rom).sync_registers();
        let line = match self.add_cycle(cycle) {
            Some(line) => line,
            None => return None,
        };

        for x in 0..32 {
            let index = self.name_table0.get_byte(x, line);
            let sprite = Sprite::new(&rom.chr_rom, index);
            let pallete_id = self.attr_table0.get_pallete_id(x, line);
            let pallete = self.pallete.get_bg_pallete(pallete_id);
            let colored_sprite = ColoredSprite::new(&sprite, &pallete);
            renderer::render(&mut self.display, &colored_sprite, x, line);
        }

        if line == 14 {
            let tmp = self.display.clone();
            self.display = Display::new(256, 240);
            Some(tmp)
        } else {
            None
        }
    }

    pub fn add_cycle(&mut self, cycle: u8) -> Option<u8> {
        self.cycle += cycle as u16;
        if self.cycle >= 314 {
            self.cycle -= 314;
            self.lines += 1;
        } else {
            return None;
        }

        if 10 <= self.lines && self.lines <= 242 && self.lines % 8 == 2 {
            Some(((self.lines - 2) / 8 - 1) as u8)
        } else if self.lines == 263 {
            self.lines = 0;
            None
        } else {
            None
        }
    }
}
