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

#[test]
fn test_access_help() {
    let mut cmd = Command::cargo_bin("qtrecurit").unwrap();
    cmd.args(["access", "--help"]).assert().success();
}

#[test]
fn test_inbox_help() {
    let mut cmd = Command::cargo_bin("qtrecurit").unwrap();
    cmd.args(["inbox", "--help"]).assert().success();
}

#[test]
fn test_inbox_sync_dry_run_json() {
    let mut cmd = Command::cargo_bin("qtrecurit").unwrap();
    cmd.args(["inbox", "sync", "--dry-run", "--format", "json"])
        .assert()
        .success()
        .stdout(predicates::str::contains("\"status\":\"dry_run\""))
        .stdout(predicates::str::contains("\"candidates\""));
}
