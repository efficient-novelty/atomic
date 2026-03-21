# Autonomous Claim Lane Progress

Last updated: 2026-03-21

This file tracks the live operational state of `desktop_claim_shadow`. It is
intentionally short and forward-looking; use
[autonomous_plan.md](autonomous_plan.md) for the remaining work sequence.

## Snapshot

- `desktop_claim_shadow` is a distinct profile with its own configs,
  narratives, and run metadata
- claim policy metadata is now honest and claim-specific:
  - `guidance_style = claim_debt_guided`
  - `late_expansion_policy = claim_generic`
  - `bucket_policy = structural_generic`
- claim-path mutator packs and exactness rechecks are landed for the current
  late bands
- `scripts/compare_runs.py` and `scripts/certify_claim_lane.py` are real and
  already enforce manifest completeness, policy honesty, fallback visibility,
  breadth gates, and runtime threshold checks
- `run.json` now carries CPU/build/git/binary fingerprints needed for claim
  certification
- `pen-cli run` and `pen-cli resume` now persist `run.json`, step summaries,
  step checkpoints, frontier snapshots, claim narratives/events, and failure
  status incrementally so interrupted claim runs stay auditable from disk
- claim auto-worker resolution is now memory-aware for
  `desktop_claim_shadow`, so the intended auto setting is capped by the
  available soft-over-target RSS headroom instead of raw logical CPU count
- claim step summaries and telemetry now persist both governor-accounted
  frontier RSS and sampled process RSS so stored claim bundles can show the
  live gap directly
- claim proof-close now drops cached evaluated terminal-prefix payloads after
  recording accept ranks, trading recomputation for a smaller live prefix-cache
  footprint on the claim lane
- `scripts/benchmark_claim_lane.py` now provides a repeatable stored-evidence
  benchmark harness for claim runs, recording median/p90/max runtime, parity
  success counts, breadth-floor hit counts, and manifest snapshots across a
  chosen benchmark bundle
- the lane is still not certification-ready because the intended full-profile
  run is not yet stable enough to produce a stored step-15 claim bundle

## Current Read

- The main blocker is still full-profile memory stability, but the live memory
  work is now in the code path itself rather than only in planning docs.
- The intended `desktop_claim_shadow_1h` auto-worker profile still aborts on
  the disclosed machine before step-15 completion with
  `memory allocation of 1212416 bytes failed`.
- Failed runs now keep a readable manifest, latest completed step,
  step-specific artifacts, and late frontier state, so the next pressure or
  allocator failure can be debugged from stored evidence instead of terminal
  output alone.
- Claim runs now also expose observed-versus-accounted RSS gap data and keep a
  smaller proof-close cache, but there is still no stored full-profile bundle
  proving that those changes are sufficient on the disclosed machine.
- Manifest completeness and failed-run survivability are no longer the gating
  issue; rerunning the intended profile with the new memory controls is.
- The benchmark harness now exists, but it still needs a real full-profile
  claim bundle before the repo can freeze runtime thresholds or cite
  benchmark/floor success from the disclosed desktop honestly.
- Breadth, parity, fallback honesty, and certification remain open, but they
  cannot move from tests into stored evidence until the full-profile claim run
  completes reliably enough to leave a bundle.

## Immediate Next Slice

1. Rerun the intended `desktop_claim_shadow_1h` profile on the disclosed
   machine and inspect the stored observed-versus-accounted RSS gap.
2. If the run still fails, use that stored gap plus the latest completed step
   to identify the remaining untracked allocation site honestly.
3. Once the run finishes, compare it against guarded from stored artifacts and
   feed the resulting bundle through `scripts/benchmark_claim_lane.py` plus
   `scripts/certify_claim_lane.py` to close the remaining parity, breadth,
   benchmark, and certification gaps.

## After That

- re-earn honest breadth and parity from stored claim-lane evidence
- freeze a benchmark/runtime threshold from the intended full profile
- produce one canonical compare report plus a passing certificate

## Verification Baseline

- `cargo test -p pen-type --lib`
- `cargo test -p pen-search --lib`
- `cargo test -p pen-store --lib manifest::tests::run_manifest_round_trip_preserves_frozen_keys`
- `cargo test -p pen-cli claim_run_writes_policy_metadata_and_claim_narrative`
- `cargo test -p pen-cli --bin pen-cli cmd_run::tests::failed_partial_claim_run_still_leaves_manifest_and_narrative_artifacts -- --exact`
- `cargo test -p pen-cli --bin pen-cli cmd_run::tests::failed_partial_late_run_still_leaves_frontier_snapshot -- --exact`
- `cargo test -p pen-search --lib claim_lane_drops_cached_terminal_payloads_after_ranking`
- `cargo test -p pen-cli --bin pen-cli cmd_run::tests::`
- `cargo test -p pen-cli --test deterministic_replay repeated_runs_with_the_same_inputs_are_deterministic -- --exact`
- `cargo test -p pen-cli --test resume_roundtrip resume_roundtrip_keeps_the_reference_sequence_and_inspect_output -- --exact`
- `cargo test -p pen-cli --test atomic_bootstrap compare_runs_reports_claim_lane_policy_and_reason_audit`
- `cargo test -p pen-cli --test atomic_bootstrap claim_certification_script_emits_failing_certificate_for_incomplete_smoke_run`
- `cargo test -p pen-cli --test deterministic_replay compare_runs_script_emits_a_deterministic_evidence_signoff`

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` behavior unchanged while the claim lane evolves.
- Treat completed claim-policy split work as baseline, not as the active
  bottleneck.
- Do not claim autonomy from labels alone.
- Do not mark the lane `unguided` before the certification gate exists and
  passes.
