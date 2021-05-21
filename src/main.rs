mod ui;

use ui::{Tui, Ui};

pub type Display = image::RgbImage;

fn main() {
    let image = image::open("sample.png").unwrap().into_rgb8();
    let mut tui = Tui::new().unwrap();
    for _ in 0..60 {
        tui.flush(&image).unwrap();
    }
}
