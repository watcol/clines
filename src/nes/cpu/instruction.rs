use super::CpuBus;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Opecode {
    pub inst: Instruction,
    pub addr: Addressing,
    pub cycle: u8,
    pub add_cycle: bool,
}

impl Opecode {
    pub fn exec(&self, bus: &mut CpuBus) -> anyhow::Result<u8> {
        let pc = bus.registers.PC - 1;
        let a = bus.registers.A;
        let x = bus.registers.X;
        let y = bus.registers.Y;
        let p = bus.registers.P.as_u8();
        let (operand, page_corssed, asm) = self.addr.operand(bus);
        let branched = self.inst.exec(bus, operand)?;
        debug!(
            "{:04X} {:?} {:<10} A:{:02X} X:{:02X} Y:{:02X} P:{:02X}",
            pc, self.inst, asm, a, x, y, p,
        );
        let cycle = self.cycle
            + ((page_corssed && self.add_cycle) as u8)
            + ((branched && self.add_cycle) as u8);
        Ok(cycle)
    }
}

// TODO: Add Implementations
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Instruction {
    ADC,
    SBC,
    AND,
    ORA,
    EOR,
    ASL,
    LSR,
    ROL,
    ROR,
    BCC,
    BCS,
    BEQ,
    BNE,
    BVC,
    BVS,
    BPL,
    BMI,
    BIT,
    JMP,
    JSR,
    RTS,
    BRK,
    RTI,
    CMP,
    CPX,
    CPY,
    INC,
    DEC,
    INX,
    DEX,
    INY,
    DEY,
    CLC,
    SEC,
    CLI,
    SEI,
    CLD,
    SED,
    CLV,
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
    TAX,
    TXA,
    TAY,
    TYA,
    TSX,
    TXS,
    PHA,
    PLA,
    PHP,
    PLP,
    NOP,
    // Illegal Opcodes
    KIL,
    SLO,
    RLA,
    SRE,
    RRA,
    SAX,
    LAX,
    DCP,
    ISC,
    ANC,
    ALR,
    ARR,
    XAA,
    AXS,
    AHX,
    SHY,
    SHX,
    TAS,
    LAS,
}

