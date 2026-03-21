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
- the lane is still not certification-ready because the intended full-profile
  run is not yet stable enough to produce a stored step-15 claim bundle

## Current Read

- The main blocker is now full-profile memory stability, not claim-policy
  separation or failed-run evidence preservation.
- The intended `desktop_claim_shadow_1h` auto-worker profile still aborts on
  the disclosed machine before step-15 completion with
  `memory allocation of 1212416 bytes failed`.
- Failed runs now keep a readable manifest, latest completed step,
  step-specific artifacts, and late frontier state, so the next pressure or
  allocator failure can be debugged from stored evidence instead of terminal
  output alone.
- Manifest completeness and failed-run survivability are no longer the gating
  issue; memory-aware execution on the intended auto-worker profile is.
- Breadth, parity, fallback honesty, and certification remain open, but they
  cannot move from tests into stored evidence until the full-profile claim run
  completes reliably enough to leave a bundle.

## Immediate Next Slice

1. Make worker resolution and memory-pressure handling claim-aware enough for
   the intended `desktop_claim_shadow_1h` profile to finish on the disclosed
   machine.
2. Rerun the full claim profile and compare it against guarded from stored
   artifacts.
3. Use that bundle to close the remaining parity, breadth, benchmark, and
   certification gaps.

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
