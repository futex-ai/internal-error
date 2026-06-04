//! Error helpers for converting one source error into internal failures.

use std::error::Error as StdError;

use crate::{DefinedAt, InternalError};

/// Converts one concrete source error into `InternalError` or a local error
/// contract with canonical internal fallback.
pub trait ErrorExt: StdError + Send + Sync + Sized + 'static {
    /// Converts this source error into `InternalError`.
    fn internal_error(self, defined_at: DefinedAt) -> InternalError;

    /// Converts this source error into `InternalError` and appends one context
    /// frame.
    fn internal_error_context<M>(self, defined_at: DefinedAt, message: M) -> InternalError
    where
        M: Into<String>;
}

impl<E> ErrorExt for E
where
    E: StdError + Send + Sync + Sized + 'static,
{
    #[track_caller]
    fn internal_error(self, defined_at: DefinedAt) -> InternalError {
        InternalError::from_source(defined_at, self)
    }

    #[track_caller]
    fn internal_error_context<M>(self, defined_at: DefinedAt, message: M) -> InternalError
    where
        M: Into<String>,
    {
        InternalError::with_context(defined_at, self, message)
    }
}
