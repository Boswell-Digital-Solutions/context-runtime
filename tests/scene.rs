//! Scene-assemble HTTP test: mints a governed two-scene continuity bundle over
//! the real axum server — the path AuthorForge uses to obtain a lineage triple.

use context_runtime::config::Config;
use context_runtime::http::{AppState, router};
use context_runtime::store::BundleStore;

async fn spawn_server() -> String {
    let state = AppState {
        cfg: Config::default(),
        store: BundleStore::new(),
    };
    let app = router(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    format!("http://{addr}")
}

fn scene_body() -> serde_json::Value {
    serde_json::json!({
        "project_id": "proj-1",
        "scene_a_id": "sc-001",
        "scene_a_text": "The lantern shattered on the harbor stones as the tide drew back.",
        "scene_b_id": "sc-002",
        "scene_b_text": "By morning she raised the lantern again, its glass whole.",
        "scope_label": "adjacent_scene"
    })
}

#[tokio::test]
async fn assemble_scenes_mints_lineage_triple_and_serves_scene_payloads() {
    let base = spawn_server().await;
    let client = reqwest::Client::new();

    // healthz advertises the scene contract.
    let hv: serde_json::Value = client
        .get(format!("{base}/healthz"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(hv["scene_assemble"]["task_family"], serde_json::json!("continuity"));

    let resp = client
        .post(format!("{base}/v1/context/assemble-scenes"))
        .json(&scene_body())
        .send()
        .await
        .unwrap();
    assert!(resp.status().is_success(), "assemble-scenes status {}", resp.status());
    let v: serde_json::Value = resp.json().await.unwrap();

    // The lineage triple AuthorForge threads into continuity_check.
    let bundle_id = v["context_bundle_id"].as_str().unwrap().to_string();
    assert!(bundle_id.starts_with("ctxb_"), "bundle id {bundle_id}");
    assert!(
        v["task_intent_id"].as_str().unwrap().starts_with("ti_continuity_"),
        "intent {}",
        v["task_intent_id"]
    );
    assert!(v["bundle_hash"].as_str().unwrap().len() >= 16);

    // Exactly the two scene refs are admitted.
    let refs = v["context_item_refs"].as_array().unwrap();
    assert_eq!(refs.len(), 2);
    let active_ref = "scene://proj-1/sc-002".to_string();
    let adjacent_ref = "scene://proj-1/sc-001".to_string();
    let ref_set: std::collections::HashSet<&str> =
        refs.iter().map(|r| r.as_str().unwrap()).collect();
    assert!(ref_set.contains(active_ref.as_str()));
    assert!(ref_set.contains(adjacent_ref.as_str()));

    // The active (consequent) scene is served back as scene_text.
    let pv: serde_json::Value = client
        .get(format!("{base}/v1/context/{bundle_id}/payload"))
        .query(&[("ref", active_ref.as_str())])
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(pv["role"].as_str().unwrap(), "active");
    assert_eq!(pv["artifact_class"].as_str().unwrap(), "scene_text");
    assert_eq!(pv["contract"]["kind"].as_str().unwrap(), "scene_text");
    assert!(pv["content"].as_str().unwrap().contains("whole"));
}

#[tokio::test]
async fn assemble_scenes_is_deterministic_and_echoes_caller_intent() {
    let base = spawn_server().await;
    let client = reqwest::Client::new();

    let mut body = scene_body();
    body["task_intent_id"] = serde_json::json!("ti_caller_supplied_001");

    let mint = || async {
        client
            .post(format!("{base}/v1/context/assemble-scenes"))
            .json(&body)
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap()
    };

    let first = mint().await;
    let second = mint().await;

    // Caller-supplied intent id is echoed verbatim (the chain shares one id).
    assert_eq!(first["task_intent_id"], serde_json::json!("ti_caller_supplied_001"));
    // Same scenes → same governed hash + id (replay-stable mint).
    assert_eq!(first["bundle_hash"], second["bundle_hash"]);
    assert_eq!(first["context_bundle_id"], second["context_bundle_id"]);
}

#[tokio::test]
async fn assemble_scenes_rejects_empty_scene_text() {
    let base = spawn_server().await;
    let client = reqwest::Client::new();

    let mut body = scene_body();
    body["scene_b_text"] = serde_json::json!("   ");

    let resp = client
        .post(format!("{base}/v1/context/assemble-scenes"))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::BAD_REQUEST);
}
