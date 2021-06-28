use crate::ui::Ui;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Button {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
    Quit,
    Reset,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Pad {
    reset: bool,
    count: u8,
}

impl Pad {
    pub fn write(&mut self, value: u8) {
        debug!("Pad Wrote");
        if value & 0x01 == 0x01 {
            self.reset = true;
        } else if self.reset && value & 0x01 == 0x00 {
            self.reset = false;
            self.count = 0;
        }
    }

    pub fn read<U: Ui>(&mut self, ui: &U) -> u8 {
        let btn = match self.count {
            0 => Button::A,
            1 => Button::B,
            2 => Button::Select,
            3 => Button::Start,
            4 => Button::Up,
            5 => Button::Down,
            6 => Button::Left,
            7 => Button::Right,
            _ => unreachable!(),
        };
        self.count += 1;
        ui.button_pressed(btn) as u8
    }
}
