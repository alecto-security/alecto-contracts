//! The Common Security Model: the normalized object set every adapter maps
//! into and every application (gateway API, client app) operates against.
//! It is the interoperability doctrine's Layer 6, the shape all Layer 1-3
//! adapters converge on.
//!
//! Field shapes are drawn from existing industry standards this product
//! already has to interoperate with, rather than invented fresh:
//! - `camera`, `access`: ONVIF Media/Device and Access Control services
//! - `person`: NIEM Justice domain's Person/role vocabulary
//! - `evidence`: ISO/IEC 27037 + SWGDE digital evidence handling practice
//!
//! See each module for the specific rationale.
//!
//! # Versioning contract
//!
//! Consumers serialize these types into stored records and wire messages,
//! so the field names are a frozen contract: see the crate-level versioning
//! rules. Additive-optional only; removals and renames of fields, types or
//! enum variants require a new schema version.
//!
//! Keep this module dependency-free of product repos: it uses only
//! `serde`, `time` and `uuid`, and must not gain a dependency on a closed
//! product repo.

pub mod access;
pub mod camera;
pub mod event;
pub mod evidence;
pub mod location;
pub mod organization;
pub mod person;
pub mod sensor;

pub use crate::common::{new_id, Id, Timestamp};
pub use access::{Credential, CredentialStatus, CredentialType, Door};
pub use camera::{Camera, CameraStatus, Recording};
pub use event::{Event, Incident, IncidentPerson, IncidentStatus};
pub use evidence::{CustodyEvent, CustodyPhase, Evidence, EvidenceHash, HashAlgorithm};
pub use location::Location;
pub use organization::{Organization, OrganizationTier};
pub use person::{Person, PersonRole, Vehicle};
pub use sensor::{Sensor, SensorType};
