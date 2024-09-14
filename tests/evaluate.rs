use std::process::Command;

#[test]
fn test_empty_file_tokenize() {
    let output = Command::new("./target/debug/lox-rs")
        .arg("evaluate")
        .arg("tests/evaluate/lox_files/true.lox")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "");
}
