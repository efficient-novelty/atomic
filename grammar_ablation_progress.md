# Grammar Ablation Progress

Last updated: 2026-04-17
Status: Workstream 0 complete; Workstream 1 started

## Completed This Turn

- Added the operational tracking docs for the grammar-ablation lane.
- Added `GrammarProfile` parsing/defaults in
  `crates/pen-search/src/config.rs`.
- Preserved default behavior by making the missing config field resolve to
  `canonical_mbtt_v1`.
- Added explicit canonical control config
  `configs/grammar_ablation_baseline.toml`.
- Persisted `grammar_profile` into `run.json` via
  `crates/pen-store/src/manifest.rs` and
  `crates/pen-cli/src/cmd_run.rs`.
- Added `grammar_profile` to the `run_started` telemetry payload.
- Surfaced `grammar_profile` in `pen-cli inspect` output, with an `unknown`
  fallback for historical runs that predate the field.
- Regenerated JSON schemas after the manifest contract change.

## Verification

- `cargo test -p pen-search missing_grammar_profile_defaults_to_canonical_mbtt_v1 -- --nocapture`
- `cargo test -p pen-search shadow_profiles_parse_with_expected_labels -- --nocapture`
- `cargo test -p pen-store run_manifest_round_trip_preserves_frozen_keys -- --nocapture`
- `cargo test -p pen-cli claim_run_writes_policy_metadata_and_claim_narrative -- --nocapture`
- `cargo run -q -p xtask -- generate-schemas`
- `cargo run -q -p pen-cli -- inspect runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15`

## Notes

- No hostile grammar semantics are live yet. This turn only landed the
  metadata/control plumbing and an explicit canonical baseline config.
- Historical runs remain readable because the new manifest field is optional on
  deserialize.
