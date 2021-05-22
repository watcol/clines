pub static OPECODES: [Opecode; 0x100] = [
    // 0x00
    Opecode {
        inst: Instruction::BRK,
        addr: Addressing::Implied,
        cycle: 7,
        add_cycle: false,
    },
    // 0x01
    Opecode {
        inst: Instruction::ORA,
        addr: Addressing::XIndirect,
        cycle: 6,
        add_cycle: false,
    },
    // 0x02 (undocumented)
    Opecode {
        inst: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
        add_cycle: false,
    },
    // 0x03 (undocumented)
    Opecode {
        inst: Instruction::SLO,
        addr: Addressing::XIndirect,
        cycle: 8,
        add_cycle: false,
    },
    // 0x04 (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0x05
    Opecode {
        inst: Instruction::ORA,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0x06
    Opecode {
        inst: Instruction::ASL,
        addr: Addressing::ZeroPage,
        cycle: 5,
        add_cycle: false,
    },
    // 0x07 (undocumented)
    Opecode {
        inst: Instruction::SLO,
        addr: Addressing::ZeroPage,
        cycle: 5,
        add_cycle: false,
    },
    // 0x08
    Opecode {
        inst: Instruction::PHP,
        addr: Addressing::Implied,
        cycle: 3,
        add_cycle: false,
    },
    // 0x09
    Opecode {
        inst: Instruction::ORA,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0x0a
    Opecode {
        inst: Instruction::ASL,
        addr: Addressing::Accumulator,
        cycle: 2,
        add_cycle: false,
    },
    // 0x0b (undocumented)
    Opecode {
        inst: Instruction::ANC,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0x0c (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0x0d
    Opecode {
        inst: Instruction::ORA,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0x0e
    Opecode {
        inst: Instruction::ASL,
        addr: Addressing::Absolute,
        cycle: 6,
        add_cycle: false,
    },
    // 0x0f (undocumented)
    Opecode {
        inst: Instruction::SLO,
        addr: Addressing::Absolute,
        cycle: 6,
        add_cycle: false,
    },
    // 0x10
    Opecode {
        inst: Instruction::BPL,
        addr: Addressing::Relative,
        cycle: 2,
        add_cycle: true,
    },
    // 0x11
    Opecode {
        inst: Instruction::ORA,
        addr: Addressing::IndirectY,
        cycle: 5,
        add_cycle: true,
    },
    // 0x12 (undocumented)
    Opecode {
        inst: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
        add_cycle: false,
    },
    // 0x13 (undocumented)
    Opecode {
        inst: Instruction::SLO,
        addr: Addressing::IndirectY,
        cycle: 8,
        add_cycle: false,
    },
    // 0x14 (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0x15
    Opecode {
        inst: Instruction::ORA,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0x16
    Opecode {
        inst: Instruction::ASL,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0x17 (undocumented)
    Opecode {
        inst: Instruction::SLO,
        addr: Addressing::ZeroPageX,
        cycle: 0,
        add_cycle: false,
    },
    // 0x18
    Opecode {
        inst: Instruction::CLC,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0x19
    Opecode {
        inst: Instruction::ORA,
        addr: Addressing::AbsoluteY,
        cycle: 4,
        add_cycle: true,
    },
    // 0x1a (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0x1b (undocumented)
    Opecode {
        inst: Instruction::SLO,
        addr: Addressing::AbsoluteY,
        cycle: 0,
        add_cycle: false,
    },
    // 0x1c (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
        add_cycle: true,
    },
    // 0x1d
    Opecode {
        inst: Instruction::ORA,
        addr: Addressing::AbsoluteX,
        cycle: 4,
        add_cycle: true,
    },
    // 0x1e
    Opecode {
        inst: Instruction::ASL,
        addr: Addressing::AbsoluteX,
        cycle: 7,
        add_cycle: false,
    },
    // 0x1f (undocumented)
    Opecode {
        inst: Instruction::SLO,
        addr: Addressing::AbsoluteX,
        cycle: 0,
        add_cycle: false,
    },
    // 0x20
    Opecode {
        inst: Instruction::JSR,
        addr: Addressing::Absolute,
        cycle: 6,
        add_cycle: false,
    },
    // 0x21
    Opecode {
        inst: Instruction::AND,
        addr: Addressing::XIndirect,
        cycle: 6,
        add_cycle: false,
    },
    // 0x22 (undocumented)
    Opecode {
        inst: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
        add_cycle: false,
    },
    // 0x23 (undocumented)
    Opecode {
        inst: Instruction::RLA,
        addr: Addressing::XIndirect,
        cycle: 8,
        add_cycle: false,
    },
    // 0x24
    Opecode {
        inst: Instruction::BIT,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0x25
    Opecode {
        inst: Instruction::AND,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0x26
    Opecode {
        inst: Instruction::ROL,
        addr: Addressing::ZeroPage,
        cycle: 5,
        add_cycle: false,
    },
    // 0x27 (undocumented)
    Opecode {
        inst: Instruction::RLA,
        addr: Addressing::ZeroPage,
        cycle: 5,
        add_cycle: false,
    },
    // 0x28
    Opecode {
        inst: Instruction::PLP,
        addr: Addressing::Implied,
        cycle: 4,
        add_cycle: false,
    },
    // 0x29
    Opecode {
        inst: Instruction::AND,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0x2a
    Opecode {
        inst: Instruction::ROL,
        addr: Addressing::Accumulator,
        cycle: 2,
        add_cycle: false,
    },
    // 0x2b (undocumented)
    Opecode {
        inst: Instruction::ANC,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0x2c
    Opecode {
        inst: Instruction::BIT,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0x2d
    Opecode {
        inst: Instruction::AND,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0x2e
    Opecode {
        inst: Instruction::ROL,
        addr: Addressing::Absolute,
        cycle: 6,
        add_cycle: false,
    },
    // 0x2f (undocumented)
    Opecode {
        inst: Instruction::RLA,
        addr: Addressing::Absolute,
        cycle: 6,
        add_cycle: false,
    },
    // 0x30
    Opecode {
        inst: Instruction::BMI,
        addr: Addressing::Relative,
        cycle: 2,
        add_cycle: true,
    },
    // 0x31
    Opecode {
        inst: Instruction::AND,
        addr: Addressing::IndirectY,
        cycle: 6,
        add_cycle: true,
    },
    // 0x32 (undocumented)
    Opecode {
        inst: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
        add_cycle: false,
    },
    // 0x33 (undocumented)
    Opecode {
        inst: Instruction::RLA,
        addr: Addressing::IndirectY,
        cycle: 8,
        add_cycle: false,
    },
    // 0x34
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0x35
    Opecode {
        inst: Instruction::AND,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0x36
    Opecode {
        inst: Instruction::ROL,
        addr: Addressing::ZeroPageX,
        cycle: 6,
        add_cycle: false,
    },
    // 0x37 (undocumented)
    Opecode {
        inst: Instruction::RLA,
        addr: Addressing::ZeroPageX,
        cycle: 6,
        add_cycle: false,
    },
    // 0x38
    Opecode {
        inst: Instruction::SEC,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0x39
    Opecode {
        inst: Instruction::AND,
        addr: Addressing::AbsoluteY,
        cycle: 4,
        add_cycle: true,
    },
    // 0x3a (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0x3b (undocumented)
    Opecode {
        inst: Instruction::RLA,
        addr: Addressing::AbsoluteY,
        cycle: 7,
        add_cycle: false,
    },
    // 0x3c (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
        add_cycle: true,
    },
    // 0x3d
    Opecode {
        inst: Instruction::AND,
        addr: Addressing::AbsoluteX,
        cycle: 4,
        add_cycle: true,
    },
    // 0x3e
    Opecode {
        inst: Instruction::ROL,
        addr: Addressing::Absolute,
        cycle: 7,
        add_cycle: false,
    },
    // 0x3f (undocumented)
    Opecode {
        inst: Instruction::RLA,
        addr: Addressing::AbsoluteX,
        cycle: 7,
        add_cycle: false,
    },
    // 0x40
    Opecode {
        inst: Instruction::RTI,
        addr: Addressing::Implied,
        cycle: 6,
        add_cycle: false,
    },
    // 0x41
    Opecode {
        inst: Instruction::EOR,
        addr: Addressing::XIndirect,
        cycle: 6,
        add_cycle: false,
    },
    // 0x42 (undocumented)
    Opecode {
        inst: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
        add_cycle: false,
    },
    // 0x43 (undocumented)
    Opecode {
        inst: Instruction::SRE,
        addr: Addressing::IndirectY,
        cycle: 3,
        add_cycle: false,
    },
    // 0x44 (undocumented)
    Opecode {
        inst: Instruction::BRK,
        addr: Addressing::Implied,
        cycle: 0,
        add_cycle: false,
    },
    // 0x45
    Opecode {
        inst: Instruction::EOR,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0x46
    Opecode {
        inst: Instruction::LSR,
        addr: Addressing::ZeroPage,
        cycle: 5,
        add_cycle: false,
    },
    // 0x47 (undocumented)
    Opecode {
        inst: Instruction::SRE,
        addr: Addressing::ZeroPage,
        cycle: 5,
        add_cycle: false,
    },
    // 0x48
    Opecode {
        inst: Instruction::PHA,
        addr: Addressing::Implied,
        cycle: 3,
        add_cycle: false,
    },
    // 0x49
    Opecode {
        inst: Instruction::EOR,
        addr: Addressing::Immediate,
        cycle: 3,
        add_cycle: false,
    },
    // 0x4a
    Opecode {
        inst: Instruction::LSR,
        addr: Addressing::Accumulator,
        cycle: 2,
        add_cycle: false,
    },
    // 0x4b (undocumented)
    Opecode {
        inst: Instruction::ALR,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0x4c
    Opecode {
        inst: Instruction::JMP,
        addr: Addressing::Absolute,
        cycle: 3,
        add_cycle: false,
    },
    // 0x4d
    Opecode {
        inst: Instruction::EOR,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0x4e (undocumented)
    Opecode {
        inst: Instruction::LSR,
        addr: Addressing::Absolute,
        cycle: 6,
        add_cycle: false,
    },
    // 0x4f
    Opecode {
        inst: Instruction::SRE,
        addr: Addressing::Absolute,
        cycle: 6,
        add_cycle: false,
    },
    // 0x50
    Opecode {
        inst: Instruction::BVC,
        addr: Addressing::Relative,
        cycle: 2,
        add_cycle: true,
    },
    // 0x51
    Opecode {
        inst: Instruction::EOR,
        addr: Addressing::IndirectY,
        cycle: 5,
        add_cycle: true,
    },
    // 0x52 (undocumented)
    Opecode {
        inst: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
        add_cycle: false,
    },
    // 0x53 (undocumented)
    Opecode {
        inst: Instruction::SRE,
        addr: Addressing::XIndirect,
        cycle: 8,
        add_cycle: false,
    },
    // 0x54 (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::ZeroPage,
        cycle: 4,
        add_cycle: false,
    },
    // 0x55
    Opecode {
        inst: Instruction::EOR,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0x56
    Opecode {
        inst: Instruction::LSR,
        addr: Addressing::ZeroPageX,
        cycle: 6,
        add_cycle: false,
    },
    // 0x57 (undocumented)
    Opecode {
        inst: Instruction::SRE,
        addr: Addressing::ZeroPageX,
        cycle: 6,
        add_cycle: false,
    },
    // 0x58
    Opecode {
        inst: Instruction::CLI,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0x59
    Opecode {
        inst: Instruction::EOR,
        addr: Addressing::AbsoluteY,
        cycle: 4,
        add_cycle: true,
    },
    // 0x5a (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
        add_cycle: false,
    },
    // 0x5b (undocumented)
    Opecode {
        inst: Instruction::SRE,
        addr: Addressing::AbsoluteY,
        cycle: 7,
        add_cycle: false,
    },
    // 0x5c (undocumented)
    Opecode {
        inst: Instruction::SRE,
        addr: Addressing::AbsoluteX,
        cycle: 4,
        add_cycle: true,
    },
    // 0x5d
    Opecode {
        inst: Instruction::EOR,
        addr: Addressing::AbsoluteX,
        cycle: 4,
        add_cycle: true,
    },
    // 0x5e
    Opecode {
        inst: Instruction::LSR,
        addr: Addressing::AbsoluteX,
        cycle: 7,
        add_cycle: false,
    },
    // 0x5f (undocumented)
    Opecode {
        inst: Instruction::SRE,
        addr: Addressing::AbsoluteX,
        cycle: 7,
        add_cycle: false,
    },
    // 0x60
    Opecode {
        inst: Instruction::RTS,
        addr: Addressing::Implied,
        cycle: 6,
        add_cycle: false,
    },
    // 0x61
    Opecode {
        inst: Instruction::ADC,
        addr: Addressing::XIndirect,
        cycle: 6,
        add_cycle: false,
    },
    // 0x62 (undocumented)
    Opecode {
        inst: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
        add_cycle: false,
    },
    // 0x63 (undocumented)
    Opecode {
        inst: Instruction::RRA,
        addr: Addressing::XIndirect,
        cycle: 8,
        add_cycle: false,
    },
    // 0x64 (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0x65
    Opecode {
        inst: Instruction::ADC,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0x66
    Opecode {
        inst: Instruction::ROR,
        addr: Addressing::ZeroPage,
        cycle: 5,
        add_cycle: false,
    },
    // 0x67 (undocumented)
    Opecode {
        inst: Instruction::RRA,
        addr: Addressing::ZeroPage,
        cycle: 5,
        add_cycle: false,
    },
    // 0x68
    Opecode {
        inst: Instruction::PLA,
        addr: Addressing::Implied,
        cycle: 4,
        add_cycle: false,
    },
    // 0x69
    Opecode {
        inst: Instruction::ADC,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0x6a
    Opecode {
        inst: Instruction::ROR,
        addr: Addressing::Accumulator,
        cycle: 2,
        add_cycle: false,
    },
    // 0x6b (undocumented)
    Opecode {
        inst: Instruction::ARR,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0x6c
    Opecode {
        inst: Instruction::JMP,
        addr: Addressing::Indirect,
        cycle: 5,
        add_cycle: false,
    },
    // 0x6d
    Opecode {
        inst: Instruction::ADC,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0x6e
    Opecode {
        inst: Instruction::ROR,
        addr: Addressing::Absolute,
        cycle: 6,
        add_cycle: false,
    },
    // 0x6f (undocumented)
    Opecode {
        inst: Instruction::RRA,
        addr: Addressing::Absolute,
        cycle: 6,
        add_cycle: false,
    },
    // 0x70
    Opecode {
        inst: Instruction::BVS,
        addr: Addressing::Relative,
        cycle: 2,
        add_cycle: true,
    },
    // 0x71
    Opecode {
        inst: Instruction::ADC,
        addr: Addressing::XIndirect,
        cycle: 6,
        add_cycle: true,
    },
    // 0x72 (undocumented)
    Opecode {
        inst: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
        add_cycle: false,
    },
    // 0x73 (undocumented)
    Opecode {
        inst: Instruction::RRA,
        addr: Addressing::IndirectY,
        cycle: 8,
        add_cycle: false,
    },
    // 0x74 (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0x75
    Opecode {
        inst: Instruction::ADC,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0x76
    Opecode {
        inst: Instruction::ROR,
        addr: Addressing::ZeroPageX,
        cycle: 6,
        add_cycle: false,
    },
    // 0x77 (undocumented)
    Opecode {
        inst: Instruction::RRA,
        addr: Addressing::ZeroPageX,
        cycle: 6,
        add_cycle: false,
    },
    // 0x78
    Opecode {
        inst: Instruction::SEI,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0x79
    Opecode {
        inst: Instruction::ADC,
        addr: Addressing::AbsoluteY,
        cycle: 4,
        add_cycle: true,
    },
    // 0x7a (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0x7b (undocumented)
    Opecode {
        inst: Instruction::RRA,
        addr: Addressing::AbsoluteY,
        cycle: 7,
        add_cycle: false,
    },
    // 0x7c (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
        add_cycle: true,
    },
    // 0x7d
    Opecode {
        inst: Instruction::ADC,
        addr: Addressing::AbsoluteX,
        cycle: 4,
        add_cycle: true,
    },
    // 0x7e
    Opecode {
        inst: Instruction::ROR,
        addr: Addressing::AbsoluteX,
        cycle: 7,
        add_cycle: false,
    },
    // 0x7f (undocumented)
    Opecode {
        inst: Instruction::RRA,
        addr: Addressing::AbsoluteX,
        cycle: 7,
        add_cycle: false,
    },
    // 0x80 (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0x81
    Opecode {
        inst: Instruction::STA,
        addr: Addressing::XIndirect,
        cycle: 6,
        add_cycle: false,
    },
    // 0x82 (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0x83 (undocumented)
    Opecode {
        inst: Instruction::SAX,
        addr: Addressing::XIndirect,
        cycle: 6,
        add_cycle: false,
    },
    // 0x84
    Opecode {
        inst: Instruction::STY,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0x85
    Opecode {
        inst: Instruction::STA,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0x86
    Opecode {
        inst: Instruction::STX,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0x87 (undocumented)
    Opecode {
        inst: Instruction::SAX,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0x88
    Opecode {
        inst: Instruction::DEY,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0x89 (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0x8a
    Opecode {
        inst: Instruction::TXA,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0x8b (undocumented)
    Opecode {
        inst: Instruction::XAA,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0x8c
    Opecode {
        inst: Instruction::STY,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0x8d
    Opecode {
        inst: Instruction::STA,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0x8e
    Opecode {
        inst: Instruction::STX,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0x8f (undocumented)
    Opecode {
        inst: Instruction::SAX,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0x90
    Opecode {
        inst: Instruction::BCC,
        addr: Addressing::Relative,
        cycle: 2,
        add_cycle: true,
    },
    // 0x91
    Opecode {
        inst: Instruction::STA,
        addr: Addressing::IndirectY,
        cycle: 6,
        add_cycle: false,
    },
    // 0x92 (undocumented)
    Opecode {
        inst: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
        add_cycle: false,
    },
    // 0x93 (undocumented)
    Opecode {
        inst: Instruction::AHX,
        addr: Addressing::IndirectY,
        cycle: 6,
        add_cycle: false,
    },
    // 0x94
    Opecode {
        inst: Instruction::STY,
        addr: Addressing::ZeroPageX,
        cycle: 0,
        add_cycle: false,
    },
    // 0x95
    Opecode {
        inst: Instruction::STA,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0x96
    Opecode {
        inst: Instruction::STX,
        addr: Addressing::ZeroPageY,
        cycle: 0,
        add_cycle: false,
    },
    // 0x97
    Opecode {
        inst: Instruction::SAX,
        addr: Addressing::ZeroPageY,
        cycle: 4,
        add_cycle: false,
    },
    // 0x98
    Opecode {
        inst: Instruction::TYA,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0x99
    Opecode {
        inst: Instruction::STA,
        addr: Addressing::AbsoluteY,
        cycle: 5,
        add_cycle: false,
    },
    // 0x9a
    Opecode {
        inst: Instruction::TXS,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0x9b (undocumented)
    Opecode {
        inst: Instruction::TAS,
        addr: Addressing::AbsoluteY,
        cycle: 5,
        add_cycle: false,
    },
    // 0x9c (undocumented)
    Opecode {
        inst: Instruction::SHY,
        addr: Addressing::AbsoluteX,
        cycle: 5,
        add_cycle: false,
    },
    // 0x9d
    Opecode {
        inst: Instruction::STA,
        addr: Addressing::AbsoluteX,
        cycle: 5,
        add_cycle: false,
    },
    // 0x9e (undocumented)
    Opecode {
        inst: Instruction::SHX,
        addr: Addressing::AbsoluteY,
        cycle: 5,
        add_cycle: false,
    },
    // 0x9f (undocumented)
    Opecode {
        inst: Instruction::AHX,
        addr: Addressing::AbsoluteY,
        cycle: 5,
        add_cycle: false,
    },
    // 0xa0
    Opecode {
        inst: Instruction::LDY,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0xa1
    Opecode {
        inst: Instruction::LDA,
        addr: Addressing::XIndirect,
        cycle: 6,
        add_cycle: false,
    },
    // 0xa2
    Opecode {
        inst: Instruction::LDX,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0xa3 (undocumented)
    Opecode {
        inst: Instruction::LAX,
        addr: Addressing::XIndirect,
        cycle: 6,
        add_cycle: false,
    },
    // 0xa4
    Opecode {
        inst: Instruction::LDY,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0xa5
    Opecode {
        inst: Instruction::LDA,
        addr: Addressing::ZeroPage,
        cycle: 0,
        add_cycle: false,
    },
    // 0xa6
    Opecode {
        inst: Instruction::LDX,
        addr: Addressing::ZeroPage,
        cycle: 0,
        add_cycle: false,
    },
    // 0xa7 (undocumented)
    Opecode {
        inst: Instruction::LAX,
        addr: Addressing::ZeroPage,
        cycle: 2,
        add_cycle: false,
    },
    // 0xa8
    Opecode {
        inst: Instruction::TAY,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0xa9
    Opecode {
        inst: Instruction::LDA,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0xaa
    Opecode {
        inst: Instruction::TAX,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0xab (undocumented)
    Opecode {
        inst: Instruction::LAX,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0xac
    Opecode {
        inst: Instruction::LDY,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0xad
    Opecode {
        inst: Instruction::LDA,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0xae
    Opecode {
        inst: Instruction::LDX,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0xaf (undocumented)
    Opecode {
        inst: Instruction::LAX,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0xb0
    Opecode {
        inst: Instruction::BCS,
        addr: Addressing::Relative,
        cycle: 2,
        add_cycle: true,
    },
    // 0xb1
    Opecode {
        inst: Instruction::LDA,
        addr: Addressing::IndirectY,
        cycle: 5,
        add_cycle: true,
    },
    // 0xb2 (undocumented)
    Opecode {
        inst: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
        add_cycle: false,
    },
    // 0xb3 (undocumented)
    Opecode {
        inst: Instruction::LAX,
        addr: Addressing::IndirectY,
        cycle: 5,
        add_cycle: true,
    },
    // 0xb4
    Opecode {
        inst: Instruction::LDY,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0xb5
    Opecode {
        inst: Instruction::LDA,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0xb6
    Opecode {
        inst: Instruction::LDX,
        addr: Addressing::ZeroPageY,
        cycle: 4,
        add_cycle: false,
    },
    // 0xb7 (undocumented)
    Opecode {
        inst: Instruction::LAX,
        addr: Addressing::ZeroPageY,
        cycle: 4,
        add_cycle: false,
    },
    // 0xb8
    Opecode {
        inst: Instruction::CLV,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0xb9
    Opecode {
        inst: Instruction::LDA,
        addr: Addressing::AbsoluteY,
        cycle: 4,
        add_cycle: true,
    },
    // 0xba
    Opecode {
        inst: Instruction::TSX,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0xbb (undocumented)
    Opecode {
        inst: Instruction::LAS,
        addr: Addressing::AbsoluteY,
        cycle: 0,
        add_cycle: true,
    },
    // 0xbc
    Opecode {
        inst: Instruction::LDY,
        addr: Addressing::AbsoluteX,
        cycle: 4,
        add_cycle: true,
    },
    // 0xbd
    Opecode {
        inst: Instruction::LDA,
        addr: Addressing::AbsoluteX,
        cycle: 4,
        add_cycle: true,
    },
    // 0xbe
    Opecode {
        inst: Instruction::LDX,
        addr: Addressing::AbsoluteY,
        cycle: 4,
        add_cycle: true,
    },
    // 0xbf (undocumented)
    Opecode {
        inst: Instruction::LAX,
        addr: Addressing::AbsoluteY,
        cycle: 4,
        add_cycle: true,
    },
    // 0xc0
    Opecode {
        inst: Instruction::CPY,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0xc1
    Opecode {
        inst: Instruction::CMP,
        addr: Addressing::XIndirect,
        cycle: 6,
        add_cycle: false,
    },
    // 0xc2 (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::Immediate,
        cycle: 6,
        add_cycle: false,
    },
    // 0xc3 (undocumented)
    Opecode {
        inst: Instruction::DCP,
        addr: Addressing::XIndirect,
        cycle: 8,
        add_cycle: false,
    },
    // 0xc4
    Opecode {
        inst: Instruction::CPY,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0xc5
    Opecode {
        inst: Instruction::CMP,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0xc6
    Opecode {
        inst: Instruction::DEC,
        addr: Addressing::ZeroPage,
        cycle: 5,
        add_cycle: false,
    },
    // 0xc7 (undocumented)
    Opecode {
        inst: Instruction::DCP,
        addr: Addressing::ZeroPage,
        cycle: 5,
        add_cycle: false,
    },
    // 0xc8
    Opecode {
        inst: Instruction::INY,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0xc9
    Opecode {
        inst: Instruction::CMP,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0xca
    Opecode {
        inst: Instruction::DEX,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0xcb (undocumented)
    Opecode {
        inst: Instruction::AXS,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0xcc
    Opecode {
        inst: Instruction::CPY,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0xcd
    Opecode {
        inst: Instruction::CMP,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0xce
    Opecode {
        inst: Instruction::DEC,
        addr: Addressing::Absolute,
        cycle: 6,
        add_cycle: false,
    },
    // 0xcf (undocumented)
    Opecode {
        inst: Instruction::DCP,
        addr: Addressing::Absolute,
        cycle: 6,
        add_cycle: false,
    },
    // 0xd0
    Opecode {
        inst: Instruction::BNE,
        addr: Addressing::Relative,
        cycle: 2,
        add_cycle: true,
    },
    // 0xd1
    Opecode {
        inst: Instruction::CMP,
        addr: Addressing::IndirectY,
        cycle: 5,
        add_cycle: true,
    },
    // 0xd2 (undocumented)
    Opecode {
        inst: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
        add_cycle: false,
    },
    // 0xd3 (undocumented)
    Opecode {
        inst: Instruction::DCP,
        addr: Addressing::IndirectY,
        cycle: 0,
        add_cycle: false,
    },
    // 0xd4 (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0xd5
    Opecode {
        inst: Instruction::CMP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0xd6
    Opecode {
        inst: Instruction::DEC,
        addr: Addressing::ZeroPageX,
        cycle: 6,
        add_cycle: false,
    },
    // 0xd7 (undocumented)
    Opecode {
        inst: Instruction::DCP,
        addr: Addressing::ZeroPage,
        cycle: 6,
        add_cycle: false,
    },
    // 0xd8
    Opecode {
        inst: Instruction::CLD,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0xd9
    Opecode {
        inst: Instruction::CMP,
        addr: Addressing::AbsoluteY,
        cycle: 4,
        add_cycle: true,
    },
    // 0xda (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0xdb (undocumented)
    Opecode {
        inst: Instruction::DCP,
        addr: Addressing::AbsoluteY,
        cycle: 7,
        add_cycle: false,
    },
    // 0xdc (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
        add_cycle: true,
    },
    // 0xdd
    Opecode {
        inst: Instruction::CMP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
        add_cycle: true,
    },
    // 0xde
    Opecode {
        inst: Instruction::DEC,
        addr: Addressing::AbsoluteX,
        cycle: 7,
        add_cycle: false,
    },
    // 0xdf (undocumented)
    Opecode {
        inst: Instruction::DCP,
        addr: Addressing::AbsoluteX,
        cycle: 7,
        add_cycle: false,
    },
    // 0xe0
    Opecode {
        inst: Instruction::CPX,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0xe1
    Opecode {
        inst: Instruction::SBC,
        addr: Addressing::XIndirect,
        cycle: 6,
        add_cycle: false,
    },
    // 0xe2 (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0xe3 (undocumented)
    Opecode {
        inst: Instruction::ISC,
        addr: Addressing::XIndirect,
        cycle: 8,
        add_cycle: false,
    },
    // 0xe4
    Opecode {
        inst: Instruction::CPX,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0xe5
    Opecode {
        inst: Instruction::SBC,
        addr: Addressing::ZeroPage,
        cycle: 3,
        add_cycle: false,
    },
    // 0xe6
    Opecode {
        inst: Instruction::INC,
        addr: Addressing::ZeroPage,
        cycle: 5,
        add_cycle: false,
    },
    // 0xe7 (undocumented)
    Opecode {
        inst: Instruction::ISC,
        addr: Addressing::ZeroPage,
        cycle: 5,
        add_cycle: false,
    },
    // 0xe8
    Opecode {
        inst: Instruction::INX,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0xe9
    Opecode {
        inst: Instruction::SBC,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0xea
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0xeb (undocumented)
    Opecode {
        inst: Instruction::SBC,
        addr: Addressing::Immediate,
        cycle: 2,
        add_cycle: false,
    },
    // 0xec
    Opecode {
        inst: Instruction::CPX,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0xed
    Opecode {
        inst: Instruction::SBC,
        addr: Addressing::Absolute,
        cycle: 4,
        add_cycle: false,
    },
    // 0xee
    Opecode {
        inst: Instruction::INC,
        addr: Addressing::Absolute,
        cycle: 6,
        add_cycle: false,
    },
    // 0xef (undocumented)
    Opecode {
        inst: Instruction::ISC,
        addr: Addressing::Absolute,
        cycle: 6,
        add_cycle: false,
    },
    // 0xf0
    Opecode {
        inst: Instruction::BEQ,
        addr: Addressing::Relative,
        cycle: 2,
        add_cycle: true,
    },
    // 0xf1
    Opecode {
        inst: Instruction::SBC,
        addr: Addressing::IndirectY,
        cycle: 5,
        add_cycle: true,
    },
    // 0xf2 (undocumented)
    Opecode {
        inst: Instruction::KIL,
        addr: Addressing::Implied,
        cycle: 0,
        add_cycle: false,
    },
    // 0xf3 (undocumented)
    Opecode {
        inst: Instruction::ISC,
        addr: Addressing::IndirectY,
        cycle: 8,
        add_cycle: false,
    },
    // 0xf4 (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0xf5
    Opecode {
        inst: Instruction::SBC,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0xf6
    Opecode {
        inst: Instruction::INC,
        addr: Addressing::ZeroPageX,
        cycle: 4,
        add_cycle: false,
    },
    // 0xf7 (undocumented)
    Opecode {
        inst: Instruction::ISC,
        addr: Addressing::ZeroPageX,
        cycle: 6,
        add_cycle: false,
    },
    // 0xf8
    Opecode {
        inst: Instruction::SED,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0xf9
    Opecode {
        inst: Instruction::SBC,
        addr: Addressing::AbsoluteY,
        cycle: 4,
        add_cycle: true,
    },
    // 0xfa (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::Implied,
        cycle: 2,
        add_cycle: false,
    },
    // 0xfb (undocumented)
    Opecode {
        inst: Instruction::ISC,
        addr: Addressing::AbsoluteY,
        cycle: 7,
        add_cycle: false,
    },
    // 0xfc (undocumented)
    Opecode {
        inst: Instruction::NOP,
        addr: Addressing::AbsoluteX,
        cycle: 4,
        add_cycle: true,
    },
    // 0xfd
    Opecode {
        inst: Instruction::SBC,
        addr: Addressing::AbsoluteX,
        cycle: 4,
        add_cycle: true,
    },
    // 0xfe
    Opecode {
        inst: Instruction::INC,
        addr: Addressing::AbsoluteX,
        cycle: 0,
        add_cycle: false,
    },
    // 0xff (undocumented)
    Opecode {
        inst: Instruction::ISC,
        addr: Addressing::AbsoluteX,
        cycle: 7,
        add_cycle: false,
    },
];
