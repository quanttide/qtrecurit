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
fn test_report_help() {
    let mut cmd = Command::cargo_bin("qtrecurit").unwrap();
    cmd.args(["report", "--help"]).assert().success();
}

#[test]
fn test_refer_help() {
    let mut cmd = Command::cargo_bin("qtrecurit").unwrap();
    cmd.args(["refer", "--help"]).assert().success();
}
