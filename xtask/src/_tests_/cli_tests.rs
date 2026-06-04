use clap::Parser;

use super::{Cli, Commands};

#[test]
fn parses_check_command() {
    let cli = Cli::try_parse_from(["xtask", "check", "--include", "rust-fmt"]).expect("parse");

    match cli.command {
        Commands::Check(args) => assert_eq!(args.include.len(), 1),
        Commands::Review | Commands::RustFileLengthLint(_) => {
            panic!("expected check command")
        }
    }
}

#[test]
fn parses_review_command() {
    let cli = Cli::try_parse_from(["xtask", "review"]).expect("parse");

    match cli.command {
        Commands::Review => {}
        Commands::Check(_) | Commands::RustFileLengthLint(_) => {
            panic!("expected review command")
        }
    }
}
