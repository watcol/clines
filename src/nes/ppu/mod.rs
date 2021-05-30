mod registers;

pub use registers::Registers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ppu {
    pub registers: Registers,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            registers: Registers::default(),
        }
    }
}
