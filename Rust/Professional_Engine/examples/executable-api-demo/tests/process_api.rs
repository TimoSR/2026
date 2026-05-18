use std::fs;
use std::io::Write;

use assert_cmd::Command;
use tempfile::NamedTempFile;

#[test]
fn run_command_supports_json_output() {
    let mut cmd = Command::cargo_bin("executable-api-demo").expect("binary should build");
    let output = cmd
        .args(["run", "--frames", "3", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).expect("stdout should be utf-8");
    let payload: serde_json::Value = serde_json::from_str(&text).expect("stdout should be json");

    assert_eq!(payload["command"], "run");
    assert_eq!(payload["frames"], 3);
}

#[test]
fn validate_assets_returns_validation_exit_code_for_invalid_manifest() {
    let mut manifest = NamedTempFile::new().expect("temp file should be created");
    writeln!(manifest, "textures/player.bmp").expect("should write manifest");

    Command::cargo_bin("executable-api-demo")
        .expect("binary should build")
        .args([
            "validate-assets",
            "--manifest",
            manifest
                .path()
                .to_str()
                .expect("path should be valid utf-8"),
        ])
        .assert()
        .code(20);
}

#[test]
fn validate_assets_succeeds_for_valid_manifest() {
    let mut manifest = NamedTempFile::new().expect("temp file should be created");
    writeln!(manifest, "textures/player.png").expect("should write manifest");
    writeln!(manifest, "audio/footstep.ogg").expect("should write manifest");

    let mut cmd = Command::cargo_bin("executable-api-demo").expect("binary should build");
    cmd.args([
        "validate-assets",
        "--manifest",
        manifest
            .path()
            .to_str()
            .expect("path should be valid utf-8"),
        "--json",
    ])
    .assert()
    .success();
}

#[test]
fn run_command_reads_json_from_file_config() {
    let file = NamedTempFile::new().expect("temp config should be created");
    fs::write(file.path(), r#"{ "frames": 9, "scene": "file-scene" }"#)
        .expect("config should be written");

    let mut cmd = Command::cargo_bin("executable-api-demo").expect("binary should build");
    let output = cmd
        .args([
            "run",
            "--frames",
            "2",
            "--config",
            file.path().to_str().expect("path should be valid utf-8"),
            "--json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let payload: serde_json::Value =
        serde_json::from_slice(&output).expect("stdout should be json");
    assert_eq!(payload["frames"], 9);
    assert_eq!(payload["scene"], "file-scene");
    assert_eq!(payload["config_source"], "file");
}
