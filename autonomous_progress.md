# Autonomous Claim Lane Progress

Last updated: 2026-03-23

This file is the short operational read on `desktop_claim_shadow`. Use
[autonomous_plan.md](autonomous_plan.md) for the full remaining sequence and
[autonomous_checklist.md](autonomous_checklist.md) for the live signoff gate.

## Current Status

- `desktop_claim_shadow` is still not certification-ready.
- The current blocker is still full-profile completion on
  `configs/desktop_claim_shadow_1h.toml`.
- The latest stored intended-profile evidence is still
  `codex-claim-release-full-v1a`: it did not re-hit the old early allocator
  cliff, but it still timed out in step `4` after `prefix_states_explored = 43`
  with the frontier queue still broad and legality summaries still rising.
- The most recent code slice is now landed, but it has not yet been followed
  by a new stored rerun on the updated binary.

## Current Read

- The old queue-startup RSS cliff is no longer the main problem.
- The active problem is still step-`4` exact remaining-two throughput and
  frontier drainage on the intended claim lane.
- The strongest stored signal remains the retained-prefix plateau from
  `codex-claim-release-full-v1a`:
  - retained prefix groups flattened after `prefix_states_explored = 24`
  - legality summaries kept climbing anyway
  - much of the later step-`4` wall time was therefore still exact terminal
    completion on surfaces that were no longer changing the retained frontier
- The code now has the instrumentation needed to stop guessing about the next
  bottleneck, but there is not yet a fresh artifact bundle showing which cost
  center now dominates on the updated binary.

## Landed For The Next Rerun

- Claim step-live checkpoints now carry remaining-one telemetry for:
  - cached bound hits
  - cached rank pre-prunes
  - materialization count and source
  - pre/post-materialization prune counts
  - exact-two preparation, exact bound screening, materialization,
    candidate sort/minimality, and frontier pop timing
- Claim cached bound checks now use a bound-only legality-cache accessor
  instead of cloning the full cached terminal completion payload just to read
  `bound`.
- Claim discovery now applies a cached-rank pre-prune before materialization
  when the cached best remaining-one rank is already incumbent-dominated, and
  it consumes the cached summary for honest generated/admissibility accounting
  before releasing that payload.

## Immediate Next Slice

1. Run one stored release A/B on an `until_step = 4` claim config built from
   the current code.
2. Inspect `reports/steps/step-04-live.ndjson` and answer these questions
   before making another search rewrite:
   - Are cached bound hits still rare, or are summary misses no longer the main
     story?
   - Are rank prunes mostly happening before or after materialization?
   - Is wall time now dominated by terminal materialization, terminal summary
     building, or frontier pop/sort cost?
   - Are bound-unknown budget exhaustions still large enough to justify broader
     exact screening work?
3. If step `4` still stalls after that rerun, use the telemetry to choose one
   next code path:
   - partial exact-two in-place processing instead of the current all-or-none
     handoff
   - claim discovery-summary / delayed-materialization work
   - frontier queue data-structure work only if the new pop/sort timer is
     genuinely large
4. Once the `until_step = 4` read is clear enough, rerun the real
   `desktop_claim_shadow_1h` profile on the same updated binary.
5. Only move on to compare, benchmark, and certification after one stored
   full-profile bundle exists on that updated code path.

## Verification Snapshot

- The current claim regression slice passed on the landed code:
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-search online_work_items_`
  - `cargo test -p pen-search prefix_queue_prefers_nearer_terminal_and_tighter_cached_continuations`
  - `cargo test -p pen-search terminal_clause_filter_skips_inadmissible_last_clauses_before_connectivity`
  - `cargo test -p pen-search terminal_prefix_bound_summary_reads_cached_bound_without_cloning_payload`
  - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`
- The broader tree is still not fully green:
  - `cargo test -p pen-search --lib` still stops at
    `engine::tests::demo_late_surface_carries_through_live_config_runs`

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` unchanged while the claim lane evolves.
- Do not reopen already-landed claim-policy split work unless the new stored
  evidence forces it.
- Do not treat labels alone as evidence of autonomy.
- Do not use `unguided` before the certification gate exists and passes.
