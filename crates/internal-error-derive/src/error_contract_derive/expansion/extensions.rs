//! Result and option helper generation for derived error enums.

use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub(super) fn expand(
    error_ident: &Ident,
    result_ext_ident: &Ident,
    option_ext_ident: &Ident,
) -> TokenStream {
    quote! {
        #[doc = "Result helpers for converting foreign errors into this error enum's canonical internal fallback."]
        pub trait #result_ext_ident<T> {
            #[doc = "Converts the error side of this result into this error enum's canonical internal fallback."]
            fn internal(self) -> ::core::result::Result<T, #error_ident>;

            #[doc = "Adds context to this result. Foreign errors collapse into the canonical internal fallback, existing internal failures gain one context frame, and handled typed variants stay unchanged."]
            fn context<M>(self, message: M) -> ::core::result::Result<T, #error_ident>
            where
                M: ::core::convert::Into<::std::string::String>;

            #[doc = "Adds lazily-produced context to this result. The message closure only runs when the error path needs new internal context."]
            fn with_context<F, M>(self, make_message: F) -> ::core::result::Result<T, #error_ident>
            where
                F: ::core::ops::FnOnce() -> M,
                M: ::core::convert::Into<::std::string::String>;
        }

        impl<T, E> #result_ext_ident<T> for ::core::result::Result<T, E>
        where
            E: ::std::error::Error + Send + Sync + 'static,
        {
            #[track_caller]
            fn internal(self) -> ::core::result::Result<T, #error_ident> {
                let caller_at =
                    ::internal_error::SourceLocation::from_location(::std::panic::Location::caller());
                match self {
                    ::core::result::Result::Ok(value) => ::core::result::Result::Ok(value),
                    ::core::result::Result::Err(source) => {
                        if ::core::any::TypeId::of::<E>() == ::core::any::TypeId::of::<#error_ident>() {
                            let source = ::std::boxed::Box::new(source)
                                as ::std::boxed::Box<dyn ::core::any::Any + Send + Sync>;
                            match source.downcast::<#error_ident>() {
                                ::core::result::Result::Ok(error) => {
                                    ::core::result::Result::Err(*error)
                                }
                                ::core::result::Result::Err(_source) => {
                                    ::core::result::Result::Err(#error_ident::internal_message_at(
                                        caller_at,
                                        "type id check did not match downcast",
                                    ))
                                }
                            }
                        } else {
                            ::core::result::Result::Err(#error_ident::internal_at(caller_at, source))
                        }
                    }
                }
            }

            #[track_caller]
            fn context<M>(self, message: M) -> ::core::result::Result<T, #error_ident>
            where
                M: ::core::convert::Into<::std::string::String>,
            {
                let caller_at =
                    ::internal_error::SourceLocation::from_location(::std::panic::Location::caller());
                match self {
                    ::core::result::Result::Ok(value) => ::core::result::Result::Ok(value),
                    ::core::result::Result::Err(source) => {
                        if ::core::any::TypeId::of::<E>() == ::core::any::TypeId::of::<#error_ident>() {
                            let source = ::std::boxed::Box::new(source)
                                as ::std::boxed::Box<dyn ::core::any::Any + Send + Sync>;
                            match source.downcast::<#error_ident>() {
                                ::core::result::Result::Ok(error) => {
                                    ::core::result::Result::Err((*error).context_at(caller_at, message))
                                }
                                ::core::result::Result::Err(_source) => {
                                    ::core::result::Result::Err(#error_ident::internal_message_at(
                                        caller_at,
                                        "type id check did not match downcast",
                                    ))
                                }
                            }
                        } else {
                            ::core::result::Result::Err(#error_ident::internal_with_at(
                                caller_at,
                                source,
                                message,
                            ))
                        }
                    }
                }
            }

            #[track_caller]
            fn with_context<F, M>(self, make_message: F) -> ::core::result::Result<T, #error_ident>
            where
                F: ::core::ops::FnOnce() -> M,
                M: ::core::convert::Into<::std::string::String>,
            {
                let caller_at =
                    ::internal_error::SourceLocation::from_location(::std::panic::Location::caller());
                match self {
                    ::core::result::Result::Ok(value) => ::core::result::Result::Ok(value),
                    ::core::result::Result::Err(source) => {
                        if ::core::any::TypeId::of::<E>() == ::core::any::TypeId::of::<#error_ident>() {
                            let source = ::std::boxed::Box::new(source)
                                as ::std::boxed::Box<dyn ::core::any::Any + Send + Sync>;
                            match source.downcast::<#error_ident>() {
                                ::core::result::Result::Ok(error) => {
                                    ::core::result::Result::Err(
                                        (*error).with_context_at(caller_at, make_message),
                                    )
                                }
                                ::core::result::Result::Err(_source) => {
                                    ::core::result::Result::Err(#error_ident::internal_message_at(
                                        caller_at,
                                        "type id check did not match downcast",
                                    ))
                                }
                            }
                        } else {
                            ::core::result::Result::Err(#error_ident::internal_with_at(
                                caller_at,
                                source,
                                make_message(),
                            ))
                        }
                    }
                }
            }
        }

        #[doc = "Option helpers for turning `None` into this error enum's canonical internal fallback."]
        pub trait #option_ext_ident<T> {
            #[doc = "Turns `None` into this error enum's canonical internal fallback from one explicit message."]
            fn ok_or_internal<M>(self, message: M) -> ::core::result::Result<T, #error_ident>
            where
                M: ::core::convert::Into<::std::string::String>;

            #[doc = "Turns `None` into this error enum's canonical internal fallback from one lazily-produced message."]
            fn ok_or_internal_else<F, M>(
                self,
                make_message: F,
            ) -> ::core::result::Result<T, #error_ident>
            where
                F: ::core::ops::FnOnce() -> M,
                M: ::core::convert::Into<::std::string::String>;
        }

        impl<T> #option_ext_ident<T> for ::core::option::Option<T> {
            #[track_caller]
            fn ok_or_internal<M>(self, message: M) -> ::core::result::Result<T, #error_ident>
            where
                M: ::core::convert::Into<::std::string::String>,
            {
                let caller_at =
                    ::internal_error::SourceLocation::from_location(::std::panic::Location::caller());
                match self {
                    ::core::option::Option::Some(value) => ::core::result::Result::Ok(value),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(#error_ident::internal_message_at(
                            caller_at,
                            message,
                        ))
                    }
                }
            }

            #[track_caller]
            fn ok_or_internal_else<F, M>(
                self,
                make_message: F,
            ) -> ::core::result::Result<T, #error_ident>
            where
                F: ::core::ops::FnOnce() -> M,
                M: ::core::convert::Into<::std::string::String>,
            {
                let caller_at =
                    ::internal_error::SourceLocation::from_location(::std::panic::Location::caller());
                match self {
                    ::core::option::Option::Some(value) => ::core::result::Result::Ok(value),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(#error_ident::internal_message_at(
                            caller_at,
                            make_message(),
                        ))
                    }
                }
            }
        }
    }
}
