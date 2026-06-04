//! Derive expansion for the `ErrorContract` proc macro.

mod expansion;
mod validation;

use proc_macro2::TokenStream;
use syn::{DeriveInput, Result};

pub(crate) fn expand(input: DeriveInput) -> Result<TokenStream> {
    validation::validate(&input)?;
    Ok(expansion::expand(&input.ident))
}
