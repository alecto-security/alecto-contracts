use serde::{Deserialize, Serialize};

use crate::common::Id;

/// Mirrors ONVIF's Area concept (Access Control Service spec): locations
/// nest, so a customer's "Store #142" can contain "West Parking Lot" can
/// contain a specific door — the same hierarchy ONVIF access-control
/// devices already expose, rather than a flat site list we'd have to
/// re-flatten every adapter's Area tree into.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: Id,
    pub organization_id: Id,
    pub name: String,
    pub parent_location_id: Option<Id>,
    pub address: Option<String>,
}
