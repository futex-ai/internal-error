use std::fs;
use std::path::PathBuf;

use super::{RustFileLengthLintMode, run_rust_file_length_lint};

#[test]
fn all_mode_fails_for_long_rust_files() {
    let root = temp_root("file-length-long");
    let source = root.join("xtask/src/main.rs");
    fs::create_dir_all(source.parent().expect("source parent")).expect("create src");
    fs::write(&source, repeated_lines(301)).expect("write long source");

    let result = run_rust_file_length_lint(&root, RustFileLengthLintMode::AllFiles);

    assert!(result.is_err());
}

#[test]
fn all_mode_passes_for_short_rust_files() {
    let root = temp_root("file-length-short");
    let source = root.join("xtask/src/main.rs");
    fs::create_dir_all(source.parent().expect("source parent")).expect("create src");
    fs::write(&source, repeated_lines(12)).expect("write short source");

    let result = run_rust_file_length_lint(&root, RustFileLengthLintMode::AllFiles);

    assert!(result.is_ok());
}

fn repeated_lines(count: usize) -> String {
    (0..count)
        .map(|index| format!("// line {index}\n"))
        .collect()
}

fn temp_root(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "internal-error-xtask-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("create temp root");
    root
}
