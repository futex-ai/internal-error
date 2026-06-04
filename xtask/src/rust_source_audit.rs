//! Rust source graph audit for orphan production files.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

pub(crate) fn run_rust_source_audit(workspace_root: &Path) -> Result<()> {
    let mut violations = Vec::new();
    for package in rust_packages(workspace_root)? {
        violations.extend(orphan_sources_for_package(&package)?);
    }
    for violation in &violations {
        eprintln!(
            "orphan Rust source file: {}",
            display_path(workspace_root, violation)
        );
    }
    if !violations.is_empty() {
        return Err(Error::RustSourceViolations {
            count: violations.len(),
        });
    }
    eprintln!("rust source audit passed");
    Ok(())
}

fn orphan_sources_for_package(package_root: &Path) -> Result<Vec<PathBuf>> {
    let src_root = package_root.join("src");
    if !src_root.exists() {
        return Ok(Vec::new());
    }
    let production_files = production_rust_files(&src_root)?;
    let reachable = reachable_sources(&src_root)?;
    Ok(production_files
        .into_iter()
        .filter(|file| !reachable.contains(file))
        .collect())
}

fn reachable_sources(src_root: &Path) -> Result<BTreeSet<PathBuf>> {
    let mut reachable = BTreeSet::new();
    let mut stack = target_roots(src_root)?;
    while let Some(file) = stack.pop() {
        if !reachable.insert(file.clone()) {
            continue;
        }
        for module_name in module_declarations(&file)? {
            for candidate in module_candidates(&file, &module_name) {
                if candidate.exists() {
                    stack.push(candidate);
                }
            }
        }
    }
    Ok(reachable)
}

fn target_roots(src_root: &Path) -> Result<Vec<PathBuf>> {
    let mut roots = Vec::new();
    for root_name in ["main.rs", "lib.rs"] {
        let path = src_root.join(root_name);
        if path.exists() {
            roots.push(path);
        }
    }
    let bin_dir = src_root.join("bin");
    if bin_dir.exists() {
        collect_rust_files(&bin_dir, &mut roots)?;
    }
    Ok(roots)
}

fn production_rust_files(src_root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_rust_files(src_root, &mut files)?;
    Ok(files
        .into_iter()
        .filter(|path| {
            !path
                .components()
                .any(|component| component.as_os_str() == "_tests_")
        })
        .collect())
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
    files.sort();
    Ok(())
}

fn module_declarations(file: &Path) -> Result<Vec<String>> {
    let source = fs::read_to_string(file)?;
    Ok(source.lines().filter_map(module_name_from_line).collect())
}

fn module_name_from_line(line: &str) -> Option<String> {
    let mut trimmed = line.split("//").next()?.trim();
    for prefix in ["pub(crate) ", "pub(super) ", "pub "] {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            trimmed = rest.trim();
            break;
        }
    }
    let rest = trimmed.strip_prefix("mod ")?;
    let name = rest.strip_suffix(';')?.trim();
    if name
        .chars()
        .all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
    {
        return Some(name.to_owned());
    }
    None
}

fn module_candidates(current_file: &Path, module_name: &str) -> Vec<PathBuf> {
    let current_dir = match current_file.parent() {
        Some(value) => value,
        None => Path::new("."),
    };
    vec![
        current_dir.join(format!("{module_name}.rs")),
        current_dir.join(module_name).join("mod.rs"),
    ]
}

fn rust_packages(workspace_root: &Path) -> Result<Vec<PathBuf>> {
    let mut packages = Vec::new();
    let xtask = workspace_root.join("xtask/Cargo.toml");
    if xtask.exists() {
        packages.push(workspace_root.join("xtask"));
    }
    let crates = workspace_root.join("crates");
    if crates.exists() {
        collect_package_roots(&crates, &mut packages)?;
    }
    packages.sort();
    Ok(packages)
}

fn collect_package_roots(path: &Path, packages: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if should_skip_dir(&entry_path) || !entry_path.is_dir() {
            continue;
        }
        if entry_path.join("Cargo.toml").exists() {
            packages.push(entry_path);
            continue;
        }
        collect_package_roots(&entry_path, packages)?;
    }
    Ok(())
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

#[cfg(test)]
#[path = "_tests_/rust_source_audit_tests.rs"]
mod rust_source_audit_tests;
