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
