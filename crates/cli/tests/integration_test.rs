use std::process::Command;

#[test]
fn test_cli_builds() {
    let output = Command::new("cargo")
        .args(["build", "--release", "--workspace"])
        .output()
        .expect("Failed to build workspace");
    assert!(output.status.success(), "Build failed:\n{}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "--help"])
        .output()
        .expect("Failed to run nexus");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nexus"));
    assert!(stdout.contains("analyze"));
    assert!(stdout.contains("impact"));
    assert!(stdout.contains("cycles"));
}

#[test]
fn test_cli_version() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "--version"])
        .output()
        .expect("Failed to run nexus");
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("0.1.0"));
}

#[test]
fn test_cli_stats_on_self() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "stats", "src/"])
        .output()
        .expect("Failed to run nexus stats");
    assert!(output.status.success(), "Stats failed:\n{}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Total files"));
    assert!(stdout.contains("Avg complexity"));
}

#[test]
fn test_cli_analyze_on_self() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "analyze", "src/"])
        .output()
        .expect("Failed to run nexus analyze");
    assert!(output.status.success(), "Analyze failed:\n{}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nexus"));
    assert!(stdout.contains("Summary"));
}

#[test]
fn test_cli_cycles_on_self() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "cycles", "src/"])
        .output()
        .expect("Failed to run nexus cycles");
    assert!(output.status.success());
}

#[test]
fn test_cli_isolated_on_self() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "isolated", "src/"])
        .output()
        .expect("Failed to run nexus isolated");
    assert!(output.status.success());
}

#[test]
fn test_cli_mermaid_on_self() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "mermaid", "src/"])
        .output()
        .expect("Failed to run nexus mermaid");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("graph TD"));
}

#[test]
fn test_cli_json_on_self() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "json", "src/"])
        .output()
        .expect("Failed to run nexus json");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("summary"));
    assert!(stdout.contains("total_files"));
}

#[test]
fn test_cli_unknown_command() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "foobar"])
        .output()
        .expect("Failed to run nexus");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown command"));
}

#[test]
fn test_build_release() {
    let output = Command::new("cargo")
        .args(["build", "--release", "--workspace"])
        .output()
        .expect("Failed to build");
    assert!(output.status.success());
    // Just verify build succeeds
}

#[test]
fn test_workspace_compiles() {
    let output = Command::new("cargo")
        .args(["check", "--workspace"])
        .output()
        .expect("Failed to check");
    assert!(output.status.success(), "Check failed:\n{}", String::from_utf8_lossy(&output.stderr));
}
