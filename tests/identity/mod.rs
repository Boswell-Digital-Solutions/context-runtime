pub fn assert_pcc_identity(v: &serde_json::Value) {
    let manifest: context_runtime::pcc::ContextBundleManifest =
        serde_json::from_value(v["manifest"].clone()).expect("PCC manifest");
    assert_eq!(v["context_bundle_id"], manifest.context_bundle_id);
    assert_eq!(v["bundle_hash"], manifest.bundle_hash);
    assert_eq!(manifest.bundle_hash.len(), 64);
    assert!(manifest.bundle_hash.bytes().all(|b| b.is_ascii_hexdigit()));
    assert_eq!(
        manifest.context_bundle_id,
        format!("ctxb.sha256.{}", manifest.bundle_hash)
    );
    assert_eq!(manifest.legacy_bundle_hash.len(), 16);
    assert_eq!(
        manifest.legacy_context_bundle_id,
        format!("ctxb_{}", manifest.legacy_bundle_hash)
    );
    assert_eq!(v["payload_refs"], v["context_item_refs"]);
    assert!(
        v["manifest"]["source_inventory"]
            .as_array()
            .unwrap()
            .iter()
            .all(|source| source.get("provenance").is_none())
    );
}
