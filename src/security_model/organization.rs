use serde::{Deserialize, Serialize};

use crate::common::Id;

/// A customer account. Scopes every other object below it — the gateway
/// and alecto-app both filter all queries by organization_id; there is no
/// cross-org visibility anywhere in the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: Id,
    pub name: String,
    /// Matches the tier names in alecto-docs/company/business-model.md —
    /// keep these two lists in sync by hand, this crate doesn't own billing.
    pub tier: OrganizationTier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationTier {
    SoftwareOnly,
    SiteGateway,
    EnterpriseGateway,
    HighAvailabilityGateway,
}
