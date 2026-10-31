# §6 — Dependencies

### Primary technical dependencies

- Rust toolchain (edition 2024).
- `precomputed-context-core` (path dependency) — the governed contract surface.
- `axum` 0.8 + `tokio` 1 — HTTP runtime (matches the ecosystem convention in
  `Forge_Command/api`).
- `serde` / `serde_json` — request/response and contract serialization.
- `sha2` 0.10 — payload content integrity hashes.
- Build-only `sha2` 0.10 and `serde_json` 1 — hash and serialize the embedded
  discovery manifest. The build invokes local Git for revision/status evidence;
  the running service does not depend on Git for health requests.
- `chrono` 0.4 — artifact record timestamps.
- `thiserror` 2, `tracing` / `tracing-subscriber` — errors and logging.
- dev: `reqwest` 0.12 — HTTP test client.

### Dependency posture

Dependencies are accepted only when they support the runtime, the real PCC
contracts, deterministic hashing, or fail-closed serving. No persistence or LLM
dependency in C1.

CI audits `Cargo.lock` with `cargo audit` on every change. A vulnerability fails
the job. A warning (unmaintained, unsound, yanked) reports without failing, so
the gate does not go red on unchanged code when the advisory feed moves.

### Advisory reach — 2026-10-31

`cargo audit` reported nine vulnerabilities in `Cargo.lock` and none of them
reaches the served binary. The runtime dependency graph
(`cargo tree --edges normal`) holds no `h2`, `rustls`, `rustls-webpki`,
`webpki-roots` or `native-tls`:

- `h2` enters only through `hyper`'s `http2` feature. `axum` is declared with
  `features = ["macros"]` and no `http2`, so the runtime build does not compile
  it. The full graph reaches `h2` through `hyper` and through
  `hyper-tls` → `reqwest`.
- The `rustls` and `rustls-webpki` advisories enter through `reqwest`'s TLS
  features. `reqwest` is in the runtime graph — `jsonschema` 0.18 depends on it
  and PCC depends on `jsonschema` — but without TLS features, so no TLS crate is
  compiled into the service. `reqwest` is also a dev dependency of this repo.

A lockfile advisory is therefore not evidence that the service is exposed. State
the reach before treating one as a runtime finding.

### Compatibility verification snapshot — 2026-10-01

The path dependency was verified against PCC revision
`b0071c01e8275800c698ef57452dd13443637459` (optional source provenance,
uniform/per-class freshness policy, tagged SHA-256 bundle identity). This is a
verification revision, not an immutable dependency pin: Cargo resolves the local
path checkout. Re-run the runtime tests after changing that checkout.

### CI revision pin — 2026-10-30

CI pins PCC. `ci/precomputed-context-core.rev` holds one full commit SHA,
currently `b0071c01e8275800c698ef57452dd13443637459`. `.github/workflows/ci.yml`
checks out PCC at that commit, beside `local-systems/context-runtime`, so the
path dependency `../../precomputed-context-core` resolves to it. The run fails
when the file is not a full SHA, when PCC is missing, or when PCC is at a
different commit. Local builds still resolve whatever checkout sits at the path.
To move the pin, change the file in a pull request and let CI prove the new
revision.
