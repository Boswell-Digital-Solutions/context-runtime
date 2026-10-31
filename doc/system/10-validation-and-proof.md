# §10 — Validation & Proof

Validation is evidence-based and fail-closed.

### Minimum validation posture

- `cargo test --offline` must pass.
- Every rejected path (stale source, missing required source, missing target,
  missing repo, scope escape, invalid payload contract) must fail closed — no
  success envelope on rejection.
- Unresolved authority cannot occur today. `src/assemble.rs` and `src/scene.rs`
  give every gathered source `AuthorityState::Accepted`, so the PCC authority
  rejection never runs here. PCC tests that rejection in its own repository.
- Assembly must be deterministic: identical inputs → identical `bundle_hash`.

### Current proof surface

`tests/assemble.rs` (hermetic temp repo + real forgeHQ if present):

- governed bundle with code-native payloads assembles and validates
- assembly is deterministic (same inputs → same hash)
- stale source fails closed
- missing target / missing repo fail closed
- store serves admitted refs and fails closed on scope escape / unknown bundle
- assembles cleanly against a real forgeHQ source file when present

`tests/http.rs`: boots the real axum server on an ephemeral port; assemble →
fetch payload → scope-escape 409 → unknown-bundle 404. The same health request
checks versioned self-reported build metadata and the exact six-file manifest.
The build script watches runtime source files, selected PCC schema files, and
Git HEAD/ref/index paths so a changed source revision or contract file causes
the embedded declaration to regenerate on the next Cargo build.

`scripts/smoke_crossing.py`: stdlib-only client (run from a project venv) that
drives the running service exactly as forgeHQ will — proves the Rust↔Python
crossing end to end.

### Running

Every cargo command needs the checkout layout first. `Cargo.toml` declares
`precomputed-context-core = { path = "../../precomputed-context-core" }`, two
levels up, so PCC must sit two directories above this checkout. A flat layout of
sibling repositories does not satisfy it, and cargo then fails before it loads
the manifest:

```text
error: failed to get `precomputed-context-core` as a dependency of package
  `context-runtime v0.1.0`
  unable to update /<parent>/precomputed-context-core
```

`cargo build`, `cargo test` and `cargo tree` are all unavailable in that state.
`cargo audit` still works, because it reads only `Cargo.lock`. CI builds the
layout it needs (see §6), so this precondition applies to a local run.

```bash
cargo test --offline
# crossing smoke:
CONTEXT_RUNTIME_BIND=127.0.0.1:8011 ./target/debug/context-runtime &
<venv>/bin/python scripts/smoke_crossing.py http://127.0.0.1:8011
```

### Continuous integration

`.github/workflows/ci.yml` runs on every pull request and every push to
`master`. The Rust job checks out PCC at the pinned revision (see §6), confirms
the path dependency resolves to it, and runs `cargo fmt --check`,
`cargo clippy --locked --all-targets -- -D warnings`, and `cargo test --locked`
on Rust 1.97.0. It then fails if either tree changed. The documentation job
rebuilds `doc/CTXSYSTEM.md` and fails when it differs from `doc/system/`.

A green run does not prove the authority rejection path. Every gathered source
is still `AuthorityState::Accepted` (see the minimum validation posture above).

### PCC compatibility regression coverage

Both HTTP assembly tests validate the native tagged ID/hash pair, PCC manifest
deserialization, legacy identity retention, admitted-ref equality, and absence of
memory provenance. The existing stale-source and determinism tests cover the
uniform policy; payload fetches exercise tagged IDs through the real router and
store. The Python crossing smoke checks the same primary and legacy identities.
See `reports/pcc-compatibility-2026-10-01.md` for the measured verification result.
