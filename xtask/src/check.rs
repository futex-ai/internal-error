//! Generic local verification orchestration.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use crate::command::{CommandRunner, CommandSpec};
use crate::error::Result;
use crate::rust_file_length_lint::{RustFileLengthLintMode, run_rust_file_length_lint};
use crate::rust_source_audit::run_rust_source_audit;
use crate::rust_trait_audit::run_rust_trait_audit;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum CheckPhase {
    Scripts,
    Workflow,
    RustAudit,
    RustFmt,
    RustClippy,
    RustTest,
}

#[derive(Debug)]
pub(crate) struct CheckSelection {
    included: BTreeSet<CheckPhase>,
    excluded: BTreeSet<CheckPhase>,
}

impl CheckSelection {
    pub(crate) fn from_filters(
        included: BTreeSet<CheckPhase>,
        excluded: BTreeSet<CheckPhase>,
    ) -> Self {
        Self { included, excluded }
    }

    pub(crate) fn contains(&self, phase: CheckPhase) -> bool {
        if self.excluded.contains(&phase) {
            return false;
        }
        self.included.is_empty() || self.included.contains(&phase)
    }
}

pub(crate) fn run_check(
    runner: &dyn CommandRunner,
    workspace_root: &Path,
    selection: &CheckSelection,
) -> Result<()> {
    if selection.contains(CheckPhase::Scripts) {
        run_script_checks(runner, workspace_root)?;
    }
    if selection.contains(CheckPhase::Workflow) {
        run_workflow_checks(runner, workspace_root)?;
    }
    if selection.contains(CheckPhase::RustAudit) {
        run_rust_audits(workspace_root)?;
    }
    if selection.contains(CheckPhase::RustFmt) {
        runner.run(&CommandSpec::new("cargo").args(["fmt", "--all", "--", "--check"]))?;
    }
    if selection.contains(CheckPhase::RustClippy) {
        runner.run(&CommandSpec::new("cargo").args([
            "clippy",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ]))?;
    }
    if selection.contains(CheckPhase::RustTest) {
        runner.run(&CommandSpec::new("cargo").args(["test", "--workspace"]))?;
    }
    Ok(())
}

fn run_script_checks(runner: &dyn CommandRunner, workspace_root: &Path) -> Result<()> {
    let markdown_files = markdown_files(workspace_root)?;
    if !markdown_files.is_empty() {
        let mut args = vec!["--yes".to_owned(), "markdownlint-cli2".to_owned()];
        args.extend(markdown_files);
        runner.run(&CommandSpec::new("npx").args(args))?;
    }
    runner.run(&CommandSpec::new("git").args(["diff", "--check"]))?;
    Ok(())
}

fn run_workflow_checks(runner: &dyn CommandRunner, workspace_root: &Path) -> Result<()> {
    if !workspace_root.join(".github/workflows").exists() {
        eprintln!("skipping workflow checks: no .github/workflows directory");
        return Ok(());
    }
    runner.run(&CommandSpec::new("actionlint").arg("-shellcheck="))?;
    Ok(())
}

fn run_rust_audits(workspace_root: &Path) -> Result<()> {
    run_rust_file_length_lint(workspace_root, RustFileLengthLintMode::ChangedFiles)?;
    run_rust_source_audit(workspace_root)?;
    run_rust_trait_audit(workspace_root)?;
    Ok(())
}

fn markdown_files(workspace_root: &Path) -> Result<Vec<String>> {
    let mut files = Vec::new();
    collect_markdown_files(workspace_root, workspace_root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_markdown_files(root: &Path, path: &Path, files: &mut Vec<String>) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if should_skip_dir(&entry_path) {
            continue;
        }
        if entry_path.is_dir() {
            collect_markdown_files(root, &entry_path, files)?;
            continue;
        }
        if is_markdown_file(&entry_path) {
            files.push(relative_path(root, &entry_path));
        }
    }
    Ok(())
}

fn is_markdown_file(path: &Path) -> bool {
    if path
        .file_name()
        .and_then(|value| value.to_str())
        .is_some_and(|name| matches!(name, "AGENTS.md" | "CLAUDE.md"))
    {
        return false;
    }
    path.extension().and_then(|value| value.to_str()) == Some("md")
}

fn should_skip_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|value| value.to_str())
        .is_some_and(|name| matches!(name, ".git" | ".context" | "node_modules" | "target"))
}

fn relative_path(root: &Path, path: &Path) -> String {
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
#[path = "_tests_/check_tests.rs"]
mod check_tests;
