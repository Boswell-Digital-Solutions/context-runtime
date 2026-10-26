# §1 — Identity & Philosophy

> **System identity — bds family (Boswell Digital Solutions business system, local-systems tier).** This service is part of the Forge ecosystem backend in `ecosystem/local-systems`. The ecosystem map gives it no Forge-family counterpart. Its contract crate is the bds `precomputed-context-core` (`ecosystem/precomputed-context-core`). It does not use the Forge-family copy `apps/public-app-local-support/forge-precomputed-context-core`.

**Repo:** `context-runtime`
**Proposed Designation:** `CTX`
**Repo Root:** `~/Forge/ecosystem/local-systems/context-runtime`
**Repo Class:** Service / Runtime (local-systems)
**Language:** Rust
**Operational Posture:** Internal business system, single-operator, local, fail-closed

### Identity note

`context-runtime` is the PCC-conforming context **runtime** for the self-healing
loop. `precomputed-context-core` (`PCC`) is a Library/Contract crate that, by
design, owns no service runtime; this repo is that runtime. It depends on PCC's
real exported contracts and never redefines them.
