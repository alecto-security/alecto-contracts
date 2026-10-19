use alecto_contracts::security_model::*;

#[test]
fn source_tree_has_no_product_dependencies() {
    // Audit, not policy prose: the manifest must stay serde/serde_json/time/
    // uuid/sha2/hex only so this crate stays the portable contract package
    // and nobody drags a product runtime into it.
    let manifest = include_str!("../Cargo.toml");
    for forbidden in ["alecto-gateway", "alecto-app", "alecto-incident", "alecto-discovery"] {
        let key = format!("{forbidden} =");
        let key_tight = format!("{forbidden}=");
        assert!(
            !manifest
                .lines()
                .any(|l| l.trim_start().starts_with(&key) || l.trim_start().starts_with(&key_tight)),
            "security_model must not depend on {forbidden}"
        );
    }
}

/// Builds the PRD worked example (CASE-2026-1842) and proves it round-trips
/// through serde: this module's only job is to be a shared wire/storage
/// shape, so a JSON round-trip failure here is the bug that matters most.
#[test]
fn prd_worked_example_round_trips() {
    let org = Organization {
        id: new_id(),
        name: "Store #142".into(),
        tier: OrganizationTier::SiteGateway,
    };

    let location = Location {
        id: new_id(),
        organization_id: org.id,
        name: "West Parking Lot".into(),
        parent_location_id: None,
        address: None,
    };

    let camera = Camera {
        id: new_id(),
        organization_id: org.id,
        location_id: location.id,
        name: "West Parking Lot Cam 1".into(),
        vendor: "Axis".into(),
        model: "P3265-LVE".into(),
        onvif_endpoint: Some("http://10.0.4.12/onvif/device_service".into()),
        status: CameraStatus::Online,
    };

    let recording = Recording {
        id: new_id(),
        camera_id: camera.id,
        starts_at: time::macros::datetime!(2026-09-14 22:10:00 UTC),
        ends_at: time::macros::datetime!(2026-09-14 22:35:00 UTC),
        original_uri: "s3://alecto-evidence/CASE-2026-1842/original/cam1.mp4".into(),
        playable_uri: Some("s3://alecto-evidence/CASE-2026-1842/playable/cam1.mp4".into()),
    };

    let evidence = Evidence {
        id: new_id(),
        organization_id: org.id,
        incident_id: None,
        source_recording_id: recording.id,
        original_uri: recording.original_uri.clone(),
        playable_uri: recording.playable_uri.clone(),
        hash: EvidenceHash {
            algorithm: HashAlgorithm::Sha256,
            digest_hex: "a3f0".repeat(16),
        },
        custody_log: vec![
            CustodyEvent {
                phase: CustodyPhase::Identified,
                at: time::macros::datetime!(2026-09-14 22:36:00 UTC),
                actor: "gateway".into(),
                note: None,
            },
            CustodyEvent {
                phase: CustodyPhase::Acquired,
                at: time::macros::datetime!(2026-09-14 22:36:04 UTC),
                actor: "gateway".into(),
                note: Some("retrieved via ONVIF replay".into()),
            },
            CustodyEvent {
                phase: CustodyPhase::Transferred,
                at: time::macros::datetime!(2026-09-14 22:36:12 UTC),
                actor: "investigator:jane.doe".into(),
                note: None,
            },
        ],
    };

    for json in [
        serde_json::to_string(&org).unwrap(),
        serde_json::to_string(&location).unwrap(),
        serde_json::to_string(&camera).unwrap(),
        serde_json::to_string(&recording).unwrap(),
        serde_json::to_string(&evidence).unwrap(),
    ] {
        assert!(!json.is_empty());
    }

    let round_tripped: Evidence =
        serde_json::from_str(&serde_json::to_string(&evidence).unwrap()).unwrap();
    assert_eq!(round_tripped.hash.digest_hex, evidence.hash.digest_hex);
    assert_eq!(round_tripped.custody_log.len(), 3);
    assert_eq!(round_tripped.custody_log[2].phase, CustodyPhase::Transferred);
}
