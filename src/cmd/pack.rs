use anyhow::Result;
use crate::util::{pack_scalar, write_bytes, Endian, OutFmt};

#[allow(clippy::too_many_arguments)]
pub fn run_pack(
    width_bits: u32,
    values: &[String],
    endian: Endian,
    signed: bool,
    outfmt: OutFmt,
    sep: &str,
    uppercase: bool,
    strict: bool,
    repeat: Option<usize>,
) -> Result<()> {
    let mut buf = Vec::new();

    for v in values {
        let n = crate::util::parse_int(v)?;
        let b = pack_scalar(n, width_bits, endian, signed, strict)?;
        buf.extend_from_slice(&b);
    }
    
    if let Some(times) = repeat {
        if times > 1 && !buf.is_empty() {
            let pattern = buf.clone();
            for _ in 1..times {
                buf.extend_from_slice(&pattern);
            }
        }
    }

    write_bytes(outfmt, &buf, sep, uppercase)
}
