use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RUNTIME_REPOSITORY: &str = "Boswell-Digital-Solutions/context-runtime";
const PCC_REPOSITORY: &str = "Boswell-Digital-Solutions/precomputed-context-core";
const PCC_SCHEMAS: &[&str] = &[
    "schemas/context_assembly_request.schema.json",
    "schemas/context_bundle_manifest.schema.json",
    "schemas/key_file_packet_contract.schema.json",
    "schemas/repo_navigation_map_contract.schema.json",
    "schemas/validation_command_packet_contract.schema.json",
];

fn git(root: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()
        .map(|s| s.trim().to_owned())
}

fn source(root: &Path, repository: &str) -> serde_json::Value {
    let commit = git(root, &["rev-parse", "HEAD"]);
    let status = git(root, &["status", "--porcelain", "--untracked-files=all"]);
    let source_state = match (&commit, &status) {
        (Some(_), Some(s)) if s.is_empty() => "clean",
        (Some(_), Some(_)) => "dirty",
        _ => "unknown",
    };
    serde_json::json!({
        "repository": repository,
        "commit": commit,
        "source_state": source_state,
    })
}

fn contract(root: &Path, repository: &str, path: &str) -> serde_json::Value {
    println!("cargo:rerun-if-changed={}", root.join(path).display());
    let bytes =
        fs::read(root.join(path)).unwrap_or_else(|e| panic!("missing contract {path}: {e}"));
    serde_json::json!({
        "repository": repository,
        "path": path,
        "bytes": bytes.len(),
        "sha256": format!("{:x}", Sha256::digest(&bytes)),
    })
}

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let pcc = root.join("../../precomputed-context-core");
    for source in [&root, &pcc] {
        for ref_name in ["HEAD", "index"] {
            if let Some(path) = git(source, &["rev-parse", "--git-path", ref_name]) {
                println!("cargo:rerun-if-changed={path}");
            }
        }
        if let Some(branch) = git(source, &["symbolic-ref", "HEAD"])
            && let Some(path) = git(source, &["rev-parse", "--git-path", &branch])
        {
            println!("cargo:rerun-if-changed={path}");
        }
    }
    for path in [
        "build.rs",
        "Cargo.toml",
        "Cargo.lock",
        "src/http.rs",
        "src/config.rs",
        "src/assemble.rs",
        "src/scene.rs",
    ] {
        println!("cargo:rerun-if-changed={}", root.join(path).display());
    }
    let mut contracts = Vec::new();
    contracts.push(contract(
        &root,
        RUNTIME_REPOSITORY,
        "schemas/discovery.v1.schema.json",
    ));
    for path in PCC_SCHEMAS {
        contracts.push(contract(&pcc, PCC_REPOSITORY, path));
    }
    contracts.sort_by(|a, b| {
        a["repository"]
            .as_str()
            .cmp(&b["repository"].as_str())
            .then_with(|| a["path"].as_str().cmp(&b["path"].as_str()))
    });
    let manifest = serde_json::json!({
        "schema": "bds.context-runtime.discovery.v1",
        "trust": "self_reported",
        "runtime": source(&root, RUNTIME_REPOSITORY),
        "pcc": source(&pcc, PCC_REPOSITORY),
        "contracts": contracts,
    });
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("output dir"));
    fs::write(
        out.join("discovery.json"),
        serde_json::to_vec(&manifest).expect("manifest JSON"),
    )
    .expect("write discovery manifest");
}
