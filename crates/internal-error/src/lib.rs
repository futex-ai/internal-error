#![warn(unreachable_pub)]

//! Shared internal error helpers for Rust error contracts.

mod context_frame;
mod defined_at;
mod error_contract;
mod error_ext;
mod internal_error;
mod logging;
mod result_ext;

pub use context_frame::ContextFrame;
pub use defined_at::{DefinedAt, SourceLocation};
pub use error_contract::ErrorContract;
pub use error_ext::ErrorExt;
pub use internal_error::{BoxError, InternalError, SourceChain};
pub use internal_error_derive::ErrorContract;
pub use logging::log_top_level_error;
pub use result_ext::ResultExt;

/// Creates a `DefinedAt` anchor for the current module and source file.
#[macro_export]
macro_rules! defined_at {
    () => {
        $crate::DefinedAt::new(module_path!(), file!(), line!(), column!())
    };
}

#[cfg(test)]
#[path = "_tests_/lib_tests.rs"]
mod lib_tests;
