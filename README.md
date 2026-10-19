# alecto-contracts

The type contract for the Alecto evidence ecosystem, as one crate every
other actor compiles against: the Common Security Model, the
evidence-bundle schema v1, and the notary record format. Closed repos pin
published versions; public consumers compile against the crate; nobody
pins the gateway or a private repo.

<p align="center">
  <a href="https://github.com/alecto-security/alecto-contracts/actions/workflows/ci.yml"><img src="https://github.com/alecto-security/alecto-contracts/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-blue.svg" alt="License: Apache-2.0"></a>
  <img src="https://img.shields.io/badge/schema_version-1-purple.svg" alt="Bundle schema version 1">
  <img src="https://img.shields.io/badge/record_version-1-orange.svg" alt="Notary record version 1">
</p>

## What lives here

| Module | Contract | Contents |
|--------|----------|----------|
| `security_model` | Layer 6, Common Security Model | Camera, Door, Credential, Person, Incident, Evidence, custody types; ONVIF- and NIEM-shaped |
| `bundle` | schema v1, [ADR-0002](https://github.com/alecto-security/evidence-bundle) | `metadata.json` and `camera.json` types, gap reasons, origin validation pair |
| `notary` | record format, [alecto-notary](https://github.com/alecto-security/alecto-notary) | Signed record types, canonical JSON, signing bytes, record hash |

What deliberately does not live here: verification logic. The seal
verifier is the `evidence-bundle` standard's job; notary chain
verification is the notary's. One implementation of each, that is the
whole idea.

## The mirror rule, executed

The bundle spec is normative in `evidence-bundle`, which also owns the
test vectors. The types in `bundle` must be able to read those vectors,
and CI re-proves it on every commit: a pinned clone of the standard's
fixture set runs through these types (`tests/vectors_mirror.rs`). The pin
lives in `vectors/evidence-bundle.tag` and bumps only through a PR, so the
types and the vectors move together by construction.

## Versioning contract

These types serialize into stored records and wire messages: field names
are frozen at bundle schema version `1` and record version `1`.

- Renaming a field, changing its type, or changing a serde container or
  variant casing is a breaking change.
- Changes are **additive-optional only**: new fields arrive as
  `Option<T>` or `#[serde(default)]`, omitted when absent, so older
  readers keep deserializing and older records keep round-tripping.
- Removing or renaming a public type or enum variant is a new schema
  version, not an edit.
- Readers of externally produced documents ignore unknown fields: a newer
  producer's added optional field must not break this release. Strictness
  belongs to the verifier. Notary records are the exception: chain links
  reject unknown fields, and their evolution is a record-version bump.
- Consuming repos pin published versions; they never pull git deps of
  public repos. Until the crate is on crates.io, adopters may use a
  tag-pinned git dependency as an explicit transition and switch to the
  registry the moment a version is published.

## Repository layout

```text
src/security_model/   the normalized object set (Layer 6)
src/bundle.rs         evidence-bundle schema v1 types
src/notary.rs         signed record format + canonical JSON
tests/vectors_mirror.rs   the mirror rule, as an executed test
vectors/              the mirror pin (which standard version we match)
scripts/fetch_vectors.sh  local equivalent of the CI mirror job
```

## License

Apache-2.0. Extracted from the producing implementations and relicensed at
extraction, as the producing repos' own change markers anticipated.
