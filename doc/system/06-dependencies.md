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

### Compatibility verification snapshot — 2026-10-01

The path dependency was verified against PCC revision
`b0071c01e8275800c698ef57452dd13443637459` (optional source provenance,
uniform/per-class freshness policy, tagged SHA-256 bundle identity). This is a
verification revision, not an immutable dependency pin: Cargo resolves the local
path checkout. Re-run the runtime tests after changing that checkout.
