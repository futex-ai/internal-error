//! CLI parsing and command dispatch for workspace automation.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::check::{CheckPhase, CheckSelection, run_check};
use crate::command::RealCommandRunner;
use crate::error::{Error, Result};
use crate::review::run_review;
use crate::rust_file_length_lint::{RustFileLengthLintMode, run_rust_file_length_lint};

#[derive(Parser, Debug)]
#[command(name = "xtask", about = "Workspace automation tasks")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the local equivalent of PR CI.
    Check(CheckArgs),
    /// Run a read-only AI code review against origin/main and local changes.
    Review,
    /// Audit changed or all Rust files for file-length limits.
    RustFileLengthLint(RustFileLengthLintArgs),
}

#[derive(Args, Debug)]
struct CheckArgs {
    /// Run only these verification phases. Repeat or use comma-separated values.
    #[arg(long, value_enum, value_delimiter = ',')]
    include: Vec<CheckPhaseArg>,
    /// Skip these verification phases. Repeat or use comma-separated values.
    #[arg(long, value_enum, value_delimiter = ',')]
    exclude: Vec<CheckPhaseArg>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum CheckPhaseArg {
    /// Markdown and whitespace diff checks.
    Scripts,
    /// GitHub Actions workflow linting.
    Workflow,
    /// Rust source, file-length, and trait-boundary audits.
    RustAudit,
    /// Rust formatting checks.
    RustFmt,
    /// Rust clippy checks.
    RustClippy,
    /// Rust workspace tests.
    RustTest,
}

impl From<CheckPhaseArg> for CheckPhase {
    fn from(phase: CheckPhaseArg) -> Self {
        match phase {
            CheckPhaseArg::Scripts => Self::Scripts,
            CheckPhaseArg::Workflow => Self::Workflow,
            CheckPhaseArg::RustAudit => Self::RustAudit,
            CheckPhaseArg::RustFmt => Self::RustFmt,
            CheckPhaseArg::RustClippy => Self::RustClippy,
            CheckPhaseArg::RustTest => Self::RustTest,
        }
    }
}

#[derive(Args, Debug)]
struct RustFileLengthLintArgs {
    /// Audit all Rust files under crates/ and xtask.
    #[arg(long)]
    all: bool,
}

pub(crate) fn main() -> ExitCode {
    let runner = RealCommandRunner;
    match run(Cli::parse(), &runner) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli, runner: &RealCommandRunner) -> Result<()> {
    let workspace_root = workspace_root()?;
    match cli.command {
        Commands::Check(args) => {
            let selection = CheckSelection::from_filters(
                args.include.into_iter().map(CheckPhase::from).collect(),
                args.exclude.into_iter().map(CheckPhase::from).collect(),
            );
            run_check(runner, workspace_root.as_path(), &selection)
        }
        Commands::Review => run_review(workspace_root.as_path()),
        Commands::RustFileLengthLint(args) => run_rust_file_length_lint(
            workspace_root.as_path(),
            if args.all {
                RustFileLengthLintMode::AllFiles
            } else {
                RustFileLengthLintMode::ChangedFiles
            },
        ),
    }
}

fn workspace_root() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let Some(workspace_root) = manifest_dir.parent() else {
        return Err(Error::MissingWorkspaceRoot { manifest_dir });
    };

    Ok(workspace_root.to_path_buf())
}

#[cfg(test)]
#[path = "_tests_/cli_tests.rs"]
mod cli_tests;
