mod instruction;
mod opecodes;

pub use instruction::{Addressing, Instruction, Opecode};
pub use opecodes::OPECODES;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cpu {
    reg: Registers,
    mem: [u8; 0x10000], // TODO: Add Structure
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct Registers {
    A: u8,
    X: u8,
    Y: u8,
    P: Status,
    S: u8,
    PC: u16,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Status {
    negative: bool,
    overflow: bool,
    reserved: bool,
    break_mode: bool,
    decimal_mode: bool,
    interrupt: bool,
    zero: bool,
    carry: bool,
}
