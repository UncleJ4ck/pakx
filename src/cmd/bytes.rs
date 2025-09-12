use anyhow::Result;
use crate::util::{read_stdin_hex, read_stdin_raw, write_bytes, OutFmt};

pub fn run_bytes(
    input_hex: bool,  // true = hex, false = raw
    outfmt: OutFmt,
    sep: &str,
    uppercase: bool,
) -> Result<()> {
    let data = if input_hex { read_stdin_hex()? } else { read_stdin_raw()? };
    write_bytes(outfmt, &data, sep, uppercase)
}