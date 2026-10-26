# context-runtime — Known Issues

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
