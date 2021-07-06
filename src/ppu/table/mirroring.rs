#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mirroring {
    None,
    Horizontal,
    Vertical,
}

impl Default for Mirroring {
    fn default() -> Self {
        Self::None
    }
}

impl Mirroring {
    pub(super) fn mirror(&self, base_table: u8) -> usize {
        match (self, base_table) {
            (Self::None, table @ 0..=3) => table as usize,
            (Self::Horizontal, 0 | 1) => 0,
            (Self::Horizontal, 2 | 3) => 2,
            (Self::Vertical, 0 | 2) => 0,
            (Self::Vertical, 1 | 3) => 1,
            _ => unreachable!(),
        }
    }
}

