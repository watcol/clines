#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttrTable([[u8; 8]; 8]);

impl Default for AttrTable {
    fn default() -> Self {
        Self([[0; 8]; 8])
    }
}

impl AttrTable {
    fn get_byte(&self, x: u8, y: u8) -> u8 {
        self.0[y as usize][x as usize]
    }

    fn set_byte(&mut self, x: u8, y: u8, value: u8) {
        self.0[y as usize][x as usize] = value
    }

    pub fn get_pallete_id(&self, x: u8, y: u8) -> u8 {
        let x = x / 2;
        let y = y / 2;
        let right = x % 2;
        let x = x / 2;
        let bottom = y % 2;
        let y = y / 2;
        let byte = self.get_byte(x, y);
        let offset = (bottom * 2 + right) * 2;
        (byte >> offset) % 4
    }

    pub fn read(&self, addr: u16) -> u8 {
        let x = (addr % 8) as u8;
        let y = (addr / 8) as u8;
        self.get_byte(x, y)
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let x = (addr % 8) as u8;
        let y = (addr / 8) as u8;
        self.set_byte(x, y, value);
    }
}
