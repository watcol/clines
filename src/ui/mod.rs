mod tui;

pub use tui::Tui;

pub trait Ui {
    fn flush(&mut self, image: &crate::Display) -> anyhow::Result<()>;
}
