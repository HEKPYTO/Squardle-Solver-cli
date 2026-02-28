use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_squaredle-cli"))
        .arg("--help")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("squaredle-cli"));
    assert!(stdout.contains("--json"));
}

#[test]
fn test_cli_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_squaredle-cli"))
        .arg("--version")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("squaredle-cli"));
}

#[test]
fn test_cli_no_color_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_squaredle-cli"))
        .arg("--help")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--no-color"));
}

#[test]
fn test_cli_json_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_squaredle-cli"))
        .arg("--help")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--json"));
}

#[test]
fn test_cli_daily_subcommand() {
    let output = Command::new(env!("CARGO_BIN_EXE_squaredle-cli"))
        .arg("--help")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("daily"));
}
