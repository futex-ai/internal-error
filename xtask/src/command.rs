//! Command execution helpers used by xtask checks.

use std::ffi::OsStr;
use std::fmt::Write;
use std::process::Command;

use crate::error::{Error, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CommandSpec {
    program: String,
    args: Vec<String>,
}

impl CommandSpec {
    pub(crate) fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
        }
    }

    pub(crate) fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub(crate) fn args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    pub(crate) fn display(&self) -> String {
        let mut display = self.program.clone();
        for arg in &self.args {
            let _ = write!(display, " {}", shell_quote(arg));
        }
        display
    }
}

pub(crate) trait CommandRunner {
    fn run(&self, command: &CommandSpec) -> Result<()>;
}

pub(crate) struct RealCommandRunner;

impl CommandRunner for RealCommandRunner {
    fn run(&self, command: &CommandSpec) -> Result<()> {
        eprintln!("$ {}", command.display());
        let status = Command::new(&command.program)
            .args(command.args.iter().map(OsStr::new))
            .status()
            .map_err_to_start(&command.display())?;

        if !status.success() {
            return Err(Error::CommandFailed {
                command: command.display(),
                status: status_string(status),
            });
        }

        Ok(())
    }
}

pub(crate) trait CommandStartResult<T> {
    fn map_err_to_start(self, command: &str) -> Result<T>;
}

impl<T> CommandStartResult<T> for std::io::Result<T> {
    fn map_err_to_start(self, command: &str) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(source) => Err(Error::CommandStart {
                command: command.to_owned(),
                source,
            }),
        }
    }
}

pub(crate) fn status_string(status: std::process::ExitStatus) -> String {
    match status.code() {
        Some(code) => code.to_string(),
        None => "terminated by signal".to_owned(),
    }
}

fn shell_quote(value: &str) -> String {
    if value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '/' | '.' | ':' | '='))
    {
        return value.to_owned();
    }
    format!("{value:?}")
}
