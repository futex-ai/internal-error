//! Shared helpers for debug-rich top-level error logging.

use std::fmt;

/// Logs one top-level error with its full `Debug` representation.
pub fn log_top_level_error<E>(error: &E)
where
    E: fmt::Debug + ?Sized,
{
    tracing::error!(error = ?error, "top-level error");
}
