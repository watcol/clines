#[macro_use]
extern crate log;

mod nes;
mod ui;

use nes::Nes;

pub type Display = image::RgbImage;

fn main() {
    fmtlog::new(
        fmtlog::Config::new()
            .level(fmtlog::LevelFilter::Debug)
            .output("log.txt"),
    )
    .set()
    .unwrap();
    let ui = ui::Tui::new().unwrap();
    let mut nes = Nes::from_path("nestest.nes", ui).unwrap();
    nes.run_loop();
}
