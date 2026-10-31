# context-runtime — Known Issues

## doc/system does not state the default bind or that no route authenticates its caller (Open 2026-10-30) — CTX-DOC-BIND-AND-CALLER-AUTH-UNSTATED-20261030

- **Found**: 2026-10-30, by the BDS-XMACH-CALLER-BOUNDARY-001 WP00 inventory (forge PR #325). Verified at `origin/master` `94a560b`. Source and documentation reading only. No running service was tested.
- **Location**: `src/config.rs:17` (default bind), `src/config.rs:26-30` (`CONTEXT_RUNTIME_BIND` override); `src/main.rs:20` (bind); `src/http.rs:31-37` (router); `doc/system/` (all parts); `doc/system/10-validation-and-proof.md:43-44` (the only mention).
- **Observed behavior**:
  - The service binds `127.0.0.1:8011` by default (`src/config.rs:17`).
  - `CONTEXT_RUNTIME_BIND` replaces the address when it is set and not empty (`src/config.rs:26-30`). The code does not check the value. Any address that `TcpListener::bind` accepts is used (`src/main.rs:20`), a non-loopback address included.
  - The router has four routes and no authentication layer or middleware (`src/http.rs:31-37`). `GET /healthz`, `POST /v1/context/assemble`, `POST /v1/context/assemble-scenes` and `GET /v1/context/{bundle_id}/payload` take no caller credential.
- **Documentation claim**: no part of `doc/system/` states the default bind, the `CONTEXT_RUNTIME_BIND` variable, that the override accepts a non-loopback address, or that no route authenticates its caller. The only mention of `127.0.0.1:8011` is the smoke command in `doc/system/10-validation-and-proof.md:43-44`. That command sets the variable explicitly and does not say that the value is the default.
- **Impact**: a reader of `doc/system/` cannot learn the network posture of the service. This is a documentation gap. It is not classified here as a security vulnerability. It is relevant to threat T-1 of the cross-machine plan BDS-XMACH-CALLER-BOUNDARY-001.
- **Proposed correction**: add to `doc/system/` (for example part 03, architecture, or part 07, scope) a statement of: the default bind `127.0.0.1:8011`; the `CONTEXT_RUNTIME_BIND` override and that it accepts any address; and that no route authenticates its caller, so the loopback bind is the only access control by default. Rebuild `doc/CTXSYSTEM.md` with `bash doc/system/BUILD.sh`. This entry does not edit `doc/system`.
- **Fix**: none. Documentation only. No runtime change is proposed.
- **Status**: Open — recorded, unresolved until the correction is verified.

## doc/system named a counterpart path that does not exist (Closed 2026-10-26) — CTX-DOC-COUNTERPART-PATH-20261026

- **Location**: `doc/system/01-identity-philosophy.md`, identity banner.
- **What was wrong**: the banner named a Forge counterpart `apps/public-app-local-support/precomputed-context-core`. That path does not exist, and the ecosystem map gives context-runtime no Forge-family counterpart.
- **Found**: 2026-10-26, in the review of plan BDS-UMIEP-v0.1.
- **Fix**: the banner now says that there is no counterpart, and names the PCC crate that this service uses.
- **Status**: Closed.

## The authority rejection path cannot occur (Open 2026-10-26) — CTX-AUTHORITY-ALWAYS-ACCEPTED-20261026

- **Location**: `src/assemble.rs` (`to_source_input`) and `src/scene.rs`; `doc/system/10-validation-and-proof.md`.
- **What is wrong**: both files set `authority_state: pcc::AuthorityState::Accepted` for every gathered source. PCC rejects a bundle with an authority conflict, but this service never gives it one. `doc/system/10-validation-and-proof.md` listed "unresolved authority" as a rejected path that must fail closed.
- **Effect**: a consumer can read a context bundle from this service as authority-checked. It is checked for freshness only.
- **Root cause**: the service has no source of authority state. The value is a constant.
- **Fix**: documentation only. Part 10 now says that the path cannot occur. No code change is authorized. A real authority source needs its own design.
- **Status**: Open. Closed scope: the documentation claim. Open scope: the constant authority state.

## The declared local gate cannot run in a flat checkout (Open 2026-10-31) — CTX-LOCAL-GATE-LAYOUT-20261031

- **Location**: `Cargo.toml:33`; `doc/system/10-validation-and-proof.md` (Running); `CLAUDE.md`.
- **What is wrong**: the manifest declares `precomputed-context-core = { path = "../../precomputed-context-core" }`, two levels up. In a flat layout of sibling repositories the path resolves above the parent directory, so cargo fails before it loads the manifest: `error: failed to get precomputed-context-core as a dependency ... unable to update /<parent>/precomputed-context-core`. `cargo build`, `cargo test` and `cargo tree` are all unavailable. `cargo audit` still works, because it reads only `Cargo.lock`.
- **Found**: 2026-10-31, in the AuthorForge support-repository review (SR-07), while trying to run the declared gate on a flat checkout.
- **What was already correct**: `doc/system/06-dependencies.md` already documents the CI layout and the `ci/precomputed-context-core.rev` pin. The gap was the local side: §10 and `CLAUDE.md` named `cargo test --offline` with no layout precondition, so a reader or an agent on a flat checkout reads the failure as a defect in the repository.
- **Fix**: §10's Running section and `CLAUDE.md` now state the precondition, show the exact error, and say that `cargo audit` is the one command that works without it.
- **Status**: Open. Closed scope: the documentation gap. Open scope: whether the two-level path should become a workspace member, a git dependency or a registry dependency, so the layout stops being a precondition. That needs its own decision.

## Cargo.lock advisories were unaudited, and none reaches the served binary (Open 2026-10-31) — CTX-LOCKFILE-ADVISORY-REACH-20261031

- **Location**: `Cargo.lock`; `.github/workflows/ci.yml`; `doc/system/06-dependencies.md`.
- **What was wrong**: no gate audited `Cargo.lock`. `cargo audit` 0.22.2 reported nine vulnerabilities over 269 crate dependencies on 2026-10-31: `h2` 0.4.13 RUSTSEC-2026-0258, `rustls` 0.23.37 (four), `rustls-webpki` 0.103.10 (four), plus `anyhow` and `tokio` warnings.
- **Found**: 2026-10-31, in the AuthorForge support-repository review (SR-06, SR-04).
- **Reach, measured**: none of the nine reaches the served binary. `cargo tree --edges normal` holds no `h2`, `rustls`, `rustls-webpki`, `webpki-roots` or `native-tls`. `h2` enters only through `hyper`'s `http2` feature, and `axum` is declared `features = ["macros"]` without `http2`. The `rustls` advisories enter through `reqwest`'s TLS features; `reqwest` is in the runtime graph (`jsonschema` 0.18 depends on it, and PCC depends on `jsonschema`) but without TLS features, so no TLS crate is compiled in. `reqwest` is additionally a dev dependency here.
- **Correction to the source review**: that review stated the eight TLS advisories arrive "only through `reqwest`, declared under `[dev-dependencies]`", and left `h2` reachability open. `reqwest` is also a runtime dependency through PCC's `jsonschema`, and `h2` is now measured as absent from the runtime graph. The conclusion is unchanged and better supported: the advisories are dev-and-test only.
- **Fix**: an `audit` job runs `cargo audit` on every change. A vulnerability fails it; a warning reports without failing, so the gate does not go red on unchanged code when the feed moves. `doc/system/06-dependencies.md` records the measured reach and the rule that a lockfile advisory is not by itself evidence of runtime exposure.
- **Correction, same day**: an earlier draft of this entry said the nine would stay in the lock and that the audit job would be green because none reaches the served build. That was wrong, and CI said so: `cargo audit` reads `Cargo.lock`, not the build graph, so it reported all nine and failed the job. All nine are instead fixed in the lock — `cargo update -p h2 -p rustls -p rustls-webpki` moves them to 0.4.20, 0.23.46 and 0.103.16 inside their existing semver ranges — so no `--ignore` entry is needed and the gate stays strict.
- **Status**: Closed. The gate exists, the nine are fixed in the lock, and the measured reach is recorded in `doc/system/06-dependencies.md`. The two `cargo audit` warnings (`anyhow` RUSTSEC-2026-0190, `tokio` RUSTSEC-2026-0386) remain reported and non-failing by design.
