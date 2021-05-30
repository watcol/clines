use super::{Display, Sprite};
use image::Rgb;

pub fn render(display: &mut Display, sprite: &Sprite, x: u8, y: u8) {
    for i in 0..8 {
        for j in 0..8 {
            let pixel = match sprite.get(i, j) {
                0 => Rgb([0x00, 0x00, 0x00]),
                1 => Rgb([0x55, 0x55, 0x55]),
                2 => Rgb([0xaa, 0xaa, 0xaa]),
                3 => Rgb([0xff, 0xff, 0xff]),
                _ => unreachable!(),
            };
            display.put_pixel((x * 8 + i) as u32, (y * 8 + j) as u32, pixel);
        }
    }
}
