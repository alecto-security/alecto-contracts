# Security Policy

## Reporting a vulnerability

Please report vulnerabilities through GitHub's private security advisories:
open the repository's **Security** tab and choose **Report a vulnerability**.

Do not open a public issue for a security report.

We aim to acknowledge reports within 72 hours and will coordinate disclosure
with you before any public advisory.

## Scope

This crate is consumed as a contract by closed producers, so a bug here can
break evidence handling everywhere. Bugs that matter most:

- **Canonicalization correctness:** `notary::canonical_json` output for a
  given value is signature-relevant forever. Output that changes for the
  same input, or a tie-break that is not total, is critical: signatures and
  chain links compute over it.
- **Serde fidelity:** a field rename, casing change, or type change that
  silently changes the wire format (the frozen contract).
- **Deserialization safety:** a panic (or unbounded resource use) on
  hostile JSON while parsing contracts.
- **Reader tolerance:** the metadata types must keep accepting additive-
  optional documents, and record types must keep rejecting them.

## Known limitations (not bugs)

- No verification logic lives here by design: the seal verifier is the
  evidence-bundle standard; chain verification is alecto-notary. This
  crate must not grow either, so there is exactly one implementation of
  each.
- `serde_json` number handling (f64 in canonical form) follows
  serde_json's own printing; producers sign over bytes they wrote, so a
  cross-parser numeric-canon mismatch is handled at producer identity,
  not here.

## Supported versions

Security fixes land on `main` and release as patch versions of the current
minor line. Breaking contract changes never take a patch route.
