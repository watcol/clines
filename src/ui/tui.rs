use std::fs::File;
use std::io::{self, stderr, Write};
use std::os::unix::io::{AsRawFd, FromRawFd};

use super::{Button, Display, Ui};
use crossterm::{
    event::{poll, read},
    terminal::{disable_raw_mode, enable_raw_mode, size},
};
use device_query::{DeviceState, Keycode};
use std::time::Duration;

const BUFSIZE: usize = 65536;

struct Buffer {
    buf: [u8; BUFSIZE],
    index: usize,
}

impl Buffer {
    fn new() -> Self {
        Self {
            buf: [0; BUFSIZE],
            index: 0,
        }
    }

    fn extend(&mut self, s: &[u8]) {
        self.buf[self.index..self.index + s.len()].copy_from_slice(s);
        self.index += s.len();
    }

    fn push(&mut self, c: u8) {
        self.buf[self.index] = c;
        self.index += 1;
    }

    fn push_int<I: itoa::Integer>(&mut self, i: I) -> io::Result<()> {
        let n = itoa::write(&mut self.buf[self.index..], i)?;
        self.index += n;
        Ok(())
    }

    fn write<W: Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        let size = self.index;
        self.index = 0;
        writer.write(&self.buf[..size])
    }
}

pub struct Tui {
    buf: Buffer,
    out: File,
    state: DeviceState,
}

impl Tui {
    pub fn new() -> anyhow::Result<Self> {
        let out = AsRawFd::as_raw_fd(&stderr());
        let mut out: File = unsafe { FromRawFd::from_raw_fd(out) };
        enable_raw_mode()?;
        // execute!(buf, EnterAlternateScreen, Hide, SavePosition)?;
        out.write_all(b"\x1b[?1049h\x1b[?25l")?;
        Ok(Self {
            buf: Buffer::new(),
            out,
            state: DeviceState::new(),
        })
    }
}

impl Ui for Tui {
    fn flush(&mut self, display: &Display) -> anyhow::Result<()> {
        let (winwidth, winheight) = size()?;
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
        // execute!(self.buf, MoveTo(initial_width, initial_height))?;
        // write_bytes!(&mut self.buf, b"\x1b[{};{}H", initial_height, initial_width)?;
        self.buf.extend(b"\x1b[");
        self.buf.push_int(initial_height)?;
        self.buf.push(b';');
        self.buf.push_int(initial_width)?;
        self.buf.push(b'H');
        for y in 0..(height / 2) {
            for x in 0..width {
                let px1 = display.get(x, y * 2);
                let px2 = display.get(x, y * 2 + 1);
                if bg_color != Some(px1) {
                    bg_color = Some(px1);
                    // execute!(
                    //     self.buf,
                    //     SetBackgroundColor(Color::Rgb {
                    //         r: px1.0[0],
                    //         g: px1.0[1],
                    //         b: px1.0[2],
                    //     })
                    // )?;
                    self.buf.extend(b"\x1b[48;2;");
                    self.buf.push_int((px1.red * 255.0) as u8)?;
                    self.buf.push(b';');
                    self.buf.push_int((px1.green * 255.0) as u8)?;
                    self.buf.push(b';');
                    self.buf.push_int((px1.blue * 255.0) as u8)?;
                    self.buf.push(b'm');
                }

                if fg_color != Some(px2) {
                    fg_color = Some(px2);
                    // execute!(
                    //     self.buf,
                    //     SetForegroundColor(Color::Rgb {
                    //         r: px2.0[0],
                    //         g: px2.0[1],
                    //         b: px2.0[2],
                    //     })
                    // )?;
                    self.buf.extend(b"\x1b[38;2;");
                    self.buf.push_int((px2.red * 255.0) as u8)?;
                    self.buf.push(b';');
                    self.buf.push_int((px2.green * 255.0) as u8)?;
                    self.buf.push(b';');
                    self.buf.push_int((px2.blue * 255.0) as u8)?;
                    self.buf.push(b'm');
                }
                if bg_color == fg_color {
                    self.buf.push(b' ');
                } else {
                    self.buf.extend(b"\xe2\x96\x84");
                }
            }

            fg_color = None;
            bg_color = None;
            // execute!(
            //     self.buf,
            //     ResetColor,
            //     MoveTo(initial_width, initial_height + (y as u16) + 1)
            // )?;
            self.buf.extend(b"\x1b[m\x1b[");
            self.buf.push_int(initial_height + (y as u16) + 1)?;
            self.buf.push(b';');
            self.buf.push_int(initial_width)?;
            self.buf.push(b'H');
            if y % 8 == 7 {
                self.buf.write(&mut self.out)?;
            }
        }
        info!("Rendered");
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
        if self.state.query_keymap().contains(&key) {
            if button != Button::Quit && button != Button::Reset {
                debug!("Button {:?} pressed.", button);
            }
            true
        } else {
            if button != Button::Quit && button != Button::Reset {
                debug!("Button {:?} not pressed.", button);
            }
            false
        }
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        while matches!(poll(Duration::from_millis(10)), Ok(true)) {
            let _ = read();
        }
        self.out.write_all(b"\x1b[?25h\x1b[?1049l").unwrap();
    }
}
