use anyhow::{anyhow, Result};
use std::io::{self, Read, Write};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Endian {
    Little,
    Big,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OutFmt {
    Raw,
    Hex,
    C,   // \xHH\xHH...
    Py,  // b"\xHH..."
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InFmt {
    Raw,
    Hex,
}

pub fn width_bytes(width_bits: u32) -> usize {
    match width_bits {
        8 => 1,
        16 => 2,
        32 => 4,
        64 => 8,
        128 => 16,
        _ => panic!("unsupported width: {width_bits}"),
    }
}

pub fn parse_int(s: &str) -> Result<i128> {
    let t = s.trim();
    if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        Ok(i128::from_str_radix(h, 16)?)
    } else {
        Ok(t.parse::<i128>()?)
    }
}

pub fn pack_scalar(n: i128, width_bits: u32, endian: Endian, signed: bool, strict: bool) -> Result<Vec<u8>> {
    let bytes = width_bytes(width_bits);
    let bits = bytes * 8;

    if strict {
        if signed {
            let min = -((1i128) << (bits - 1));
            let max = ((1i128) << (bits - 1)) - 1;
            if n < min || n > max {
                return Err(anyhow!("value {n} does not fit in signed {bits}-bit"));
            }
        } else {
            if n < 0 {
                return Err(anyhow!("negative value {n} not allowed for unsigned (use --signed or remove --strict)"));
            }
            let max = ((1i128) << bits) - 1;
            if n > max {
                return Err(anyhow!("value {n} does not fit in unsigned {bits}-bit"));
            }
        }
    }

    let mask: u128 = if bits == 128 { u128::MAX } else { (1u128 << bits) - 1 };
    let v = (n as u128) & mask;

    let full = match endian {
        Endian::Little => v.to_le_bytes(),
        Endian::Big => v.to_be_bytes(),
    };
    let out = match endian {
        Endian::Little => full[..bytes].to_vec(),
        Endian::Big => full[16 - bytes..].to_vec(),
    };
    Ok(out)
}

pub fn unpack_scalar(bytes: &[u8], width_bits: u32, endian: Endian, signed: bool) -> i128 {
    let len = width_bytes(width_bits);
    assert_eq!(bytes.len(), len);

    let v = match endian {
        Endian::Little => {
            let mut tmp = [0u8; 16];
            tmp[..len].copy_from_slice(bytes);
            u128::from_le_bytes(tmp)
        }
        Endian::Big => {
            let mut tmp = [0u8; 16];
            tmp[16 - len..].copy_from_slice(bytes);
            u128::from_be_bytes(tmp)
        }
    };

    if signed {
        let bits = len * 8;
        let sign_bit = 1u128 << (bits - 1);
        let mask = (1u128 << bits) - 1;
        let val = v & mask;
        if (val & sign_bit) != 0 {
            // negative -> sign extend
            let ext_mask = (!0u128) << bits;
            (val | ext_mask) as i128
        } else {
            val as i128
        }
    } else {
        v as i128
    }
}

pub fn read_stdin_raw() -> Result<Vec<u8>> {
    let mut b = Vec::new();
    io::stdin().read_to_end(&mut b)?;
    Ok(b)
}

pub fn read_stdin_hex() -> Result<Vec<u8>> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    parse_hex_str(&s)
}

pub fn parse_hex_str(s: &str) -> Result<Vec<u8>> {
    use anyhow::anyhow;
    let mut out = Vec::new();

    for raw in s.split(|c: char| c.is_whitespace() || [':', ',', '-', ';'].contains(&c)) {
        if raw.is_empty() { continue; }
        let t = raw.strip_prefix("0x").or_else(|| raw.strip_prefix("0X")).unwrap_or(raw);

        if t.is_empty() { continue; }
        if t.len() % 2 != 0 {
            return Err(anyhow!("odd number of hex nibbles in token: {raw}"));
        }
        if !t.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow!("non-hex digit in token: {raw}"));
        }

        for i in (0..t.len()).step_by(2) {
            let byte = u8::from_str_radix(&t[i..i+2], 16)?;
            out.push(byte);
        }
    }
    Ok(out)
}

pub fn write_bytes(outfmt: OutFmt, data: &[u8], sep: &str, uppercase: bool) -> Result<()> {
    match outfmt {
        OutFmt::Raw => {
            let mut w = io::stdout().lock();
            w.write_all(data)?;
        }
        OutFmt::Hex => {
            let mut first = true;
            for b in data {
                if !first {
                    print!("{sep}");
                }
                if uppercase {
                    print!("{b:02X}");
                } else {
                    print!("{b:02x}");
                }
                first = false;
            }
            println!();
        }
        OutFmt::C => {
            // \xHH\xHH...
            for b in data {
                if uppercase {
                    print!("\\x{b:02X}");
                } else {
                    print!("\\x{b:02x}");
                }
            }
            println!();
        }
        OutFmt::Py => {
            print!("b\"");
            for b in data {
                if uppercase {
                    print!("\\x{b:02X}");
                } else {
                    print!("\\x{b:02x}");
                }
            }
            println!("\"");
        }
    }
    Ok(())
}


