#[macro_use]
extern crate log;

mod nes;
mod ui;

use nes::Nes;
use std::env::args;

pub type Display = image::RgbImage;

fn main() {
    fmtlog::default().set().unwrap();
    let ui = ui::Gui::new().unwrap();
    let mut nes = Nes::from_path(args().nth(1).unwrap(), ui).unwrap();
    nes.run_loop();
}
