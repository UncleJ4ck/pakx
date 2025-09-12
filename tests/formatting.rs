use assert_cmd::Command;

fn bin() -> Command { Command::cargo_bin("pakx").unwrap() }

#[test]
fn p32_le_default() {
    let mut cmd = bin();
    cmd.args(["p32", "0x11223344", "--out", "hex"]);
    cmd.assert().success().stdout("44 33 22 11\n");
}

#[test]
fn p32_be() {
    let mut cmd = bin();
    cmd.args(["p32", "--be", "0x11223344", "--out", "hex"]);
    cmd.assert().success().stdout("11 22 33 44\n");
}

#[test]
fn u64_hex_le_vs_be() {
    let data = "01 02 03 04 05 06 07 08\n";
    let mut le = bin();
    le.args(["u64", "--in", "hex"]);
    le.write_stdin(data);
    le.assert().success().stdout("578437695752307201\n");

    let mut be = bin();
    be.args(["u64", "--in", "hex", "--be"]);
    be.write_stdin(data);
    be.assert().success().stdout("72623859790382856\n"); 
}

#[test]
fn bswap16_32() {
    let mut s16 = bin();
    s16.args(["bswap", "--width", "16", "--out", "hex", "0x1234"]);
    s16.assert().success().stdout("34 12\n");

    let mut s32 = bin();
    s32.args(["bswap", "--width", "32", "--out", "hex", "0x11223344"]);
    s32.assert().success().stdout("44 33 22 11\n");
}
