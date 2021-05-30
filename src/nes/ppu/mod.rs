mod attr_table;
mod bus;
mod name_table;
mod registers;

use attr_table::AttrTable;
use name_table::NameTable;
pub use registers::Registers;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Ppu {
    pub registers: Registers,
    name_table0: NameTable,
    attr_table0: AttrTable,
}
