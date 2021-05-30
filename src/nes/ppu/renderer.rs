use super::{ColoredSprite, Display};

pub fn render(display: &mut Display, sprite: &ColoredSprite, x: u8, y: u8) {
    for i in 0..8 {
        for j in 0..8 {
            display.put_pixel((x * 8 + i) as u32, (y * 8 + j) as u32, sprite.get(i, j));
        }
    }
}
