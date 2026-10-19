use serde::{Deserialize, Serialize};

use crate::common::{Id, Timestamp};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sensor {
    pub id: Id,
    pub location_id: Id,
    pub name: String,
    pub sensor_type: SensorType,
    pub last_reading_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SensorType {
    Motion,
    Contact,
    Environmental,
    Other,
}
