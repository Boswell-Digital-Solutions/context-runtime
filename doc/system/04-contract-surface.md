# §4 — Contract Surface

### HTTP API

- `GET /healthz` → `{ ok, service, contract, envelope, payload_contracts[],
  scene_assemble{ endpoint, task_family, payload_contracts[] } }`.
- `POST /v1/context/assemble` (code-fix path)
  - request: `{ repo_id, repo_root, target_file, task_intent_id?, task_family?,
    task_version?, max_source_age_minutes?, override_posture? }`
  - response: `{ context_bundle_id, bundle_hash, manifest, payload_refs,
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
