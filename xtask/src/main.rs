#![warn(unreachable_pub)]

//! Command-line entry point for workspace automation tasks.

mod check;
mod cli;
mod command;
mod error;
mod review;
mod rust_file_length_lint;
mod rust_source_audit;
mod rust_trait_audit;

fn main() -> std::process::ExitCode {
    cli::main()
}
