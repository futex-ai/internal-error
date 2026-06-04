//! Inherent helper generation for derived error enums.

use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub(super) fn expand(error_ident: &Ident) -> TokenStream {
    quote! {
        impl #error_ident {
            const DEFINED_AT: ::internal_error::DefinedAt = ::internal_error::DefinedAt::new(
                module_path!(),
                file!(),
                line!(),
                column!(),
            );

            fn internal_at<E>(caller_at: ::internal_error::SourceLocation, source: E) -> Self
            where
                E: ::std::error::Error + Send + Sync + 'static,
            {
                Self::from(::internal_error::InternalError::from_source_at(
                    Self::DEFINED_AT,
                    caller_at,
                    source,
                ))
            }

            fn internal_with_at<E, M>(
                caller_at: ::internal_error::SourceLocation,
                source: E,
                message: M,
            ) -> Self
            where
                E: ::std::error::Error + Send + Sync + 'static,
                M: ::core::convert::Into<::std::string::String>,
            {
                Self::from(::internal_error::InternalError::with_context_at(
                    Self::DEFINED_AT,
                    caller_at,
                    source,
                    message,
                ))
            }

            fn internal_message_at<M>(
                caller_at: ::internal_error::SourceLocation,
                message: M,
            ) -> Self
            where
                M: ::core::convert::Into<::std::string::String>,
            {
                Self::from(::internal_error::InternalError::message_at(
                    Self::DEFINED_AT,
                    caller_at,
                    message,
                ))
            }

            fn context_at<M>(self, caller_at: ::internal_error::SourceLocation, message: M) -> Self
            where
                M: ::core::convert::Into<::std::string::String>,
            {
                match self {
                    Self::Internal(internal) => {
                        Self::from(internal.context_at(Self::DEFINED_AT, caller_at, message))
                    }
                    other => other,
                }
            }

            fn with_context_at<F, M>(
                self,
                caller_at: ::internal_error::SourceLocation,
                make_message: F,
            ) -> Self
            where
                F: ::core::ops::FnOnce() -> M,
                M: ::core::convert::Into<::std::string::String>,
            {
                match self {
                    Self::Internal(internal) => {
                        Self::from(internal.context_at(
                            Self::DEFINED_AT,
                            caller_at,
                            make_message(),
                        ))
                    }
                    other => other,
                }
            }

            #[doc = "Converts one concrete source error into this error enum's canonical internal fallback."]
            #[track_caller]
            pub fn internal<E>(source: E) -> Self
            where
                E: ::std::error::Error + Send + Sync + 'static,
            {
                let caller_at =
                    ::internal_error::SourceLocation::from_location(::std::panic::Location::caller());
                Self::internal_at(caller_at, source)
            }

            #[doc = "Converts one concrete source error into this error enum's canonical internal fallback and appends one context frame."]
            #[track_caller]
            pub fn internal_with<E, M>(source: E, message: M) -> Self
            where
                E: ::std::error::Error + Send + Sync + 'static,
                M: ::core::convert::Into<::std::string::String>,
            {
                let caller_at =
                    ::internal_error::SourceLocation::from_location(::std::panic::Location::caller());
                Self::internal_with_at(caller_at, source, message)
            }

            #[doc = "Creates this error enum's canonical internal fallback from one explicit internal message."]
            #[track_caller]
            pub fn internal_message<M>(message: M) -> Self
            where
                M: ::core::convert::Into<::std::string::String>,
            {
                let caller_at =
                    ::internal_error::SourceLocation::from_location(::std::panic::Location::caller());
                Self::internal_message_at(caller_at, message)
            }

            #[doc = "Appends one context frame to an existing `InternalError` and returns this error enum's canonical internal fallback."]
            #[track_caller]
            pub fn internal_context<M>(
                source: ::internal_error::InternalError,
                message: M,
            ) -> Self
            where
                M: ::core::convert::Into<::std::string::String>,
            {
                let caller_at =
                    ::internal_error::SourceLocation::from_location(::std::panic::Location::caller());
                Self::from(source.context_at(Self::DEFINED_AT, caller_at, message))
            }

            #[doc = "Appends one context frame when this error is the canonical internal fallback. Handled typed variants stay unchanged."]
            #[track_caller]
            pub fn context<M>(self, message: M) -> Self
            where
                M: ::core::convert::Into<::std::string::String>,
            {
                let caller_at =
                    ::internal_error::SourceLocation::from_location(::std::panic::Location::caller());
                self.context_at(caller_at, message)
            }

            #[doc = "Appends one lazily-produced context frame when this error is the canonical internal fallback. Handled typed variants stay unchanged."]
            #[track_caller]
            pub fn with_context<F, M>(self, make_message: F) -> Self
            where
                F: ::core::ops::FnOnce() -> M,
                M: ::core::convert::Into<::std::string::String>,
            {
                let caller_at =
                    ::internal_error::SourceLocation::from_location(::std::panic::Location::caller());
                self.with_context_at(caller_at, make_message)
            }
        }
    }
}