impl Instruction {
    fn exec(&self, bus: &mut CpuBus, operand: Operand) -> anyhow::Result<bool> {
        let mut branched = false;
        match self {
            // Arithmetic Operations
            Self::ADC => {
                let acc = bus.registers.A;
                let op = operand.get_byte(bus)?;
                let (res, carry1) = acc.overflowing_add(op);
                let (res, carry2) = res.overflowing_add(bus.registers.P.carry as u8);
                bus.registers.P.overflow = ((acc ^ op) & 0x80 == 0) && ((acc ^ res) & 0x80 == 0x80);
                bus.registers.P.carry = carry1 || carry2;
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.A = res;
            }
            Self::SBC => {
                let acc = bus.registers.A;
                let op = operand.get_byte(bus)?;
                let (res, carry1) = acc.overflowing_sub(op);
                let (res, carry2) = res.overflowing_sub((!bus.registers.P.carry) as u8);
                bus.registers.P.overflow =
                    ((acc ^ op) & 0x80 == 0x80) && ((acc ^ res) & 0x80 == 0x80);
                bus.registers.P.carry = !(carry1 || carry2);
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.A = res;
            }
            Self::AND => {
                let res = bus.registers.A & operand.get_byte(bus)?;
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.A = res;
            }
            Self::ORA => {
                let res = bus.registers.A | operand.get_byte(bus)?;
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.A = res;
            }
            Self::EOR => {
                let res = bus.registers.A ^ operand.get_byte(bus)?;
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.A = res;
            }
            Self::ASL => match operand {
                Operand::Accumulator => {
                    let acc = bus.registers.A;
                    let res = acc << 1;
                    bus.registers.P.carry = acc & 0x80 == 0x80;
                    bus.registers.P.negative = res & 0x80 == 0x80;
                    bus.registers.P.zero = res == 0;
                    bus.registers.A = res;
                }
                Operand::Address(addr) => {
                    let acc = bus.get_byte(addr);
                    let res = acc << 1;
                    bus.registers.P.carry = acc & 0x80 == 0x80;
                    bus.registers.P.negative = res & 0x80 == 0x80;
                    bus.registers.P.zero = res == 0;
                    bus.set_byte(addr, res);
                }
                _ => unreachable!(),
            },
            Self::LSR => match operand {
                Operand::Accumulator => {
                    let acc = bus.registers.A;
                    let res = acc >> 1;
                    bus.registers.P.carry = acc & 0x1 == 0x1;
                    bus.registers.P.negative = res & 0x80 == 0x80;
                    bus.registers.P.zero = res == 0;
                    bus.registers.A = res;
                }
                Operand::Address(addr) => {
                    let acc = bus.get_byte(addr);
                    let res = acc >> 1;
                    bus.registers.P.carry = acc & 0x1 == 0x1;
                    bus.registers.P.negative = res & 0x80 == 0x80;
                    bus.registers.P.zero = res == 0;
                    bus.set_byte(addr, res);
                }
                _ => unreachable!(),
            },
            Self::ROL => match operand {
                Operand::Accumulator => {
                    let acc = bus.registers.A;
                    let res = (acc << 1) | (bus.registers.P.carry as u8);
                    bus.registers.P.carry = acc & 0x80 == 0x80;
                    bus.registers.P.negative = res & 0x80 == 0x80;
                    bus.registers.P.zero = res == 0;
                    bus.registers.A = res;
                }
                Operand::Address(addr) => {
                    let acc = bus.get_byte(addr);
                    let res = (acc << 1) | (bus.registers.P.carry as u8);
                    bus.registers.P.carry = acc & 0x80 == 0x80;
                    bus.registers.P.negative = res & 0x80 == 0x80;
                    bus.registers.P.zero = res == 0;
                    bus.set_byte(addr, res);
                }
                _ => unreachable!(),
            },
            Self::ROR => match operand {
                Operand::Accumulator => {
                    let acc = bus.registers.A;
                    let res = (acc >> 1) | (bus.registers.P.carry as u8 * 0x80);
                    bus.registers.P.carry = acc & 0x1 == 0x1;
                    bus.registers.P.negative = res & 0x80 == 0x80;
                    bus.registers.P.zero = res == 0;
                    bus.registers.A = res;
                }
                Operand::Address(addr) => {
                    let acc = bus.get_byte(addr);
                    let res = (acc >> 1) | (bus.registers.P.carry as u8 * 0x80);
                    bus.registers.P.carry = acc & 0x1 == 0x1;
                    bus.registers.P.negative = res & 0x80 == 0x80;
                    bus.registers.P.zero = res == 0;
                    bus.set_byte(addr, res);
                }
                _ => unreachable!(),
            },
            // Branches
            Self::BCC => {
                if !bus.registers.P.carry {
                    branched = true;
                    bus.registers.PC = operand.get_address()?;
                }
            }
            Self::BCS => {
                if bus.registers.P.carry {
                    branched = true;
                    bus.registers.PC = operand.get_address()?;
                }
            }
            Self::BEQ => {
                if bus.registers.P.zero {
                    branched = true;
                    bus.registers.PC = operand.get_address()?;
                }
            }
            Self::BNE => {
                if !bus.registers.P.zero {
                    branched = true;
                    bus.registers.PC = operand.get_address()?;
                }
            }
            Self::BVC => {
                if !bus.registers.P.overflow {
                    branched = true;
                    bus.registers.PC = operand.get_address()?;
                }
            }
            Self::BVS => {
                if bus.registers.P.overflow {
                    branched = true;
                    bus.registers.PC = operand.get_address()?;
                }
            }
            Self::BPL => {
                if !bus.registers.P.negative {
                    branched = true;
                    bus.registers.PC = operand.get_address()?;
                }
            }
            Self::BMI => {
                if bus.registers.P.negative {
                    branched = true;
                    bus.registers.PC = operand.get_address()?;
                }
            }
            // Bit Check
            Self::BIT => {
                let op = operand.get_byte(bus)?;
                bus.registers.P.zero = (bus.registers.A & op) == 0;
                bus.registers.P.negative = op & 0x80 == 0x80;
                bus.registers.P.overflow = op & 0x40 == 0x40;
            }
            // Jump
            Self::JMP => bus.registers.PC = operand.get_address()?,
            Self::JSR => {
                bus.push_word(bus.registers.PC - 1)?;
                bus.registers.PC = operand.get_address()?;
            }
            Self::RTS => {
                let addr = bus.pop_word();
                bus.registers.PC = addr + 1;
            }
            // Interruptions
            Self::BRK => {
                if !bus.registers.P.interrupt {
                    bus.registers.P.break_mode = true;
                    bus.registers.PC += 1;
                    bus.push_word(bus.registers.PC)?;
                    bus.push_byte(bus.registers.P.as_u8())?;
                    bus.registers.P.interrupt = true;
                    bus.registers.PC = bus.get_word(0xfffe);
                }
            }
            Self::RTI => {
                bus.registers.P = super::Status::from_u8(bus.pop_byte());
                bus.registers.PC = bus.pop_word();
            }
            // Comparison
            Self::CMP => {
                let res = bus.registers.A as i16 - operand.get_byte(bus)? as i16;
                bus.registers.P.carry = res >= 0;
                bus.registers.P.negative = (res as u8) & 0x80 == 0x80;
                bus.registers.P.zero = (res as u8) == 0;
            }
            Self::CPX => {
                let res = bus.registers.X as i16 - operand.get_byte(bus)? as i16;
                bus.registers.P.carry = res >= 0;
                bus.registers.P.negative = (res as u8) & 0x80 == 0x80;
                bus.registers.P.zero = (res as u8) == 0;
            }
            Self::CPY => {
                let res = bus.registers.Y as i16 - operand.get_byte(bus)? as i16;
                bus.registers.P.carry = res >= 0;
                bus.registers.P.negative = (res as u8) & 0x80 == 0x80;
                bus.registers.P.zero = (res as u8) == 0;
            }
            // Increment/Decrement
            Self::INC => {
                let addr = operand.get_address()?;
                let (res, _) = bus.get_byte(addr).overflowing_add(1);
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.set_byte(addr, res);
            }
            Self::DEC => {
                let addr = operand.get_address()?;
                let (res, _) = bus.get_byte(addr).overflowing_sub(1);
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.set_byte(addr, res);
            }
            Self::INX => {
                let (res, _) = bus.registers.X.overflowing_add(1);
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.X = res;
            }
            Self::DEX => {
                let (res, _) = bus.registers.X.overflowing_sub(1);
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.X = res;
            }
            Self::INY => {
                let (res, _) = bus.registers.Y.overflowing_add(1);
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.Y = res;
            }
            Self::DEY => {
                let (res, _) = bus.registers.Y.overflowing_sub(1);
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.Y = res;
            }
            // Flag controls
            Self::CLC => bus.registers.P.carry = false,
            Self::SEC => bus.registers.P.carry = true,
            Self::CLI => bus.registers.P.interrupt = false,
            Self::SEI => bus.registers.P.interrupt = true,
            Self::CLD => bus.registers.P.decimal_mode = false,
            Self::SED => bus.registers.P.decimal_mode = true,
            Self::CLV => bus.registers.P.overflow = false,
            // Load/Store
            Self::LDA => {
                let val = operand.get_byte(bus)?;
                bus.registers.P.negative = val & 0x80 == 0x80;
                bus.registers.P.zero = val == 0;
                bus.registers.A = val;
            }
            Self::LDX => {
                let val = operand.get_byte(bus)?;
                bus.registers.P.negative = val & 0x80 == 0x80;
                bus.registers.P.zero = val == 0;
                bus.registers.X = val;
            }
            Self::LDY => {
                let val = operand.get_byte(bus)?;
                bus.registers.P.negative = val & 0x80 == 0x80;
                bus.registers.P.zero = val == 0;
                bus.registers.Y = val;
            }
            Self::STA => bus.set_byte(operand.get_address()?, bus.registers.A),
            Self::STX => bus.set_byte(operand.get_address()?, bus.registers.X),
            Self::STY => bus.set_byte(operand.get_address()?, bus.registers.Y),
            // Transformations
            Self::TAX => bus.registers.X = bus.registers.A,
            Self::TXA => bus.registers.A = bus.registers.X,
            Self::TAY => bus.registers.Y = bus.registers.A,
            Self::TYA => bus.registers.A = bus.registers.Y,
            Self::TSX => bus.registers.X = bus.registers.S,
            Self::TXS => bus.registers.S = bus.registers.X,
            // Stack controls
            Self::PHA => bus.push_byte(bus.registers.A)?,
            Self::PLA => {
                let res = bus.pop_byte();
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.A = res;
            }
            Self::PHP => bus.push_byte(bus.registers.P.as_u8())?,
            Self::PLP => bus.registers.P = super::Status::from_u8(bus.pop_byte()),
            // NOP/KIL
            Self::NOP => {}
            Self::KIL => panic!("Process Killed by the instruction."),
            // Illegal Operations
            Self::SLO => {
                let addr = operand.get_address()?;
                let acc = bus.get_byte(addr);
                let (res, _) = acc.overflowing_mul(2);
                bus.registers.P.carry = acc & 0x80 == 0x80;
                bus.set_byte(addr, res);
                let res = bus.registers.A | res;
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.A = res;
            }
            Self::RLA => {
                let addr = operand.get_address()?;
                let acc = bus.get_byte(addr);
                let res = (acc << 1) | (bus.registers.P.carry as u8);
                bus.registers.P.carry = acc & 0x80 == 0x80;
                bus.set_byte(addr, res);
                let res = bus.registers.A & res;
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.A = res;
            }
            Self::SRE => {
                let addr = operand.get_address()?;
                let acc = bus.get_byte(addr);
                let res = acc >> 1;
                bus.registers.P.carry = acc & 0x1 == 0x1;
                bus.set_byte(addr, res);
                let res = bus.registers.A ^ res;
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.A = res;
            }
            Self::RRA => {
                let addr = operand.get_address()?;
                let acc = bus.get_byte(addr);
                let res = (acc >> 1) | (bus.registers.P.carry as u8 * 0x80);
                bus.registers.P.carry = acc & 0x1 == 0x1;
                bus.set_byte(addr, res);
                let acc = bus.registers.A;
                let op = bus.get_byte(addr);
                let (res, carry1) = acc.overflowing_add(op);
                let (res, carry2) = res.overflowing_add(bus.registers.P.carry as u8);
                bus.registers.P.overflow = ((acc ^ op) & 0x80 == 0) && ((acc ^ res) & 0x80 == 0x80);
                bus.registers.P.carry = carry1 || carry2;
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.A = res;
            }
            Self::SAX => bus.set_byte(operand.get_address()?, bus.registers.A & bus.registers.Y),
            Self::LAX => {
                let val = operand.get_byte(bus)?;
                bus.registers.P.negative = val & 0x80 == 0x80;
                bus.registers.P.zero = val == 0;
                bus.registers.A = val;
                bus.registers.X = val;
            }
            Self::DCP => {
                let addr = operand.get_address()?;
                let val = bus.get_byte(addr);
                let (res, _) = val.overflowing_sub(1);
                bus.set_byte(addr, res);
                let res = bus.registers.A as i16 - val as i16;
                bus.registers.P.carry = res >= 0;
                bus.registers.P.negative = (res as u8) & 0x80 == 0x80;
                bus.registers.P.zero = (res as u8) == 0;
            }
            Self::ISC => {
                let addr = operand.get_address()?;
                let val = bus.get_byte(addr);
                let (res, _) = val.overflowing_add(1);
                bus.set_byte(addr, res);
                let acc = bus.registers.A;
                let (res, carry1) = acc.overflowing_sub(val);
                let (res, carry2) = res.overflowing_sub((!bus.registers.P.carry) as u8);
                bus.registers.P.overflow =
                    ((acc ^ val) & 0x80 == 0x80) && ((acc ^ res) & 0x80 == 0x80);
                bus.registers.P.carry = carry1 || carry2;
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.A = res;
            }
            Self::ANC => {
                let res = bus.registers.A & operand.get_byte(bus)?;
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.carry = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.A = res;
            }
            Self::ALR => {
                let res1 = bus.registers.A & operand.get_byte(bus)?;
                let res2 = res1 >> 1;
                bus.registers.P.carry = res1 & 0x1 == 0x1;
                bus.registers.P.negative = res2 & 0x80 == 0x80;
                bus.registers.P.zero = res2 == 0;
                bus.registers.A = res2;
            }
            Self::ARR => {
                let res1 = bus.registers.A & operand.get_byte(bus)?;
                let res2 = (res1 >> 1) | (bus.registers.P.carry as u8 * 0x80);
                bus.registers.P.carry = res1 & 0x1 == 0x1;
                bus.registers.P.negative = res2 & 0x80 == 0x80;
                bus.registers.P.zero = res2 == 0;
                bus.registers.A = res2;
            }
            Self::XAA => {
                let res = bus.registers.X & operand.get_byte(bus)?;
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.A = res;
            }
            Self::AXS => {
                let res =
                    (bus.registers.A & bus.registers.X) as i16 - operand.get_byte(bus)? as i16;
                bus.registers.P.carry = res >= 0;
                bus.registers.P.negative = (res as u8) & 0x80 == 0x80;
                bus.registers.P.zero = (res as u8) == 0;
                bus.registers.X = res as u8;
            }
            Self::LAS => {
                let res = operand.get_byte(bus)? & bus.registers.S;
                bus.registers.P.negative = res & 0x80 == 0x80;
                bus.registers.P.zero = res == 0;
                bus.registers.A = res;
                bus.registers.X = res;
                bus.registers.Y = res;
            }
            Self::AHX | Self::SHX | Self::SHY | Self::TAS => {
                anyhow::bail!("{:?} is unimplemented.", self)
            }
        }
        Ok(branched)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Addressing {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Relative,
    Indirect,
    IndirectX,
    IndirectY,
}

impl Addressing {
    fn operand(&self, bus: &mut CpuBus) -> (Operand, bool, String) {
        match self {
            Self::Implied => (Operand::None, false, String::new()),
            Self::Accumulator => (Operand::Accumulator, false, String::new()),
            Self::Immediate => {
                let val = bus.increment_byte();
                (Operand::Value(val), false, format!("#{:02X}", val))
            }
            Self::ZeroPage => {
                let addr = bus.increment_byte() as u16;
                (Operand::Address(addr), false, format!("{:02X}", addr))
            }
            Self::ZeroPageX => {
                let addr = bus.increment_byte();
                (
                    Operand::Address(addr.overflowing_add(bus.registers.X).0 as u16),
                    false,
                    format!("{:02X}, X", addr),
                )
            }
            Self::ZeroPageY => {
                let addr = bus.increment_byte();
                (
                    Operand::Address(addr.overflowing_add(bus.registers.Y).0 as u16),
                    false,
                    format!("{:02X}, Y", addr),
                )
            }
            Self::Absolute => {
                let addr = bus.increment_word();
                (Operand::Address(addr), false, format!("{:04X}", addr))
            }
            Self::AbsoluteX => {
                let base = bus.increment_word();
                let addr = base + bus.registers.X as u16;
                (
                    Operand::Address(addr),
                    base / 0x100 != addr / 0x100,
                    format!("{:04X}, X", base),
                )
            }
            Self::AbsoluteY => {
                let base = bus.increment_word();
                let (addr, _) = base.overflowing_add(bus.registers.Y as u16);
                (
                    Operand::Address(addr),
                    base / 0x100 != addr / 0x100,
                    format!("{:04X}, X", base),
                )
            }
            Self::Relative => {
                let offset = bus.increment_byte() as i8;
                (
                    Operand::Address((bus.registers.PC as i16 + offset as i16) as u16),
                    false,
                    format!("{:02X}", offset),
                )
            }
            Self::Indirect => {
                let addr = bus.increment_word();
                (
                    Operand::Address(bus.get_word(addr)),
                    false,
                    format!("({:04X})", addr),
                )
            }
            Self::IndirectX => {
                let base = bus.increment_byte();
                let (addr, _) = base.overflowing_add(bus.registers.X);
                (
                    Operand::Address(bus.get_word_page(addr)),
                    false,
                    format!("({:02X}, X)", base),
                )
            }
            Self::IndirectY => {
                let base_addr = bus.increment_byte();
                let base = bus.get_word_page(base_addr);
                let (addr, _) = base.overflowing_add(bus.registers.Y as u16);
                (
                    Operand::Address(addr),
                    base / 0x100 != addr / 0x100,
                    format!("({:02X}), Y", base_addr),
                )
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operand {
    None,
    Accumulator,
    Value(u8),
    Address(u16),
}

impl Operand {
    fn get_address(self) -> anyhow::Result<u16> {
        match self {
            Self::Address(addr) => Ok(addr),
            _ => anyhow::bail!("Cannot get memory of the operand."),
        }
    }

    fn get_byte(self, bus: &mut CpuBus) -> anyhow::Result<u8> {
        match self {
            Self::Accumulator => Ok(bus.registers.A),
            Self::Value(val) => Ok(val),
            Self::Address(addr) => Ok(bus.get_byte(addr)),
            _ => anyhow::bail!("Missing the value."),
        }
    }
}
