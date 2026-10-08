# §4 — Contract Surface

### HTTP API

- `GET /healthz` → `{ ok, service, contract, envelope, payload_contracts[],
  scene_assemble{ endpoint, task_family, payload_contracts[] }, discovery }`.
  The additive `discovery` object follows `schemas/discovery.v1.schema.json`:
  runtime and PCC repository identities, full build-source commits, source
  states, and a sorted six-file contract manifest of byte lengths and SHA-256
  values. It explicitly declares `trust: self_reported`. The metadata is
  captured at build time; `/healthz` does not run Git or read the workspace.
  A matching response does not attest the process, prove live contract behavior,
  authorize a trial, or confer an approval/healthy-now/deployed state. Missing
  or unknown provenance remains visible instead of being synthesized.
- `POST /v1/context/assemble` (code-fix path)
  - request: `{ repo_id, repo_root, target_file, task_intent_id?, task_family?,
    task_version?, max_source_age_minutes?, override_posture? }`
  - response: `{ task_intent_id, context_bundle_id, bundle_hash, manifest, payload_refs,
    context_item_refs }` where `manifest` is PCC's `ContextBundleManifest`
    verbatim and `context_item_refs == payload_refs` (the forgeHQ seam).
- `POST /v1/context/assemble-scenes` (continuity / scene path)
  - request: `{ project_id, scene_a_id, scene_a_text, scene_b_id, scene_b_text,
    scope_label?, task_intent_id?, task_version?, max_source_age_minutes? }`
  - response: same shape as `/assemble` (`{ task_intent_id, context_bundle_id,
    bundle_hash, manifest, payload_refs, context_item_refs }`). Mints a governed
    PCC bundle over two adjacent scenes — scene_b → `ActiveScene`, scene_a →
    `AdjacentSceneSummaryOrClippedBody`, no lore/style — under `task_family
    "continuity"`. The returned lineage triple `{ task_intent_id,
    context_bundle_id, bundle_hash }` is what AuthorForge threads into its
    NeuronForge `continuity_check` task. Scene payloads serve as `scene_text`.
    Empty scene text → `400`.
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

### PCC compatibility (2026-10-01)

Both assembly paths use PCC's uniform freshness policy, without per-class
exceptions. Existing code and scene sources have no governed-memory provenance;
the runtime does not admit governed-memory sources in these paths.

The primary identity is PCC's `ctxb.sha256.<64 lowercase hex>` bundle ID and
64-character SHA-256 `bundle_hash`. The manifest also carries PCC's
`legacy_context_bundle_id` (`ctxb_<16 hex>`) and `legacy_bundle_hash` for
migration reference. HTTP responses pass the manifest through verbatim; payload
lookup uses the primary ID, with no legacy-ID alias. Consumers must treat IDs as
opaque and retain the returned ID/hash pair. PCC owns its canonical hash input;
this identity is not a claim of RFC 8785 canonical JSON hashing.

## Source containment (2026-10-08)

Target paths must be relative and cannot contain parent or root components.
Gathering canonicalizes the selected repository and every target, adjacent and
document source before reading; a resolved source outside the repository is
rejected. Sources must be regular UTF-8 files of at most 2 MiB, including files
that grow during reading. This boundary checks containment within the supplied
repository; it does not turn a caller-supplied repository root into a registry
authorization claim. The local operator chooses that root.
