use serde::{Deserialize, Serialize};

use crate::common::{Id, Timestamp};

/// Field shape follows ONVIF's Device/Media services (the protocol Layer 1
/// adapters already speak per interoperability.md) rather than an arbitrary
/// shape — adapters map ONVIF's own DeviceInformation/Profile responses
/// onto this with minimal translation instead of inventing a second schema
/// per vendor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub id: Id,
    pub organization_id: Id,
    pub location_id: Id,
    pub name: String,
    pub vendor: String,
    pub model: String,
    /// None when the camera is only reachable through an NVR/VMS
    /// (Layer 3 integration) rather than addressed directly.
    pub onvif_endpoint: Option<String>,
    pub status: CameraStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CameraStatus {
    Online,
    Offline,
    Degraded,
}

/// One retrieved clip. `original_uri` is the untouched export required by
/// evidence-integrity.md; `playable_uri` is the normalized copy the PRD's
/// workflow step 4 produces "when needed" — never the other way around.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recording {
    pub id: Id,
    pub camera_id: Id,
    pub starts_at: Timestamp,
    pub ends_at: Timestamp,
    pub original_uri: String,
    pub playable_uri: Option<String>,
}
