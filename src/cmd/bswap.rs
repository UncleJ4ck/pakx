use anyhow::Result;
use crate::util::{pack_scalar, write_bytes, Endian, OutFmt};

pub fn run_bswap(
    width_bits: u32,
    value: &str,
    outfmt: OutFmt,
    sep: &str,
    uppercase: bool,
) -> Result<()> {
    let n = crate::util::parse_int(value)?;
    let mut b = pack_scalar(n, width_bits, Endian::Big, /*signed*/ false, /*strict*/ false)?;
    b.reverse();
    write_bytes(outfmt, &b, sep, uppercase)
}
