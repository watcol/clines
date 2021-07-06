use super::Ui;
use crate::display::Display;
use crate::pad::Button;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

pub struct Gui {
    win: Window,
}

impl Gui {
    pub fn new() -> anyhow::Result<Self> {
        let win = Window::new(
            "NES",
            Display::WIDTH,
            Display::HEIGHT,
            WindowOptions {
                resize: true,
                scale: Scale::FitScreen,
                scale_mode: ScaleMode::AspectRatioStretch,
                ..Default::default()
            },
        )?;
        Ok(Gui { win })
    }
}

impl Ui for Gui {
    fn flush(&mut self, display: &Display) -> anyhow::Result<()> {
        let buffer = display
            .pixels()
            .map(|(_, _, rgb)| rgb.as_u32())
            .collect::<Vec<_>>();
        self.win
            .update_with_buffer(&buffer, Display::WIDTH, Display::HEIGHT)?;
        Ok(())
    }

    fn button_pressed(&self, button: Button) -> bool {
        let key = match button {
            Button::A => Key::Z,
            Button::B => Key::X,
            Button::Select => Key::Space,
            Button::Start => Key::Enter,
            Button::Up => Key::Up,
            Button::Down => Key::Down,
            Button::Left => Key::Left,
            Button::Right => Key::Right,
            Button::Quit => Key::Escape,
            Button::Reset => Key::R,
        };
        self.win.is_key_down(key)
    }
}
