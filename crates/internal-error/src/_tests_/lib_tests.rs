use std::error::Error as StdError;

use crate::{DefinedAt, ErrorExt as _, InternalError, ResultExt as _};

const TEST_DEFINED_AT: DefinedAt = crate::defined_at!();
const PARENT_DEFINED_AT: DefinedAt = DefinedAt::new("tests::parent", "parent.rs", 10, 3);

#[test]
fn defined_at_tracks_the_current_test_module() {
    assert!(TEST_DEFINED_AT.module_path().contains("lib_tests"));
    assert!(TEST_DEFINED_AT.file().ends_with("src/_tests_/lib_tests.rs"));
    assert!(TEST_DEFINED_AT.line() > 0);
    assert!(TEST_DEFINED_AT.column() > 0);
}

#[test]
fn from_source_captures_the_outer_call_site() {
    let expected_line = line!() + 1;
    let error = build_error_from_tracked_helper();

    assert_eq!(error.caller_at().file(), file!());
    assert_eq!(error.caller_at().line(), expected_line);
}

#[test]
fn context_frames_stack_without_replacing_the_source_chain() {
    let error = std::io::Error::other("disk offline")
        .internal_error_context(TEST_DEFINED_AT, "read config file")
        .context(PARENT_DEFINED_AT, "bootstrap runtime");

    let frames = error.context_frames();

    assert_eq!(frames.len(), 2);
    assert_eq!(frames[0].message(), "read config file");
    assert_eq!(frames[1].message(), "bootstrap runtime");
    assert_eq!(
        error
            .source_chain()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        vec!["disk offline".to_owned()]
    );
}

#[test]
fn explicit_message_uses_a_real_source_error() {
    let error = InternalError::message(TEST_DEFINED_AT, "unsupported schema");

    assert_eq!(error.to_string(), "internal error");
    assert_eq!(
        error.source().map(ToString::to_string),
        Some("unsupported schema".to_owned())
    );
}

#[test]
fn result_helpers_recontextualize_existing_internal_errors_without_nesting() {
    let inner = std::io::Error::other("connection reset")
        .internal_error_context(TEST_DEFINED_AT, "query row");
    let result: Result<(), InternalError> = Err(inner);

    let error = result
        .internal_error_context(PARENT_DEFINED_AT, "load workspace")
        .unwrap_err();

    assert_eq!(error.defined_at(), TEST_DEFINED_AT);
    assert_eq!(error.context_frames().len(), 2);
    assert_eq!(error.context_frames()[0].defined_at(), TEST_DEFINED_AT);
    assert_eq!(error.context_frames()[0].message(), "query row");
    assert_eq!(error.context_frames()[1].defined_at(), PARENT_DEFINED_AT);
    assert_eq!(error.context_frames()[1].message(), "load workspace");
    assert_eq!(
        error
            .source_chain()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        vec!["connection reset".to_owned()]
    );

    let source = StdError::source(&error).expect("source should be preserved");
    assert_eq!(source.to_string(), "connection reset");
    assert!(source.source().is_none());
}

#[track_caller]
fn build_error_from_tracked_helper() -> InternalError {
    std::io::Error::other("boom").internal_error(TEST_DEFINED_AT)
}
