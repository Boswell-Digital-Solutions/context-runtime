//! The scene-assemble pipeline: a two-scene continuity packet → the PCC
//! governance envelope → `precomputed_context_core::assemble_context` → a stored
//! bundle whose lineage triple ({task_intent_id, context_bundle_id, bundle_hash})
//! AuthorForge threads into its continuity_check task.
//!
//! This is the scene-native sibling of [`crate::assemble`]. That module adapts a
//! *code* target on disk onto PCC's (already scene-named) governance classes;
//! this module skips disk gather entirely and maps the live manuscript's two
//! scenes onto their natural classes:
//!
//!   - scene_b (the consequent — where a continuity discrepancy surfaces) →
//!     `ActiveScene`
//!   - scene_a (the antecedent — the prior/adjacent context) →
//!     `AdjacentSceneSummaryOrClippedBody`
//!
//! No lore/style sources participate in v1, so no `AcceptedLoreRecord` /
//! `AcceptedStyleRuleRecord` refs are required (PCC only requires that *provided*
//! target refs are backed by a matching-class source). The envelope (deterministic
//! `bundle_hash`, freshness, replay eligibility) is PCC's real contract — never
//! reinvented here.

use std::collections::HashMap;

use precomputed_context_core as pcc;

use crate::assemble::{AssembledBundle, StoredPayload};
use crate::error::{ContextError, Result};
use crate::payload::content_hash;

/// Continuity is its own task family — distinct from the code-fix path so the
/// lineage handle reads honestly downstream (NeuronForge, pact).
pub const CONTINUITY_TASK_FAMILY: &str = "continuity";

#[derive(Clone, Debug)]
pub struct SceneAssembleParams {
    /// AuthorForge project id — namespaces the scene payload refs.
    pub project_id: String,
    pub scene_a_id: String,
    pub scene_a_text: String,
    pub scene_b_id: String,
    pub scene_b_text: String,
    /// Adjacent-scene by default; carried through so the manifest reads honestly.
    pub scope_label: String,
    /// Caller-supplied intent id, or a derived `ti_continuity_*` when absent.
    pub task_intent_id: String,
    pub task_version: String,
    /// Scenes are read live from the manuscript, so age is 0; the ceiling only
    /// guards against a caller that supplies a stale snapshot.
    pub max_source_age_minutes: u64,
}

/// Stable payload ref for a scene under a project (mirrors the `file://` /
/// `doc://` ref scheme of the code-fix path).
fn scene_ref(project_id: &str, scene_id: &str) -> String {
    format!("scene://{project_id}/{scene_id}")
}

fn scene_source(payload_ref: String, source_class: pcc::SourceClass) -> pcc::SourceInput {
    pcc::SourceInput {
        payload_ref,
        source_class,
        // Live manuscript text — always fresh relative to the assembly clock.
        age_minutes: 0,
        authority_state: pcc::AuthorityState::Accepted,
        is_override: false,
    }
}

fn scene_payload(
    payload_ref: &str,
    scene_id: &str,
    role: &str,
    source_class: pcc::SourceClass,
    content: &str,
) -> StoredPayload {
    StoredPayload {
        payload_ref: payload_ref.to_string(),
        role: role.to_string(),
        source_class: source_class.as_str().to_string(),
        // Scene text rides as-is in v1; a code-native scene contract is a later
        // slice (the consumption side is deferred — the lane echoes the lineage,
        // it does not yet fetch the payload).
        artifact_class: "scene_text".to_string(),
        content_hash: content_hash(content),
        content: content.to_string(),
        contract: serde_json::json!({
            "kind": "scene_text",
            "scene_id": scene_id,
            "role": role,
        }),
    }
}

/// Mint a governed continuity bundle over two adjacent scenes.
pub fn assemble_scenes(params: &SceneAssembleParams) -> Result<AssembledBundle> {
    if params.scene_a_text.trim().is_empty() {
        return Err(ContextError::BadRequest("scene_a_text is empty".into()));
    }
    if params.scene_b_text.trim().is_empty() {
        return Err(ContextError::BadRequest("scene_b_text is empty".into()));
    }

    let active_ref = scene_ref(&params.project_id, &params.scene_b_id);
    let adjacent_ref = scene_ref(&params.project_id, &params.scene_a_id);

    let sources = vec![
        scene_source(active_ref.clone(), pcc::SourceClass::ActiveScene),
        scene_source(
            adjacent_ref.clone(),
            pcc::SourceClass::AdjacentSceneSummaryOrClippedBody,
        ),
    ];

    let request = pcc::ContextAssemblyRequest {
        task_intent_id: params.task_intent_id.clone(),
        task_family: CONTINUITY_TASK_FAMILY.to_string(),
        task_version: params.task_version.clone(),
        target_refs: pcc::TargetRefs {
            active_scene_ref: Some(active_ref.clone()),
            adjacent_scene_ref: Some(adjacent_ref.clone()),
            accepted_lore_record_refs: vec![],
            accepted_style_rule_refs: vec![],
        },
        allowed_source_classes: vec![
            pcc::SourceClass::ActiveScene,
            pcc::SourceClass::AdjacentSceneSummaryOrClippedBody,
        ],
        freshness_policy: pcc::FreshnessPolicy {
            max_source_age_minutes: params.max_source_age_minutes,
        },
        // Scenes never override accepted style rules in this path.
        override_posture: pcc::OverridePosture::DisallowAll,
        sources,
    };

    let output = pcc::assemble_context(&request)
        .map_err(|e| ContextError::AssemblyRejected(e.to_string()))?;

    let mut payloads: HashMap<String, StoredPayload> = HashMap::new();
    payloads.insert(
        active_ref.clone(),
        scene_payload(
            &active_ref,
            &params.scene_b_id,
            "active",
            pcc::SourceClass::ActiveScene,
            &params.scene_b_text,
        ),
    );
    payloads.insert(
        adjacent_ref.clone(),
        scene_payload(
            &adjacent_ref,
            &params.scene_a_id,
            "adjacent",
            pcc::SourceClass::AdjacentSceneSummaryOrClippedBody,
            &params.scene_a_text,
        ),
    );

    Ok(AssembledBundle {
        repo_id: params.project_id.clone(),
        // No file target; the "anchor" is the active (consequent) scene.
        target_rel: params.scene_b_id.clone(),
        manifest: output.manifest,
        payload_refs: output.payload_refs,
        payloads,
    })
}
