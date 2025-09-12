use predicates::prelude::*;
use assert_cmd::Command;

fn bin() -> Command {
    Command::cargo_bin("pakx").unwrap()
}

#[test]
fn pack32_be_hex() {
    let mut cmd = bin();
    cmd.args(["pack", "--width", "32", "--be", "--out", "hex", "0x41424344"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"(?i)^41 42 43 44\n$").unwrap());
}

#[test]
fn p64_sugar_le_default_hex() {
    let mut cmd = bin();
    cmd.args(["p64", "0x0102030405060708", "--out", "hex"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^08 07 06 05 04 03 02 01\n$").unwrap());
}

#[test]
fn p64_be_hex() {
    let mut cmd = bin();
    cmd.args(["p64", "--be", "0x0102030405060708", "--out", "hex"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^01 02 03 04 05 06 07 08\n$").unwrap());
}

#[test]
fn bswap64_hex() {
    let mut cmd = bin();
    cmd.args(["bswap", "--width", "64", "--out", "hex", "0x4142434445464748"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^48 47 46 45 44 43 42 41\n$").unwrap());
}

#[test]
fn unpack32_hex_be() {
    let mut cmd = bin();
    cmd.args(["unpack", "--width", "32", "--in", "hex", "--be"]);
    cmd.write_stdin("41 42 43 44\n");
    cmd.assert().success().stdout("1094861636\n");
}

#[test]
fn strict_overflow_errors() {
    let mut cmd = bin();
    cmd.args(["p8", "0x1ff", "--strict"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("does not fit"));
}

#[test]
fn pack_signed_negative_strict_ok() {
    let mut cmd = bin();
    cmd.args(["p16", "--signed", "--strict", "-1", "--out", "hex"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^ff ff\n$").unwrap());
}

#[test]
fn unpack_signed_vs_unsigned() {
    let mut signed = bin();
    signed.args(["u16", "--signed", "--in", "hex"]);
    signed.write_stdin("ff ff\n");
    signed.assert().success().stdout("-1\n");

    let mut uns = bin();
    uns.args(["u16", "--in", "hex"]);
    uns.write_stdin("ff ff\n");
    uns.assert().success().stdout("65535\n");
}

#[test]
fn count_limits_output() {
    let mut cmd = bin();
    cmd.args(["unpack", "--width", "32", "--in", "hex", "--be", "--count", "1"]);
    cmd.write_stdin("00 00 00 2a 00 00 00 2b\n");
    cmd.assert().success().stdout("42\n");
}

#[test]
fn repeat_sequence() {
    let mut cmd = bin();
    cmd.args([
        "pack","--width","16","--be","--out","hex","--repeat","3","0x4142","0x4344"
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(
            r"^(41 42 43 44 ){2}41 42 43 44\n$"
        ).unwrap());
}

#[test]
fn hex_separator_and_uppercase() {
    let mut cmd = bin();
    cmd.args(["p32", "--be", "0x41424344", "--out", "hex", "--sep", ":", "--uppercase"]);
    cmd.assert().success().stdout("41:42:43:44\n");
}

#[test]
fn c_and_py_formats() {
    let mut cfmt = bin();
    cfmt.args(["p8", "0x41", "--out", "c", "--uppercase"]);
    cfmt.assert().success().stdout("\\x41\n");
    let mut pyfmt = bin();
    pyfmt.args(["p16", "--be", "0x4142", "--out", "py"]);
    pyfmt.assert().success().stdout("b\"\\x41\\x42\"\n");
}