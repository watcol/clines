mod nes;
mod ui;

use nes::Nes;

pub type Display = image::RgbImage;

fn main() {
    let mut nes = Nes::from_path("sample1.nes").unwrap();
    nes.run_loop();
}
