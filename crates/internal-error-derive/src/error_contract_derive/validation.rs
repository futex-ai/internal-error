//! Validation for `ErrorContract` derive inputs.

use syn::{Data, DeriveInput, Error, Field, Fields, Result, Variant};

pub(super) fn validate(input: &DeriveInput) -> Result<()> {
    validate_no_generics(input)?;
    let internal_variant = internal_variant(input)?;
    validate_internal_variant(internal_variant)
}

fn validate_no_generics(input: &DeriveInput) -> Result<()> {
    if input.generics.params.is_empty() {
        return Ok(());
    }

    Err(Error::new_spanned(
        &input.generics,
        "ErrorContract derive does not support generic error enums",
    ))
}

fn internal_variant(input: &DeriveInput) -> Result<&Variant> {
    let Data::Enum(data) = &input.data else {
        return Err(Error::new_spanned(
            input,
            "ErrorContract derive only supports enums",
        ));
    };

    data.variants
        .iter()
        .find(|variant| variant.ident == "Internal")
        .ok_or_else(|| {
            Error::new_spanned(
                &input.ident,
                "ErrorContract derive requires an `Internal(#[from] InternalError)` variant",
            )
        })
}

fn validate_internal_variant(variant: &Variant) -> Result<()> {
    let Fields::Unnamed(fields) = &variant.fields else {
        return Err(Error::new_spanned(
            variant,
            "`Internal` must be a tuple variant with exactly one `#[from] InternalError` field",
        ));
    };

    if fields.unnamed.len() != 1 {
        return Err(Error::new_spanned(
            fields,
            "`Internal` must contain exactly one `#[from] InternalError` field",
        ));
    }

    validate_internal_field(&fields.unnamed[0])
}

fn validate_internal_field(field: &Field) -> Result<()> {
    if !field.attrs.iter().any(|attr| attr.path().is_ident("from")) {
        return Err(Error::new_spanned(
            field,
            "`Internal` must use `#[from] InternalError`",
        ));
    }

    let syn::Type::Path(path) = &field.ty else {
        return Err(Error::new_spanned(
            &field.ty,
            "`Internal` must wrap `InternalError`",
        ));
    };

    let is_internal_error = path
        .path
        .segments
        .last()
        .map(|segment| segment.ident == "InternalError")
        .unwrap_or(false);

    if is_internal_error {
        return Ok(());
    }

    Err(Error::new_spanned(
        &field.ty,
        "`Internal` must wrap `InternalError`",
    ))
}
