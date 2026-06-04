//! Error types for workspace automation commands.

use std::path::PathBuf;

/// Result type used by xtask commands.
pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Error type used by xtask commands.
#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    /// A child command exited with a non-zero status.
    #[error("[xtask/error] command `{command}` failed with status {status}")]
    CommandFailed {
        /// The command line that exited unsuccessfully.
        command: String,
        /// The reported process exit status.
        status: String,
    },
    /// A child command could not be started.
    #[error("[xtask/error] command `{command}` could not be started")]
    CommandStart {
        /// The command line that failed to start.
        command: String,
        /// The underlying IO failure.
        source: std::io::Error,
    },
    /// The requested git base reference is not available locally.
    #[error("[xtask/error] git base `{base_ref}` is not available")]
    GitBaseUnavailable {
        /// The base ref that could not be resolved.
        base_ref: String,
    },
    /// Cargo did not expose a usable workspace root.
    #[error("[xtask/error] missing workspace root for manifest dir {manifest_dir}")]
    MissingWorkspaceRoot {
        /// The manifest directory reported by Cargo.
        manifest_dir: PathBuf,
    },
    /// The AI review command failed.
    #[error("[xtask/error] review command failed with status {status}")]
    ReviewFailed {
        /// The reported process exit status.
        status: String,
    },
    /// Rust file length audit found violations.
    #[error("[xtask/error] found {count} Rust file length violation(s)")]
    RustFileLengthViolations {
        /// Number of violations found.
        count: usize,
    },
    /// Rust source layout audit found violations.
    #[error("[xtask/error] found {count} Rust source layout violation(s)")]
    RustSourceViolations {
        /// Number of violations found.
        count: usize,
    },
    /// Rust trait double audit found violations.
    #[error("[xtask/error] found {count} Rust trait audit violation(s)")]
    RustTraitViolations {
        /// Number of violations found.
        count: usize,
    },
    /// A filesystem operation failed.
    #[error("[xtask/error] IO failure")]
    Io {
        /// The underlying IO failure.
        source: std::io::Error,
    },
    /// Child command output was not valid UTF-8.
    #[error("[xtask/error] command output was not valid UTF-8")]
    Utf8 {
        /// The underlying UTF-8 conversion failure.
        source: std::string::FromUtf8Error,
    },
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Self::Io { source }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(source: std::string::FromUtf8Error) -> Self {
        Self::Utf8 { source }
    }
}
