//! Contract types for the Alecto evidence ecosystem, as one crate that is
//! dependency-free of product repos: the Common Security Model every adapter
//! maps into, the evidence-bundle schema v1 every producer writes and every
//! consumer reads, and the notary record format the custody chain is made
//! of.
//!
//! This crate carries shapes and canonicalization only. It deliberately
//! contains no verification logic: the seal verifier lives in the public
//! [evidence-bundle](https://github.com/alecto-security/evidence-bundle)
//! standard, and notary record verification lives in
//! [alecto-notary](https://github.com/alecto-security/alecto-notary).
//! Keeping the split means there is exactly one implementation of each.
//!
//! # The mirror rule (normative)
//!
//! The bundle spec and its test vectors are owned by `evidence-bundle`.
//! The types in `bundle` mirror that spec, and CI in this repo re-checks
//! the match by running the vectors through these types on every commit:
//! producer and contract types cannot drift apart silently.
//!
//! # Versioning contract
//!
//! These types are written as if already published, because closed repos
//! consume them: their serde field names are a frozen contract.
//!
//! - The serde field names in this crate are frozen at schema version `1`
//!   (and record version `1` for notary records). Renaming a field,
//!   changing its type, or changing a serde container or variant rename is
//!   a breaking change.
//! - Changes are **additive-optional only**: you may add a new field when
//!   it is `Option<T>` (or `#[serde(default)]`) and omitted when absent,
//!   so older readers keep deserializing and older records keep
//!   round-tripping.
//! - Removing or renaming a public type (or a variant of a public enum) is
//!   a breaking change requiring a new schema version, not an edit.
//!
//! # Readers tolerate, writers commit
//!
//! Structs that parse externally produced documents (bundle `metadata`,
//! per-camera records) ignore unknown fields: schema v1 is
//! additive-optional, so a bundle written by a newer producer with an
//! added optional field must still be readable by this version of the
//! types. Strictness belongs to the verifier, not the reader. Notary
//! records are the exception and reject unknown fields: a chain link has
//! an exact shape, versioned by its `v` field.
//!
//! Keep this crate dependency-free of product repos: it uses only `serde`,
//! `serde_json`, `time`, `uuid`, `sha2` and `hex`, and must not gain a
//! dependency on any closed product repo.

pub mod bundle;
pub mod common;
pub mod error;
pub mod notary;
pub mod security_model;

pub use crate::error::{ContractError, Result};
