use super::{Button, Display, Ui};
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use picto::color::Rgb;

pub struct Gui {
    win: Window,
}

fn rgb2u32(rgb: Rgb) -> u32 {
    let Rgb { red, green, blue } = rgb;
    ((red * 255.0) as u32) << 16 | ((green * 255.0) as u32) << 8 | ((blue * 255.0) as u32)
}

impl Gui {
    pub fn new() -> anyhow::Result<Self> {
        let win = Window::new(
            "NES",
            256,
            240,
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
            .map(|(_, _, rgb)| rgb2u32(rgb.get()))
            .collect::<Vec<_>>();
        self.win.update_with_buffer(
            &buffer,
            display.width() as usize,
            display.height() as usize,
        )?;
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
