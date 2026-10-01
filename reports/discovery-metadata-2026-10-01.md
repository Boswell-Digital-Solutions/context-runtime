# context-runtime discovery metadata verification

Measured 2026-10-01. Source base: `bce3c4cf46944c503358b967c2b8b631e4404158` (merged PCC repair). Change class C0/C2/C5/C6. The runtime owns only an additive, self-reported build declaration; PCC still owns its contracts. This report is evidence of the local change, not an admission of desktop discovery or trial authority.

## Behavior and source limits

`build.rs` records the runtime and PCC Git revisions and source states, hashes the runtime discovery schema and five selected PCC schema files, and embeds sorted metadata into the binary. An unavailable Git revision is `null` with source state `unknown`; uncommitted source is `dirty`. `/healthz` returns it without runtime filesystem or Git access. The declaration can be copied by another responder, so it cannot attest process identity. No PACT endpoint or service behavior was added. The six schema files use only local JSON Schema references (`#/definitions/`), so the selected closure does not require network resolution.

Cargo watches the exact schema inputs, relevant runtime sources, and Git HEAD/ref/index paths. The manifest is regenerated on the next Cargo build when those change. An unrelated working-file change after building does not rewrite the already built binary; `source_state` describes build time, not present workspace state. A source archive without Git reports unknown revision instead of inventing a pin.

## Verification

- `cargo test --locked --offline --target-dir /tmp/hfx-pcc-context-check`: 11 integration tests passed with loopback permission. The first sandbox run could not bind a local listener; it was rerun with permitted loopback.
- `cargo build --locked --offline --target-dir /tmp/hfx-pcc-context-check`: passed.
- `bash doc/system/BUILD.sh`: passed, designation CTX, 11 parts.
- Changed-file Rust formatting and `git diff --check`: passed.
- The existing repository-wide `cargo fmt --all --check` fails on pre-existing formatting in untouched files. `cargo clippy --all-targets -- -D warnings` fails on pre-existing `collapsible_if` findings in `src/config.rs` and `type_complexity` in `src/gather.rs`. Neither finding comes from this metadata change. They are not claimed as passed.

Human review: confirm the self-reported trust wording, selected contract file set, and additive health schema before merging. Hephaestus may display declarations only after pinning the admitted owner revision. Trial readiness remains blocked.
