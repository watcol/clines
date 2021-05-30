#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sprite([[u8; 8]; 8]);

impl Sprite {
    pub fn new<T: AsRef<[u8]>>(chr_rom: T, index: u8) -> Self {
        let chunk = chr_rom.as_ref().chunks(0x10).nth(index as usize).unwrap();
        let mut sprite = [[0; 8]; 8];
        for i in 0..8 {
            let mut byte1 = chunk[i];
            let mut byte2 = chunk[i + 8];
            for j in (0..8).rev() {
                let bit1 = byte1 % 2;
                let bit2 = byte2 % 2;
                byte1 /= 2;
                byte2 /= 2;
                sprite[i][j] = bit2 * 2 + bit1;
            }
        }

        Self(sprite)
    }

    pub fn get(&self, x: u8, y: u8) -> u8 {
        self.0[y as usize][x as usize]
    }
}
