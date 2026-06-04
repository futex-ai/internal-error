//! Read-only AI review command.

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::command::{CommandStartResult, status_string};
use crate::error::{Error, Result};

pub(crate) fn run_review(workspace_root: &Path) -> Result<()> {
    let prompt = review_prompt(workspace_root)?;
    let review_dir = std::env::temp_dir().join(format!(
        "internal-error-xtask-review-{}",
        std::process::id()
    ));
    fs::create_dir_all(&review_dir)?;

    let mut child = Command::new("codex")
        .args([
            "--ask-for-approval",
            "never",
            "exec",
            "--ephemeral",
            "--ignore-rules",
            "--model",
            "gpt-5.5",
            "--config",
            "model_reasoning_effort=\"xhigh\"",
            "--sandbox",
            "read-only",
            "--skip-git-repo-check",
            "--cd",
        ])
        .arg(&review_dir)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err_to_start("codex review")?;

    {
        let Some(mut stdin) = child.stdin.take() else {
            return Err(Error::ReviewFailed {
                status: "stdin unavailable".to_owned(),
            });
        };
        stdin.write_all(prompt.as_bytes())?;
    }

    let status = child.wait()?;
    if !status.success() {
        return Err(Error::ReviewFailed {
            status: status_string(status),
        });
    }
    Ok(())
}

fn review_prompt(workspace_root: &Path) -> Result<String> {
    let status = git_capture(workspace_root, &["status", "--short"])?;
    let merge_base_diff = git_capture(workspace_root, &["diff", "--stat", "origin/main..."])?;
    let merge_base_names =
        git_capture(workspace_root, &["diff", "--name-status", "origin/main..."])?;
    let staged_diff = git_capture(workspace_root, &["diff", "--staged", "--stat", "--"])?;
    let staged_names = git_capture(workspace_root, &["diff", "--staged", "--name-status", "--"])?;
    let unstaged_diff = git_capture(workspace_root, &["diff", "--stat", "--"])?;
    let unstaged_names = git_capture(workspace_root, &["diff", "--name-status", "--"])?;
    let untracked = git_capture(
        workspace_root,
        &["ls-files", "--others", "--exclude-standard"],
    )?;

    Ok(review_prompt_from_parts(
        workspace_root,
        &ReviewPromptParts {
            status,
            merge_base_diff,
            merge_base_names,
            staged_diff,
            staged_names,
            unstaged_diff,
            unstaged_names,
            untracked,
        },
    ))
}

fn review_prompt_from_parts(workspace_root: &Path, parts: &ReviewPromptParts) -> String {
    format!(
        r#"You are reviewing local changes for the standalone internal-error Rust workspace.

Repository path:
{repo}

Review the local diff against origin/main. You may inspect the repository path
read-only for context. Focus on concrete bugs, missing tests, stale docs,
generated artifact drift, release automation risks, and CI failures. Do not
treat speculative or purely theoretical concerns as findings.

Return numbered findings first. Include file paths and line references when
possible. If there are no findings, say so clearly and mention residual test
risk.

Git status:
{status}

Merge-base diffstat:
{merge_base_diff}

Merge-base changed files:
{merge_base_names}

Staged diffstat:
{staged_diff}

Staged changed files:
{staged_names}

Unstaged diffstat:
{unstaged_diff}

Unstaged changed files:
{unstaged_names}

Untracked files:
{untracked}
"#,
        repo = workspace_root.display(),
        status = parts.status,
        merge_base_diff = parts.merge_base_diff,
        merge_base_names = parts.merge_base_names,
        staged_diff = parts.staged_diff,
        staged_names = parts.staged_names,
        unstaged_diff = parts.unstaged_diff,
        unstaged_names = parts.unstaged_names,
        untracked = parts.untracked,
    )
}

fn git_capture(workspace_root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(workspace_root)
        .output()?;
    if !output.status.success() {
        return Err(Error::GitBaseUnavailable {
            base_ref: "origin/main".to_owned(),
        });
    }
    Ok(String::from_utf8(output.stdout)?)
}

struct ReviewPromptParts {
    status: String,
    merge_base_diff: String,
    merge_base_names: String,
    staged_diff: String,
    staged_names: String,
    unstaged_diff: String,
    unstaged_names: String,
    untracked: String,
}
