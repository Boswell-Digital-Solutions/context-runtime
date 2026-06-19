# context-runtime — Compiled System Reference

**Designation:** CTX
**Document role:** Canonical compiled technical reference for the context-runtime PCC-conforming context runtime
**Source:** `doc/system/`
**Build command:** `bash doc/system/BUILD.sh`
**Document version:** 2.0 (2026-06-19) — BDS canonical-compliance migration (7-group class-aware structure, truth classes, designation-bound fail-closed assembly, authored governance trio)
**Protocol:** BDS Documentation Protocol v2.0; BDS Repo Documentation System Canonical Compliance Standard

> **Generated artifact warning:** `doc/CTXSYSTEM.md` is assembled output. Edit the
> source modules under `doc/system/` and rebuild. Hand edits to the compiled
> artifact are overwritten by the next build.

Assembly contract:

- Command: `bash doc/system/BUILD.sh`
- Validation: `bash doc/system/validate_snapshots.sh` runs during assembly
- Primary output: `doc/CTXSYSTEM.md`

This `doc/system/` tree is the canonical source of truth for context-runtime. It
uses explicit **truth classes**: *canonical facts* define the runtime role, PCC
contract conformance, versioning/slice contract, and ecosystem boundaries;
*snapshot facts* are dated, audit-derived counts (modules, tests, slices).
context-runtime is the runtime that honors `precomputed-context-core` (PCC)
contracts; it consumes, never redefines them. See §7 for the scope/authority
boundary and §8 for ownership and designation doctrine.

| Part | File | Contents |
| --- | --- | --- |
| §1 | `00_overview/01-identity-philosophy.md` | Repo identity, PCC-conforming runtime posture |
| §2 | `00_overview/02-purpose-and-scope.md` | Purpose and scope |
| §3 | `00_overview/03-architecture.md` | Architecture overview (gather → assemble → serve) |
| §4 | `10_service-contract/04-contract-surface.md` | Contract surface (PCC-exported contracts honored) |
| §5 | `20_runtime/05-versioning-slice-progression.md` | Versioning & slice progression |
| §6 | `30_dependencies/06-dependencies.md` | Dependencies (incl. PCC pin) |
| §7 | `40_governance/07-scope.md` | Service authority boundary, truth classes |
| §8 | `40_governance/08-governance.md` | Ownership, designation doctrine, authority hierarchy |
| §9 | `40_governance/09-change-control.md` | Change classes, evidence, verification commands |
| §10 | `50_operations/10-validation-and-proof.md` | Validation & proof |
| §11 | `99_appendices/11-glossary-and-paths.md` | Glossary & paths |

## Quick Assembly

```bash
bash doc/system/BUILD.sh
```

---

# §1 — Identity & Philosophy

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

---

# §2 — Purpose & Scope

`context-runtime` turns a code target (repo + file) into a **governed context
bundle** the self-healing AI shaper can build a fix from, safely.

### Role in the self-healing loop

```
[context-runtime: gather -> assemble(PCC) -> serve]
   -> forgeHQ AI shaper (context packet -> generate -> pact-verify -> propose)
   -> DataForge-Local healing-proposals -> FC /self-healing -> operator -> apply
```

It fills the gap forgeHQ explicitly deferred: forgeHQ's pipeline governs *scope*
(`ContextBundle.context_item_refs`; the generator fail-closes on any ref outside
the admitted set) but does not produce the admitted refs in a governed way, nor
hold the content behind them ("the adapter boundary is intentionally not wired").
`context-runtime` is that producer and that adapter.

### In-scope posture

- Read-only repo discovery (gather candidate sources for a target file).
- Run the real `precomputed_context_core::assemble_context` to produce a
  governed `ContextBundleManifest` (admissibility / freshness / authority /
  deterministic `bundle_hash` / replay eligibility).
- Build a validated code-native PCC contract behind each admitted ref
  (`KeyFilePacket` / `RepoNavigationMap` / `ValidationCommandPacket`).
- Serve the manifest + payloads over HTTP, fail-closed on scope escape.

### Out-of-scope posture

This repo does not run an LLM, does not persist truth (DataForge-Local owns
durable operational truth), does not verify fixes (that is `pact`), and does not
mint upstream truth. It governs and serves context; it proposes nothing.

---

# §3 — Architecture Overview

A thin axum service over a pure assembly pipeline.

### Module map

- `gather.rs` — read-only repo discovery. For a target file, produces four
  governed source roles (target / adjacent / repo-truth / validation) plus
  deterministic `RepoFacts` (dirs, entry points, canonical docs, validation
  commands). Detects stack (rust / python / node / generic).
