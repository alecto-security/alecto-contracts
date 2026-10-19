use alecto_contracts::bundle::*;
use alecto_contracts::ContractError;

fn sample() -> Metadata {
    serde_json::from_str(
        r#"{
        "schema_version": "1",
        "case_id": "CASE-2026-9001",
        "requested_window": {"start": "2026-10-02T18:30:00Z", "end": "2026-10-02T18:45:00Z"},
        "retrieved": [
            {"source_id": "CAM-A", "start": "2026-10-02T18:30:00Z", "end": "2026-10-02T18:45:00Z",
             "file": "original/CAM-A_20261002T1830-1845.mp4",
             "source_sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}
        ],
        "gaps": [
            {"source_id": "CAM-B", "start": "2026-10-02T18:30:00Z", "end": "2026-10-02T18:45:00Z",
             "reason": "no_coverage"}
        ],
        "retrieved_at": "2026-10-02T18:50:12Z",
        "gateway": {"id": "gateway-1", "version": "0.1.0", "software_only": true},
        "bundle_hash": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    }"#,
    )
    .unwrap()
}

#[test]
fn sample_round_trips() {
    let meta = sample();
    assert!(meta.schema_version_is_current());
    assert_eq!(meta.gaps[0].reason.as_str(), "no_coverage");
    assert_eq!(meta.retrieved.len(), 1);
    assert!(meta.origin.is_none());
    assert!(meta.provenance.is_none());
    assert_eq!(meta.report_status, ReportStatus::Pending);

    let json = serde_json::to_string(&meta).unwrap();
    let back: Metadata = serde_json::from_str(&json).unwrap();
    assert_eq!(back.bundle_hash, meta.bundle_hash);
    assert_eq!(back.gaps, meta.gaps);
}

/// The reader-tolerance rule: a newer producer adds an optional field, and
/// this version of the types must still read the bundle. Strictness belongs
/// to the verifier, not the reader.
#[test]
fn unknown_fields_are_ignored_on_read() {
    let with_future_field = r#"{
        "schema_version": "1",
        "case_id": "CASE-2026-9001",
        "requested_window": {"start": "a", "end": "b"},
        "retrieved": [],
        "gaps": [],
        "retrieved_at": "2026-10-02T18:50:12Z",
        "gateway": {"id": "gateway-1", "version": "0.1.0", "software_only": true},
        "bundle_hash": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "future_additive_field": {"anything": true}
    }"#;
    let meta: Metadata = serde_json::from_str(with_future_field)
        .expect("additive field must not break the v1 reader");
    assert!(meta.retrieved.is_empty());
}

#[test]
fn gaps_round_trip_all_reasons() {
    for (reason, name) in [
        (GapReason::NoCoverage, "no_coverage"),
        (GapReason::RecordingStartsLate, "recording_starts_late"),
        (GapReason::SourceUnreachable, "source_unreachable"),
        (GapReason::CodecUnsupported, "codec_unsupported"),
        (GapReason::LiveOnly, "live_only"),
        (GapReason::RetentionDeleted, "retention_deleted"),
        (GapReason::DiskRegionsUnrecovered, "disk_regions_unrecovered"),
    ] {
        let json = serde_json::to_string(&reason).unwrap();
        assert_eq!(json, format!("\"{name}\""), "wire name is frozen");
        let back: GapReason = serde_json::from_str(&json).unwrap();
        assert_eq!(back.as_str(), name);
    }
}

#[test]
fn origin_validation_pairs() {
    use Origin as O;
    assert!(O::from_parts(None, None, "user:42").unwrap().is_none());

    let operator = O::from_parts(Some("operator"), None, "user:42").unwrap().unwrap();
    assert_eq!(operator.kind, "operator");
    assert_eq!(operator.approved_by, None);

    let rule = O::from_parts(Some("rule"), Some("jane.doe"), "rule:school-custodian")
        .unwrap()
        .unwrap();
    assert_eq!(rule.approved_by.as_deref(), Some("jane.doe"));

    for (origin, approved, operator, expected) in [
        (None, Some("x"), "user:42", "approved_by requires origin"),
        (
            Some("operator"),
            Some("x"),
            "user:42",
            "only valid with origin \"rule\"",
        ),
        (Some("rule"), None, "user:42", "requires approved_by"),
        (
            Some("rule"),
            Some("jane"),
            "user:42",
            "requires operator \"rule:<id>\"",
        ),
        (Some("emergency"), None, "user:42", "must be \"operator\" or \"rule\""),
    ] {
        let err = O::from_parts(origin, approved, operator).expect_err(expected);
        assert!(
            err.to_string().contains(expected),
            "message drifted: {err} (expected substring {expected:?})"
        );
    }
}

#[test]
fn contract_errors_display_their_message() {
    let err = ContractError::new("plain text");
    assert_eq!(err.to_string(), "plain text");
}
