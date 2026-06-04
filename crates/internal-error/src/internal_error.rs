//! Shared internal error carrier with context stacking and one source chain.

use std::{error::Error as StdError, fmt, panic::Location};

use thiserror::Error;

use crate::{ContextFrame, DefinedAt, SourceLocation};

/// Boxed source error used by `InternalError`.
pub type BoxError = Box<dyn StdError + Send + Sync + 'static>;

/// Shared internal-failure carrier used across crate boundaries.
#[derive(Error)]
#[error("internal error")]
pub struct InternalError {
    defined_at: DefinedAt,
    caller_at: SourceLocation,
    context_frames: Vec<ContextFrame>,
    #[source]
    source: BoxError,
}

impl InternalError {
    /// Creates a new internal error from one concrete source error.
    #[track_caller]
    pub fn from_source<E>(defined_at: DefinedAt, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        let caller_at = SourceLocation::from_location(Location::caller());
        Self::from_source_at(defined_at, caller_at, source)
    }

    /// Creates a new internal error from one explicit caller location and one
    /// concrete source error.
    pub fn from_source_at<E>(defined_at: DefinedAt, caller_at: SourceLocation, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::from_boxed_source_at(defined_at, caller_at, Box::new(source))
    }

    /// Creates a new internal error from one source error and one context frame.
    #[track_caller]
    pub fn with_context<E, M>(defined_at: DefinedAt, source: E, message: M) -> Self
    where
        E: StdError + Send + Sync + 'static,
        M: Into<String>,
    {
        let caller_at = SourceLocation::from_location(Location::caller());
        Self::with_context_at(defined_at, caller_at, source, message)
    }

    /// Creates a new internal error from one explicit caller location, one
    /// source error, and one context frame.
    pub fn with_context_at<E, M>(
        defined_at: DefinedAt,
        caller_at: SourceLocation,
        source: E,
        message: M,
    ) -> Self
    where
        E: StdError + Send + Sync + 'static,
        M: Into<String>,
    {
        Self::from_source_at(defined_at, caller_at, source)
            .context_at(defined_at, caller_at, message)
    }

    /// Creates a new internal error from one explicit internal message.
    #[track_caller]
    pub fn message<M>(defined_at: DefinedAt, message: M) -> Self
    where
        M: Into<String>,
    {
        let caller_at = SourceLocation::from_location(Location::caller());
        Self::message_at(defined_at, caller_at, message)
    }

    /// Creates a new internal error from one explicit caller location and one
    /// explicit internal message.
    pub fn message_at<M>(defined_at: DefinedAt, caller_at: SourceLocation, message: M) -> Self
    where
        M: Into<String>,
    {
        Self::from_boxed_source_at(
            defined_at,
            caller_at,
            Box::new(MessageError::new(message.into())),
        )
    }

    /// Appends one context frame without changing the underlying source chain.
    #[track_caller]
    pub fn context<M>(self, defined_at: DefinedAt, message: M) -> Self
    where
        M: Into<String>,
    {
        let caller_at = SourceLocation::from_location(Location::caller());
        self.context_at(defined_at, caller_at, message)
    }

    /// Returns the module-level error contract anchor where this internal
    /// error was first created.
    pub const fn defined_at(&self) -> DefinedAt {
        self.defined_at
    }

    /// Returns the creation call site for this internal error.
    pub const fn caller_at(&self) -> SourceLocation {
        self.caller_at
    }

    /// Returns the ordered context frame stack for this internal error.
    pub fn context_frames(&self) -> &[ContextFrame] {
        &self.context_frames
    }

    /// Returns the underlying source error.
    pub fn source_ref(&self) -> &(dyn StdError + Send + Sync + 'static) {
        self.source.as_ref()
    }

    /// Iterates over the underlying source chain without including `self`.
    pub fn source_chain(&self) -> SourceChain<'_> {
        SourceChain::new(self.source_ref())
    }

    /// Appends one context frame using one explicit caller location without
    /// changing the underlying source chain.
    pub fn context_at<M>(
        mut self,
        defined_at: DefinedAt,
        caller_at: SourceLocation,
        message: M,
    ) -> Self
    where
        M: Into<String>,
    {
        self.context_frames
            .push(ContextFrame::new(defined_at, caller_at, message.into()));
        self
    }

    pub(crate) fn from_boxed_source_at(
        defined_at: DefinedAt,
        caller_at: SourceLocation,
        source: BoxError,
    ) -> Self {
        match source.downcast::<InternalError>() {
            Ok(internal) => *internal,
            Err(source) => Self {
                defined_at,
                caller_at,
                context_frames: Vec::new(),
                source,
            },
        }
    }
}

impl fmt::Debug for InternalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source_chain = self
            .source_chain()
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        formatter
            .debug_struct("InternalError")
            .field("defined_at", &self.defined_at)
            .field("caller_at", &self.caller_at)
            .field("context_frames", &self.context_frames)
            .field("source_chain", &source_chain)
            .finish()
    }
}

/// Iterator over the underlying source chain of an `InternalError`.
pub struct SourceChain<'a> {
    next: Option<&'a (dyn StdError + 'static)>,
}

impl<'a> SourceChain<'a> {
    fn new(source: &'a (dyn StdError + 'static)) -> Self {
        Self { next: Some(source) }
    }
}

impl<'a> Iterator for SourceChain<'a> {
    type Item = &'a (dyn StdError + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next.take()?;
        self.next = current.source();
        Some(current)
    }
}

#[derive(Debug, Error)]
#[error("{message}")]
struct MessageError {
    message: String,
}

impl MessageError {
    fn new(message: String) -> Self {
        Self { message }
    }
}
