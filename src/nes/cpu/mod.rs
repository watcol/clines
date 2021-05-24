mod bus;
mod instruction;
mod opecodes;
mod registers;

pub use instruction::{Addressing, Instruction, Opecode};
pub use opecodes::OPECODES;
pub use registers::{Registers, Status};

use super::Nes;
use bus::CpuBus;

pub fn run(nes: &mut Nes) -> anyhow::Result<u8> {
    let mut bus = CpuBus::new(nes);
    let opecode = OPECODES[bus.increment_byte() as usize];
    opecode.exec(&mut bus)
}
