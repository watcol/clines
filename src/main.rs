mod nes;
mod ui;

use nes::Rom;

pub type Display = image::RgbImage;

fn main() {
    let rom = Rom::from_path("sample1.nes").unwrap();
    println!("{:?}", rom);
}
