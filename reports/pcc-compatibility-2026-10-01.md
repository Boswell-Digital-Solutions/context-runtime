# PCC compatibility verification

Measured 2026-10-01 UTC. Change class: C1 contract-surface conformance, with C0
reference updates and C5 regression assertions.

## Source revisions and authority

- Runtime base: `fbde4fdbd0ca2ab2b6321ba486881392d6c4891f`.
- PCC checkout: `b0071c01e8275800c698ef57452dd13443637459` (clean).
- PCC owns the contracts and identity algorithm; runtime consumes them.
- No dependency or lockfile change; the local path dependency is not a revision pin.

## Failure and repair

Before the repair, `cargo check --locked --offline --target-dir
/tmp/hfx-pcc-context-check` failed with four E0063 errors: missing `provenance`
and `class_overrides` initializers in both code and scene assembly. Existing tests
and the Python smoke also expected the obsolete primary `ctxb_` prefix.

Both adapters now explicitly supply no memory provenance and use PCC's uniform
freshness constructor. HTTP responses continue to expose the real PCC manifest.
Regression assertions cover SHA-256 primary identities, retained legacy fields,
manifest deserialization, ref equality, and absent memory provenance.

## Verification

- `cargo test --locked --offline --target-dir /tmp/hfx-pcc-context-check`:
  **11 passed** (7 pipeline/store, 1 code HTTP, 3 scene HTTP). The real forgeHQ
  fixture was present. The sandbox denied socket binding on the initial run;
  the complete suite passed with loopback permission.
- `cargo build --locked --offline --target-dir /tmp/hfx-pcc-context-check`: passed.
- `python3 scripts/smoke_crossing.py http://127.0.0.1:59691`: passed against the
  newly built temporary server and real forgeHQ target; health, assembly,
  tagged identity, validated code payload, scope-escape 409. Server stopped.
- Smoke primary ID:
  `ctxb.sha256.80edbdcd88c84eb058e69c2568d2378e8994fad1e40f54b7831b1df91fad7826`.
- `bash doc/system/BUILD.sh`: BUILD_OK, designation CTX, 11 chapters.
- `git diff --check`: passed.

## Limits and operator action

This proves local runtime compatibility with the recorded PCC revision. It does
not prove a downstream PACT/Hephaestus live loop, deployment, or new discovery
behavior. Historical Hephaestus captures and source pins remain unchanged.
Primary bundle lookup has no legacy-ID alias. Review and merge the repair before
updating dependent source pins or claiming the old captured blocker resolved.
Rollback is a revert of this repair, which restores the known compile failure
against this PCC checkout.
