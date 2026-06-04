//! Trait implemented by error enums with canonical internal fallback support.

use std::error::Error as StdError;

use crate::{DefinedAt, InternalError};

/// Shared contract implemented by error enums that expose
/// `Internal(#[from] InternalError)`.
pub trait ErrorContract: From<InternalError> + Sized {
    /// Module-level anchor for this error contract.
    const DEFINED_AT: DefinedAt;

    /// Converts one concrete source error into this contract's canonical
    /// internal fallback.
    #[track_caller]
    fn internal<E>(source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::from(InternalError::from_source(Self::DEFINED_AT, source))
    }

    /// Converts one concrete source error into this contract's canonical
    /// internal fallback and appends one context frame.
    #[track_caller]
    fn internal_with<E, M>(source: E, message: M) -> Self
    where
        E: StdError + Send + Sync + 'static,
        M: Into<String>,
    {
        Self::from(InternalError::with_context(
            Self::DEFINED_AT,
            source,
            message,
        ))
    }

    /// Creates one explicit internal-message failure for this contract.
    #[track_caller]
    fn internal_message<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::from(InternalError::message(Self::DEFINED_AT, message))
    }

    /// Appends one context frame to an existing `InternalError` and returns
    /// this contract's canonical internal fallback.
    #[track_caller]
    fn internal_context<M>(source: InternalError, message: M) -> Self
    where
        M: Into<String>,
    {
        Self::from(source.context(Self::DEFINED_AT, message))
    }
}
