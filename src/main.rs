mod rom;
mod ui;

use rom::Rom;

pub type Display = image::RgbImage;

fn main() {
    let rom = Rom::from_path("sample1.nes").unwrap();
    println!("{:?}", rom);
}