- `assemble.rs` — the pipeline. Maps each source role to a PCC governance
  `SourceClass`, builds a `ContextAssemblyRequest`, calls the real
  `assemble_context`, then builds a validated code-native payload behind each
  admitted ref. Returns an `AssembledBundle`.
- `payload.rs` — constructs and validates the code-native PCC contracts
  (`KeyFilePacketContract` / `RepoNavigationMapContract` /
  `ValidationCommandPacketContract`) on a fully-valid `ArtifactRecord` base.
- `store.rs` — in-process bundle store; the authority for which refs are
  admitted in a bundle. Fails closed on unknown bundle or scope escape.
- `http.rs` — axum 0.8 surface (see Contract Surface).
- `config.rs`, `error.rs`, `main.rs` — env config, fail-closed error→HTTP
  mapping, server entrypoint.

### Two-surface design (Option 3)

The **envelope** is PCC's `ContextBundleManifest` — the only thing in PCC that
emits a hashed, replay-eligible governed bundle. The **payload** behind each ref
is a PCC code-native contract. The code→authoring class mapping
(target→`ActiveScene`, adjacent→`AdjacentScene`, repo-truth→`AcceptedLoreRecord`,
validation→`AcceptedStyleRuleRecord`) lives entirely inside `assemble.rs`; the
wire output is refs + a hash + typed payloads.

---

# §4 — Contract Surface

### HTTP API

- `GET /healthz` → `{ ok, service, contract, envelope, payload_contracts[] }`.
- `POST /v1/context/assemble`
  - request: `{ repo_id, repo_root, target_file, task_intent_id?, task_family?,
    task_version?, max_source_age_minutes?, override_posture? }`
  - response: `{ context_bundle_id, bundle_hash, manifest, payload_refs,
    context_item_refs }` where `manifest` is PCC's `ContextBundleManifest`
    verbatim and `context_item_refs == payload_refs` (the forgeHQ seam).
- `GET /v1/context/{bundle_id}/payload?ref=<payload_ref>`
  - response: `{ payload_ref, role, source_class, artifact_class, content_hash,
    content, contract }` where `contract` is the validated code-native PCC
    contract.
  - **fail-closed:** unknown bundle → `404`; ref not in the admitted inventory
    (scope escape) → `409`.

### Governed contracts consumed from PCC (the authority)

- Envelope: `precomputed_context_core::assemble_context` →
  `ContextBundleManifest`.
- Payloads: `KeyFilePacketContract`, `RepoNavigationMapContract`,
  `ValidationCommandPacketContract` on `ArtifactRecord`, each validated via its
  own `.validate()` and PCC's canonical admissibility algebra.

### Contract rule

`context-runtime` is a runtime *against* PCC's contracts. PCC is the authority
for contract shape; this repo never redefines a contract — it gathers, governs,
and serves.

---

# §5 — Versioning & Slice Progression

This repo advances through bounded slices, fail-closed.

### Current slice position — C1 (done)

C1 = "governed bundle over HTTP + Rust↔Python crossing", Option 3:

- Real `assemble_context` envelope from gathered live repo sources.
- Code-native, `.validate()`-passing PCC payloads behind every admitted ref.
- axum HTTP surface, fail-closed on scope escape / unknown bundle.
- Deterministic `bundle_hash` (replay handle).
- Rust↔Python crossing proven via `scripts/smoke_crossing.py`.

### Planned

- **C2** — richer gathering (import-aware adjacency, multiple key files,
  per-source authority levels) and DataForge-Local persistence + replay of
  bundles.
- **C3** — wire into forgeHQ: a context-client driver feeds
  `ContextBundleService` / `candidate_design` / `candidate_generation` with the
  governed refs + payloads.
- **C4** — pact verification stage consumes the bundle handle; then the forgeHQ
  AI shaper (request context packet → NeuroForge generate → pact-verify →
  propose).

---

# §6 — Dependencies

### Primary technical dependencies

- Rust toolchain (edition 2024).
- `precomputed-context-core` (path dependency) — the governed contract surface.
- `axum` 0.8 + `tokio` 1 — HTTP runtime (matches the ecosystem convention in
  `Forge_Command/api`).
- `serde` / `serde_json` — request/response and contract serialization.
- `sha2` 0.10 — payload content integrity hashes.
- `chrono` 0.4 — artifact record timestamps.
- `thiserror` 2, `tracing` / `tracing-subscriber` — errors and logging.
- dev: `reqwest` 0.12 — HTTP test client.

### Dependency posture

Dependencies are accepted only when they support the runtime, the real PCC
contracts, deterministic hashing, or fail-closed serving. No persistence or LLM
dependency in C1.

---

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

---

# §8 — Governance

