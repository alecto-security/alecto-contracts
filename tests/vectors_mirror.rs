//! The mirror rule, executed: the types in this crate must deserialize the
//! test vectors owned by the public `evidence-bundle` standard. CI clones
//! the pinned tag and points `VECTORS_DIR` at its `fixtures/` directory.
//! Locally, run `scripts/fetch_vectors.sh` first; without `VECTORS_DIR`
//! these tests declare themselves skipped so plain `cargo test` works
//! offline.

use std::path::{Path, PathBuf};

use alecto_contracts::bundle::{CameraRecord, GapReason, Metadata};

fn vectors_dir() -> Option<PathBuf> {
    let dir = std::env::var("VECTORS_DIR").ok()?;
    let path = PathBuf::from(dir);
    path.is_dir().then_some(path)
}

fn fixture(vectors: &Path, name: &str) -> PathBuf {
    vectors.join(name)
}

#[test]
fn vector_mirror_runs_the_bundle_types_against_the_pinned_set() {
    let Some(vectors) = vectors_dir() else {
        eprintln!("skipped: VECTORS_DIR not set (run scripts/fetch_vectors.sh)");
        return;
    };

    for name in [
        "minimal-full-coverage",
        "coverage-gaps",
        "notary-provenance",
        "disk-image-provenance",
    ] {
        let meta_text = std::fs::read_to_string(fixture(&vectors, name).join("metadata.json"))
            .unwrap_or_else(|e| panic!("{name}: {e}"));
        let meta: Metadata = serde_json::from_str(&meta_text).unwrap();
        assert!(meta.schema_version_is_current(), "{name}: schema_version");
        assert_eq!(meta.bundle_hash.len(), 64, "{name}: bundle_hash shape");
        assert!(!meta.retrieved.is_empty(), "{name}: has at least one retrieval");

        if name == "coverage-gaps" {
            assert_eq!(
                meta.gaps[0].reason, GapReason::SourceUnreachable,
                "the recorded gap is a source_unreachable: the enum covers what the producer emits"
            );
        }
        if name == "notary-provenance" {
            assert_eq!(meta.provenance.as_ref().expect("kind").kind, "notary");
        }
        if name == "disk-image-provenance" {
            assert_eq!(meta.provenance.as_ref().expect("kind").kind, "disk_image");
            assert!(
                meta.retrieved.iter().all(|r| r.start == "unknown" && r.end == "unknown"),
                "carved streams carry no trustworthy clock"
            );
        }

        let cameras_text =
            std::fs::read_to_string(fixture(&vectors, name).join("camera.json")).unwrap_or_default();
        if !cameras_text.is_empty() {
            let cameras: Vec<CameraRecord> = serde_json::from_str(&cameras_text).unwrap();
            assert!(!cameras.is_empty(), "{name}: camera records parse through the contract");
            assert!(
                cameras.iter().all(|c| !c.rtsp_uri_redacted.contains('@') || c.rtsp_uri_redacted.contains("***")),
                "credentials stay redacted even in fixtures"
            );
        }
    }
}

#[test]
fn the_tampered_negative_vectors_are_metadata_valid_but_packed_bad() {
    let Some(vectors) = vectors_dir() else {
        eprintln!("skipped: VECTORS_DIR not set (run scripts/fetch_vectors.sh)");
        return;
    };

    // The negative vectors keep their metadata intact (the tamper lands in
    // the payload bytes or inside the signed proof), so the *types* parse
    // them: catching the tamper is the seal verifier's job, which is why
    // separate crates exist.
    for name in [
        "tampered-original",
        "tampered-notary-signature",
        "tampered-notary-chain",
    ] {
        let meta_text =
            std::fs::read_to_string(fixture(&vectors, name).join("metadata.json"))
                .unwrap_or_else(|e| panic!("{name}: {e}"));
        let meta: Metadata = serde_json::from_str(&meta_text).unwrap();
        assert!(meta.schema_version_is_current(), "{name}: schema_version");
    }
}
