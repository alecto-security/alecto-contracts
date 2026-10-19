# Changelog

Versions follow SemVer. Bundle schema changes are governed by the
evidence-bundle standard (docs/SPEC.md section 7); notary record changes by
the record-version field.

## [Unreleased]

- Vector mirror pin bumped to `evidence-bundle v0.2.0`: its fixture set adds
  the `tampered-notary-signature` and `tampered-notary-chain` negative
  vectors, and the mirror test now covers them.

## [0.1.0] - 2026-10-19

Born public, born tagged.

### Added

- `security_model`: the Common Security Model, extracted from the
  producing workspace, Apache-2.0 licensed at extraction as its own change
  marker requested. Frozen serde names, additive-optional policy carried
  over from the original.
- `bundle`: evidence-bundle schema v1 types (metadata, camera records,
  gap reasons, origin validation, provenance). Metadata readers ignore
  unknown fields: the spec is additive-optional and strictness belongs to
  the seal verifier.
- `notary`: signed record format, canonical JSON (sorted keys, no
  whitespace), signing bytes, record hash. Chain links reject unknown
  fields; their evolution is a record-version bump.
- CI: test, clippy `-D warnings`, no-network crate guard, and the vector
  mirror: the pinned `evidence-bundle` fixture set runs through these
  types on every commit.
