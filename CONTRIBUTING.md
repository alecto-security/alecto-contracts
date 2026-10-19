# Contributing

Issues, bug reports and pull requests are welcome. This repo is the type
contract layer, so most changes here are mirrors of decisions made
elsewhere first: that ordering is deliberate.

## Order of changes

1. **Schema or record-format changes** start in the standard that owns
   them: bundle schema in [evidence-bundle](https://github.com/alecto-security/evidence-bundle)
   (spec + fixtures first), notary record format in
   [alecto-notary](https://github.com/alecto-security/alecto-notary).
2. **The mirror follows in the same cycle**: update types here plus the
   pinned tag in `vectors/evidence-bundle.tag`, in this same PR.
3. **Additive-optional only** (crate-level versioning rules). Removing or
   renaming anything public is a new schema or record version, never an
   edit inside v1.
4. **Wire-format changes ship as MAJOR**; additive changes as MINOR;
   fixes patch.

## Code changes

- Tests for new behavior; clippy must stay green at `-D warnings`.
- Keep the dependency list: `serde`, `serde_json`, `time`, `uuid`,
  `sha2`, `hex`. The manifest-audit test fails on product-repo deps, and
  the no-network guard fails on network-capable crates.
- New contract surface needs a fixture or vector that exercises it. A
  contract without a vector that binds it is prose, not a contract.

## Setup

Rust stable, nothing else.

```bash
cargo test
cargo clippy --all-targets -- -D warnings
./scripts/fetch_vectors.sh && VECTORS_DIR=.vectors/fixtures cargo test
```

## Hygiene

Real bundles and chains are evidence: never attach case data to an issue
or PR. Contract discussions use the synthetic vectors.
