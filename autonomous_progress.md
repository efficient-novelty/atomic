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
- This turn earned the first stored `until_step = 4` rerun on the newer
  telemetry binary (`codex-claim-release-step4-telemetry-v1`) and one follow-up
  stored rerun on a narrow partial exact-two split patch
  (`codex-claim-release-step4-split-handoff-v1`).
- The split-handoff patch was not kept: the verification rerun did not beat the
  baseline checkpoints and its exact-bound timer stayed at `0`, so the patch
  never activated on the hot early step-`4` surface and was reverted.

## Current Read

- The old queue-startup RSS cliff is still not the main problem.
- The active problem is still step-`4` exact remaining-two throughput, but the
  new telemetry now narrows the hot cost much more sharply than the older
  full-profile artifact did.
- `codex-claim-release-step4-telemetry-v1` showed that the hot early step-`4`
  surface is still dominated by direct compact remaining-one materialization
  that mostly dies only after materialization:
  - at `prefix_states_explored = 5`, elapsed time was `423.8s`
  - the frontier queue was still `2770`
  - legality summaries had risen to `28765`
  - remaining-one groups materialized had reached `23220`
  - post-materialization rank prunes had reached `23205`
  - cached bound hits were still `0`
  - exact bound screening time was still `0 ms`
  - terminal summary build time was still `0 ms`
  - frontier pop/sort time was still `0 ms`
  - terminal materialization time had already reached `393661 ms`
- That means the lane is still paying heavily for payload that does not survive
  long enough to justify full compact materialization, and the current hot
  surface is not yet spending meaningful time in cached bound reuse, exact
  screening, or frontier drain.
- The narrow partial exact-two split handoff probe did not earn keep:
  - `codex-claim-release-step4-split-handoff-v1` reached the same early
    checkpoints slightly later than the baseline (`427.3s` at
    `prefix_states_explored = 5` versus `423.8s`)
  - it preserved the same materialization and post-prune counts at those
    checkpoints
  - its exact-bound timer still stayed at `0 ms` through
    `prefix_states_explored = 6`
  - so the split branch did not actually engage on the current hot path
- The next honest code path is therefore no longer partial exact-two handoff
  first. The next slice should move toward claim discovery summary /
  delayed materialization work.

## Immediate Next Slice

1. Keep the code at the reverted pre-split baseline; do not stack the rejected
   partial-handoff patch back on top.
2. Implement one narrow discovery-summary / delayed-materialization path for
   claim remaining-one processing so discovery can store a compact pruning
   summary before paying full compact materialization cost.
3. Rerun the same stored release `until_step = 4` profile on that narrower
   patch and inspect `reports/steps/step-04-live.ndjson` again.
4. Keep that next patch only if the stored step-`4` artifacts show at least one
   real win:
   - fewer `remaining_one_materialized`
   - materially lower `terminal_materialize_millis`
   - materially earlier matching checkpoints
   - or a clearer shift from post-materialization death toward earlier pruning
5. Only rerun the real `desktop_claim_shadow_1h` profile after the stored
   `until_step = 4` evidence improves on the same binary.

## Evidence Snapshot

- New stored rerun on the newer telemetry binary:
  `runs/codex-claim-release-step4-telemetry-v1`
- New stored rerun on the rejected partial split probe:
  `runs/codex-claim-release-step4-split-handoff-v1`
- Both runs were manually stopped after enough step-`4` evidence was written,
  so their `run.json` files remain in `running` state; the useful artifacts for
  this slice are the persisted `reports/steps/step-04-live.ndjson` streams.

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` unchanged while the claim lane evolves.
- Do not reopen already-landed claim-policy split work unless the new stored
  evidence forces it.
- Do not treat labels alone as evidence of autonomy.
- Do not use `unguided` before the certification gate exists and passes.
