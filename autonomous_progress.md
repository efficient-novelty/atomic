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
- This turn landed one narrow claim remaining-one discovery-summary /
  delayed-materialization patch and re-earned one stored release
  `until_step = 4` rerun on that patch
  (`codex-claim-release-step4-delayed-summary-v1`).
- The delayed-materialization patch is kept: the stored rerun materially cut
  early step-`4` materialization work and reached the same hot checkpoint
  earlier, so it earned keep under the current step-`4` rule.

## Current Read

- Delayed materialization is now clearly a real win on the hot early step-`4`
  surface.
- At the matching `prefix_states_explored = 5` checkpoint, the new stored rerun
  (`codex-claim-release-step4-delayed-summary-v1`) improved on
  `codex-claim-release-step4-telemetry-v1` like this:
  - elapsed time dropped to `367.4s` from `423.8s`
  - `remaining_one_materialized` dropped to `15` from `23220`
  - post-materialization rank prunes dropped to `0` from `23205`
  - pre-materialization rank prunes rose to `23205` from `0`
  - `terminal_materialize_millis` dropped to `107 ms` from `393661 ms`
  - `terminal_summary_build_millis` rose to `354939 ms` from `0 ms`
  - frontier queue length stayed at `2770`
  - legality summaries stayed at `28765`
  - retained prefix-cache groups stayed at `15`
  - retained prefix-cache candidates stayed at `38108`
  - observed RSS dropped to about `74.7 MiB` from about `83.5 MiB`
- `remaining_one_cached_bound_hits` still stayed at `0`, but
  `remaining_one_cached_rank_prunes` rose to `23205`, so the current win is
  not cached bound reuse first. The lane is now storing compact pruning
  summaries and consuming them later to kill almost all of the old dead
  remaining-one groups before full compact materialization.
- Because the retained-prefix/frontier shape stayed flat at the matching
  checkpoint while the hot timer moved from materialization into summary build,
  this looks like a real throughput win rather than a weakened search lane.
- The hot step-`4` question is therefore no longer whether delayed
  materialization works. The next honest read now needs the real
  `desktop_claim_shadow_1h` rerun on this same binary before more search-code
  changes stack on top.

## Immediate Next Slice

1. Keep the delayed-materialization patch; do not reopen the rejected partial
   split-handoff path.
2. Rerun the real `desktop_claim_shadow_1h` profile on the same release binary
   and inspect the stored artifacts rather than terminal output.
3. If that rerun still stalls in step `4`, use the stored artifacts to decide
   whether the next narrow fix belongs in cheaper terminal-summary
   construction, earlier incumbent pruning, or some newly exposed later-step
   pressure story.
4. Only after the updated full-profile bundle is read should compare,
   benchmark, and certification work move again.

## Evidence Snapshot

- New stored rerun on the kept delayed-summary patch:
  `runs/codex-claim-release-step4-delayed-summary-v1`
- Matching baseline for comparison:
  `runs/codex-claim-release-step4-telemetry-v1`
- The new rerun was manually stopped after enough step-`4` evidence at
  `prefix_states_explored = 5`, so its `run.json` still remains in `running`
  state; the useful artifact for this slice is the persisted
  `reports/steps/step-04-live.ndjson` stream.
- Regression set run on the patch:
  `cargo test -p pen-search claim_`
  `cargo test -p pen-search online_work_items_`
  `cargo test -p pen-search prefix_queue_prefers_nearer_terminal_and_tighter_cached_continuations`
  `cargo test -p pen-search terminal_clause_filter_skips_inadmissible_last_clauses_before_connectivity`

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` unchanged while the claim lane evolves.
- Do not reopen already-landed claim-policy split work unless the new stored
  evidence forces it.
- Do not treat labels alone as evidence of autonomy.
- Do not use `unguided` before the certification gate exists and passes.
