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
- the lane is still not certification-ready because the intended full-profile
  run is not yet stable enough to produce a stored step-15 claim bundle

## Current Read

- The main blocker is now runtime survivability, not claim-policy separation.
- The intended `desktop_claim_shadow_1h` auto-worker profile still aborts on
  the disclosed machine before artifact flush with
  `memory allocation of 1212416 bytes failed`.
- Because the CLI still writes `run.json` and step artifacts only after the
  full run returns, that failure currently destroys the evidence we most need
  to debug it.
- Manifest completeness is no longer the gating issue; failed-run evidence
  preservation and memory-aware execution are.
- Breadth, parity, fallback honesty, and certification remain open, but they
  cannot move from tests into stored evidence until the full-profile claim run
  completes reliably enough to leave a bundle.

## Immediate Next Slice

1. Persist `run.json` and step artifacts incrementally so failed long claim
   runs are still auditable.
2. Make worker resolution and memory-pressure handling claim-aware enough for
   the intended `desktop_claim_shadow_1h` profile to finish on the disclosed
   machine.
3. Then rerun the full claim profile, compare it against guarded, and use the
   resulting bundle to close the remaining parity/breadth/certification gaps.

## After That

- re-earn honest breadth and parity from stored claim-lane evidence
- freeze a benchmark/runtime threshold from the intended full profile
- produce one canonical compare report plus a passing certificate

## Verification Baseline

- `cargo test -p pen-type --lib`
- `cargo test -p pen-search --lib`
- `cargo test -p pen-store --lib manifest::tests::run_manifest_round_trip_preserves_frozen_keys`
- `cargo test -p pen-cli claim_run_writes_policy_metadata_and_claim_narrative`
- `cargo test -p pen-cli --test atomic_bootstrap compare_runs_reports_claim_lane_policy_and_reason_audit`
- `cargo test -p pen-cli --test atomic_bootstrap claim_certification_script_emits_failing_certificate_for_incomplete_smoke_run`

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` behavior unchanged while the claim lane evolves.
- Treat completed claim-policy split work as baseline, not as the active
  bottleneck.
- Do not claim autonomy from labels alone.
- Do not mark the lane `unguided` before the certification gate exists and
  passes.
