use std::process::Command;

#[test]
fn test_basic_parse() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("parse")
        .arg("tests/parse/lox_files/basic.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "foohello");
}
