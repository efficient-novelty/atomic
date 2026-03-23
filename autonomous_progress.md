# Autonomous Claim Lane Progress

Last updated: 2026-03-23

This file is the short operational read on `desktop_claim_shadow`. Use
[autonomous_plan.md](autonomous_plan.md) for the full remaining sequence and
[autonomous_checklist.md](autonomous_checklist.md) for the live signoff gate.

## Current Status

- `desktop_claim_shadow` is still not certification-ready.
- The current blocker is still full-profile completion on
  `configs/desktop_claim_shadow_1h.toml`.
- The latest stored intended-profile evidence is now
  `codex-claim-release-full-delayed-summary-v1`: it used the clean release
  binary at commit `00bc5b3`, did not re-hit the old allocator cliff, and was
  manually stopped after enough step-`4` evidence at
  `prefix_states_explored = 59`.
- The delayed-materialization patch is now kept on real full-profile evidence,
  not only on the earlier `until_step = 4` probe: the new stored full rerun
  pushed materially farther through the same hot step-`4` surface while
  keeping the frontier and retained-prefix shape honest.

## Current Read

- Delayed materialization is now clearly a real full-profile win on step `4`,
  not just an early `until_step = 4` probe win.
- Against the prior intended-profile baseline
  (`codex-claim-release-full-v1a`), the new stored full rerun
  (`codex-claim-release-full-delayed-summary-v1`) improved matching step-`4`
  checkpoints like this while keeping the same frontier and retained-prefix
  shape:
  - at `prefix_states_explored = 24`, elapsed time dropped to `1854.5s` from
    `2000.8s`, and observed RSS dropped to about `218.7 MiB` from about
    `250.0 MiB`
  - at `prefix_states_explored = 39`, elapsed time dropped to `3010.8s` from
    `3440.6s`, and observed RSS dropped to about `237.8 MiB` from about
    `275.0 MiB`
  - at `prefix_states_explored = 43`, elapsed time dropped to `3309.4s` from
    `3844.7s`, while frontier queue length, legality summaries, and retained
    prefix-cache shape stayed identical at `2732`, `205199`, and `39 / 144845`
- At about the same wall clock as the old run's last stored checkpoint
  (`3936.1s`), the new rerun had already reached `prefix_states_explored = 52`
  instead of `44`, drained the frontier to `2723` instead of `2731`, and kept
  observed RSS to about `252.7 MiB` instead of about `268.0 MiB`.
- At the manual stop checkpoint on the new rerun
  (`prefix_states_explored = 59`, `4529.4s`):
  - frontier queue length had moved down to `2716`
  - legality summaries had risen to `279487`
  - retained prefix-cache groups and candidates were still flat at
    `39 / 144845`
  - `remaining_one_materialized` was still only `39`
  - pre-materialization rank prunes had risen to `273957`
  - post-materialization rank prunes stayed at `0`
  - `terminal_summary_build_millis` had risen to `4387822 ms`
  - `terminal_materialize_millis` was still only `527 ms`
  - cached bound hits still stayed at `0`
- That means the hot full-profile step-`4` question is no longer whether
  delayed materialization works. It does. The dominant cost has now moved into
  building the compact remaining-one pruning summaries themselves while the
  retained-prefix shape stays flat, so the next honest fix should target
  cheaper terminal-summary construction or a still-earlier incumbent-based kill
  before summary build, not memory compaction or later-step pressure first.

## Immediate Next Slice

1. Keep the delayed-materialization patch and the new stored full-profile
   evidence; do not reopen the rejected partial split-handoff path or the old
   materialization-first diagnosis.
2. Patch one narrow remaining-one terminal-summary or earlier-incumbent path
   that can reduce `terminal_summary_build_millis` before full compact
   materialization is even considered.
3. Re-earn one stored release `until_step = 4` rerun on that patch first and
   compare its step-`4` telemetry against
   `codex-claim-release-full-delayed-summary-v1` and
   `codex-claim-release-step4-delayed-summary-v1`.
4. Only after the updated `until_step = 4` evidence shows a real summary-side
   win should the real `desktop_claim_shadow_1h` profile, compare,
   benchmark, and certification work move again.

## Evidence Snapshot

- New stored full-profile rerun on the kept delayed-summary patch:
  `runs/codex-claim-release-full-delayed-summary-v1`
- Previous intended-profile baseline:
  `runs/codex-claim-release-full-v1a`
- Earlier decisive `until_step = 4` rerun on the same kept patch:
  `runs/codex-claim-release-step4-delayed-summary-v1`
- Matching pre-delay step-`4` baseline:
  `runs/codex-claim-release-step4-telemetry-v1`
- The new full-profile rerun was manually stopped after enough step-`4`
  evidence at `prefix_states_explored = 59`, so its `run.json` still remains
  in `running` state; the useful artifact for this slice is the persisted
  `reports/steps/step-04-live.ndjson` stream.
- No additional regression tests ran this turn because this slice was a stored
  release rerun and artifact read on already-committed search code.

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` unchanged while the claim lane evolves.
- Do not reopen already-landed claim-policy split work unless the new stored
  evidence forces it.
- Do not treat labels alone as evidence of autonomy.
- Do not use `unguided` before the certification gate exists and passes.
