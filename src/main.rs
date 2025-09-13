use anyhow::Result;
use clap::{Parser, Subcommand, Args, ValueEnum};

use pakx::cmd::{pack::run_pack, unpack::run_unpack, bswap::run_bswap, bytes::run_bytes};
use pakx::util::{Endian, OutFmt};

#[derive(Parser)]
#[command(
    name = "pakx",
    version,
    about = "Tiny, pipe-friendly CLI for packing / unpacking / endianness / byte handling.",
    long_about = None,
    arg_required_else_help = true,
    before_help = r#"
PPPP    A   K   K X   X 
P   P  A A  K  K   X X  
PPPP  AAAAA KKK     X   
P     A   A K  K   X X  
P     A   A K   K X   X 
"#
)]struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Pack(GeneralPack),
    Unpack(GeneralUnpack),
    Bswap(BswapArgs),
    Bytes(BytesArgs),
    P8(PackSugar),
    P16(PackSugar),
    P32(PackSugar),
    P64(PackSugar),
    P128(PackSugar),
    U8(UnpackSugar),
    U16(UnpackSugar),
    U32(UnpackSugar),
    U64(UnpackSugar),
    U128(UnpackSugar),
}

#[derive(Args)]
struct GeneralPack {
    #[arg(long, value_parser = parse_width)]
    width: u32,
    #[arg(long, conflicts_with = "le")]
    be: bool,
    #[arg(long, conflicts_with = "be")]
    le: bool,
    #[arg(long)]
    signed: bool,
    #[arg(long, value_enum, default_value_t = OutFmtArg::Raw)]
    out: OutFmtArg,
    #[arg(long, default_value = " ")]
    sep: String,
    #[arg(long)]
    uppercase: bool,
    #[arg(long, conflicts_with = "trunc")]
    strict: bool,
    #[arg(long, hide = true)]
    trunc: bool,
    #[arg(long)]
    repeat: Option<usize>,
    #[arg(allow_negative_numbers = true)]
    values: Vec<String>,
}

#[derive(Args)]
struct GeneralUnpack {
    #[arg(long, value_parser = parse_width)]
    width: u32,
    #[arg(long, value_enum, default_value_t = InFmtArg::Raw)]
    r#in: InFmtArg,
    #[arg(long, conflicts_with = "le")]
    be: bool,
    #[arg(long, conflicts_with = "be")]
    le: bool,
    #[arg(long)]
    signed: bool,
    #[arg(long)]
    count: Option<usize>,
}

#[derive(Args)]
struct BswapArgs {
    #[arg(long, value_parser = parse_width)]
    width: u32,
    #[arg(long, value_enum, default_value_t = OutFmtArg::Hex)]
    out: OutFmtArg,
    #[arg(long, default_value = " ")]
    sep: String,
    #[arg(long)]
    uppercase: bool,
    value: String,
}

#[derive(Args)]
struct BytesArgs {
    #[arg(long, value_enum, default_value_t = InFmtArg::Hex)]
    r#in: InFmtArg,
    #[arg(long, value_enum, default_value_t = OutFmtArg::Hex)]
    out: OutFmtArg,
    #[arg(long, default_value = " ")]
    sep: String,
    #[arg(long)]
    uppercase: bool,
}

#[derive(Args)]
struct PackSugar {
    #[arg(long, conflicts_with = "le")]
    be: bool,
    #[arg(long, conflicts_with = "be")]
    le: bool,
    #[arg(long)]
    signed: bool,
    #[arg(long, value_enum, default_value_t = OutFmtArg::Raw)]
    out: OutFmtArg,
    #[arg(long, default_value = " ")]
    sep: String,
    #[arg(long)]
    uppercase: bool,
    #[arg(long, conflicts_with = "trunc")]
    strict: bool,
    #[arg(long, hide = true)]
    trunc: bool,
    #[arg(long)]
    repeat: Option<usize>,
    #[arg(allow_negative_numbers = true)]
    values: Vec<String>,
}

