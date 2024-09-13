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
    let expected_output = "VAR var null\nIDENTIFIER language null\nEQUAL = null\nSTRING lox lox\nSEMICOLON ; null\nLEFT_PAREN ( null\nLEFT_PAREN ( null\nRIGHT_PAREN ) null\nSEMICOLON ; null\nEOF  null";
    assert_eq!(stdout.trim(), expected_output);
}
