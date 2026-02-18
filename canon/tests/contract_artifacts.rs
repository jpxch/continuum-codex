use std::process::Command;
use serde_json::Value;
use std::fs;

#[test]
fn artifacts_shape_is_stable() {
    let status = Command::new("cargo")
        .args(["run", "--bin", "compile_bleach_zanpakuto"])
        .status()
        .expect("Failed to run compiler");

    assert!(status.success());

    let data = fs::read_to_string("../artifacts/bleach/zanpakuto.json")
        .expect("Failed to read artifact");

    let v: Value = serde_json::from_str(&data).unwrap();

    let first = &v [0];

    assert!(first["shikai"]["release_command"].is_object());
    assert!(first["shikai"]["release_command"]["jp"].is_string());

    assert!(first["bankai"]["name"].is_object());
    assert!(first["bankai"]["name"]["jp"].is_string());
}