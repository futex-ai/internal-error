use std::cell::Cell;

use internal_error::InternalError;
use thiserror::Error;

#[derive(Debug, Error, internal_error::ErrorContract)]
enum DemoError {
    #[error("[demo/config] invalid port")]
    InvalidPort,

    #[error("[demo/config] internal error")]
    Internal(#[from] InternalError),
}

type DemoResult<T> = Result<T, DemoError>;

#[test]
fn derived_result_helpers_infer_the_local_error_contract() {
    let (expected_line, error) = parse_port_with_line("abc");
    let error = error.expect_err("invalid port should fail");

    let DemoError::Internal(internal) = error else {
        panic!("expected internal error");
    };

    assert_eq!(internal.defined_at().file(), file!());
    assert_eq!(internal.caller_at().file(), file!());
    assert_eq!(internal.caller_at().line(), expected_line);
    assert_eq!(internal.context_frames().len(), 1);
    assert_eq!(
        internal.context_frames()[0].message(),
        "parse configured port"
    );
    assert_eq!(
        internal.source_ref().to_string(),
        "invalid digit found in string"
    );
}

#[test]
fn derived_internal_context_appends_without_nesting() {
    let error = bootstrap("abc").expect_err("bootstrap should fail");

    let DemoError::Internal(internal) = error else {
        panic!("expected internal error");
    };

    assert_eq!(internal.context_frames().len(), 2);
    assert_eq!(
        internal.context_frames()[0].message(),
        "parse configured port"
    );
    assert_eq!(internal.context_frames()[1].message(), "bootstrap runtime");
    assert_eq!(
        internal.source_ref().to_string(),
        "invalid digit found in string"
    );
}

#[test]
fn derived_internal_message_creates_the_canonical_internal_variant() {
    let error = DemoError::internal_message("unsupported schema");

    let DemoError::Internal(internal) = error else {
        panic!("expected internal error");
    };

    assert_eq!(internal.to_string(), "internal error");
    assert_eq!(internal.source_ref().to_string(), "unsupported schema");
}

#[test]
fn derived_error_context_helpers_only_change_internal_errors() {
    let error = DemoError::internal_message("unsupported schema").context("bootstrap runtime");

    let DemoError::Internal(internal) = error else {
        panic!("expected internal error");
    };

    assert_eq!(internal.context_frames().len(), 1);
    assert_eq!(internal.context_frames()[0].message(), "bootstrap runtime");
    assert_eq!(internal.source_ref().to_string(), "unsupported schema");

    let typed = DemoError::InvalidPort.context("ignored context");
    assert!(matches!(typed, DemoError::InvalidPort));
}

#[test]
fn derived_lazy_helpers_only_evaluate_messages_on_needed_paths() {
    let result_called = Cell::new(false);
    let ok = Ok::<u16, std::num::ParseIntError>(7).with_context(|| {
        result_called.set(true);
        "ignored result context"
    });
    assert_eq!(ok.expect("ok result should stay ok"), 7);
    assert!(!result_called.get());

    let error_called = Cell::new(false);
    let typed = DemoError::InvalidPort.with_context(|| {
        error_called.set(true);
        "ignored error context"
    });
    assert!(matches!(typed, DemoError::InvalidPort));
    assert!(!error_called.get());

    let option_called = Cell::new(false);
    let some = Some(11u16).ok_or_internal_else(|| {
        option_called.set(true);
        "ignored option context"
    });
    assert_eq!(some.expect("some should stay some"), 11);
    assert!(!option_called.get());
}

#[test]
fn derived_option_helpers_turn_none_into_internal_errors() {
    let error = missing_workspace_id(None).expect_err("none should become internal");

    let DemoError::Internal(internal) = error else {
        panic!("expected internal error");
    };

    assert!(internal.context_frames().is_empty());
    assert_eq!(
        internal.source_ref().to_string(),
        "session missing workspace id"
    );
}

fn parse_port(raw: &str) -> DemoResult<u16> {
    parse_port_with_line(raw).1
}

fn parse_port_with_line(raw: &str) -> (u32, DemoResult<u16>) {
    let expected_line = line!() + 2;
    let result = (|| {
        let port = raw.parse::<u16>().context("parse configured port")?;

        if port == 0 {
            return Err(DemoError::InvalidPort);
        }

        Ok(port)
    })();

    (expected_line, result)
}

fn bootstrap(raw: &str) -> DemoResult<u16> {
    match parse_port(raw) {
        Ok(port) => Ok(port),
        Err(err) => Err(err.context("bootstrap runtime")),
    }
}

fn missing_workspace_id(maybe_workspace_id: Option<u16>) -> DemoResult<u16> {
    maybe_workspace_id.ok_or_internal("session missing workspace id")
}
