mod gui;
mod tui;

pub use gui::Gui;
pub use tui::Tui;

use super::display::Display;
use super::pad::Button;

pub trait Ui {
    fn flush(&mut self, image: &Display) -> anyhow::Result<()>;
    fn button_pressed(&self, _button: Button) -> bool;
}
