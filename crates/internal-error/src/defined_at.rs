//! Module-level error contract anchors and concrete caller locations.

use std::{fmt, panic::Location};

/// Module-level anchor for an error contract.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DefinedAt {
    module_path: &'static str,
    file: &'static str,
    line: u32,
    column: u32,
}

impl DefinedAt {
    /// Creates a new module-level error contract anchor.
    pub const fn new(
        module_path: &'static str,
        file: &'static str,
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            module_path,
            file,
            line,
            column,
        }
    }

    /// Returns the Rust module path where the error contract is anchored.
    pub const fn module_path(&self) -> &'static str {
        self.module_path
    }

    /// Returns the source file where the error contract is anchored.
    pub const fn file(&self) -> &'static str {
        self.file
    }

    /// Returns the line where the error contract is anchored.
    pub const fn line(&self) -> u32 {
        self.line
    }

    /// Returns the column where the error contract is anchored.
    pub const fn column(&self) -> u32 {
        self.column
    }
}

impl fmt::Display for DefinedAt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} ({}:{}:{})",
            self.module_path, self.file, self.line, self.column
        )
    }
}

/// Concrete call site where an internal error was created or recontextualized.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SourceLocation {
    file: &'static str,
    line: u32,
    column: u32,
}

impl SourceLocation {
    /// Creates a new concrete source location.
    pub const fn new(file: &'static str, line: u32, column: u32) -> Self {
        Self { file, line, column }
    }

    /// Returns the source file for this call site.
    pub const fn file(&self) -> &'static str {
        self.file
    }

    /// Returns the source line for this call site.
    pub const fn line(&self) -> u32 {
        self.line
    }

    /// Returns the source column for this call site.
    pub const fn column(&self) -> u32 {
        self.column
    }

    /// Converts one Rust `Location` into a concrete source location.
    pub fn from_location(location: &'static Location<'static>) -> Self {
        Self::new(location.file(), location.line(), location.column())
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}:{}", self.file, self.line, self.column)
    }
}
