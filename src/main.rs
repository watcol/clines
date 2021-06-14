#[macro_use]
extern crate log;

mod nes;
mod ui;

use nes::Nes;
use std::env::args;

fn main() {
    fmtlog::new(
        fmtlog::Config::new()
            .level(fmtlog::LevelFilter::Debug)
            .output("log.txt"),
    )
    .set()
    .unwrap();
    let ui = ui::Tui::new().unwrap();
    let mut nes = Nes::from_path(args().nth(1).unwrap(), ui).unwrap();
    nes.run_loop();
}
