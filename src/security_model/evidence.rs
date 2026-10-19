use serde::{Deserialize, Serialize};

use crate::common::{Id, Timestamp};

/// SHA-256 only, on purpose — not a placeholder pending more options.
/// MD5 (broken 2004/2008) and SHA-1 (broken by SHAttered, 2017) are
/// excluded rather than merely un-preferred: an evidence hash on a
/// deprecated algorithm is a legal liability the moment it's challenged in
/// court, per ISO/IEC 27037 and SWGDE practice. Don't add a variant here
/// without checking that guidance first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HashAlgorithm {
    Sha256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceHash {
    pub algorithm: HashAlgorithm,
    pub digest_hex: String,
}

/// The four core processes ISO/IEC 27037 defines for digital evidence
/// handling (Identification, Collection, Acquisition, Preservation), plus
/// `Transferred` for the PRD's final handoff step ("transfers it to an
/// authorized destination"). Every entry needs a phase from this exact
/// list to be admissible chain-of-custody, not a free-text status string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustodyPhase {
    Identified,
    Collected,
    Acquired,
    Preserved,
    Transferred,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustodyEvent {
    pub phase: CustodyPhase,
    pub at: Timestamp,
    /// Who or what performed this step — a person_id for a human transfer,
    /// or a system/service identifier for an automated one. Never empty:
    /// an unattributed custody event breaks the chain it's meant to prove.
    pub actor: String,
    pub note: Option<String>,
}

/// Maps directly onto the PRD's evidence bundle
/// (original/, playable/, SHA256SUMS, audit-log.json) — this struct's
/// fields, serialized, ARE those files, not a separate representation of
/// them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: Id,
    pub organization_id: Id,
    pub incident_id: Option<Id>,
    pub source_recording_id: Id,
    pub original_uri: String,
    pub playable_uri: Option<String>,
    pub hash: EvidenceHash,
    pub custody_log: Vec<CustodyEvent>,
}
