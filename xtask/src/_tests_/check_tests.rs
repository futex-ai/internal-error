use std::collections::BTreeSet;

use super::{CheckPhase, CheckSelection};

#[test]
fn empty_filters_include_every_phase() {
    let selection = CheckSelection::from_filters(BTreeSet::new(), BTreeSet::new());

    assert!(selection.contains(CheckPhase::Scripts));
    assert!(selection.contains(CheckPhase::Workflow));
    assert!(selection.contains(CheckPhase::RustAudit));
    assert!(selection.contains(CheckPhase::RustFmt));
    assert!(selection.contains(CheckPhase::RustClippy));
    assert!(selection.contains(CheckPhase::RustTest));
}

#[test]
fn include_filter_limits_phase_set() {
    let selection =
        CheckSelection::from_filters(BTreeSet::from([CheckPhase::RustFmt]), BTreeSet::new());

    assert!(selection.contains(CheckPhase::RustFmt));
    assert!(!selection.contains(CheckPhase::RustTest));
}

#[test]
fn exclude_filter_wins_over_include() {
    let selection = CheckSelection::from_filters(
        BTreeSet::from([CheckPhase::RustFmt]),
        BTreeSet::from([CheckPhase::RustFmt]),
    );

    assert!(!selection.contains(CheckPhase::RustFmt));
}
