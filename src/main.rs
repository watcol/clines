#[macro_use]
extern crate log;

mod nes;
mod ui;

use nes::Nes;

pub type Display = image::RgbImage;

fn main() {
    fmtlog::new(fmtlog::Config::new().level(fmtlog::LevelFilter::Debug))
        .set()
        .unwrap();
    let gui = ui::Gui::new().unwrap();
    let mut nes = Nes::from_path("sample1.nes", gui).unwrap();
    nes.run_loop();
}
