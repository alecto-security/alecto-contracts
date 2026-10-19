//! Contract C2: the notary signed record format. Canonical JSON = sorted
//! keys, no whitespace; `record_hash` covers the whole record including
//! `sig`. The production recorder and chain verifier live in
//! [alecto-notary](https://github.com/alecto-security/alecto-notary); the
//! types and canonicalization here are the portable copy integrators and
//! verifiers compile against.
//!
//! Records reject unknown fields on purpose: a chain link has an exact
//! shape, and the format is versioned by its `v` field, so introducing new
//! link shapes means a new version, not a silent addition.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::error::{ContractError, Result};

pub const RECORD_VERSION: u32 = 1;
/// `prev_record_hash` of the first record in a camera chain.
pub const GENESIS_HASH: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Segment,
    Gap,
    Deleted,
    Checkpoint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Clock {
    /// `ntp` | `rtc` | `host`
    pub source: String,
    /// Known offset against a reference, in milliseconds, when measured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skew_ms: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub v: u32,
    pub kind: Kind,
    pub device_id: String,
    pub key_id: String,
    pub camera_id: String,
    pub seq: u64,
    pub start_utc: String,
    pub end_utc: String,
    /// Segment path relative to the data root (`segment`, and `deleted` for
    /// the removed file).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
    /// Why a gap or deletion happened.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// `deleted`: seq of the segment record this removes the file for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_seq: Option<u64>,
    /// `checkpoint`: head of the chain at checkpoint time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_seq: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    pub prev_record_hash: String,
    pub clock: Clock,
    /// Base64 Ed25519 signature over the canonical record without `sig`.
    pub sig: String,
}

/// Canonical JSON: object keys sorted, no insignificant whitespace. The
/// signature and the chain hash are both computed over this form, so this
/// function's output must never change for a given value.
pub fn canonical_json(value: &Value) -> String {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let parts: Vec<String> = keys
                .into_iter()
                .map(|k| {
                    format!(
                        "{}:{}",
                        serde_json::to_string(k).expect("string serializes"),
                        canonical_json(&map[k])
                    )
                })
                .collect();
            format!("{{{}}}", parts.join(","))
        }
        Value::Array(items) => {
            let parts: Vec<String> = items.iter().map(canonical_json).collect();
            format!("[{}]", parts.join(","))
        }
        other => serde_json::to_string(other).expect("scalar serializes"),
    }
}

impl Record {
    pub fn to_value(&self) -> Result<Value> {
        serde_json::to_value(self).map_err(|e| ContractError::new(e.to_string()))
    }

    /// Bytes the signature covers: canonical record with `sig` removed.
    pub fn signing_bytes(&self) -> Result<Vec<u8>> {
        let mut value = self.to_value()?;
        value
            .as_object_mut()
            .ok_or_else(|| ContractError::new("record is not an object"))?
            .remove("sig");
        Ok(canonical_json(&value).into_bytes())
    }

    /// Hash the next record links to: SHA-256 of the canonical record
    /// including `sig`.
    pub fn record_hash(&self) -> Result<String> {
        let value = self.to_value()?;
        Ok(hex::encode(Sha256::digest(
            canonical_json(&value).as_bytes(),
        )))
    }
}