**Truth class:** canonical doctrine

Ownership, review, and change-authority boundaries for this documentation system.
§7 defines context-runtime's *service* authority; this chapter defines the
*documentation* authority that governs how this `doc/system/` tree is owned,
designated, and changed.

## Ownership

| Artifact | Owner |
|----------|-------|
| `doc/system/` source modules | context-runtime repository (this repo) |
| `doc/CTXSYSTEM.md` compiled artifact | context-runtime repository — generated, never hand-edited |
| `CTX` designation | ForgeCommand designation registry (governed registry state, not local repo opinion) |
| Ecosystem composite compiled system reference | ForgeCommand |

The operating context is a single-operator governed environment: compliance state
must be explicit, visible, and reconstructable; remediation is bounded and
reviewable; approval remains human-authoritative.

## Designation doctrine

- The designation is exactly three letters (`CTX`), unique across the governed
  repo registry, and stable once assigned.
- The compiled artifact filename is bound to the designation:
  `doc/{DESIGNATION}SYSTEM.md` → `doc/CTXSYSTEM.md`. `BUILD.sh` fails closed if the requested
  output path does not end in `CTXSYSTEM.md`.
- Designation changes occur only through explicit change control in the
  ForgeCommand registry, never by local edit.
- Legacy outputs (`SYSTEM.md` at repo root, two-letter prefixed artifacts, the
  bootstrap `doc/SYSTEM.md`) are non-canonical; if detected they are migration
  signals, not truth surfaces.

## Authority hierarchy

When documentation sources conflict, resolve in this order:

1. `doc/CTXSYSTEM.md` — the compiled system reference (implemented reality)
2. `CLAUDE.md` — AI implementation instructions and working rules
3. Module/feature specs and plans under `docs/`
4. README and ad-hoc notes

The compiled system reference wins because it describes implemented reality; all
other surfaces describe intent, instruction, or history.

## Truth-class enforcement

Every statement in this tree is a **canonical fact** (service role, contracts, and
invariants — changed only through change control, §9) or a **snapshot fact**
(audit-derived counts: routes, tables, tests, coverage — labelled with a
measurement date and corrected by re-measurement, not change control). Snapshot
facts must never be promoted to guarantees; release/readiness language is
constrained per §7.

## Editing rule

Source modules under `doc/system/` are the only editing surface. The compiled
artifact is regenerated by `bash doc/system/BUILD.sh` and validated by
`doc/system/validate_snapshots.sh` during assembly. A hand edit to `doc/CTXSYSTEM.md` is a
governance violation and is overwritten by the next build.

## Enforcement

ForgeCommand is the enforcement surface for documentation compliance. Where
automated enforcement is not yet wired up for this repo, enforcement is manual but
explicit: the change-control workflow in §9.

---

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

---

# §10 — Validation & Proof

Validation is evidence-based and fail-closed.

### Minimum validation posture

- `cargo test --offline` must pass.
- Every rejected path (stale source, missing required source, unresolved
  authority, missing target, missing repo, scope escape, invalid payload
  contract) must fail closed — no success envelope on rejection.
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
fetch payload → scope-escape 409 → unknown-bundle 404.

`scripts/smoke_crossing.py`: stdlib-only client (run from a project venv) that
drives the running service exactly as forgeHQ will — proves the Rust↔Python
crossing end to end.

### Running

```bash
cargo test --offline
# crossing smoke:
CONTEXT_RUNTIME_BIND=127.0.0.1:8011 ./target/debug/context-runtime &
<venv>/bin/python scripts/smoke_crossing.py http://127.0.0.1:8011
```

---

# §11 — Glossary & Paths

### Important paths

- Repo root: `~/Forge/ecosystem/local-systems/context-runtime`
- Documentation source root: `doc/system/`
- Build entry: `doc/system/BUILD.sh` → `doc/CTXSYSTEM.md`
- Pipeline: `src/assemble.rs`; HTTP: `src/http.rs`; gather: `src/gather.rs`;
  payloads: `src/payload.rs`
- Contract authority: `../../precomputed-context-core` (`PCC`)
- Consumer: `../forgeHQ` (`ContextBundleService`, candidate pipeline)

### Glossary

- **Envelope:** PCC's `ContextBundleManifest` — governed, hashed, replay-eligible
  bundle of source refs.
- **Payload:** the validated code-native PCC contract behind one admitted ref.
- **Admitted ref:** a `payload_ref` present in a bundle's inventory; the only
  refs the service will serve and the generator may modify.
- **Scope escape:** a request for a ref not admitted in the bundle — rejected
  fail-closed (`409`).
- **Fail-closed:** an invalid or rejected path returns an error and never a
  success envelope.
