#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameTable(pub(super) [[u8; 32]; 30]);

impl Default for NameTable {
    fn default() -> Self {
        Self([[0; 32]; 30])
    }
}

impl NameTable {
    pub fn get_byte(&self, x: u8, y: u8) -> u8 {
        self.0[y as usize][x as usize]
    }

    fn set_byte(&mut self, x: u8, y: u8, value: u8) {
        self.0[y as usize][x as usize] = value;
    }

    pub fn read(&self, addr: u16) -> u8 {
        let x = (addr % 32) as u8;
        let y = (addr / 32) as u8;
        self.get_byte(x, y)
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let x = (addr % 32) as u8;
        let y = (addr / 32) as u8;
        self.set_byte(x, y, value);
    }
}
