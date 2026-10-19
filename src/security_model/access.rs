use serde::{Deserialize, Serialize};

use crate::common::Id;

/// Modeled on ONVIF's Access Control Service (AccessPoint/Door) rather than
/// a generic "door" shape: an AccessPoint controls passage in one direction
/// between two Areas, and a single physical door can have more than one
/// AccessPoint (in vs. out). Adapters for access-control vendors (Layer 2)
/// map onto this directly since it's the same relationship ONVIF already
/// standardizes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Door {
    pub id: Id,
    pub location_id: Id,
    pub name: String,
    pub from_location_id: Option<Id>,
    pub to_location_id: Option<Id>,
}

/// Wire-level credential exchange (OSDP, Wiegand) is out of scope for this
/// model — that's an adapter/protocol concern. What crosses into the
/// Common Security Model is only what ONVIF's CredentialInfo exposes:
/// who holds it and whether it's currently valid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub id: Id,
    pub holder_person_id: Id,
    pub credential_type: CredentialType,
    pub status: CredentialStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialType {
    Badge,
    Pin,
    Biometric,
    MobileCredential,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialStatus {
    Active,
    Suspended,
    Expired,
}
