#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Context {
    pub ppu_addr: u16,
    pub ppu_addr_tmp: Option<u8>,
    pub cycle: u16,
    pub lines: u16,
}

impl Context {
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
