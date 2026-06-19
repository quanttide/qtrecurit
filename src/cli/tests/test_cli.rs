use assert_cmd::Command;

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("qtrecurit").unwrap();
    cmd.arg("--help").assert().success();
}

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("qtrecurit").unwrap();
    cmd.arg("--version").assert().success();
}

#[test]
fn test_status_help() {
    let mut cmd = Command::cargo_bin("qtrecurit").unwrap();
    cmd.args(["status", "--help"]).assert().success();
}


