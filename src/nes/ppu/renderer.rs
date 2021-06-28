use super::ColoredTile;

pub type Display = image::RgbImage;

pub fn render(display: &mut Display, tile: &ColoredTile, x: u8, y: u8) {
    for i in 0..8 {
        for j in 0..8 {
            display.put_pixel((x + i) as u32, (y + j) as u32, tile.get(i, j));
        }
    }
}
