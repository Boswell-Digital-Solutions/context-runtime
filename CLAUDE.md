# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

context-runtime (CTX) is the PCC-conforming context runtime for the Forge self-healing loop. For a
target file it gathers candidate sources read-only, runs the real
`precomputed_context_core::assemble_context` to produce a governed `ContextBundleManifest`
(admissibility / freshness / authority / deterministic `bundle_hash` / replay eligibility), builds a
validated code-native PCC contract behind each admitted ref (`KeyFilePacket` / `RepoNavigationMap` /
`ValidationCommandPacket`), and serves both over HTTP — fail-closed on scope escape. Rust, built on
axum + tokio.

Canonical docs live in `doc/system/` and assemble to `doc/CTXSYSTEM.md` (`bash doc/system/BUILD.sh`).
Read that first; the README is a pointer.

## Common Commands

```bash
cargo build --offline --bin context-runtime
CONTEXT_RUNTIME_BIND=127.0.0.1:8011 ./target/debug/context-runtime   # run

cargo test --offline    # unit/integration tests

# Rust<->Python crossing smoke test (run client from a project venv):
<venv>/bin/python scripts/smoke_crossing.py http://127.0.0.1:8011
```

## Architecture

- `src/main.rs` / `src/http.rs` — HTTP server (axum)
- `src/gather.rs` — read-only candidate-source gathering
- `src/assemble.rs` — calls `precomputed_context_core::assemble_context` to produce the governed
  `ContextBundleManifest`
- `src/payload.rs` — code-native PCC contract payloads (`KeyFilePacket` / `RepoNavigationMap` /
  `ValidationCommandPacket`)
- `src/store.rs`, `src/scene.rs`, `src/config.rs`, `src/error.rs` — supporting runtime surfaces
- `precomputed-context-core` (sibling repo, path dependency) owns the actual contract shape

### API

- `POST /v1/context/assemble` → `{ context_bundle_id, bundle_hash, manifest, payload_refs, context_item_refs }`
- `GET /v1/context/{bundle_id}/payload?ref=<payload_ref>` → typed payload (`409` on scope escape, `404` on unknown bundle)
- `GET /healthz`

## Notes

- **Boundaries:** this service has no LLM, no persistence, no verification, and issues no
  proposals. It only governs and serves context. PCC (`precomputed-context-core`) owns contract
  shape; DataForge-Local owns durable truth; pact verifies; forgeHQ proposes. Do not blur these
  boundaries by adding responsibilities that belong to another service.
- Any code change updates `doc/system/` in the same change; rebuild with `bash doc/system/BUILD.sh`.
