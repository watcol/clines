#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Context {
    pub ppu_addr: u16,
    pub ppu_addr_tmp: Option<u8>,
}
