use predicates::prelude::*;
use assert_cmd::Command;

fn bin() -> Command { Command::cargo_bin("pakx").unwrap() }

#[test]
fn bytes_hex_to_hex_normalizes_separators() {
    let mut cmd = bin();
    cmd.args(["bytes", "--in", "hex", "--out", "hex"]);
    cmd.write_stdin("de:ad, be ef  0x00\n");
    cmd.assert().success().stdout("de ad be ef 00\n");
}

#[test]
fn bytes_hex_odd_nibbles_is_error() {
    let mut cmd = bin();
    cmd.args(["bytes", "--in", "hex", "--out", "hex"]);
    cmd.write_stdin("0 a b cc d");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("odd number of hex nibbles"));
}

#[test]
fn unpack_hex_bad_byte_errors() {
    let mut cmd = bin();
    cmd.args(["unpack", "--width", "8", "--in", "hex"]);
    cmd.write_stdin("gg\n");
    cmd.assert()
        .failure()       
        .stderr(
           predicate::str::contains("non-hex digit")
               .or(predicate::str::contains("invalid"))
               .or(predicate::str::contains("bad hex byte")),
       );
}
