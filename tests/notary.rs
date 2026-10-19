use alecto_contracts::notary::{canonical_json, Kind, Clock, Record, GENESIS_HASH, RECORD_VERSION};
use serde_json::json;

#[test]
fn canonical_sorts_keys_and_strips_whitespace() {
    let v = json!({"b": 1, "a": {"d": [1, 2], "c": "x"}});
    assert_eq!(canonical_json(&v), r#"{"a":{"c":"x","d":[1,2]},"b":1}"#);
}

#[test]
fn canonical_is_stable_across_invocations() {
    let a = canonical_json(&json!({"z": [0, {"y": 1}], "a2": null, "m": true, "n": 1.5}));
    let b = canonical_json(&json!({"n": 1.5, "m": true, "a2": null, "z": [0, {"y": 1}]}));
    assert_eq!(a, b, "key order and whitespace must not leak into the bytes");
}

fn segment_record() -> Record {
    serde_json::from_str(
        r#"{
        "v": 1,
        "kind": "segment",
        "device_id": "notary-1",
        "key_id": "key-2026-w43",
        "camera_id": "CAM-W01",
        "seq": 41,
        "start_utc": "2026-10-14T22:10:00Z",
        "end_utc": "2026-10-14T22:11:00Z",
        "file": "20261014/22/CAM-W01/00041.m4s",
        "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "bytes": 482113,
        "prev_record_hash": "0000000000000000000000000000000000000000000000000000000000000000",
        "clock": {"source": "ntp"},
        "sig": "c2ln"
    }"#,
    )
    .unwrap()
}

#[test]
fn record_constants_and_shapes() {
    assert_eq!(RECORD_VERSION, 1);
    assert!(GENESIS_HASH.chars().all(|c| c == '0'));
    let r = segment_record();
    assert_eq!(r.kind, Kind::Segment);
    assert_eq!(r.clock, Clock { source: "ntp".into(), skew_ms: None });
}

#[test]
fn signing_bytes_exclude_the_signature() {
    let r = segment_record();
    let signing = String::from_utf8(r.signing_bytes().unwrap()).unwrap();
    assert!(
        !signing.contains("c2ln"),
        "the signature bytes must not be inside the signed text"
    );
    assert!(signing.starts_with("{\"bytes\":"));
    assert!(signing.contains("\"seq\":41"));
}

#[test]
fn record_hash_is_a_sha256_of_canonical_form() {
    let r = segment_record();
    let h = r.record_hash().unwrap();
    assert_eq!(h.len(), 64);
    assert!(h.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
}

#[test]
fn unknown_fields_reject_in_records() {
    let with_extra = r#"{
        "v": 1, "kind": "gap", "device_id": "d", "key_id": "k", "camera_id": "CAM-1",
        "seq": 1, "start_utc": "s", "end_utc": "e",
        "prev_record_hash": "0000000000000000000000000000000000000000000000000000000000000000",
        "clock": {"source": "rtc"}, "sig": "x",
        "sneaky_future_field": true
    }"#;
    let parsed: Result<Record, _> = serde_json::from_str(with_extra);
    assert!(
        parsed.is_err(),
        "record links have an exact shape; surprises are version bumps, not extensions"
    );
}
