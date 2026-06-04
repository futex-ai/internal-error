//! Result helpers for converting `Result<T, E>` into internal failures.

use std::error::Error as StdError;

use crate::{DefinedAt, InternalError};

/// Converts `Result<T, E>` into `Result<T, InternalError>` or one local error
/// contract with canonical internal fallback.
pub trait ResultExt<T> {
    /// Converts the error side of this result into `InternalError`.
    fn internal_error(self, defined_at: DefinedAt) -> Result<T, InternalError>;

    /// Converts the error side of this result into `InternalError` and appends
    /// one context frame.
    fn internal_error_context<M>(
        self,
        defined_at: DefinedAt,
        message: M,
    ) -> Result<T, InternalError>
    where
        M: Into<String>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    #[track_caller]
    fn internal_error(self, defined_at: DefinedAt) -> Result<T, InternalError> {
        match self {
            Ok(value) => Ok(value),
            Err(source) => Err(InternalError::from_source(defined_at, source)),
        }
    }

    #[track_caller]
    fn internal_error_context<M>(
        self,
        defined_at: DefinedAt,
        message: M,
    ) -> Result<T, InternalError>
    where
        M: Into<String>,
    {
        let message = message.into();
        match self {
            Ok(value) => Ok(value),
            Err(source) => Err(InternalError::with_context(defined_at, source, message)),
        }
    }
}
