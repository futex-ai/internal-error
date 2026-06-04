//! Rust file length lint for changed or all workspace source files.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{Error, Result};

const MAX_RUST_LINES: usize = 300;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RustFileLengthLintMode {
    ChangedFiles,
    AllFiles,
}

pub(crate) fn run_rust_file_length_lint(
    workspace_root: &Path,
    mode: RustFileLengthLintMode,
) -> Result<()> {
    let files = match mode {
        RustFileLengthLintMode::ChangedFiles => changed_rust_files(workspace_root)?,
        RustFileLengthLintMode::AllFiles => all_rust_files(workspace_root)?,
    };
    let violations = file_length_violations(&files)?;
    for violation in &violations {
        eprintln!(
            "{} has {} lines; maximum is {}",
            display_path(workspace_root, &violation.path),
            violation.lines,
            MAX_RUST_LINES
        );
    }
    if !violations.is_empty() {
        return Err(Error::RustFileLengthViolations {
            count: violations.len(),
        });
    }
    eprintln!("rust file length lint passed for {} file(s)", files.len());
    Ok(())
}

fn file_length_violations(files: &[PathBuf]) -> Result<Vec<FileLengthViolation>> {
    let mut violations = Vec::new();
    for file in files {
        let lines = fs::read_to_string(file)?.lines().count();
        if lines > MAX_RUST_LINES {
            violations.push(FileLengthViolation {
                path: file.clone(),
                lines,
            });
        }
    }
    Ok(violations)
}

fn changed_rust_files(workspace_root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = BTreeSet::new();
    for path in git_lines(
        workspace_root,
        &[
            "diff",
            "--name-only",
            "--diff-filter=ACMRT",
            "origin/main...",
            "--",
        ],
    )? {
        insert_if_rust_workspace_file(workspace_root, &path, &mut files);
    }
    for path in git_lines(
        workspace_root,
        &["ls-files", "--others", "--exclude-standard"],
    )? {
        insert_if_rust_workspace_file(workspace_root, &path, &mut files);
    }
    Ok(files.into_iter().collect())
}

fn all_rust_files(workspace_root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for root in ["crates", "xtask"] {
        let path = workspace_root.join(root);
        if path.exists() {
            collect_rust_files(&path, &mut files)?;
        }
    }
    files.sort();
    Ok(files)
}

fn collect_rust_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if should_skip_dir(&entry_path) {
            continue;
        }
        if entry_path.is_dir() {
            collect_rust_files(&entry_path, files)?;
            continue;
        }
        if entry_path.extension().and_then(|value| value.to_str()) == Some("rs") {
            files.push(entry_path);
        }
    }
    Ok(())
}

fn insert_if_rust_workspace_file(root: &Path, path: &str, files: &mut BTreeSet<PathBuf>) {
    if !is_rust_workspace_path(path) {
        return;
    }
    let file = root.join(path);
    if file.exists() {
        files.insert(file);
    }
}

fn is_rust_workspace_path(path: &str) -> bool {
    path.ends_with(".rs") && (path.starts_with("crates/") || path.starts_with("xtask/"))
}

fn git_lines(workspace_root: &Path, args: &[&str]) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(args)
        .current_dir(workspace_root)
        .output()?;
    if !output.status.success() {
        return Err(Error::GitBaseUnavailable {
            base_ref: "origin/main".to_owned(),
        });
    }
    Ok(String::from_utf8(output.stdout)?
        .lines()
        .map(ToOwned::to_owned)
        .collect())
}

fn should_skip_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|value| value.to_str())
        .is_some_and(|name| matches!(name, ".git" | ".context" | "node_modules" | "target"))
}

fn display_path(root: &Path, path: &Path) -> String {
    let relative = match path.strip_prefix(root) {
        Ok(value) => value,
        Err(_error) => path,
    };
    relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

struct FileLengthViolation {
    path: PathBuf,
    lines: usize,
}

#[cfg(test)]
#[path = "_tests_/rust_file_length_lint_tests.rs"]
mod rust_file_length_lint_tests;
