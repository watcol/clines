use std::{error::Error, fmt, fs::File, io::Read, path::Path};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mirroring: Mirroring,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mirroring {
    None,
    Horizontal,
    Vertical,
}

impl Rom {
    pub fn from_bytes<T: IntoIterator<Item = u8>>(bytes: T) -> anyhow::Result<Self> {
        let mut bytes = bytes.into_iter();
        validate_byte(&mut bytes, 0x4e)?;
        validate_byte(&mut bytes, 0x45)?;
        validate_byte(&mut bytes, 0x53)?;
        validate_byte(&mut bytes, 0x1a)?;
        let prg_len = get_byte(&mut bytes, String::from("PRG ROM length"))?;
        let chr_len = get_byte(&mut bytes, String::from("CHR ROM length"))?;
        let flag_6 = get_byte(&mut bytes, String::from("Flag byte"))?;
        let vertical_mirror = flag_6 & 0x01 == 0x01;
        let no_mirror = flag_6 & 0x08 == 0x08;
        let _flag_7 = get_byte(&mut bytes, String::from("Flag byte"))?;
        let _flag_8 = get_byte(&mut bytes, String::from("Flag byte"))?;
        let _flag_9 = get_byte(&mut bytes, String::from("Flag byte"))?;
        let _flag_10 = get_byte(&mut bytes, String::from("Flag byte"))?;
        let _flag_11 = get_byte(&mut bytes, String::from("Flag byte"))?;
        let _flag_12 = get_byte(&mut bytes, String::from("Flag byte"))?;
        let _flag_13 = get_byte(&mut bytes, String::from("Flag byte"))?;
        let _flag_14 = get_byte(&mut bytes, String::from("Flag byte"))?;
        let _flag_15 = get_byte(&mut bytes, String::from("Flag byte"))?;
        let prg_rom = take_bytes(
            &mut bytes,
            0x4000 * (prg_len as usize),
            String::from("PRG ROM"),
        )?;
        let chr_rom = take_bytes(
            &mut bytes,
            0x2000 * (chr_len as usize),
            String::from("CHR ROM"),
        )?;
        let mirroring = if no_mirror {
            Mirroring::None
        } else if vertical_mirror {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        };
        Ok(Rom {
            prg_rom,
            chr_rom,
            mirroring,
        })
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Self::from_bytes(buf)
    }
}

fn stringify(byte: u8) -> String {
    format!("{:#04x}", byte)
}

fn take_bytes<I: Iterator<Item = u8>>(
    iter: &mut I,
    len: usize,
    expected: String,
) -> anyhow::Result<Vec<u8>> {
    let bytes = iter.take(len).collect::<Vec<_>>();
    if bytes.len() == len {
        Ok(bytes)
    } else {
        Err(ParseError {
            expected,
            found: String::from("EOF"),
        }
        .into())
    }
}

fn validate_byte<I: Iterator<Item = u8>>(iter: &mut I, byte: u8) -> anyhow::Result<()> {
    let b = get_byte(iter, stringify(byte))?;
    if b == byte {
        Ok(())
    } else {
        Err(ParseError {
            found: stringify(b),
            expected: stringify(byte),
        }
        .into())
    }
}

fn get_byte<I: Iterator<Item = u8>>(iter: &mut I, expected: String) -> anyhow::Result<u8> {
    match iter.next() {
        Some(b) => Ok(b),
        None => Err(ParseError {
            expected,
            found: String::from("EOF"),
        }
        .into()),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    expected: String,
    found: String,
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expected {}, but found {}.", self.expected, self.found)
    }
}
