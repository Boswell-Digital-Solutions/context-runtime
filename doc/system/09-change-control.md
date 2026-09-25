# §9 — Change Control

**Truth class:** canonical doctrine

This chapter defines how changes to context-runtime are classified, evidenced,
verified, and rolled back. Every change class names the evidence and verification
commands that must accompany it. context-runtime is an internal Forge ecosystem
service; nothing here authorizes public-release or production-certification claims.

## Change Classes

| Class | Scope | Example |
|-------|-------|---------|
| C0 | Documentation only | Editing `doc/system/` chapters, rebuilding `doc/CTXSYSTEM.md` |
| C1 | Contract-surface conformance | Honoring PCC's exported contracts at the boundary (§4) |
| C2 | Context runtime | gather/assemble/serve execution logic (§3) |
| C3 | Versioning / slice progression | A new slice, slice gating, version bump (§5) |
| C4 | Dependencies | PCC pin / crate dependency changes (§6) |
| C5 | Validation & proof | Proof artifacts, validation gates (§10) |
| C6 | Configuration / security | Env contract, fail-closed posture |

## Required Evidence Per Change Class

- **C0** — rebuilt artifact (`bash doc/system/BUILD.sh` → `BUILD_OK`), edited
  source chapter (never a hand-edit to `doc/CTXSYSTEM.md`).
- **C1** — proof the boundary still conforms to PCC's exported contracts *exactly*
  — context-runtime consumes, never redefines, those contracts.
- **C2** — tests for the changed gather/assemble/serve path; fail-closed on
  ambiguity preserved.
- **C3** — the slice documented in §5 under design approval (slice-by-slice).
- **C4** — the PCC pin / dependency change reflected in §6, built cleanly.
- **C5** — the validation/proof evidence in §10.
- **C6** — env/setting reflected with secrets never hard-coded; fail-closed kept.

## Required Verification Commands

```bash
cargo test                              # full suite
cargo build                             # builds against the pinned PCC contract
bash doc/system/BUILD.sh                # doc changes (C0) -> BUILD_OK designation=CTX
```

## Contract-Conformance / Boundary Rules

context-runtime stays the *runtime* that honors PCC's contracts (§7): a change must
not redefine, fork, or shadow a PCC-exported contract — it consumes them. Slice
progression stays governed (design approval per slice); the runtime fails closed
on missing/invalid context rather than fabricating it.

## Documentation Change Rules (C0)

`doc/system/` source modules are the only editing surface. The compiled
`doc/CTXSYSTEM.md` is regenerated, never hand-edited (§8). Snapshot facts are
re-measured and re-dated, not asserted as guarantees.

## Release / Readiness Claim Rules

No change may introduce public-release, public-SaaS, or production-certification
language, or present a coverage percentage as a guarantee, unless a governed
release slice proves that specific claim.
