//! Code generation for the `ErrorContract` derive macro.

mod extensions;
mod inherent;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub(super) fn expand(error_ident: &Ident) -> TokenStream {
    let result_ext_ident = format_ident!("{error_ident}ResultExt");
    let option_ext_ident = format_ident!("{error_ident}OptionExt");
    let inherent_impl = inherent::expand(error_ident);
    let extension_traits = extensions::expand(error_ident, &result_ext_ident, &option_ext_ident);

    quote! {
        #inherent_impl

        impl ::internal_error::ErrorContract for #error_ident {
            const DEFINED_AT: ::internal_error::DefinedAt = Self::DEFINED_AT;
        }

        #extension_traits
    }
}
