use syn::parse_quote;

use crate::error_contract_derive::expand;

#[test]
fn accepts_enum_with_canonical_internal_variant() {
    let input = parse_quote! {
        enum Error {
            Internal(#[from] InternalError),
        }
    };

    let expanded = expand(input).expect("derive should accept canonical shape");
    let tokens = expanded.to_string();

    assert!(tokens.contains("impl :: internal_error :: ErrorContract for Error"));
    assert!(tokens.contains("pub fn context"));
    assert!(tokens.contains("pub fn with_context"));
    assert!(tokens.contains("pub fn internal_message"));
    assert!(tokens.contains("pub fn internal_context"));
    assert!(tokens.contains("trait ErrorResultExt"));
    assert!(tokens.contains("trait ErrorOptionExt"));
    assert!(tokens.contains("fn context"));
    assert!(tokens.contains("fn with_context"));
    assert!(tokens.contains("fn ok_or_internal"));
    assert!(tokens.contains("fn ok_or_internal_else"));
}

#[test]
fn rejects_missing_internal_variant() {
    let input = parse_quote! {
        enum Error {
            InvalidPort,
        }
    };

    let error = expand(input).expect_err("derive should require internal variant");
    assert!(error.to_string().contains("Internal"));
}

#[test]
fn rejects_internal_variant_without_from_attribute() {
    let input = parse_quote! {
        enum Error {
            Internal(InternalError),
        }
    };

    let error = expand(input).expect_err("derive should require #[from]");
    assert!(error.to_string().contains("#[from]"));
}

#[test]
fn rejects_internal_variant_with_wrong_type() {
    let input = parse_quote! {
        enum Error {
            Internal(#[from] std::io::Error),
        }
    };

    let error = expand(input).expect_err("derive should require InternalError");
    assert!(error.to_string().contains("InternalError"));
}
