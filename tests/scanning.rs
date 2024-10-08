use std::process::Command;

#[test]
fn test_scanning_empty_file_tokenize() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("tokenize")
        .arg("tests/scanning/lox_files/empty.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "EOF  null");
}

#[test]
fn test_scanning_var_declaration_tokenize() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("tokenize")
        .arg("tests/scanning/lox_files/var_declaration.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected_output = "VAR var null\nIDENTIFIER language null\nEQUAL = null\nSTRING \"lox\" lox\nSEMICOLON ; null\nLEFT_PAREN ( null\nLEFT_PAREN ( null\nRIGHT_PAREN ) null\nSEMICOLON ; null\nLEFT_PAREN ( null\nLEFT_BRACE { null\nSTAR * null\nDOT . null\nCOMMA , null\nPLUS + null\nSTAR * null\nRIGHT_BRACE } null\nRIGHT_PAREN ) null\nSEMICOLON ; null\nEOF  null";
    assert_eq!(stdout.trim(), expected_output);
}

#[test]
fn test_scanning_error_unknown_keyword() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("tokenize")
        .arg("tests/scanning/lox_files/error.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected_output =
        "COMMA , null\nDOT . null\nIDENTIFIER $ null\nLEFT_PAREN ( null\nIDENTIFIER # null\nSEMICOLON ; null\nIDENTIFIER # null\nLESS < null\nIDENTIFIER |SPACE null\nIDENTIFIER | null\nGREATER > null\nLEFT_PAREN ( null\nSEMICOLON ; null\nRIGHT_PAREN ) null\nLESS < null\nIDENTIFIER |TAB null\nIDENTIFIER | null\nGREATER > null\nIDENTIFIER @ null\nSEMICOLON ; null\nEOF  null";
    assert_eq!(stdout.trim(), expected_output);
}

#[test]
fn test_scanning_double_characters() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("tokenize")
        .arg("tests/scanning/lox_files/double_characters.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected_output =
        "EQUAL = null\nLEFT_BRACE { null\nEQUAL_EQUAL == null\nEQUAL = null\nRIGHT_BRACE } null\nSEMICOLON ; null\nBANG ! null\nBANG_EQUAL != null\nEQUAL_EQUAL == null\nSEMICOLON ; null\nLESS < null\nLESS_EQUAL <= null\nGREATER > null\nGREATER_EQUAL >= null\nSEMICOLON ; null\nSLASH / null\nLEFT_PAREN ( null\nRIGHT_PAREN ) null\nEOF  null";
    assert_eq!(stdout.trim(), expected_output);
}

#[test]
fn test_scanning_string_handling() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("tokenize")
        .arg("tests/scanning/lox_files/string.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected_output = "STRING \"foo baz\" foo baz\nEOF  null";
    assert_eq!(stdout.trim(), expected_output);
}

#[test]
fn test_scanning_number_handling() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("tokenize")
        .arg("tests/scanning/lox_files/number.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected_output =
        "NUMBER 42 42.0\nNUMBER 1234.1234 1234.1234\nNUMBER 48.0000 48.0\nEOF  null";
    assert_eq!(stdout.trim(), expected_output);
}

#[test]
fn test_scanning_identifier_handling() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("tokenize")
        .arg("tests/scanning/lox_files/identifier.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected_output =
        "IDENTIFIER foo null\nIDENTIFIER bar null\nIDENTIFIER _hello null\nIDENTIFIER _123_hello null\nNUMBER 6 6.0\nIDENTIFIER az null\nNUMBER 6 6.0\nIDENTIFIER ar null\nIDENTIFIER f00 null\nIDENTIFIER fo null\nAND and null\nLEFT_PAREN ( null\nRIGHT_PAREN ) null\nLESS < null\nGREATER > null\nLESS < null\nGREATER > null\nEOF  null";
    assert_eq!(stdout.trim(), expected_output);
}
