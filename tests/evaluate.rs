use std::process::Command;

#[test]
fn test_evaluate_true() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("evaluate")
        .arg("tests/evaluate/lox_files/true.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "true");
}

#[test]
fn test_evaluate_false() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("evaluate")
        .arg("tests/evaluate/lox_files/false.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "false");
}

#[test]
fn test_evaluate_nil() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("evaluate")
        .arg("tests/evaluate/lox_files/nil.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "nil");
}
