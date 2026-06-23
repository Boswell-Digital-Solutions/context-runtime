//! HTTP surface (axum 0.8).
//!
//! - POST /v1/context/assemble        → governed ContextBundleManifest + admitted refs
//! - POST /v1/context/assemble-scenes → governed two-scene continuity bundle (lineage triple)
//! - GET  /v1/context/{id}/payload    → one code-native payload (fail-closed on scope escape)
//! - GET  /healthz                    → liveness + contract identity

use std::path::PathBuf;
use std::time::SystemTime;

use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use precomputed_context_core as pcc;

use crate::assemble::{AssembleParams, StoredPayload, assemble};
use crate::config::Config;
use crate::error::{ContextError, Result};
use crate::payload::content_hash;
use crate::scene::{SceneAssembleParams, assemble_scenes};
use crate::store::BundleStore;

#[derive(Clone)]
pub struct AppState {
    pub cfg: Config,
    pub store: BundleStore,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/v1/context/assemble", post(assemble_handler))
        .route("/v1/context/assemble-scenes", post(assemble_scenes_handler))
        .route("/v1/context/{bundle_id}/payload", get(payload_handler))
        .with_state(state)
}

async fn healthz() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "ok": true,
        "service": "context-runtime",
        "contract": "context_assembly.phase1 + code_native_payloads + scene_assemble",
        "envelope": "precomputed_context_core::assemble_context",
        "payload_contracts": [
            "key_file_packet",
            "repo_navigation_map",
            "validation_command_packet"
        ],
        "scene_assemble": {
            "endpoint": "/v1/context/assemble-scenes",
            "task_family": "continuity",
            "payload_contracts": ["scene_text"]
        }
    }))
}

#[derive(Debug, Deserialize)]
struct AssembleReq {
    repo_id: String,
    repo_root: String,
    target_file: String,
    #[serde(default)]
    task_intent_id: Option<String>,
    #[serde(default)]
    task_family: Option<String>,
    #[serde(default)]
    task_version: Option<String>,
    #[serde(default)]
    max_source_age_minutes: Option<u64>,
    #[serde(default)]
    override_posture: Option<String>,
}

#[derive(Debug, Serialize)]
struct AssembleResp {
    /// Echoed so the whole chain (context → verification) shares one intent id;
    /// pact binds its receipt to {task_intent_id, context_bundle_id, bundle_hash}.
    task_intent_id: String,
    context_bundle_id: String,
    bundle_hash: String,
    manifest: pcc::ContextBundleManifest,
    /// Admitted refs.
    payload_refs: Vec<String>,
    /// Same list, named for the forgeHQ ContextBundle seam.
    context_item_refs: Vec<String>,
}

fn parse_override(raw: Option<&str>) -> Result<pcc::OverridePosture> {
    match raw.unwrap_or("disallow_all") {
        "disallow_all" => Ok(pcc::OverridePosture::DisallowAll),
        "allow_style" | "allow_accepted_style_rule_records" => {
            Ok(pcc::OverridePosture::AllowAcceptedStyleRuleRecords)
        }
        other => Err(ContextError::BadRequest(format!(
            "unknown override_posture '{other}' (expected disallow_all | allow_style)"
        ))),
    }
}

async fn assemble_handler(
    State(state): State<AppState>,
    Json(req): Json<AssembleReq>,
) -> Result<Json<AssembleResp>> {
    let task_intent_id = req.task_intent_id.unwrap_or_else(|| {
        let seed = format!("{}:{}", req.repo_id, req.target_file);
        format!("ti_codefix_{}", &content_hash(&seed)[..16])
    });

    let params = AssembleParams {
        repo_id: req.repo_id,
        repo_root: PathBuf::from(req.repo_root),
        target_file: req.target_file,
        task_intent_id,
        task_family: req.task_family.unwrap_or_else(|| "code_fix".to_string()),
        task_version: req.task_version.unwrap_or_else(|| "v1".to_string()),
        max_source_age_minutes: req
            .max_source_age_minutes
            .unwrap_or(state.cfg.default_max_source_age_minutes),
        override_posture: parse_override(req.override_posture.as_deref())?,
    };

    let bundle = assemble(&params, SystemTime::now())?;
    let resp = AssembleResp {
        task_intent_id: params.task_intent_id.clone(),
        context_bundle_id: bundle.manifest.context_bundle_id.clone(),
        bundle_hash: bundle.manifest.bundle_hash.clone(),
        manifest: bundle.manifest.clone(),
        payload_refs: bundle.payload_refs.clone(),
        context_item_refs: bundle.payload_refs.clone(),
    };
    state.store.put(bundle);
    Ok(Json(resp))
}

#[derive(Debug, Deserialize)]
struct AssembleScenesReq {
    /// AuthorForge project id — namespaces the scene payload refs.
    project_id: String,
    scene_a_id: String,
    scene_a_text: String,
    scene_b_id: String,
    scene_b_text: String,
    #[serde(default)]
    scope_label: Option<String>,
    #[serde(default)]
    task_intent_id: Option<String>,
    #[serde(default)]
    task_version: Option<String>,
    #[serde(default)]
    max_source_age_minutes: Option<u64>,
}

/// Mint a governed continuity bundle over two adjacent scenes and return its
/// lineage triple. The triple is what AuthorForge threads into its
/// continuity_check task so the whole chain (context → check → pact) shares one
/// intent id and a verifiable bundle hash.
async fn assemble_scenes_handler(
    State(state): State<AppState>,
    Json(req): Json<AssembleScenesReq>,
) -> Result<Json<AssembleResp>> {
    let task_intent_id = req.task_intent_id.unwrap_or_else(|| {
        let seed = format!("{}:{}:{}", req.project_id, req.scene_a_id, req.scene_b_id);
        format!("ti_continuity_{}", &content_hash(&seed)[..16])
    });

    let params = SceneAssembleParams {
        project_id: req.project_id,
        scene_a_id: req.scene_a_id,
        scene_a_text: req.scene_a_text,
        scene_b_id: req.scene_b_id,
        scene_b_text: req.scene_b_text,
        scope_label: req
            .scope_label
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "adjacent_scene".to_string()),
        task_intent_id,
        task_version: req.task_version.unwrap_or_else(|| "v1".to_string()),
        max_source_age_minutes: req
            .max_source_age_minutes
            .unwrap_or(state.cfg.default_max_source_age_minutes),
    };

    let bundle = assemble_scenes(&params)?;
    let resp = AssembleResp {
        task_intent_id: params.task_intent_id.clone(),
        context_bundle_id: bundle.manifest.context_bundle_id.clone(),
        bundle_hash: bundle.manifest.bundle_hash.clone(),
        manifest: bundle.manifest.clone(),
        payload_refs: bundle.payload_refs.clone(),
        context_item_refs: bundle.payload_refs.clone(),
    };
    state.store.put(bundle);
    Ok(Json(resp))
}

#[derive(Debug, Deserialize)]
struct RefQuery {
    #[serde(rename = "ref")]
    reference: String,
}

async fn payload_handler(
    State(state): State<AppState>,
    Path(bundle_id): Path<String>,
    Query(q): Query<RefQuery>,
) -> Result<Json<StoredPayload>> {
    let payload = state.store.get_payload(&bundle_id, &q.reference)?;
    Ok(Json(payload))
}
