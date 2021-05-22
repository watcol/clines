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

#[allow(dead_code)]
pub static OPCODES: [Opecode; 0x100] = [
    // 0x00
    Opecode {
        kind: Instruction::BRK,
        addr: Addressing::Implied,
        cycle: 7,
    },
    // 0x01
    Opecode {
        kind: Instruction::ORA,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0x02 (undocumented)
    Opecode {
        kind: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x03 (undocumented)
    Opecode {
        kind: Instruction::SLO,
        addr: Addressing::XIndirect,
        cycle: 8,
    },
    // 0x04 (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x05
    Opecode {
        kind: Instruction::ORA,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x06
    Opecode {
        kind: Instruction::ASL,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x07 (undocumented)
    Opecode {
        kind: Instruction::SLO,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x08
    Opecode {
        kind: Instruction::PHP,
        addr: Addressing::Implied,
        cycle: 3,
    },
    // 0x09
    Opecode {
        kind: Instruction::ORA,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x0a
    Opecode {
        kind: Instruction::ASL,
        addr: Addressing::Accumulator,
        cycle: 2,
    },
    // 0x0b (undocumented)
    Opecode {
        kind: Instruction::ANC,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x0c (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x0d
    Opecode {
        kind: Instruction::ORA,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x0e
    Opecode {
        kind: Instruction::ASL,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x0f (undocumented)
    Opecode {
        kind: Instruction::SLO,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x10
    Opecode {
        kind: Instruction::BPL,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0x11
    Opecode {
        kind: Instruction::ORA,
        addr: Addressing::IndirectY,
        cycle: 5,
    },
    // 0x12 (undocumented)
    Opecode {
        kind: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x13 (undocumented)
    Opecode {
        kind: Instruction::SLO,
        addr: Addressing::IndirectY,
        cycle: 8,
    },
    // 0x14 (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x15
    Opecode {
        kind: Instruction::ORA,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x16
    Opecode {
        kind: Instruction::ASL,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x17 (undocumented)
    Opecode {
        kind: Instruction::SLO,
        addr: Addressing::ZeroPageX,
        cycle: 0,
    },
    // 0x18
    Opecode {
        kind: Instruction::CLC,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x19
    Opecode {
        kind: Instruction::ORA,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0x1a (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x1b (undocumented)
    Opecode {
        kind: Instruction::SLO,
        addr: Addressing::AbsoluteY,
        cycle: 0,
    },
    // 0x1c (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x1d
    Opecode {
        kind: Instruction::ORA,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x1e
    Opecode {
        kind: Instruction::ASL,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0x1f (undocumented)
    Opecode {
        kind: Instruction::SLO,
        addr: Addressing::AbsoluteX,
        cycle: 0,
    },
    // 0x20
    Opecode {
        kind: Instruction::JSR,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x21
    Opecode {
        kind: Instruction::AND,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0x22 (undocumented)
    Opecode {
        kind: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x23 (undocumented)
    Opecode {
        kind: Instruction::RLA,
        addr: Addressing::XIndirect,
        cycle: 8,
    },
    // 0x24
    Opecode {
        kind: Instruction::BIT,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x25
    Opecode {
        kind: Instruction::AND,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x26
    Opecode {
        kind: Instruction::ROL,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x27 (undocumented)
    Opecode {
        kind: Instruction::RLA,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x28
    Opecode {
        kind: Instruction::PLP,
        addr: Addressing::Implied,
        cycle: 4,
    },
    // 0x29
    Opecode {
        kind: Instruction::AND,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x2a
    Opecode {
        kind: Instruction::ROL,
        addr: Addressing::Accumulator,
        cycle: 2,
    },
    // 0x2b (undocumented)
    Opecode {
        kind: Instruction::ANC,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x2c
    Opecode {
        kind: Instruction::BIT,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x2d
    Opecode {
        kind: Instruction::AND,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x2e
    Opecode {
        kind: Instruction::ROL,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x2f (undocumented)
    Opecode {
        kind: Instruction::RLA,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x30
    Opecode {
        kind: Instruction::BMI,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0x31
    Opecode {
        kind: Instruction::AND,
        addr: Addressing::IndirectY,
        cycle: 6,
    },
    // 0x32 (undocumented)
    Opecode {
        kind: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x33 (undocumented)
    Opecode {
        kind: Instruction::RLA,
        addr: Addressing::IndirectY,
        cycle: 8,
    },
    // 0x34
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x35
    Opecode {
        kind: Instruction::AND,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x36
    Opecode {
        kind: Instruction::ROL,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0x37 (undocumented)
    Opecode {
        kind: Instruction::RLA,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0x38
    Opecode {
        kind: Instruction::SEC,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x39
    Opecode {
        kind: Instruction::AND,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0x3a (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x3b (undocumented)
    Opecode {
        kind: Instruction::RLA,
        addr: Addressing::AbsoluteY,
        cycle: 7,
    },
    // 0x3c (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x3d
    Opecode {
        kind: Instruction::AND,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x3e
    Opecode {
        kind: Instruction::ROL,
        addr: Addressing::Absolute,
        cycle: 7,
    },
    // 0x3f (undocumented)
    Opecode {
        kind: Instruction::RLA,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0x40
    Opecode {
        kind: Instruction::RTI,
        addr: Addressing::Implied,
        cycle: 6,
    },
    // 0x41
    Opecode {
        kind: Instruction::EOR,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0x42 (undocumented)
    Opecode {
        kind: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x43 (undocumented)
    Opecode {
        kind: Instruction::SRE,
        addr: Addressing::IndirectY,
        cycle: 3,
    },
    // 0x44 (undocumented)
    Opecode {
        kind: Instruction::BRK,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x45
    Opecode {
        kind: Instruction::EOR,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x46
    Opecode {
        kind: Instruction::LSR,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x47 (undocumented)
    Opecode {
        kind: Instruction::SRE,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x48
    Opecode {
        kind: Instruction::PHA,
        addr: Addressing::Implied,
        cycle: 3,
    },
    // 0x49
    Opecode {
        kind: Instruction::EOR,
        addr: Addressing::Immediate,
        cycle: 3,
    },
    // 0x4a
    Opecode {
        kind: Instruction::LSR,
        addr: Addressing::Accumulator,
        cycle: 2,
    },
    // 0x4b (undocumented)
    Opecode {
        kind: Instruction::ALR,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x4c
    Opecode {
        kind: Instruction::JMP,
        addr: Addressing::Absolute,
        cycle: 3,
    },
    // 0x4d
    Opecode {
        kind: Instruction::EOR,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x4e (undocumented)
    Opecode {
        kind: Instruction::LSR,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x4f
    Opecode {
        kind: Instruction::SRE,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x50
    Opecode {
        kind: Instruction::BVC,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0x51
    Opecode {
        kind: Instruction::EOR,
        addr: Addressing::IndirectY,
        cycle: 5,
    },
    // 0x52 (undocumented)
    Opecode {
        kind: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x53 (undocumented)
    Opecode {
        kind: Instruction::SRE,
        addr: Addressing::XIndirect,
        cycle: 8,
    },
    // 0x54 (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::ZeroPage,
        cycle: 4,
    },
    // 0x55
    Opecode {
        kind: Instruction::EOR,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x56
    Opecode {
        kind: Instruction::LSR,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0x57 (undocumented)
    Opecode {
        kind: Instruction::SRE,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0x58
    Opecode {
        kind: Instruction::CLI,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x59
    Opecode {
        kind: Instruction::EOR,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0x5a (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x5b (undocumented)
    Opecode {
        kind: Instruction::SRE,
        addr: Addressing::AbsoluteY,
        cycle: 7,
    },
    // 0x5c (undocumented)
    Opecode {
        kind: Instruction::SRE,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x5d
    Opecode {
        kind: Instruction::EOR,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x5e
    Opecode {
        kind: Instruction::LSR,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0x5f (undocumented)
    Opecode {
        kind: Instruction::SRE,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0x60
    Opecode {
        kind: Instruction::RTS,
        addr: Addressing::Implied,
        cycle: 6,
    },
    // 0x61
    Opecode {
        kind: Instruction::ADC,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0x62 (undocumented)
    Opecode {
        kind: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x63 (undocumented)
    Opecode {
        kind: Instruction::RRA,
        addr: Addressing::XIndirect,
        cycle: 8,
    },
    // 0x64 (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x65
    Opecode {
        kind: Instruction::ADC,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x66
    Opecode {
        kind: Instruction::ROR,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x67 (undocumented)
    Opecode {
        kind: Instruction::RRA,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x68
    Opecode {
        kind: Instruction::PLA,
        addr: Addressing::Implied,
        cycle: 4,
    },
    // 0x69
    Opecode {
        kind: Instruction::ADC,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x6a
    Opecode {
        kind: Instruction::ROR,
        addr: Addressing::Accumulator,
        cycle: 2,
    },
    // 0x6b (undocumented)
    Opecode {
        kind: Instruction::ARR,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x6c
    Opecode {
        kind: Instruction::JMP,
        addr: Addressing::Indirect,
        cycle: 5,
    },
    // 0x6d
    Opecode {
        kind: Instruction::ADC,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x6e
    Opecode {
        kind: Instruction::ROR,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x6f (undocumented)
    Opecode {
        kind: Instruction::RRA,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x70
    Opecode {
        kind: Instruction::BVS,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0x71
    Opecode {
        kind: Instruction::ADC,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0x72 (undocumented)
    Opecode {
        kind: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x73 (undocumented)
    Opecode {
        kind: Instruction::RRA,
        addr: Addressing::IndirectY,
        cycle: 8,
    },
    // 0x74 (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x75
    Opecode {
        kind: Instruction::ADC,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x76
    Opecode {
        kind: Instruction::ROR,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0x77 (undocumented)
    Opecode {
        kind: Instruction::RRA,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0x78
    Opecode {
        kind: Instruction::SEI,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x79
    Opecode {
        kind: Instruction::ADC,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0x7a (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x7b (undocumented)
    Opecode {
        kind: Instruction::RRA,
        addr: Addressing::AbsoluteY,
        cycle: 7,
    },
    // 0x7c (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x7d
    Opecode {
        kind: Instruction::ADC,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x7e
    Opecode {
        kind: Instruction::ROR,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0x7f (undocumented)
    Opecode {
        kind: Instruction::RRA,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0x80 (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x81
    Opecode {
        kind: Instruction::STA,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0x82 (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x83 (undocumented)
    Opecode {
        kind: Instruction::SAX,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0x84
    Opecode {
        kind: Instruction::STY,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x85
    Opecode {
        kind: Instruction::STA,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x86
    Opecode {
        kind: Instruction::STX,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x87 (undocumented)
    Opecode {
        kind: Instruction::SAX,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x88
    Opecode {
        kind: Instruction::DEY,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x89 (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x8a
    Opecode {
        kind: Instruction::TXA,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x8b (undocumented)
    Opecode {
        kind: Instruction::XAA,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x8c
    Opecode {
        kind: Instruction::STY,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x8d
    Opecode {
        kind: Instruction::STA,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x8e
    Opecode {
        kind: Instruction::STX,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x8f (undocumented)
    Opecode {
        kind: Instruction::SAX,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x90
    Opecode {
        kind: Instruction::BCC,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0x91
    Opecode {
        kind: Instruction::STA,
        addr: Addressing::IndirectY,
        cycle: 6,
    },
    // 0x92 (undocumented)
    Opecode {
        kind: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x93 (undocumented)
    Opecode {
        kind: Instruction::AHX,
        addr: Addressing::IndirectY,
        cycle: 6,
    },
    // 0x94
    Opecode {
        kind: Instruction::STY,
        addr: Addressing::ZeroPageX,
        cycle: 0,
    },
    // 0x95
    Opecode {
        kind: Instruction::STA,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x96
    Opecode {
        kind: Instruction::STX,
        addr: Addressing::ZeroPageY,
        cycle: 0,
    },
    // 0x97
    Opecode {
        kind: Instruction::SAX,
        addr: Addressing::ZeroPageY,
        cycle: 4,
    },
    // 0x98
    Opecode {
        kind: Instruction::TYA,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x99
    Opecode {
        kind: Instruction::STA,
        addr: Addressing::AbsoluteY,
        cycle: 5,
    },
    // 0x9a
    Opecode {
        kind: Instruction::TXS,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x9b (undocumented)
    Opecode {
        kind: Instruction::TAS,
        addr: Addressing::AbsoluteY,
        cycle: 5,
    },
    // 0x9c (undocumented)
    Opecode {
        kind: Instruction::SHY,
        addr: Addressing::AbsoluteX,
        cycle: 5,
    },
    // 0x9d
    Opecode {
        kind: Instruction::STA,
        addr: Addressing::AbsoluteX,
        cycle: 5,
    },
    // 0x9e (undocumented)
    Opecode {
        kind: Instruction::SHX,
        addr: Addressing::AbsoluteY,
        cycle: 5,
    },
    // 0x9f (undocumented)
    Opecode {
        kind: Instruction::AHX,
        addr: Addressing::AbsoluteY,
        cycle: 5,
    },
    // 0xa0
    Opecode {
        kind: Instruction::LDY,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xa1
    Opecode {
        kind: Instruction::LDA,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0xa2
    Opecode {
        kind: Instruction::LDX,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xa3 (undocumented)
    Opecode {
        kind: Instruction::LAX,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0xa4
    Opecode {
        kind: Instruction::LDY,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0xa5
    Opecode {
        kind: Instruction::LDA,
        addr: Addressing::ZeroPage,
        cycle: 0,
    },
    // 0xa6
    Opecode {
        kind: Instruction::LDX,
        addr: Addressing::ZeroPage,
        cycle: 0,
    },
    // 0xa7 (undocumented)
    Opecode {
        kind: Instruction::LAX,
        addr: Addressing::ZeroPage,
        cycle: 2,
    },
    // 0xa8
    Opecode {
        kind: Instruction::TAY,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xa9
    Opecode {
        kind: Instruction::LDA,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xaa
    Opecode {
        kind: Instruction::TAX,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xab (undocumented)
    Opecode {
        kind: Instruction::LAX,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xac
    Opecode {
        kind: Instruction::LDY,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xad
    Opecode {
        kind: Instruction::LDA,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xae
    Opecode {
        kind: Instruction::LDX,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xaf (undocumented)
    Opecode {
        kind: Instruction::LAX,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xb0
    Opecode {
        kind: Instruction::BCS,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0xb1
    Opecode {
        kind: Instruction::LDA,
        addr: Addressing::IndirectY,
        cycle: 5,
    },
    // 0xb2 (undocumented)
    Opecode {
        kind: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0xb3 (undocumented)
    Opecode {
        kind: Instruction::LAX,
        addr: Addressing::IndirectY,
        cycle: 5,
    },
    // 0xb4
    Opecode {
        kind: Instruction::LDY,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0xb5
    Opecode {
        kind: Instruction::LDA,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0xb6
    Opecode {
        kind: Instruction::LDX,
        addr: Addressing::ZeroPageY,
        cycle: 4,
    },
    // 0xb7 (undocumented)
    Opecode {
        kind: Instruction::LAX,
        addr: Addressing::ZeroPageY,
        cycle: 4,
    },
    // 0xb8
    Opecode {
        kind: Instruction::CLV,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xb9
    Opecode {
        kind: Instruction::LDA,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0xba
    Opecode {
        kind: Instruction::TSX,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xbb (undocumented)
    Opecode {
        kind: Instruction::LAS,
        addr: Addressing::AbsoluteY,
        cycle: 0,
    },
    // 0xbc
    Opecode {
        kind: Instruction::LDY,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0xbd
    Opecode {
        kind: Instruction::LDA,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0xbe
    Opecode {
        kind: Instruction::LDX,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0xbf (undocumented)
    Opecode {
        kind: Instruction::LAX,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0xc0
    Opecode {
        kind: Instruction::CPY,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xc1
    Opecode {
        kind: Instruction::CMP,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0xc2 (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::Immediate,
        cycle: 6,
    },
    // 0xc3 (undocumented)
    Opecode {
        kind: Instruction::DCP,
        addr: Addressing::XIndirect,
        cycle: 8,
    },
    // 0xc4
    Opecode {
        kind: Instruction::CPY,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0xc5
    Opecode {
        kind: Instruction::CMP,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0xc6
    Opecode {
        kind: Instruction::DEC,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0xc7 (undocumented)
    Opecode {
        kind: Instruction::DCP,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0xc8
    Opecode {
        kind: Instruction::INY,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xc9
    Opecode {
        kind: Instruction::CMP,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xca
    Opecode {
        kind: Instruction::DEX,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xcb (undocumented)
    Opecode {
        kind: Instruction::AXS,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xcc
    Opecode {
        kind: Instruction::CPY,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xcd
    Opecode {
        kind: Instruction::CMP,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xce
    Opecode {
        kind: Instruction::DEC,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0xcf (undocumented)
    Opecode {
        kind: Instruction::DCP,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0xd0
    Opecode {
        kind: Instruction::BNE,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0xd1
    Opecode {
        kind: Instruction::CMP,
        addr: Addressing::IndirectY,
        cycle: 5,
    },
    // 0xd2 (undocumented)
    Opecode {
        kind: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0xd3 (undocumented)
    Opecode {
        kind: Instruction::DCP,
        addr: Addressing::IndirectY,
        cycle: 0,
    },
    // 0xd4 (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0xd5
    Opecode {
        kind: Instruction::CMP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0xd6
    Opecode {
        kind: Instruction::DEC,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0xd7 (undocumented)
    Opecode {
        kind: Instruction::DCP,
        addr: Addressing::ZeroPage,
        cycle: 6,
    },
    // 0xd8
    Opecode {
        kind: Instruction::CLD,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xd9
    Opecode {
        kind: Instruction::CMP,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0xda (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xdb (undocumented)
    Opecode {
        kind: Instruction::DCP,
        addr: Addressing::AbsoluteY,
        cycle: 7,
    },
    // 0xdc (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0xdd
    Opecode {
        kind: Instruction::CMP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0xde
    Opecode {
        kind: Instruction::DEC,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0xdf (undocumented)
    Opecode {
        kind: Instruction::DCP,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0xe0
    Opecode {
        kind: Instruction::CPX,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xe1
    Opecode {
        kind: Instruction::SBC,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0xe2 (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xe3 (undocumented)
    Opecode {
        kind: Instruction::ISC,
        addr: Addressing::XIndirect,
        cycle: 8,
    },
    // 0xe4
    Opecode {
        kind: Instruction::CPX,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0xe5
    Opecode {
        kind: Instruction::SBC,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0xe6
    Opecode {
        kind: Instruction::INC,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0xe7 (undocumented)
    Opecode {
        kind: Instruction::ISC,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0xe8
    Opecode {
        kind: Instruction::INX,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xe9
    Opecode {
        kind: Instruction::SBC,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xea
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xeb (undocumented)
    Opecode {
        kind: Instruction::SBC,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xec
    Opecode {
        kind: Instruction::CPX,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xed
    Opecode {
        kind: Instruction::SBC,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xee
    Opecode {
        kind: Instruction::INC,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0xef (undocumented)
    Opecode {
        kind: Instruction::ISC,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0xf0
    Opecode {
        kind: Instruction::BEQ,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0xf1
    Opecode {
        kind: Instruction::SBC,
        addr: Addressing::IndirectY,
        cycle: 5,
    },
    // 0xf2 (undocumented)
    Opecode {
        kind: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0xf3 (undocumented)
    Opecode {
        kind: Instruction::ISC,
        addr: Addressing::IndirectY,
        cycle: 8,
    },
    // 0xf4 (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0xf5
    Opecode {
        kind: Instruction::SBC,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0xf6
    Opecode {
        kind: Instruction::INC,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0xf7 (undocumented)
    Opecode {
        kind: Instruction::ISC,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0xf8
    Opecode {
        kind: Instruction::SED,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xf9
    Opecode {
        kind: Instruction::SBC,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0xfa (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xfb (undocumented)
    Opecode {
        kind: Instruction::ISC,
        addr: Addressing::AbsoluteY,
        cycle: 7,
    },
    // 0xfc (undocumented)
    Opecode {
        kind: Instruction::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0xfd
    Opecode {
        kind: Instruction::SBC,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0xfe
    Opecode {
        kind: Instruction::INC,
        addr: Addressing::AbsoluteX,
        cycle: 0,
    },
    // 0xff (undocumented)
    Opecode {
        kind: Instruction::ISC,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Opecode {
    kind: Instruction,
    addr: Addressing,
    cycle: u8,
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
    XIndirect,
    IndirectY,
    Indirect,
}
