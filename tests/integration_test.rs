use std::process::Command;

#[test]
fn test_empty_file_tokenize() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("tokenize")
        .arg("tests/lox_files/empty.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "EOF  null");
}

#[test]
fn test_var_declaration_tokenize() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("tokenize")
        .arg("tests/lox_files/var_declaration.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected_output = "VAR var null\nIDENTIFIER language null\nEQUAL = null\nSTRING lox lox\nSEMICOLON ; null\nLEFT_PAREN ( null\nLEFT_PAREN ( null\nRIGHT_PAREN ) null\nSEMICOLON ; null\nLEFT_PAREN ( null\nLEFT_BRACE { null\nSTAR * null\nDOT . null\nCOMMA , null\nPLUS + null\nSTAR * null\nRIGHT_BRACE } null\nRIGHT_PAREN ) null\nSEMICOLON ; null\nEQUAL = null\nLEFT_BRACE { null\nEQUAL_EQUAL == null\nEQUAL = null\nRIGHT_BRACE } null\nSEMICOLON ; null\nEOF  null";
    assert_eq!(stdout.trim(), expected_output);
}

#[test]
fn test_error_unknown_keyword() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("tokenize")
        .arg("tests/lox_files/error.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected_output =
        "COMMA , null\nDOT . null\nLEFT_PAREN ( null\nSEMICOLON ; null\nEOF  null";
    assert_eq!(stdout.trim(), expected_output);
}
