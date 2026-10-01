//! HTTP surface test: boots the real axum server on an ephemeral port and
//! drives it with an HTTP client — the same path forgeHQ will use.

mod common;
mod identity;

use common::TempRepo;
use context_runtime::config::Config;
use context_runtime::http::{AppState, router};
use context_runtime::store::BundleStore;
use sha2::{Digest, Sha256};

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

#[tokio::test]
async fn http_assemble_fetch_payload_and_scope_escape() {
    let repo = TempRepo::python();
    let base = spawn_server().await;
    let client = reqwest::Client::new();

    // healthz
    let health = client.get(format!("{base}/healthz")).send().await.unwrap();
    assert!(health.status().is_success());
    let hv: serde_json::Value = health.json().await.unwrap();
    assert_eq!(hv["ok"], serde_json::json!(true));
    let discovery = &hv["discovery"];
    assert_eq!(discovery["schema"], "bds.context-runtime.discovery.v1");
    assert_eq!(discovery["trust"], "self_reported");
    assert_eq!(
        discovery["runtime"]["repository"],
        "Boswell-Digital-Solutions/context-runtime"
    );
    assert_eq!(
        discovery["pcc"]["repository"],
        "Boswell-Digital-Solutions/precomputed-context-core"
    );
    let contracts = discovery["contracts"].as_array().unwrap();
    assert_eq!(contracts.len(), 6);
    for contract in contracts {
        assert!(contract["bytes"].as_u64().unwrap() > 0);
        let digest = contract["sha256"].as_str().unwrap();
        assert_eq!(digest.len(), 64);
        assert!(digest.bytes().all(|b| b.is_ascii_hexdigit()));
    }
    assert_eq!(contracts[0]["path"], "schemas/discovery.v1.schema.json");
    let runtime_schema = std::fs::read(format!(
        "{}/schemas/discovery.v1.schema.json",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();
    assert_eq!(contracts[0]["bytes"], runtime_schema.len());
    assert_eq!(
        contracts[0]["sha256"],
        format!("{:x}", Sha256::digest(&runtime_schema))
    );

    // assemble
    let body = serde_json::json!({
        "repo_id": "fixture",
        "repo_root": repo.root.to_string_lossy(),
        "target_file": "pkg/target.py"
    });
    let resp = client
        .post(format!("{base}/v1/context/assemble"))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert!(
        resp.status().is_success(),
        "assemble status {}",
        resp.status()
    );
    let v: serde_json::Value = resp.json().await.unwrap();
    let bundle_id = v["context_bundle_id"].as_str().unwrap().to_string();
    assert!(bundle_id.starts_with("ctxb.sha256."));
    identity::assert_pcc_identity(&v);
    // task_intent_id is echoed so the chain (context → pact verify) shares it.
    assert!(
        v["task_intent_id"]
            .as_str()
            .unwrap()
            .starts_with("ti_codefix_")
    );
    let refs = v["context_item_refs"].as_array().unwrap();
    assert_eq!(refs.len(), 4);
    let first_ref = refs[0].as_str().unwrap().to_string();

    // fetch an admitted payload
    let payload = client
        .get(format!("{base}/v1/context/{bundle_id}/payload"))
        .query(&[("ref", first_ref.as_str())])
        .send()
        .await
        .unwrap();
    assert!(
        payload.status().is_success(),
        "payload status {}",
        payload.status()
    );
    let pv: serde_json::Value = payload.json().await.unwrap();
    assert_eq!(pv["payload_ref"].as_str().unwrap(), first_ref);
    assert!(pv["contract"].is_object());
    assert!(!pv["content_hash"].as_str().unwrap().is_empty());

    // scope escape: a ref not in the admitted inventory → 409 CONFLICT
    let escape = client
        .get(format!("{base}/v1/context/{bundle_id}/payload"))
        .query(&[("ref", "file://fixture/not/admitted.py")])
        .send()
        .await
        .unwrap();
    assert_eq!(escape.status(), reqwest::StatusCode::CONFLICT);

    // unknown bundle → 404
    let missing = client
        .get(format!("{base}/v1/context/ctxb_unknown/payload"))
        .query(&[("ref", first_ref.as_str())])
        .send()
        .await
        .unwrap();
    assert_eq!(missing.status(), reqwest::StatusCode::NOT_FOUND);

    drop(repo);
}
