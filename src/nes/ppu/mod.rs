mod attr_table;
mod bus;
mod name_table;
mod pallete;
mod registers;
mod renderer;
mod sprite;
mod tile;

use crate::nes::Rom;

pub use registers::Registers;
pub use renderer::Display;
pub use sprite::ObjectAttributeMemory;

use attr_table::AttrTable;
use bus::PpuBus;
use name_table::NameTable;
use pallete::Pallete;
use tile::{ColoredTile, Tile};

#[derive(Clone, Debug)]
pub struct Ppu {
    pub registers: Registers,
    pub oam: ObjectAttributeMemory,
    display: Display,
    cycle: u16,
    lines: u16,
    ppu_addr: u16,
    ppu_addr_tmp: Option<u8>,
    name_table0: NameTable,
    attr_table0: AttrTable,
    name_table1: NameTable,
    attr_table1: AttrTable,
    name_table2: NameTable,
    attr_table2: AttrTable,
    name_table3: NameTable,
    attr_table3: AttrTable,
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
            oam: ObjectAttributeMemory::default(),
            name_table0: NameTable::default(),
            attr_table0: AttrTable::default(),
            name_table1: NameTable::default(),
            attr_table1: AttrTable::default(),
            name_table2: NameTable::default(),
            attr_table2: AttrTable::default(),
            name_table3: NameTable::default(),
            attr_table3: AttrTable::default(),
            pallete: Pallete::default(),
        }
    }
}

impl Ppu {
    pub fn run(&mut self, rom: &Rom, cycle: u8) -> Option<Display> {
        PpuBus::new(self, rom).sync_registers();
        self.oam.sync_registers(&mut self.registers);
        let line = match self.add_cycle(cycle) {
            Some(line) => line,
            None => return None,
        };

        self.render_line(line, rom);

        if line == 14 {
            self.render_sprite(rom);
            Some(self.display.clone())
        } else {
            None
        }
    }

    pub fn render_line(&mut self, line: u8, rom: &Rom) {
        for x in 0..32 {
            let index = self.name_table0.get_byte(x, line);
            let tile = Tile::new(&rom.chr_rom, index as u16);
            let pallete_id = self.attr_table0.get_pallete_id(x, line);
            let pallete = self.pallete.get_bg_pallete(pallete_id);
            let colored_tile = ColoredTile::new(&tile, &pallete);
            renderer::render(&mut self.display, &colored_tile, x * 8, line * 8);
        }
    }

    pub fn render_sprite(&mut self, rom: &Rom) {
        for id in 0..64 {
            let sprite = self.oam.get(id);
            if sprite.attr.hide || sprite.y >= 239 {
                continue;
            }
            let tile = Tile::new(&rom.chr_rom, 0x100 + sprite.tile as u16);
            let pallete = self.pallete.get_sprite_pallete(sprite.attr.pallete);
            let colored_tile = ColoredTile::new(&tile, &pallete);
            renderer::render(&mut self.display, &colored_tile, sprite.x, sprite.y + 1);
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
            self.registers.ppu_status.vblank = false;
            self.lines = 0;
            None
        } else if self.lines == 243 {
            self.registers.ppu_status.vblank = true;
            None
        } else {
            None
        }
    }
}
