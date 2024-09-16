use std::process::Command;

#[test]
fn test_parse_string() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("parse")
        .arg("tests/parse/lox_files/string.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "foohello");
}

#[test]
fn test_parse_number() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("parse")
        .arg("tests/parse/lox_files/number.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "(+ 2.0 3.0)");
}
