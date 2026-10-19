use serde::{Deserialize, Serialize};

use crate::common::{Id, Timestamp};
use crate::security_model::person::PersonRole;

/// One observation from one source (camera analytics, a door, a sensor).
/// `source_id` is deliberately untyped (just an Id) rather than an enum of
/// Camera/Door/Sensor — an adapter emitting this doesn't need to know which
/// kind of device is on the other end of that ID, only that it happened.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Id,
    pub organization_id: Id,
    pub location_id: Id,
    pub occurred_at: Timestamp,
    pub event_type: String,
    pub source_id: Id,
    pub related_recording_id: Option<Id>,
}

/// A named collection of events a person is investigating, with the
/// NIEM-style person/role associations (see `person::PersonRole`) attached
/// directly rather than left implicit in free-text notes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub id: Id,
    pub organization_id: Id,
    pub title: String,
    pub status: IncidentStatus,
    pub opened_at: Timestamp,
    pub closed_at: Option<Timestamp>,
    pub event_ids: Vec<Id>,
    pub persons: Vec<IncidentPerson>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentPerson {
    pub person_id: Id,
    pub role: PersonRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentStatus {
    Open,
    Verified,
    Resolved,
    Dismissed,
}
