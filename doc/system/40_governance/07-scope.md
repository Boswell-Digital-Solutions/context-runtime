# §7 — Scope

**Truth class:** canonical doctrine

This `doc/system/` tree is the modular source of the **context-runtime compiled
system reference**, assembled into the designation-bound artifact
`doc/CTXSYSTEM.md` (designation `CTX`) via `bash doc/system/BUILD.sh`. This chapter
defines context-runtime's authority and where it ends. context-runtime is an
internal Forge ecosystem service — single-operator, local, fail-closed — not a
public product and not externally release-certified.

## context-runtime Service Authority

context-runtime is the **PCC-conforming context runtime** for the self-healing
loop: the Rust service that *gathers → assembles → serves* context, conforming to
the contracts exported by `precomputed-context-core` (`PCC`). Its authority is
runtime-oriented: it executes the context stage against PCC's real exported
contracts and serves the result. PCC is the library/contract crate that owns the
contract definitions; this repo is the runtime that honors them.

## What context-runtime Owns

- The **context runtime** — the gather/assemble/serve execution of the context
  stage.
- **Contract-surface conformance** (§4) — honoring PCC's exported contracts at the
  boundary, exactly as PCC defines them.
- **Versioning / slice progression** (§5) — the slice-by-slice progression of this
  runtime under design approval.
- **Validation & proof** (§10) of its own runtime behavior.

## What context-runtime Does Not Own

- **The PCC contracts themselves.** `precomputed-context-core` owns the contract
  definitions; context-runtime depends on and never redefines them.
- **Canonical durable truth.** DataForge owns durable records.
- **Decision / application authority.** Acceptance and application belong to the
  operator/control plane (ForgeCommand); this runtime serves context, it does not
  decide or apply.
- **Orchestration / scheduling.** ForgeCommand is the operator/control plane.

## Release / Readiness Language Restrictions

This documentation describes an internal service under governed, slice-by-slice
development. It must be described as a verification-current internal service, not
as externally release-certified, and must not claim public-release/SaaS readiness
or present coverage percentages as guarantees unless a later governed slice proves
the specific claim.

## Documentation truth classes

- **Canonical facts** define context-runtime's runtime role, PCC contract
  conformance, versioning/slice contract, and ecosystem boundaries. They change
  only through deliberate change control (§9).
- **Snapshot facts** are audit-derived counts (modules, tests, slices) labelled
  with a measurement date and corrected by re-measurement, not change control.

Ownership, designation doctrine, and the authority hierarchy that govern this tree
are defined in §8.
