//! Context frames added while internal errors cross higher-level boundaries.

use crate::{DefinedAt, SourceLocation};

/// One breadcrumb added while an internal error moves through higher-level code.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextFrame {
    defined_at: DefinedAt,
    caller_at: SourceLocation,
    message: String,
}

impl ContextFrame {
    pub(crate) fn new(defined_at: DefinedAt, caller_at: SourceLocation, message: String) -> Self {
        Self {
            defined_at,
            caller_at,
            message,
        }
    }

    /// Returns the error-contract anchor that added this context frame.
    pub const fn defined_at(&self) -> DefinedAt {
        self.defined_at
    }

    /// Returns the call site that added this context frame.
    pub const fn caller_at(&self) -> SourceLocation {
        self.caller_at
    }

    /// Returns the human-readable breadcrumb message for this frame.
    pub fn message(&self) -> &str {
        &self.message
    }
}
