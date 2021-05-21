use std::{
    io::{self, BufWriter},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, RestorePosition, SavePosition, Show},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use image::RgbImage;

fn paint_image<W: io::Write>(buf: &mut W, image: &RgbImage) -> anyhow::Result<()> {
    let (winwidth, winheight) = terminal::size()?;
    let width = image.width();
    let height = image.height();
    let width16 = width as u16;
    let height16 = height as u16;
    let initial_width = if winwidth > width16 {
        (winwidth - width16) / 2
    } else {
        0
    };
    let initial_height = if winheight > height16 {
        (winheight - (height16 / 2)) / 2
    } else {
        0
    };
    let mut fg_color = None;
    let mut bg_color = None;
    queue!(buf, MoveTo(initial_width, initial_height))?;
    for y in 0..(height / 2) {
        for x in 0..width {
            let px1 = *image.get_pixel(x, y * 2);
            let px2 = *image.get_pixel(x, y * 2 + 1);
            if fg_color != Some(px1) {
                fg_color = Some(px1);
                queue!(
                    buf,
                    SetForegroundColor(Color::Rgb {
                        r: px1.0[0],
                        g: px1.0[1],
                        b: px1.0[2]
                    })
                )?;
            }

            if bg_color != Some(px2) {
                bg_color = Some(px2);
                queue!(
                    buf,
                    SetBackgroundColor(Color::Rgb {
                        r: px2.0[0],
                        g: px2.0[1],
                        b: px2.0[2]
                    })
                )?;
            }

            queue!(buf, Print("▀"))?;
        }

        fg_color = None;
        bg_color = None;
        queue!(
            buf,
            ResetColor,
            MoveTo(initial_width, initial_height + (y as u16) + 1)
        )?;
    }
    Ok(())
}

fn main() {
    let image = image::open("sample.png").unwrap().into_rgb8();
    let output = io::stderr();
    let mut output = BufWriter::new(output.lock());
    execute!(output, EnterAlternateScreen, Hide, SavePosition).unwrap();
    for _ in 0..30 {
        use io::Write;
        paint_image(&mut output, &image).unwrap();
        output.flush().unwrap();
        sleep(Duration::from_millis(10));
    }
    execute!(output, LeaveAlternateScreen, Show, RestorePosition).unwrap();
}
