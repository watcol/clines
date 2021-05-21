use std::io::{stderr, BufWriter, Stderr, Write};

use super::Ui;
use crossterm::{
    cursor::{Hide, MoveTo, RestorePosition, SavePosition, Show},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct Tui {
    buf: BufWriter<Stderr>,
}

impl Tui {
    pub fn new() -> anyhow::Result<Self> {
        let mut buf = BufWriter::new(stderr());
        execute!(buf, EnterAlternateScreen, Hide, SavePosition)?;
        Ok(Self { buf })
    }
}

impl Ui for Tui {
    fn flush(&mut self, display: &crate::Display) -> anyhow::Result<()> {
        let (winwidth, winheight) = terminal::size()?;
        let width = display.width();
        let height = display.height();
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
        queue!(self.buf, MoveTo(initial_width, initial_height))?;
        for y in 0..(height / 2) {
            for x in 0..width {
                let px1 = *display.get_pixel(x, y * 2);
                let px2 = *display.get_pixel(x, y * 2 + 1);
                if fg_color != Some(px1) {
                    fg_color = Some(px1);
                    queue!(
                        self.buf,
                        SetForegroundColor(Color::Rgb {
                            r: px1.0[0],
                            g: px1.0[1],
                            b: px1.0[2],
                        })
                    )?;
                }

                if bg_color != Some(px2) {
                    bg_color = Some(px2);
                    queue!(
                        self.buf,
                        SetBackgroundColor(Color::Rgb {
                            r: px2.0[0],
                            g: px2.0[1],
                            b: px2.0[2],
                        })
                    )?;
                }

                queue!(self.buf, Print('▀'))?;
            }

            fg_color = None;
            bg_color = None;
            queue!(
                self.buf,
                ResetColor,
                MoveTo(initial_width, initial_height + (y as u16) + 1)
            )?;
        }
        self.buf.flush()?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        execute!(self.buf, LeaveAlternateScreen, Show, RestorePosition).unwrap();
    }
}