#[derive(Args)]
struct UnpackSugar {
    #[arg(long, value_enum, default_value_t = InFmtArg::Raw)]
    r#in: InFmtArg,
    #[arg(long, conflicts_with = "le")]
    be: bool,
    #[arg(long, conflicts_with = "be")]
    le: bool,
    #[arg(long)]
    signed: bool,
    #[arg(long)]
    count: Option<usize>,
}

#[derive(Copy, Clone, Eq, PartialEq, ValueEnum)]
enum OutFmtArg { Raw, Hex, C, Py }
#[derive(Copy, Clone, Eq, PartialEq, ValueEnum)]
enum InFmtArg { Raw, Hex }

fn endian_from(be: bool, _le: bool) -> Endian {
    if be { Endian::Big } else { Endian::Little }
}


fn outfmt_of(arg: OutFmtArg) -> OutFmt {
    match arg {
        OutFmtArg::Raw => OutFmt::Raw,
        OutFmtArg::Hex => OutFmt::Hex,
        OutFmtArg::C   => OutFmt::C,
        OutFmtArg::Py  => OutFmt::Py,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Pack(a) => {
            let endian = endian_from(a.be, a.le);
            run_pack(a.width, &a.values, endian, a.signed, outfmt_of(a.out), &a.sep, a.uppercase, a.strict, a.repeat)
        }
        Cmd::Unpack(a) => {
            let endian = endian_from(a.be, a.le);
            run_unpack(a.width, matches!(a.r#in, InFmtArg::Hex), endian, a.signed, a.count)
        }
        Cmd::Bswap(a) => {
            run_bswap(a.width, &a.value, outfmt_of(a.out), &a.sep, a.uppercase)
        }
        Cmd::Bytes(a) => {
            run_bytes(matches!(a.r#in, InFmtArg::Hex), outfmt_of(a.out), &a.sep, a.uppercase)
        }

        // Sugar: p*
        Cmd::P8(a)   => run_pack(8,   &a.values, endian_from(a.be, a.le), a.signed, outfmt_of(a.out), &a.sep, a.uppercase, a.strict, a.repeat),
        Cmd::P16(a)  => run_pack(16,  &a.values, endian_from(a.be, a.le), a.signed, outfmt_of(a.out), &a.sep, a.uppercase, a.strict, a.repeat),
        Cmd::P32(a)  => run_pack(32,  &a.values, endian_from(a.be, a.le), a.signed, outfmt_of(a.out), &a.sep, a.uppercase, a.strict, a.repeat),
        Cmd::P64(a)  => run_pack(64,  &a.values, endian_from(a.be, a.le), a.signed, outfmt_of(a.out), &a.sep, a.uppercase, a.strict, a.repeat),
        Cmd::P128(a) => run_pack(128, &a.values, endian_from(a.be, a.le), a.signed, outfmt_of(a.out), &a.sep, a.uppercase, a.strict, a.repeat),

        // Sugar: u*
        Cmd::U8(a)   => run_unpack(8,   matches!(a.r#in, InFmtArg::Hex), endian_from(a.be, a.le), a.signed, a.count),
        Cmd::U16(a)  => run_unpack(16,  matches!(a.r#in, InFmtArg::Hex), endian_from(a.be, a.le), a.signed, a.count),
        Cmd::U32(a)  => run_unpack(32,  matches!(a.r#in, InFmtArg::Hex), endian_from(a.be, a.le), a.signed, a.count),
        Cmd::U64(a)  => run_unpack(64,  matches!(a.r#in, InFmtArg::Hex), endian_from(a.be, a.le), a.signed, a.count),
        Cmd::U128(a) => run_unpack(128, matches!(a.r#in, InFmtArg::Hex), endian_from(a.be, a.le), a.signed, a.count),
    }
}

fn parse_width(s: &str) -> Result<u32, String> {
    match s {
        "8" => Ok(8),
        "16" => Ok(16),
        "32" => Ok(32),
        "64" => Ok(64),
        "128" => Ok(128),
        _ => Err("width must be one of: 8, 16, 32, 64, 128".into()),
    }
}
