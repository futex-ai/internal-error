#![warn(unreachable_pub)]

//! Derive macros for `internal-error`.

mod error_contract_derive;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Derives the shared `internal_error::ErrorContract` binding for one error
/// enum with an explicit `Internal(#[from] InternalError)` variant.
#[proc_macro_derive(ErrorContract)]
pub fn derive_error_contract(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    error_contract_derive::expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[cfg(test)]
#[path = "_tests_/lib_tests.rs"]
mod lib_tests;
