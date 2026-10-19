use serde::{Deserialize, Serialize};

use crate::common::Id;

/// Deliberately thin: this is the entity a camera/access event references,
/// not a case-management record. Real identity fields (DOB, address, etc.)
/// belong in whatever downstream justice/case system consumes an
/// exchange — see `PersonRole` below on why the shape stops here.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: Id,
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Same role vocabulary NIEM's Justice domain uses for a Person's
/// relationship to an Incident (Victim/Subject/Witness). Using NIEM's own
/// terms — instead of inventing our own — is what makes an incident export
/// mappable into a law-enforcement system's existing intake without a
/// translation layer; that system already speaks this vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonRole {
    Victim,
    Subject,
    Witness,
    Officer,
    Employee,
    Visitor,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: Id,
    pub plate: Option<String>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
}
