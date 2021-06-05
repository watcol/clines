#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct Registers {
    pub A: u8,
    pub X: u8,
    pub Y: u8,
    pub P: Status,
    pub S: u8,
    pub PC: u16,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            A: 0x00,
            X: 0x00,
            Y: 0x00,
            P: Status::default(),
            S: 0xFD,
            PC: 0x8000,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Status {
    pub negative: bool,
    pub overflow: bool,
    pub reserved: bool,
    pub break_mode: bool,
    pub decimal_mode: bool,
    pub interrupt: bool,
    pub zero: bool,
    pub carry: bool,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            negative: false,
            overflow: false,
            reserved: true,
            break_mode: false,
            decimal_mode: false,
            interrupt: true,
            zero: false,
            carry: false,
        }
    }
}

impl Status {
    pub fn from_u8(byte: u8) -> Self {
        Self {
            negative: byte & 0x80 == 0x80,
            overflow: byte & 0x40 == 0x40,
            reserved: byte & 0x20 == 0x20,
            break_mode: byte & 0x10 == 0x10,
            decimal_mode: byte & 0x08 == 0x08,
            interrupt: byte & 0x04 == 0x04,
            zero: byte & 0x02 == 0x02,
            carry: byte & 0x01 == 0x01,
        }
    }
    pub fn as_u8(&self) -> u8 {
        (self.negative as u8) * 0x80
            + (self.overflow as u8) * 0x40
            + (self.reserved as u8) * 0x20
            + (self.break_mode as u8) * 0x10
            + (self.decimal_mode as u8) * 0x08
            + (self.interrupt as u8) * 0x04
            + (self.zero as u8) * 0x02
            + (self.carry as u8)
    }
}
