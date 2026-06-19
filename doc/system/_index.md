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
