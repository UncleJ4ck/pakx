use anyhow::Result;
use crate::util::{read_stdin_hex, read_stdin_raw, unpack_scalar, width_bytes, Endian};

pub fn run_unpack(
    width_bits: u32,
    input_hex: bool,      // true = hex, false = raw
    endian: Endian,
    signed: bool,
    count: Option<usize>,
) -> Result<()> {
    let data = if input_hex { read_stdin_hex()? } else { read_stdin_raw()? };
    let w = width_bytes(width_bits);

    let mut printed = 0usize;
    for chunk in data.chunks(w) {
        if chunk.len() != w { break; }
        let v = unpack_scalar(chunk, width_bits, endian, signed);
        println!("{v}");
        printed += 1;
        if let Some(c) = count {
            if printed >= c { break; }
        }
    }
    Ok(())
}
