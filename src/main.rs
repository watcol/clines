use nes_cli::{Gui, Nes};
use std::env::args;

fn main() {
    fmtlog::new(
        fmtlog::Config::new()
            .format(fmtlog::formats::SIMPLE1)
            .level(fmtlog::LevelFilter::Debug)
            .output("log.txt"),
    )
    .set()
    .unwrap();
    let ui = Gui::new().unwrap();
    let mut nes = Nes::from_path(args().nth(1).unwrap(), ui).unwrap();
    nes.run_loop();
}
