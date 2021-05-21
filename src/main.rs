mod ui;

use ui::{Gui, Tui, Ui};

pub type Display = image::RgbImage;

fn main() {
    let image = image::open("sample.png").unwrap().into_rgb8();
    let mut tui = Gui::new().unwrap();
    for _ in 0..60 {
        tui.flush(&image).unwrap();
    }
}
