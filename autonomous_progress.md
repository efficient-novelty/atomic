# Autonomous Claim Lane Progress

Last updated: 2026-03-21

This file tracks the live operational state of `desktop_claim_shadow`. It is
intentionally short and forward-looking; use
[autonomous_plan.md](autonomous_plan.md) for the remaining work sequence.

## Snapshot

- `desktop_claim_shadow` is a distinct profile with its own configs,
  narratives, and run metadata
- claim admissibility is now structurally guided:
  - `guidance_style = claim_debt_guided`
  - claim mode does not use a named `focus_family`
- claim late expansion is now claim-specific:
  - `late_expansion_policy = claim_generic`
- claim bucket scheduling is now claim-specific:
  - `bucket_policy = structural_generic`
- the lane is still mixed-mode at the evidence/certification layer:
  - `kappa 7-9` now have claim-specific later-band mutator packs
  - `scripts/compare_runs.py` now audits claim-policy honesty, exact-screen
    reason coverage, prune-class coverage, fallback evidence, and step-15
    claim-run coverage
  - `scripts/certify_claim_lane.py` now emits a stored pass/fail certificate
    from claim artifacts
  - `run.json` now records CPU, worker-count, build-profile, target, git,
    `Cargo.lock`, and binary fingerprints for claim certification
  - stored breadth/floor evidence and certification pass status are still open
- the lane is not yet certification-ready and it is still not honest to call
  it `unguided`

## Current Read

- The main blocker is no longer admissibility tuning or inherited late-surface
  routing.
- Claim mode now has its own `ClaimGeneric` late surface in
  `crates/pen-search/src/enumerate.rs` and
  `crates/pen-search/src/prefix_memo.rs`.
- Claim-generic mutator packs are now real for kappa `4-9`, with the new
  kappa `9` widening still preserving guarded acceptance through the existing
  claim-lane tests.
- The scheduler taxonomy blocker is now closed in code and policy metadata:
  claim runs use structural-generic bucket keys instead of semantic-family
  labels.
- Claim-path terminal-prefix completion and cached exact-bound reuse are now
  directly rechecked by tests under the structural-generic claim scheduler
  surface.
- The next blockers are stored late-step breadth/floor evidence, stored parity
  signoff evidence on the claim lane itself, and the still-incomplete
  certification surfaces.
- Reporting now pushes exact-screen reasons into telemetry and the compare
  script, and the certification script can already fail honestly on missing
  parity, breadth, runtime, or manifest evidence.
- Manifest completeness is now closed in code/tests, including the smoke
  certification path.
- The remaining blockers are no longer missing provenance/build fields in
  `run.json`; they are the missing step-15 claim bundle, the still-open
  breadth floors, and runtime stability on the intended
  `desktop_claim_shadow_1h` auto-worker profile, which currently aborts before
  artifact flush with `memory allocation of 1212416 bytes failed`.

## Immediate Next Slice

1. Stabilize a stored step-15 claim bundle on the disclosed machine under the
   intended `desktop_claim_shadow_1h` profile; the latest auto-worker attempt
   still aborted before writing `run.json`.
2. Once that bundle exists, run the compare/certification scripts on the real
   evidence and record the remaining parity/breadth failures.
3. Then close the remaining breadth/floor misses and benchmark the certified
   runtime threshold on the claim lane itself.

## After That

- earn honest breadth and floor evidence under the claim-specific mutator path
- make retuning change search shape, not just time allocation
- harden manifest, benchmarking, compare output, and certification

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
- Do not switch policy metadata ahead of the real behavior.
- Do not claim autonomy from labels alone.
- Do not mark the lane `unguided` before the certification gate exists and
  passes.
