use super::Ui;
use minifb::{Scale, ScaleMode, Window, WindowOptions};

pub struct Gui {
    win: Window,
}

fn rgb2u32(rgb: image::Rgb<u8>) -> u32 {
    let [r, g, b] = rgb.0;
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

impl Gui {
    pub fn new() -> anyhow::Result<Self> {
        let win = Window::new(
            "CliNES",
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
    fn flush(&mut self, display: &crate::Display) -> anyhow::Result<()> {
        let buffer = display
            .enumerate_pixels()
            .map(|(_, _, rgb)| rgb2u32(*rgb))
            .collect::<Vec<_>>();
        self.win.update_with_buffer(
            &buffer,
            display.width() as usize,
            display.height() as usize,
        )?;
        Ok(())
    }
}
