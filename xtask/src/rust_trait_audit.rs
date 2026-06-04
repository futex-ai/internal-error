//! Rust trait-boundary audit for handwritten test doubles.

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

pub(crate) fn run_rust_trait_audit(workspace_root: &Path) -> Result<()> {
    let files = rust_test_files(workspace_root)?;
    let violations = trait_double_violations(&files)?;
    for violation in &violations {
        eprintln!(
            "handwritten trait double in {}:{}",
            display_path(workspace_root, &violation.path),
            violation.line
        );
    }
    if !violations.is_empty() {
        return Err(Error::RustTraitViolations {
            count: violations.len(),
        });
    }
    eprintln!("rust trait audit passed");
    Ok(())
}

fn trait_double_violations(files: &[PathBuf]) -> Result<Vec<TraitAuditViolation>> {
    let mut violations = Vec::new();
    for file in files {
        let source = fs::read_to_string(file)?;
        for (index, line) in source.lines().enumerate() {
            if is_trait_impl_line(line) {
                violations.push(TraitAuditViolation {
                    path: file.clone(),
                    line: index + 1,
                });
            }
        }
    }
    Ok(violations)
}

fn is_trait_impl_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("impl ") && trimmed.contains(" for ") && !trimmed.contains("From<")
}

fn rust_test_files(workspace_root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for root in ["crates", "xtask"] {
        let path = workspace_root.join(root);
        if path.exists() {
            collect_test_files(&path, &mut files)?;
        }
    }
    files.sort();
    Ok(files)
}

fn collect_test_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if should_skip_dir(&entry_path) {
            continue;
        }
        if entry_path.is_dir() {
            collect_test_files(&entry_path, files)?;
            continue;
        }
        if is_test_file(&entry_path) {
            files.push(entry_path);
        }
    }
    Ok(())
}

fn is_test_file(path: &Path) -> bool {
    path.extension().and_then(|value| value.to_str()) == Some("rs")
        && path
            .components()
            .any(|component| component.as_os_str() == "_tests_" || component.as_os_str() == "tests")
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

struct TraitAuditViolation {
    path: PathBuf,
    line: usize,
}

#[cfg(test)]
#[path = "_tests_/rust_trait_audit_tests.rs"]
mod rust_trait_audit_tests;
