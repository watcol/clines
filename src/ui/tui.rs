use std::io::{stderr, BufWriter, Stderr, Write};

use super::Ui;
use crate::nes::{Button, Display};
use crossterm::{
    cursor::{Hide, MoveTo, RestorePosition, SavePosition, Show},
    event::{poll, read},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use device_query::{DeviceState, Keycode};
use std::time::Duration;

pub struct Tui {
    buf: BufWriter<Stderr>,
    state: DeviceState,
}

impl Tui {
    pub fn new() -> anyhow::Result<Self> {
        let mut buf = BufWriter::new(stderr());
        enable_raw_mode()?;
        execute!(buf, EnterAlternateScreen, Hide, SavePosition)?;
        Ok(Self {
            buf,
            state: DeviceState::new(),
        })
    }
}

impl Ui for Tui {
    fn flush(&mut self, display: &Display) -> anyhow::Result<()> {
        let (winwidth, winheight) = terminal::size()?;
        let width = display.width();
        let height = display.height();
        let width16 = width as u16;
        let height16 = (height / 2) as u16;
        let initial_width = if winwidth > width16 {
            (winwidth - width16) / 2
        } else {
            0
        };
        let initial_height = if winheight > height16 {
            (winheight - height16) / 2
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
                if bg_color != Some(px1) {
                    bg_color = Some(px1);
                    queue!(
                        self.buf,
                        SetBackgroundColor(Color::Rgb {
                            r: px1.0[0],
                            g: px1.0[1],
                            b: px1.0[2],
                        })
                    )?;
                }

                if fg_color != Some(px2) {
                    fg_color = Some(px2);
                    queue!(
                        self.buf,
                        SetForegroundColor(Color::Rgb {
                            r: px2.0[0],
                            g: px2.0[1],
                            b: px2.0[2],
                        })
                    )?;
                }

                queue!(self.buf, Print('▄'))?;
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

    fn button_pressed(&self, button: Button) -> bool {
        let key = match button {
            Button::A => Keycode::Z,
            Button::B => Keycode::X,
            Button::Select => Keycode::Space,
            Button::Start => Keycode::Enter,
            Button::Up => Keycode::Up,
            Button::Down => Keycode::Down,
            Button::Left => Keycode::Left,
            Button::Right => Keycode::Right,
            Button::Quit => Keycode::Escape,
            Button::Reset => Keycode::R,
        };
        self.state.query_keymap().contains(&key)
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        while matches!(poll(Duration::from_millis(10)), Ok(true)) {
            let _ = read();
        }
        execute!(self.buf, LeaveAlternateScreen, Show, RestorePosition).unwrap();
    }
}
