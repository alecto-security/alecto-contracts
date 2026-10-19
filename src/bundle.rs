//! The evidence-bundle schema v1 types, mirroring the normative spec (and
//! test vectors) owned by the public
//! [evidence-bundle](https://github.com/alecto-security/evidence-bundle)
//! standard. Serialized field names are frozen at `schema_version "1"`.
//!
//! Two of the machine files are typed here: `metadata.json` (the case and
//! retrieval record) and `camera.json` (the per-source descriptor array).
//! The prose files (`custody-log.json`, `audit-log.json`) are written by
//! producers and read by humans; they have no typed surface yet by design,
//! so this crate does not invent one ahead of an integrator conversation.

use serde::{Deserialize, Serialize};

use crate::error::{ContractError, Result};

pub const SCHEMA_VERSION: &str = "1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Retrieved {
    pub source_id: String,
    pub start: String,
    pub end: String,
    pub file: String,
    /// SHA-256 of the byte-identical original file (the seal must cover it
    /// with exactly this digest: the seal verifier checks the pair).
    pub source_sha256: String,
}

/// Why a window segment is missing. Gaps are first-class records, never
/// silent omissions. Adding a variant is additive; removing or renaming one
/// is a breaking change per the crate versioning rules.
// NOTE(port): the producer gates reason strings against this same set; a
// reason produced in the wild that is not in this enum fails to parse in
// strict readers. The seal verifier keeps the record honest either way
// because the bytes are sealed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapReason {
    NoCoverage,
    RecordingStartsLate,
    SourceUnreachable,
    CodecUnsupported,
    /// Source resolved to a live stream with no replay path: it cannot
    /// serve a historical window, and capturing live to label it as the
    /// requested window is forbidden.
    LiveOnly,
    /// Notary retention removed the segment before it was requested.
    RetentionDeleted,
    /// A forensic extraction left regions of the disk image unrecovered.
    DiskRegionsUnrecovered,
}

impl GapReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoCoverage => "no_coverage",
            Self::RecordingStartsLate => "recording_starts_late",
            Self::SourceUnreachable => "source_unreachable",
            Self::CodecUnsupported => "codec_unsupported",
            Self::LiveOnly => "live_only",
            Self::RetentionDeleted => "retention_deleted",
            Self::DiskRegionsUnrecovered => "disk_regions_unrecovered",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gap {
    pub source_id: String,
    pub start: String,
    pub end: String,
    pub reason: GapReason,
}

impl Gap {
    pub fn new(
        source_id: impl Into<String>,
        start: &str,
        end: &str,
        reason: GapReason,
    ) -> Self {
        Self {
            source_id: source_id.into(),
            start: start.to_string(),
            end: end.to_string(),
            reason,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ReportStatus {
    #[default]
    Pending,
    Generated,
    Deferred,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayInfo {
    pub id: String,
    pub version: String,
    pub software_only: bool,
}

/// Who asked for this retrieval: a person (default) or a pre-approved rule.
/// Additive and optional, so `schema_version` stays `"1"` and older bundles
/// verify unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Origin {
    /// `operator` | `rule`
    pub kind: String,
    /// Required for `rule`: the named person who approved the rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_by: Option<String>,
}

impl Origin {
    /// Validate the optional `origin` / `approved_by` pair. A rule must
    /// carry its approver, and its operator must be `rule:<id>`.
    pub fn from_parts(
        origin: Option<&str>,
        approved_by: Option<&str>,
        operator: &str,
    ) -> Result<Option<Origin>> {
        match (origin.map(str::trim), approved_by.map(str::trim)) {
            (None, None) => Ok(None),
            (None, Some(_)) => Err(ContractError::new("approved_by requires origin")),
            (Some("operator"), None) => Ok(Some(Origin { kind: "operator".into(), approved_by: None })),
            (Some("operator"), Some(_)) => Err(ContractError::new(
                "approved_by is only valid with origin \"rule\"",
            )),
            (Some("rule"), approver) => {
                let approver = approver.unwrap_or("");
                if approver.len() < 3 {
                    return Err(ContractError::new(
                        "origin \"rule\" requires approved_by (the named person who approved the rule)",
                    ));
                }
                if !operator.trim().starts_with("rule:") || operator.trim().len() <= 5 {
                    return Err(ContractError::new(
                        "origin \"rule\" requires operator \"rule:<id>\"",
                    ));
                }
                Ok(Some(Origin { kind: "rule".into(), approved_by: Some(approver.to_string()) }))
            }
            (Some(other), _) => Err(ContractError::new(format!(
                "origin must be \"operator\" or \"rule\", got {other:?}"
            ))),
        }
    }
}

/// Where the footage came from when it was not pulled from a live camera.
/// `kind` is `notary` or `disk_image`; `detail` is kind-specific and is
/// hashed with the bundle. Proof-file requirements per kind are defined by
/// the spec (and enforced by the seal verifier), not re-derived here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provenance {
    pub kind: String,
    pub detail: serde_json::Value,
}

/// The case + retrieval record (`metadata.json`). Schema v1 fields are
/// frozen; unknown fields are **ignored** on read: the spec's
/// additive-optional rule means a bundle written by a newer producer with
/// an added optional field must stay readable. Removals and renames are
/// breaking changes, never done inside v1.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub schema_version: String,
    pub case_id: String,
    pub requested_window: Window,
    pub retrieved: Vec<Retrieved>,
    pub gaps: Vec<Gap>,
    pub retrieved_at: String,
    pub gateway: GatewayInfo,
    pub bundle_hash: String,
    #[serde(default)]
    pub report_status: ReportStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<Origin>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<Provenance>,
}

impl Metadata {
    /// Shape checks a reader should apply before trusting the record: the
    /// exact bundle_hash and case-id grammars are the verifier's job (it
    /// re-computes them against sealed bytes); the types guarantee wire
    /// shape only.
    pub fn schema_version_is_current(&self) -> bool {
        self.schema_version == SCHEMA_VERSION
    }
}

/// One entry of `camera.json` (an array), the per-source descriptor set.
/// Credentials are removed at the source: the RTSP field carries the
/// redacted URI shape only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraRecord {
    pub source_id: String,
    pub name: String,
    pub vendor: String,
    pub model: String,
    pub firmware: String,
    pub onvif_profile: String,
    pub rtsp_uri_redacted: String,
    pub resolution: String,
    pub fps: String,
    pub codec: String,
    pub time_source: String,
}
