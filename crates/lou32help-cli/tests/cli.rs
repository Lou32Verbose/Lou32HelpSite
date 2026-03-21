use lou32help_test_fixtures::{RECIPE_DOC, write_workspace as write_fixture_workspace};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

fn write_workspace() -> TempDir {
    let temp = write_fixture_workspace(&[("content/powershell/networking/bits.md", RECIPE_DOC)]);
    copy_build_support(temp.path());
    temp
}

fn copy_build_support(root: &Path) {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("repo root");

    fs::copy(repo_root.join("Cargo.toml"), root.join("Cargo.toml")).expect("workspace manifest");
    fs::copy(repo_root.join("Cargo.lock"), root.join("Cargo.lock")).expect("workspace lockfile");
    copy_dir_recursive(&repo_root.join("crates"), &root.join("crates"));
}

fn copy_dir_recursive(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create destination");
    for entry in fs::read_dir(source).expect("read source dir") {
        let entry = entry.expect("dir entry");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if source_path.is_dir() {
            copy_dir_recursive(&source_path, &destination_path);
        } else {
            fs::copy(&source_path, &destination_path).expect("copy file");
        }
    }
}

fn cli_bin() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_lou32help")
        .map(PathBuf::from)
        .expect("binary path")
}

fn run_cli(root: &Path, args: &[&str]) -> std::process::Output {
    Command::new(cli_bin())
        .arg("--root")
        .arg(root)
        .args(args)
        .output()
        .expect("run cli")
}

#[test]
fn check_passes_for_valid_workspace() {
    let temp = write_workspace();
    let output = run_cli(temp.path(), &["check"]);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("No validation issues found."));
}

#[test]
fn show_prints_matching_document() {
    let temp = write_workspace();
    let output = run_cli(
        temp.path(),
        &["show", "/powershell/networking/bits-transfer/"],
    );

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Bits Transfer"));
    assert!(stdout.contains("Slug: /powershell/networking/bits-transfer/"));
}

#[test]
fn search_enforces_min_query_length() {
    let temp = write_workspace();
    let output = run_cli(temp.path(), &["search", "b"]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("at least 2 character"));
}

#[test]
fn build_generates_site_artifacts() {
    let temp = write_workspace();
    let output = run_cli(temp.path(), &["build"]);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(temp.path().join("dist/site/index.html").exists());

    let search_index = fs::read_to_string(temp.path().join("dist/site/assets/search-index.json"))
        .expect("search index");
    assert!(!search_index.contains(r#""body":"#));
}
