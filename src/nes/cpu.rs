#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cpu {
    reg: Register,
    mem: [u8; 0x10000],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct Register {
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
pub static OPCODES: [Instruction; 0x100] = [
    // 0x00
    Instruction {
        kind: InstKind::BRK,
        addr: Addressing::Implied,
        cycle: 7,
    },
    // 0x01
    Instruction {
        kind: InstKind::ORA,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0x02 (undocumented)
    Instruction {
        kind: InstKind::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x03 (undocumented)
    Instruction {
        kind: InstKind::SLO,
        addr: Addressing::XIndirect,
        cycle: 8,
    },
    // 0x04 (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x05
    Instruction {
        kind: InstKind::ORA,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x06
    Instruction {
        kind: InstKind::ASL,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x07 (undocumented)
    Instruction {
        kind: InstKind::SLO,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x08
    Instruction {
        kind: InstKind::PHP,
        addr: Addressing::Implied,
        cycle: 3,
    },
    // 0x09
    Instruction {
        kind: InstKind::ORA,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x0a
    Instruction {
        kind: InstKind::ASL,
        addr: Addressing::Accumulator,
        cycle: 2,
    },
    // 0x0b (undocumented)
    Instruction {
        kind: InstKind::ANC,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x0c (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x0d
    Instruction {
        kind: InstKind::ORA,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x0e
    Instruction {
        kind: InstKind::ASL,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x0f (undocumented)
    Instruction {
        kind: InstKind::SLO,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x10
    Instruction {
        kind: InstKind::BPL,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0x11
    Instruction {
        kind: InstKind::ORA,
        addr: Addressing::IndirectY,
        cycle: 5,
    },
    // 0x12 (undocumented)
    Instruction {
        kind: InstKind::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x13 (undocumented)
    Instruction {
        kind: InstKind::SLO,
        addr: Addressing::IndirectY,
        cycle: 8,
    },
    // 0x14 (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x15
    Instruction {
        kind: InstKind::ORA,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x16
    Instruction {
        kind: InstKind::ASL,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x17 (undocumented)
    Instruction {
        kind: InstKind::SLO,
        addr: Addressing::ZeroPageX,
        cycle: 0,
    },
    // 0x18
    Instruction {
        kind: InstKind::CLC,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x19
    Instruction {
        kind: InstKind::ORA,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0x1a (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x1b (undocumented)
    Instruction {
        kind: InstKind::SLO,
        addr: Addressing::AbsoluteY,
        cycle: 0,
    },
    // 0x1c (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x1d
    Instruction {
        kind: InstKind::ORA,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x1e
    Instruction {
        kind: InstKind::ASL,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0x1f (undocumented)
    Instruction {
        kind: InstKind::SLO,
        addr: Addressing::AbsoluteX,
        cycle: 0,
    },
    // 0x20
    Instruction {
        kind: InstKind::JSR,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x21
    Instruction {
        kind: InstKind::AND,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0x22 (undocumented)
    Instruction {
        kind: InstKind::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x23 (undocumented)
    Instruction {
        kind: InstKind::RLA,
        addr: Addressing::XIndirect,
        cycle: 8,
    },
    // 0x24
    Instruction {
        kind: InstKind::BIT,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x25
    Instruction {
        kind: InstKind::AND,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x26
    Instruction {
        kind: InstKind::ROL,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x27 (undocumented)
    Instruction {
        kind: InstKind::RLA,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x28
    Instruction {
        kind: InstKind::PLP,
        addr: Addressing::Implied,
        cycle: 4,
    },
    // 0x29
    Instruction {
        kind: InstKind::AND,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x2a
    Instruction {
        kind: InstKind::ROL,
        addr: Addressing::Accumulator,
        cycle: 2,
    },
    // 0x2b (undocumented)
    Instruction {
        kind: InstKind::ANC,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x2c
    Instruction {
        kind: InstKind::BIT,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x2d
    Instruction {
        kind: InstKind::AND,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x2e
    Instruction {
        kind: InstKind::ROL,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x2f (undocumented)
    Instruction {
        kind: InstKind::RLA,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x30
    Instruction {
        kind: InstKind::BMI,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0x31
    Instruction {
        kind: InstKind::AND,
        addr: Addressing::IndirectY,
        cycle: 6,
    },
    // 0x32 (undocumented)
    Instruction {
        kind: InstKind::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x33 (undocumented)
    Instruction {
        kind: InstKind::RLA,
        addr: Addressing::IndirectY,
        cycle: 8,
    },
    // 0x34
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x35
    Instruction {
        kind: InstKind::AND,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x36
    Instruction {
        kind: InstKind::ROL,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0x37 (undocumented)
    Instruction {
        kind: InstKind::RLA,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0x38
    Instruction {
        kind: InstKind::SEC,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x39
    Instruction {
        kind: InstKind::AND,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0x3a (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x3b (undocumented)
    Instruction {
        kind: InstKind::RLA,
        addr: Addressing::AbsoluteY,
        cycle: 7,
    },
    // 0x3c (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x3d
    Instruction {
        kind: InstKind::AND,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x3e
    Instruction {
        kind: InstKind::ROL,
        addr: Addressing::Absolute,
        cycle: 7,
    },
    // 0x3f (undocumented)
    Instruction {
        kind: InstKind::RLA,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0x40
    Instruction {
        kind: InstKind::RTI,
        addr: Addressing::Implied,
        cycle: 6,
    },
    // 0x41
    Instruction {
        kind: InstKind::EOR,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0x42 (undocumented)
    Instruction {
        kind: InstKind::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x43 (undocumented)
    Instruction {
        kind: InstKind::SRE,
        addr: Addressing::IndirectY,
        cycle: 3,
    },
    // 0x44 (undocumented)
    Instruction {
        kind: InstKind::BRK,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x45
    Instruction {
        kind: InstKind::EOR,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x46
    Instruction {
        kind: InstKind::LSR,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x47 (undocumented)
    Instruction {
        kind: InstKind::SRE,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x48
    Instruction {
        kind: InstKind::PHA,
        addr: Addressing::Implied,
        cycle: 3,
    },
    // 0x49
    Instruction {
        kind: InstKind::EOR,
        addr: Addressing::Immediate,
        cycle: 3,
    },
    // 0x4a
    Instruction {
        kind: InstKind::LSR,
        addr: Addressing::Accumulator,
        cycle: 2,
    },
    // 0x4b (undocumented)
    Instruction {
        kind: InstKind::ALR,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x4c
    Instruction {
        kind: InstKind::JMP,
        addr: Addressing::Absolute,
        cycle: 3,
    },
    // 0x4d
    Instruction {
        kind: InstKind::EOR,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x4e (undocumented)
    Instruction {
        kind: InstKind::LSR,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x4f
    Instruction {
        kind: InstKind::SRE,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x50
    Instruction {
        kind: InstKind::BVC,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0x51
    Instruction {
        kind: InstKind::EOR,
        addr: Addressing::IndirectY,
        cycle: 5,
    },
    // 0x52 (undocumented)
    Instruction {
        kind: InstKind::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x53 (undocumented)
    Instruction {
        kind: InstKind::SRE,
        addr: Addressing::XIndirect,
        cycle: 8,
    },
    // 0x54 (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::ZeroPage,
        cycle: 4,
    },
    // 0x55
    Instruction {
        kind: InstKind::EOR,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x56
    Instruction {
        kind: InstKind::LSR,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0x57 (undocumented)
    Instruction {
        kind: InstKind::SRE,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0x58
    Instruction {
        kind: InstKind::CLI,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x59
    Instruction {
        kind: InstKind::EOR,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0x5a (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x5b (undocumented)
    Instruction {
        kind: InstKind::SRE,
        addr: Addressing::AbsoluteY,
        cycle: 7,
    },
    // 0x5c (undocumented)
    Instruction {
        kind: InstKind::SRE,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x5d
    Instruction {
        kind: InstKind::EOR,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x5e
    Instruction {
        kind: InstKind::LSR,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0x5f (undocumented)
    Instruction {
        kind: InstKind::SRE,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0x60
    Instruction {
        kind: InstKind::RTS,
        addr: Addressing::Implied,
        cycle: 6,
    },
    // 0x61
    Instruction {
        kind: InstKind::ADC,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0x62 (undocumented)
    Instruction {
        kind: InstKind::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x63 (undocumented)
    Instruction {
        kind: InstKind::RRA,
        addr: Addressing::XIndirect,
        cycle: 8,
    },
    // 0x64 (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x65
    Instruction {
        kind: InstKind::ADC,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x66
    Instruction {
        kind: InstKind::ROR,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x67 (undocumented)
    Instruction {
        kind: InstKind::RRA,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0x68
    Instruction {
        kind: InstKind::PLA,
        addr: Addressing::Implied,
        cycle: 4,
    },
    // 0x69
    Instruction {
        kind: InstKind::ADC,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x6a
    Instruction {
        kind: InstKind::ROR,
        addr: Addressing::Accumulator,
        cycle: 2,
    },
    // 0x6b (undocumented)
    Instruction {
        kind: InstKind::ARR,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x6c
    Instruction {
        kind: InstKind::JMP,
        addr: Addressing::Indirect,
        cycle: 5,
    },
    // 0x6d
    Instruction {
        kind: InstKind::ADC,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x6e
    Instruction {
        kind: InstKind::ROR,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x6f (undocumented)
    Instruction {
        kind: InstKind::RRA,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0x70
    Instruction {
        kind: InstKind::BVS,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0x71
    Instruction {
        kind: InstKind::ADC,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0x72 (undocumented)
    Instruction {
        kind: InstKind::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x73 (undocumented)
    Instruction {
        kind: InstKind::RRA,
        addr: Addressing::IndirectY,
        cycle: 8,
    },
    // 0x74 (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x75
    Instruction {
        kind: InstKind::ADC,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x76
    Instruction {
        kind: InstKind::ROR,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0x77 (undocumented)
    Instruction {
        kind: InstKind::RRA,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0x78
    Instruction {
        kind: InstKind::SEI,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x79
    Instruction {
        kind: InstKind::ADC,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0x7a (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x7b (undocumented)
    Instruction {
        kind: InstKind::RRA,
        addr: Addressing::AbsoluteY,
        cycle: 7,
    },
    // 0x7c (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x7d
    Instruction {
        kind: InstKind::ADC,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0x7e
    Instruction {
        kind: InstKind::ROR,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0x7f (undocumented)
    Instruction {
        kind: InstKind::RRA,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0x80 (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x81
    Instruction {
        kind: InstKind::STA,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0x82 (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x83 (undocumented)
    Instruction {
        kind: InstKind::SAX,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0x84
    Instruction {
        kind: InstKind::STY,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x85
    Instruction {
        kind: InstKind::STA,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x86
    Instruction {
        kind: InstKind::STX,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x87 (undocumented)
    Instruction {
        kind: InstKind::SAX,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0x88
    Instruction {
        kind: InstKind::DEY,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x89 (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x8a
    Instruction {
        kind: InstKind::TXA,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x8b (undocumented)
    Instruction {
        kind: InstKind::XAA,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0x8c
    Instruction {
        kind: InstKind::STY,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x8d
    Instruction {
        kind: InstKind::STA,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x8e
    Instruction {
        kind: InstKind::STX,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x8f (undocumented)
    Instruction {
        kind: InstKind::SAX,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0x90
    Instruction {
        kind: InstKind::BCC,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0x91
    Instruction {
        kind: InstKind::STA,
        addr: Addressing::IndirectY,
        cycle: 6,
    },
    // 0x92 (undocumented)
    Instruction {
        kind: InstKind::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0x93 (undocumented)
    Instruction {
        kind: InstKind::AHX,
        addr: Addressing::IndirectY,
        cycle: 6,
    },
    // 0x94
    Instruction {
        kind: InstKind::STY,
        addr: Addressing::ZeroPageX,
        cycle: 0,
    },
    // 0x95
    Instruction {
        kind: InstKind::STA,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0x96
    Instruction {
        kind: InstKind::STX,
        addr: Addressing::ZeroPageY,
        cycle: 0,
    },
    // 0x97
    Instruction {
        kind: InstKind::SAX,
        addr: Addressing::ZeroPageY,
        cycle: 4,
    },
    // 0x98
    Instruction {
        kind: InstKind::TYA,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x99
    Instruction {
        kind: InstKind::STA,
        addr: Addressing::AbsoluteY,
        cycle: 5,
    },
    // 0x9a
    Instruction {
        kind: InstKind::TXS,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0x9b (undocumented)
    Instruction {
        kind: InstKind::TAS,
        addr: Addressing::AbsoluteY,
        cycle: 5,
    },
    // 0x9c (undocumented)
    Instruction {
        kind: InstKind::SHY,
        addr: Addressing::AbsoluteX,
        cycle: 5,
    },
    // 0x9d
    Instruction {
        kind: InstKind::STA,
        addr: Addressing::AbsoluteX,
        cycle: 5,
    },
    // 0x9e (undocumented)
    Instruction {
        kind: InstKind::SHX,
        addr: Addressing::AbsoluteY,
        cycle: 5,
    },
    // 0x9f (undocumented)
    Instruction {
        kind: InstKind::AHX,
        addr: Addressing::AbsoluteY,
        cycle: 5,
    },
    // 0xa0
    Instruction {
        kind: InstKind::LDY,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xa1
    Instruction {
        kind: InstKind::LDA,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0xa2
    Instruction {
        kind: InstKind::LDX,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xa3 (undocumented)
    Instruction {
        kind: InstKind::LAX,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0xa4
    Instruction {
        kind: InstKind::LDY,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0xa5
    Instruction {
        kind: InstKind::LDA,
        addr: Addressing::ZeroPage,
        cycle: 0,
    },
    // 0xa6
    Instruction {
        kind: InstKind::LDX,
        addr: Addressing::ZeroPage,
        cycle: 0,
    },
    // 0xa7 (undocumented)
    Instruction {
        kind: InstKind::LAX,
        addr: Addressing::ZeroPage,
        cycle: 2,
    },
    // 0xa8
    Instruction {
        kind: InstKind::TAY,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xa9
    Instruction {
        kind: InstKind::LDA,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xaa
    Instruction {
        kind: InstKind::TAX,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xab (undocumented)
    Instruction {
        kind: InstKind::LAX,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xac
    Instruction {
        kind: InstKind::LDY,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xad
    Instruction {
        kind: InstKind::LDA,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xae
    Instruction {
        kind: InstKind::LDX,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xaf (undocumented)
    Instruction {
        kind: InstKind::LAX,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xb0
    Instruction {
        kind: InstKind::BCS,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0xb1
    Instruction {
        kind: InstKind::LDA,
        addr: Addressing::IndirectY,
        cycle: 5,
    },
    // 0xb2 (undocumented)
    Instruction {
        kind: InstKind::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0xb3 (undocumented)
    Instruction {
        kind: InstKind::LAX,
        addr: Addressing::IndirectY,
        cycle: 5,
    },
    // 0xb4
    Instruction {
        kind: InstKind::LDY,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0xb5
    Instruction {
        kind: InstKind::LDA,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0xb6
    Instruction {
        kind: InstKind::LDX,
        addr: Addressing::ZeroPageY,
        cycle: 4,
    },
    // 0xb7 (undocumented)
    Instruction {
        kind: InstKind::LAX,
        addr: Addressing::ZeroPageY,
        cycle: 4,
    },
    // 0xb8
    Instruction {
        kind: InstKind::CLV,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xb9
    Instruction {
        kind: InstKind::LDA,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0xba
    Instruction {
        kind: InstKind::TSX,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xbb (undocumented)
    Instruction {
        kind: InstKind::LAS,
        addr: Addressing::AbsoluteY,
        cycle: 0,
    },
    // 0xbc
    Instruction {
        kind: InstKind::LDY,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0xbd
    Instruction {
        kind: InstKind::LDA,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0xbe
    Instruction {
        kind: InstKind::LDX,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0xbf (undocumented)
    Instruction {
        kind: InstKind::LAX,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0xc0
    Instruction {
        kind: InstKind::CPY,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xc1
    Instruction {
        kind: InstKind::CMP,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0xc2 (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::Immediate,
        cycle: 6,
    },
    // 0xc3 (undocumented)
    Instruction {
        kind: InstKind::DCP,
        addr: Addressing::XIndirect,
        cycle: 8,
    },
    // 0xc4
    Instruction {
        kind: InstKind::CPY,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0xc5
    Instruction {
        kind: InstKind::CMP,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0xc6
    Instruction {
        kind: InstKind::DEC,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0xc7 (undocumented)
    Instruction {
        kind: InstKind::DCP,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0xc8
    Instruction {
        kind: InstKind::INY,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xc9
    Instruction {
        kind: InstKind::CMP,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xca
    Instruction {
        kind: InstKind::DEX,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xcb (undocumented)
    Instruction {
        kind: InstKind::AXS,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xcc
    Instruction {
        kind: InstKind::CPY,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xcd
    Instruction {
        kind: InstKind::CMP,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xce
    Instruction {
        kind: InstKind::DEC,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0xcf (undocumented)
    Instruction {
        kind: InstKind::DCP,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0xd0
    Instruction {
        kind: InstKind::BNE,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0xd1
    Instruction {
        kind: InstKind::CMP,
        addr: Addressing::IndirectY,
        cycle: 5,
    },
    // 0xd2 (undocumented)
    Instruction {
        kind: InstKind::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0xd3 (undocumented)
    Instruction {
        kind: InstKind::DCP,
        addr: Addressing::IndirectY,
        cycle: 0,
    },
    // 0xd4 (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0xd5
    Instruction {
        kind: InstKind::CMP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0xd6
    Instruction {
        kind: InstKind::DEC,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0xd7 (undocumented)
    Instruction {
        kind: InstKind::DCP,
        addr: Addressing::ZeroPage,
        cycle: 6,
    },
    // 0xd8
    Instruction {
        kind: InstKind::CLD,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xd9
    Instruction {
        kind: InstKind::CMP,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0xda (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xdb (undocumented)
    Instruction {
        kind: InstKind::DCP,
        addr: Addressing::AbsoluteY,
        cycle: 7,
    },
    // 0xdc (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0xdd
    Instruction {
        kind: InstKind::CMP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0xde
    Instruction {
        kind: InstKind::DEC,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0xdf (undocumented)
    Instruction {
        kind: InstKind::DCP,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
    // 0xe0
    Instruction {
        kind: InstKind::CPX,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xe1
    Instruction {
        kind: InstKind::SBC,
        addr: Addressing::XIndirect,
        cycle: 6,
    },
    // 0xe2 (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xe3 (undocumented)
    Instruction {
        kind: InstKind::ISC,
        addr: Addressing::XIndirect,
        cycle: 8,
    },
    // 0xe4
    Instruction {
        kind: InstKind::CPX,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0xe5
    Instruction {
        kind: InstKind::SBC,
        addr: Addressing::ZeroPage,
        cycle: 3,
    },
    // 0xe6
    Instruction {
        kind: InstKind::INC,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0xe7 (undocumented)
    Instruction {
        kind: InstKind::ISC,
        addr: Addressing::ZeroPage,
        cycle: 5,
    },
    // 0xe8
    Instruction {
        kind: InstKind::INX,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xe9
    Instruction {
        kind: InstKind::SBC,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xea
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xeb (undocumented)
    Instruction {
        kind: InstKind::SBC,
        addr: Addressing::Immediate,
        cycle: 2,
    },
    // 0xec
    Instruction {
        kind: InstKind::CPX,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xed
    Instruction {
        kind: InstKind::SBC,
        addr: Addressing::Absolute,
        cycle: 4,
    },
    // 0xee
    Instruction {
        kind: InstKind::INC,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0xef (undocumented)
    Instruction {
        kind: InstKind::ISC,
        addr: Addressing::Absolute,
        cycle: 6,
    },
    // 0xf0
    Instruction {
        kind: InstKind::BEQ,
        addr: Addressing::Relative,
        cycle: 2,
    },
    // 0xf1
    Instruction {
        kind: InstKind::SBC,
        addr: Addressing::IndirectY,
        cycle: 5,
    },
    // 0xf2 (undocumented)
    Instruction {
        kind: InstKind::KIL,
        addr: Addressing::Implied,
        cycle: 0,
    },
    // 0xf3 (undocumented)
    Instruction {
        kind: InstKind::ISC,
        addr: Addressing::IndirectY,
        cycle: 8,
    },
    // 0xf4 (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0xf5
    Instruction {
        kind: InstKind::SBC,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0xf6
    Instruction {
        kind: InstKind::INC,
        addr: Addressing::ZeroPageX,
        cycle: 4,
    },
    // 0xf7 (undocumented)
    Instruction {
        kind: InstKind::ISC,
        addr: Addressing::ZeroPageX,
        cycle: 6,
    },
    // 0xf8
    Instruction {
        kind: InstKind::SED,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xf9
    Instruction {
        kind: InstKind::SBC,
        addr: Addressing::AbsoluteY,
        cycle: 4,
    },
    // 0xfa (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::Implied,
        cycle: 2,
    },
    // 0xfb (undocumented)
    Instruction {
        kind: InstKind::ISC,
        addr: Addressing::AbsoluteY,
        cycle: 7,
    },
    // 0xfc (undocumented)
    Instruction {
        kind: InstKind::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0xfd
    Instruction {
        kind: InstKind::SBC,
        addr: Addressing::AbsoluteX,
        cycle: 4,
    },
    // 0xfe
    Instruction {
        kind: InstKind::INC,
        addr: Addressing::AbsoluteX,
        cycle: 0,
    },
    // 0xff (undocumented)
    Instruction {
        kind: InstKind::ISC,
        addr: Addressing::AbsoluteX,
        cycle: 7,
    },
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Instruction {
    kind: InstKind,
    addr: Addressing,
    cycle: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum InstKind {
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
