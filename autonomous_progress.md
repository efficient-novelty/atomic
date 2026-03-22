# Autonomous Claim Lane Progress

Last updated: 2026-03-22

This file is the short operational read on `desktop_claim_shadow`. Use
[autonomous_plan.md](autonomous_plan.md) for the remaining sequence and
[autonomous_checklist.md](autonomous_checklist.md) for the live signoff gate.

## Current Status

- `desktop_claim_shadow` is still not certification-ready.
- The current blocker is full-profile completion on the intended
  `desktop_claim_shadow_1h` auto-worker profile.
- The latest known full-profile failure is still
  `memory allocation of 1212416 bytes failed`.
- Failure bundles are now auditable from disk, so the next work is about
  finishing the run, not preserving evidence.

## Relevant Baseline

- Claim metadata and reporting surfaces are already honest enough to audit:
  - `guidance_style = claim_debt_guided`
  - `late_expansion_policy = claim_generic`
  - `bucket_policy = structural_generic`
- Claim runs already persist:
  - `run.json`
  - step summaries and narratives
  - frontier snapshots
  - observed RSS alongside governor-accounted RSS
  - `step_live_checkpoint` streams for steps `4` and `5`
- Compare, benchmark, and certification scripts already exist; the missing
  ingredient is one stable full-profile bundle to feed them.

## Latest Evidence That Matters

- `codex-claim-shared-signature-v1` showed that sharing cloned signature
  payloads was real but not enough:
  - comparable early step-`4` checkpoint still about `3.06 GiB`
- `codex-claim-frontier-catalog-reuse-v1` changed the picture materially:
  - the old step-`4` startup cliff no longer appears in stored evidence
  - the first stored step-`4` frontier-progress checkpoint is about `66.4 MiB`
    after `422.9s`
  - recorded counts there were:
    - frontier groups: `2774`
    - legality summaries: `10193`
    - partial-prefix-bound entries: `5084`
    - retained prefix-cache groups: `13`
- `codex-claim-release-step5-v1` then showed that the optimized claim binary
  no longer has an early memory emergency on step `4`:
  - after `1777.1s` it was still only about `167.1 MiB` observed RSS
  - by then it had enumerated `310916028` candidates while exploring `16`
    prefix states
  - the practical read is that the early release build is now runtime-bound in
    exact remaining-two discovery rather than RSS-bound
- The new claim fast path now streams compact terminal materialization when no
  cached completion summary exists and reuses the shared serialized prefix key
  for work-item ordering instead of allocating a duplicate copy.
- `codex-claim-release-step4-fastpath-v2` shows the new release binary is
  meaningfully faster on the same hot step-`4` path:
  - `prefix_states_explored = 5` landed at `515.6s` versus `600.3s`
  - `prefix_states_explored = 7` landed at `701.1s` versus `804.2s`
  - observed RSS stayed below about `89.6 MiB` through that partial rerun

## Current Read

- The queue-side step-`4` startup cliff is no longer the main bottleneck.
- The optimized claim lane is now visibly bottlenecked by step-`4`
  exact-remaining-two throughput before it re-exposes any early RSS crisis.
- The next decision should come from another optimized rerun after each narrow
  throughput fix, not from reopening already-landed claim memory compaction in
  isolation.
- Breadth, parity, benchmark, and certification remain blocked on that full
  rerun finishing and leaving a stable stored bundle.

## Immediate Next Slice

1. Keep tightening the release-build step-`4` exact remaining-two claim path
   until the intended profile moves materially farther within the same budget.
2. Rerun `desktop_claim_shadow_1h` on the optimized binary and inspect the
   stored RSS-gap and step-live evidence to see how far the full profile gets.
3. If it completes, run compare, benchmark, and certification on that bundle.
4. If it still fails later, use the stored artifacts to choose the next narrow
   late-step memory or throughput fix.

## Verification Notes

- Recent targeted claim-memory regressions passed:
  - `cargo test -p pen-search online_work_items_cache_exact_filtered_next_clauses`
  - `cargo test -p pen-search online_work_items_reuse_full_catalog_when_no_filter_applies`
  - `cargo test -p pen-search claim_materialization_consumes_cached_terminal_completion_summary`
- Recent targeted claim fast-path regressions also passed:
  - `cargo test -p pen-search claim_compact_materialization_matches_summary_expansion_without_cache`
  - `cargo test -p pen-search prefix_queue_prefers_nearer_terminal_and_tighter_cached_continuations`
- The broader tree is still not fully green:
  - `cargo test -p pen-search --lib` still stops at
    `engine::tests::demo_late_surface_carries_through_live_config_runs`

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` unchanged while the claim lane evolves.
- Do not treat labels alone as evidence of autonomy.
- Do not use `unguided` before the certification gate exists and passes.
