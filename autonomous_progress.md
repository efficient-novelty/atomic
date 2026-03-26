# Autonomous Claim Lane Progress

Last updated: 2026-03-26

This file is the live status read for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next
execution slice, [autonomous_plan.md](autonomous_plan.md) for the staged path
to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the
final gate.

## Current Status

- `desktop_claim_shadow` is not signoff-ready.
- Delayed materialization, the narrow remaining-one incumbent-primary fast
  path, the one-pass `structural_nu` summary-build fast path, and the
  algebraic `nu` ceiling patch are all landed in code.
- A narrow remaining-one context-equivalence quotient was implemented,
  validated locally, rerun on the stored short step-`4` profile, and then
  dropped from code after the stored evidence failed to earn keep.
- A narrow step-`4` frontier-pop incumbent-arrival ordering experiment was
  also implemented, validated locally, rerun on the stored short step-`4`
  profile, and then dropped from code after the stored evidence failed to earn
  keep.
- A narrow claim-only exact-two-step local ordering pass was also implemented,
  validated locally, rerun on the stored short step-`4` profile, and then
  dropped from code after the stored evidence showed immediate activation but
  no real step-`4` win.
- A narrow claim-only proof-close handoff ordering pass was then implemented,
  validated locally, rerun on the stored short step-`4` profile, and then
  dropped from code after the stored evidence showed that the new proof-close
  telemetry never activated on the live claim lane.
- A narrow claim-only post-plateau discovery summary-skip pass was then
  implemented, validated locally, rerun on the stored short step-`4`
  profile, and then dropped from code after the stored evidence showed that it
  only moved the hot cost from summary build into direct compact
  materialization while making the wall clock materially worse.
- A follow-up narrow post-plateau materialize-side fast path was then
  implemented, validated locally, rerun on the stored short step-`4`
  profile, and then dropped from code after the stored evidence showed that
  the intended direct-compact surface never actually engaged on the live claim
  lane.
- A new stored short rerun now exists for that failed slice:
  `runs/codex-claim-release-step4-context-equivalence-v1`.
- A second new stored short rerun now exists for the later failed slice:
  `runs/codex-claim-release-step4-incumbent-ordering-v1`.
- A third new stored short rerun now exists for that dropped exact-two-step
  local ordering slice:
  `runs/codex-claim-release-step4-local-two-step-order-v2`.
- A fourth new stored short rerun now exists for that later dropped
  proof-close handoff slice:
  `runs/codex-claim-release-step4-proof-close-handoff-v1`.
- A fifth new stored short rerun now exists for that later dropped
  post-plateau summary-skip slice:
  `runs/codex-claim-release-step4-post-plateau-v1`.
- A sixth new stored short rerun now exists for that later dropped
  post-plateau materialize-side slice:
  `runs/codex-claim-release-step4-post-plateau-materialize-v1`.
- The current short step-`4` iteration baseline remains
  `runs/codex-claim-release-step4-algebraic-v1`; neither the later failed
  reruns, the dropped exact-two-step local ordering rerun, nor the dropped
  proof-close handoff rerun, nor the dropped post-plateau summary-skip rerun,
  nor the dropped post-plateau materialize-side rerun have earned
  replacement.
- The current full-profile iteration baseline remains
  `runs/codex-claim-release-full-nu-profile-v1`.
- An earlier attempt at the same new run id family,
  `runs/codex-claim-release-step4-local-two-step-order-v1`, should be ignored:
  it was started before a forced fresh `--release` rebuild and still emitted
  the dropped stale-binary telemetry shape, so it is not part of the active
  comparison set.
- The claim lane is still compute-bound in step `4`.
- The attempted quotient counters recorded no live reuse on the stored short
  rerun: hits stayed `0`, reused summaries stayed `0`, and misses froze at
  `2775` from the first frontier-progress checkpoint through the manual stop.
- The later frontier-order counters also recorded no live activation on the
  stored short rerun: `remaining_one_prefixes_seen` stayed `0`,
  `remaining_one_incumbent_priority_promotions` stayed `0`,
  `remaining_one_incumbent_improving_groups_seen` stayed `0`, and
  `remaining_one_incumbent_improvement_prefix_state` stayed `0` through the
  manual stop.
- The later post-plateau materialize-side counters likewise showed that the
  intended direct-compact surface still never engaged on the stored short
  rerun: `post_plateau_materialize_fast_path_kept = 0` and
  `post_plateau_materialized_compact_direct = 0` at the stored `1`, `5`, and
  manual-stop `9` checkpoints, while `post_plateau_materialize_fast_path_skipped`
  only rose because the gate kept declining to open that surface.
- The broad `cargo test -p pen-search claim_` sweep was re-earned again on
  this desktop in this turn after the old heavy witness shape was removed.

## Latest Evidence

- Current short baseline:
  `runs/codex-claim-release-step4-algebraic-v1`
- Against `runs/codex-claim-release-step4-nu-profile-v1`, that current short
  baseline kept the same honest retained-prefix shape and legality growth but
  reached the matching hot step-`4` checkpoints materially earlier:
  - `prefix_states_explored = 43` at `1629.3s` instead of `1728.7s`
  - `prefix_states_explored = 52` at `1975.0s` instead of `2090.3s`
  - `prefix_states_explored = 59` at `2252.6s` instead of `2360.5s`
  - At those same checkpoints, retained prefix cache stayed
    `39 groups / 144845 candidates` and legality summaries stayed
    `205199`, `246986`, and `279487` respectively.
  - At those same checkpoints, `terminal_summary_build_millis` fell to
    `1524266`, `1849248`, and `2111246` instead of
    `1622973`, `1963028`, and `2215871`.
- New stored short rerun that did not advance the baseline:
  `runs/codex-claim-release-step4-context-equivalence-v1`
- Against `runs/codex-claim-release-step4-algebraic-v1`, the new
  context-equivalence rerun kept the same honest queue, legality, and retained
  prefix-cache shape at the comparable early checkpoints, but it did not
  improve them:
  - `prefix_states_explored = 1` at `37.4s` instead of `35.7s`
  - `prefix_states_explored = 5` at `204.7s` instead of `198.5s`
  - `prefix_states_explored = 10` at `389.8s` instead of `380.4s`
  - At those same checkpoints, frontier queue length stayed
    `2774`, `2770`, and `2765` respectively.
  - At those same checkpoints, legality summaries stayed
    `10193`, `28765`, and `51980` respectively.
  - At those same checkpoints, retained prefix cache stayed
    `13/32520`, `15/38108`, and `19/53693` respectively.
  - At those same checkpoints, `terminal_summary_build_millis` rose to
    `34532`, `191121`, and `362946` instead of
    `32990`, `186848`, and `355887`.
  - `remaining_one_context_equivalence_hits = 0`
  - `remaining_one_context_equivalence_reused_summaries = 0`
  - `remaining_one_context_equivalence_misses = 2775` from the first stored
    frontier-progress checkpoint through the manual stop.
- That rerun was then manually stopped after enough stored evidence:
  - `prefix_states_explored = 13` at `507.6s`
  - frontier queue length `= 2762`
  - legality summaries `= 65909`
  - retained prefix cache `= 19 groups / 53693 candidates`
  - `remaining_one_context_equivalence_hits = 0`
  - `remaining_one_context_equivalence_reused_summaries = 0`
  - `remaining_one_context_equivalence_misses = 2775`
  - `terminal_summary_build_millis = 472868`
  - observed RSS `~ 112 MiB`
- New stored short rerun that also did not advance the baseline:
  `runs/codex-claim-release-step4-incumbent-ordering-v1`
- Against `runs/codex-claim-release-step4-algebraic-v1`, the new
  frontier-order rerun kept the same honest queue, legality, retained-prefix
  shape, and cached rank-prune counts at the comparable checkpoints, but it
  was slower at each one:
  - `prefix_states_explored = 1` at `39.3s` instead of `35.7s`
  - `prefix_states_explored = 5` at `223.5s` instead of `198.5s`
  - `prefix_states_explored = 10` at `427.3s` instead of `380.4s`
  - `prefix_states_explored = 43` at `1835.9s` instead of `1629.3s`
  - `prefix_states_explored = 52` at `2224.0s` instead of `1975.0s`
  - `prefix_states_explored = 59` at `2519.5s` instead of `2252.6s`
  - At those same checkpoints, frontier queue length stayed
    `2774`, `2770`, `2765`, `2732`, `2723`, and `2716` respectively.
  - At those same checkpoints, legality summaries stayed
    `10193`, `28765`, `51980`, `205199`, `246986`, and `279487`
    respectively.
  - At those same checkpoints, retained prefix cache stayed
    `13/32520`, `15/38108`, `19/53693`, `39/144845`, `39/144845`, and
    `39/144845` respectively.
  - At those same checkpoints, `remaining_one_cached_rank_prunes` stayed
    `4631`, `23205`, `46421`, `199653`, `241449`, and `273957`
    respectively.
  - At those same checkpoints, `terminal_summary_build_millis` rose to
    `36947`, `213448`, `407816`, `1748516`, `2119645`, and `2402376`
    instead of `32990`, `186848`, `355887`, `1524266`, `1849248`, and
    `2111246`.
  - `remaining_one_prefixes_seen = 0`
  - `remaining_one_incumbent_priority_promotions = 0`
  - `remaining_one_incumbent_improving_groups_seen = 0`
  - `remaining_one_incumbent_improvement_prefix_state = 0`
- That rerun was then manually stopped after enough stored evidence:
  - `prefix_states_explored = 161` at `7221.1s`
  - frontier queue length `= 2614`
  - legality summaries `= 753073`
  - retained prefix cache `= 41 groups / 154842 candidates`
  - `remaining_one_cached_rank_prunes = 747643`
  - `remaining_one_prefixes_seen = 0`
  - `remaining_one_incumbent_priority_promotions = 0`
  - `remaining_one_incumbent_improving_groups_seen = 0`
  - `remaining_one_incumbent_improvement_prefix_state = 0`
  - `terminal_summary_build_millis = 6883748`
  - observed RSS `~ 610.7 MiB`
- New stored short rerun for the now-dropped exact-two-step local ordering
  slice:
  `runs/codex-claim-release-step4-local-two-step-order-v2`
- Against `runs/codex-claim-release-step4-algebraic-v1`, the new local
  exact-two-step rerun activates immediately on stored telemetry, but the
  early checkpoints still move the wrong direction:
  - `prefix_states_explored = 1` at `61.7s` instead of `35.7s`
  - `prefix_states_explored = 5` at `249.6s` instead of `198.5s`
  - At those same checkpoints, frontier queue length still stayed
    `2774` and `2770` respectively.
  - At those same checkpoints, legality summaries still stayed
    `10193` and `28765` respectively.
  - At those same checkpoints, the retained prefix cache only moved from
    `1/2794` to `2/5588` instead of `13/32520` to `15/38108`; the stored shape
    is still honest, but it is materially behind the baseline.
  - At those same checkpoints, `remaining_one_cached_rank_prunes` rose only
    slightly to `4643` and `23218` instead of `4631` and `23205`.
  - At those same checkpoints, `remaining_one_materialized` fell sharply to
    `1` and `2` instead of `13` and `15`.
  - At those same checkpoints, `candidate_sort_millis` and
    `terminal_materialize_millis` also fell to `16/9` and `34/19` instead of
    `170/86` and `203/104`.
  - But the hot cost stayed in the same place and got worse early:
    `terminal_summary_build_millis` rose to `58966` and `237791` instead of
    `32990` and `186848`.
  - The new exact-two-step local telemetry did activate on the live claim
    lane:
    `local_exact_two_step_first_activation_prefix_state = 1`,
    `local_exact_two_step_improving_prefixes_surfaced = 4644` then `4653`,
    and `local_exact_two_step_incumbent_priority_promotions = 4629` then
    `4638`.
- That rerun was then manually stopped after enough early activation evidence:
  - `prefix_states_explored = 5` at `249.6s`
  - frontier queue length `= 2770`
  - legality summaries `= 28765`
  - retained prefix cache `= 2 groups / 5588 candidates`
  - `remaining_one_cached_rank_prunes = 23218`
  - `remaining_one_materialized = 2`
  - `local_exact_two_step_first_activation_prefix_state = 1`
  - `local_exact_two_step_improving_prefixes_surfaced = 4653`
  - `local_exact_two_step_incumbent_priority_promotions = 4638`
  - `terminal_summary_build_millis = 237791`
  - observed RSS `~ 46.8 MiB`
- That rerun therefore did not earn keep and the local exact-two-step ordering
  patch has now been dropped from code.
- New stored short rerun for the now-dropped proof-close handoff slice:
  `runs/codex-claim-release-step4-proof-close-handoff-v1`
- Against `runs/codex-claim-release-step4-algebraic-v1`, the new proof-close
  handoff rerun kept the same honest discovery-side shape at every comparable
  stored checkpoint, but the intended proof-close surface still never engaged:
  - `prefix_states_explored = 1` at `36.5s` instead of `35.7s`
  - `prefix_states_explored = 5` at `201.8s` instead of `198.5s`
  - `prefix_states_explored = 43` at `1623.8s` instead of `1629.3s`
  - `prefix_states_explored = 52` at `1968.1s` instead of `1975.0s`
  - `prefix_states_explored = 59` at `2230.4s` instead of `2252.6s`
  - At those same checkpoints, frontier queue length still stayed
    `2774`, `2770`, `2732`, `2723`, and `2716` respectively.
  - At those same checkpoints, legality summaries still stayed
    `10193`, `28765`, `205199`, `246986`, and `279487` respectively.
  - At those same checkpoints, the retained prefix cache still stayed
    `13/32520`, `15/38108`, and then `39/144845` at `43/52/59`.
  - At those same checkpoints, `remaining_one_cached_rank_prunes` and
    `remaining_one_materialized` still stayed exactly
    `4631/13`, `23205/15`, `199653/39`, `241449/39`, and `273957/39`.
  - At those same checkpoints, `terminal_summary_build_millis` moved only
    slightly to `33691`, `189133`, `1516022`, `1839230`, and `2084626`
    instead of `32990`, `186848`, `1524266`, `1849248`, and `2111246`.
  - But the new proof-close handoff telemetry stayed pinned at zero at every
    stored checkpoint:
    `proof_close_handoff_first_activation_prefix_state = 0`,
    `proof_close_handoff_improving_groups_surfaced = 0`, and
    `proof_close_handoff_incumbent_priority_promotions = 0`.
- That rerun was then manually stopped after enough stored keep/drop evidence:
  - `prefix_states_explored = 136` at `5388.3s`
  - frontier queue length `= 2639`
  - legality summaries `= 636998`
  - retained prefix cache `= 40 groups / 147639 candidates`
  - `remaining_one_cached_rank_prunes = 631544`
  - `remaining_one_materialized = 40`
  - `terminal_summary_build_millis = 5061269`
  - `proof_close_handoff_first_activation_prefix_state = 0`
  - `proof_close_handoff_improving_groups_surfaced = 0`
  - `proof_close_handoff_incumbent_priority_promotions = 0`
  - observed RSS `~ 530.2 MiB`
- That rerun therefore did not earn keep and the proof-close handoff patch has
  now been dropped from code.
- New stored short rerun for the now-dropped post-plateau summary-skip slice:
  `runs/codex-claim-release-step4-post-plateau-v1`
- Against `runs/codex-claim-release-step4-algebraic-v1`, the new
  post-plateau rerun did activate the intended telemetry before proof-close
  and it did slash stored `terminal_summary_build_millis`, but it did not earn
  keep because it only moved the hot step-`4` cost into direct compact
  materialization while making wall-clock progress much worse:
  - `prefix_states_explored = 1` at `37.7s` instead of `35.7s`
  - `prefix_states_explored = 5` at `354.4s` instead of `198.5s`
  - `prefix_states_explored = 43` at `3113.3s` instead of `1629.3s`
  - `prefix_states_explored = 52` at `3853.5s` instead of `1975.0s`
  - `prefix_states_explored = 59` at `4403.0s` instead of `2252.6s`
  - At those same checkpoints, frontier queue length still stayed
    `2774`, `2770`, `2732`, `2723`, and `2716` respectively.
  - At those same checkpoints, legality summaries still stayed
    `10193`, `28765`, `205199`, `246986`, and `279487` respectively.
  - At those same checkpoints, the retained prefix cache still stayed
    `13/32520`, `15/38108`, and then `39/144845` at `43/52/59`.
  - At those same checkpoints, `terminal_summary_build_millis` fell sharply to
    `34920`, `67319`, `317305`, `317305`, and `317305` instead of
    `32990`, `186848`, `1524266`, `1849248`, and `2111246`.
  - But that same shortcut surface then exploded the materialize-side cost:
    `terminal_materialize_millis` rose to `261792`, `2591286`, and `3797292`
    at the `5`, `43`, and `59` checkpoints instead of `104`, `466`, and `466`.
  - The new post-plateau telemetry did activate on the live claim lane:
    `post_plateau_terminal_summary_builds` stayed `0`,
    `post_plateau_terminal_summary_builds_skipped` rose to
    `14018`, `158608`, `200404`, and `232912`,
    and `post_plateau_flatten_first_prefix_state` first recorded `25`.
  - That same rerun also showed why the patch is not a keep:
    `remaining_one_materialized` rose to `14032`, `158639`, `200435`, and
    `232943` instead of `15`, `39`, `39`, and `39`, and every one of those
    materializations stayed on the direct compact path.
- That rerun was then manually stopped after enough stored keep/drop evidence:
  - `prefix_states_explored = 92` at `7187.0s`
  - frontier queue length `= 2683`
  - legality summaries `= 432706`
  - retained prefix cache `= 40 groups / 147639 candidates`
  - `terminal_summary_build_millis = 352986`
  - `terminal_materialize_millis = 6367302`
  - `remaining_one_materialized = 381637`
  - `remaining_one_materialized_compact_direct = 381637`
  - `post_plateau_terminal_summary_builds = 0`
  - `post_plateau_terminal_summary_builds_skipped = 381606`
  - `post_plateau_flatten_first_prefix_state = 75`
  - observed RSS `~ 421.5 MiB`
- That rerun therefore did not earn keep and the post-plateau summary-skip
  patch has now been dropped from code.
- New stored short rerun for the now-dropped narrow post-plateau
  materialize-side slice:
  `runs/codex-claim-release-step4-post-plateau-materialize-v1`
- Against `runs/codex-claim-release-step4-algebraic-v1`, the new
  materialize-side rerun did record the intended plateau-side telemetry, but
  it still did not earn keep because the direct-compact surface itself never
  engaged and the early checkpoints moved slightly the wrong direction:
  - `prefix_states_explored = 1` at `36.3s` instead of `35.7s`
  - `prefix_states_explored = 5` at `202.0s` instead of `198.5s`
  - `prefix_states_explored = 9` at `337.2s` instead of `332.5s`
  - At those same stored checkpoints, frontier queue length still stayed
    `2774`, `2770`, and `2766` respectively.
  - At those same stored checkpoints, the retained prefix cache still stayed
    `13/32520`, `15/38108`, and `19/53693` respectively.
  - At those same stored checkpoints, `terminal_summary_build_millis` rose to
    `33516`, `189616`, and `314814` instead of
    `32990`, `186848`, and `310348`.
  - `terminal_materialize_millis` stayed controlled at `85`, `103`, and `155`,
    so the rerun did not recreate the old materialize blowup.
  - But the new materialize-side telemetry still failed the real engagement
    test:
    `post_plateau_materialize_fast_path_kept = 0`,
    `post_plateau_materialized_compact_direct = 0`, and
    `post_plateau_materialize_fast_path_skipped` rose only to
    `4630`, `23204`, and `41776`.
  - The plateau markers did move on stored evidence:
    `post_plateau_flatten_first_prefix_state` recorded `1`, then `5`, then
    `8`, while `post_plateau_materialize_first_activation_prefix_state`
    stayed pinned at `1`.
- That rerun was then manually stopped after enough stored non-engagement
  evidence:
  - `prefix_states_explored = 9` at `337.2s`
  - frontier queue length `= 2766`
  - legality summaries `= 47337`
  - retained prefix cache `= 19 groups / 53693 candidates`
  - `terminal_summary_build_millis = 314814`
  - `terminal_materialize_millis = 155`
  - `post_plateau_materialize_fast_path_kept = 0`
  - `post_plateau_materialized_compact_direct = 0`
  - `post_plateau_materialize_fast_path_skipped = 41776`
  - observed RSS `~ 98.5 MiB`
- That rerun therefore did not earn keep and the narrow post-plateau
  materialize-side patch has now also been dropped from code.
- Current full baseline:
  `runs/codex-claim-release-full-nu-profile-v1`
- The earlier intended-profile rerun
  `runs/codex-claim-release-full-algebraic-v1` still remains the latest full
  evidence that did not advance that baseline:
  - `prefix_states_explored = 43` at `1703.4s` instead of `1629.6s`
  - `prefix_states_explored = 52` at `2081.4s` instead of `2039.7s`
  - `prefix_states_explored = 59` at `2369.7s` instead of `2367.7s`
  - At those same checkpoints, retained prefix cache stayed
    `39 groups / 144845 candidates` and legality summaries stayed
    `205199`, `246986`, and `279487` respectively.
  - At those same checkpoints, `terminal_summary_build_millis` rose to
    `1602527`, `1962557`, and `2237096` instead of
    `1520539`, `1910369`, and `2221499`.
- The following validations were re-earned in this turn:
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-search online_work_items_`
  - `cargo test -p pen-search prefix_queue_prefers_nearer_terminal_and_tighter_cached_continuations`
  - `cargo test -p pen-search terminal_clause_filter_skips_inadmissible_last_clauses_before_connectivity`
  - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`
- Historical caution still matters, but the specific old `claim_` crash shape is
  no longer the active test surface:
  - Windows logged `Resource-Exhaustion-Detector` warnings at `2026-03-25
    06:41:02` and `2026-03-25 06:46:00` during the older broad claim sweep.
  - That older crash path depended on a heavy witness setup that bootstrapped
    and enumerated a full step-`4` prefix surface just to prove one algebraic
    prune witness.
  - That setup is now removed from the active regression set, which is why the
    current `claim_` filter can be trusted again as a local validation step.

## Current Read

- The algebraic ceiling patch stays landed as a narrow local improvement on the
  short step-`4` profile.
- The attempted context-equivalence slice is exhausted on stored evidence and
  is not kept in code.
- The attempted frontier-pop incumbent-arrival ordering slice is also exhausted
  on stored evidence and is not kept in code.
- The attempted exact-two-step local ordering slice is the first post-algebraic
  experiment that actually activates on the live claim lane, but its stored
  `1/5` checkpoints were still slower than the short baseline, so it too has
  been dropped from code.
- The attempted proof-close handoff slice then showed that the intended
  proof-close surface still never activates before the hot stored step-`4`
  checkpoints, so that slice is now also exhausted and dropped from code.
- The attempted post-plateau discovery summary-skip slice did activate and it
  did cut stored summary-build time hard, but it only did so by forcing almost
  every post-plateau prefix onto the direct compact materialize path, so that
  slice is now also exhausted and dropped from code.
- The attempted narrow post-plateau materialize-side follow-up then showed
  that a one-clause direct-compact gate does not actually open on the live
  claim lane at the honest early checkpoints, so that narrower materialize
  surface is now also exhausted and dropped from code.
- Step `4` still remains the dominant honest blocker on the real profile, and
  no post-algebraic slice has yet earned baseline replacement from stored
  evidence.
- Memory remains controlled on the short reruns; the wall is still compute, not
  allocator or RSS pressure.
- The next honest move is no longer another post-plateau direct-compact gate;
  the stored evidence now points back to the post-plateau summary-build wall
  itself on the retained remaining-one surface, because the materialize-side
  follow-up never actually engaged the intended direct path.

## Immediate Order

1. Patch one narrow post-plateau summary-build-side throughput or reuse cut on
   the retained remaining-one surface that keeps the algebraic baseline's
   direct compact behavior unchanged.
2. Add telemetry and tests for that refined summary-side surface, then
   re-earn one stored release `until_step = 4` rerun.
3. Keep that next patch only if the stored rerun keeps the same honest
   retained-prefix shape while improving wall-clock progress and without
   reopening the old direct-materialize blowup.
4. Only after that short rerun earns keep should another real
   `desktop_claim_shadow_1h` full-profile rerun happen.

## Active Baselines

- Full-profile baseline before delayed summary:
  `runs/codex-claim-release-full-v1a`
- Earlier full-profile baseline:
  `runs/codex-claim-release-full-delayed-summary-v1`
- Current full-profile iteration baseline:
  `runs/codex-claim-release-full-nu-profile-v1`
- Most recent intended-profile evidence that did not advance that baseline:
  `runs/codex-claim-release-full-algebraic-v1`
- Earlier short step-`4` baseline:
  `runs/codex-claim-release-step4-delayed-summary-v1`
- Previous short step-`4` baseline:
  `runs/codex-claim-release-step4-nu-profile-v1`
- Current short step-`4` iteration baseline:
  `runs/codex-claim-release-step4-algebraic-v1`
- Most recent short evidence that did not advance that baseline:
  `runs/codex-claim-release-step4-post-plateau-materialize-v1`
- Earlier short evidence that also did not advance that baseline:
  `runs/codex-claim-release-step4-post-plateau-v1`
- Earlier short evidence that also did not advance that baseline:
  `runs/codex-claim-release-step4-proof-close-handoff-v1`
- Earlier short evidence that also did not advance that baseline:
  `runs/codex-claim-release-step4-local-two-step-order-v2`
- Earlier short evidence that also did not advance that baseline:
  `runs/codex-claim-release-step4-incumbent-ordering-v1`
- Earlier short evidence that also did not advance that baseline:
  `runs/codex-claim-release-step4-context-equivalence-v1`
- Ignore as stale-binary noise, not active evidence:
  `runs/codex-claim-release-step4-local-two-step-order-v1`
- Pre-delay materialization baseline:
  `runs/codex-claim-release-step4-telemetry-v1`

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` unchanged.
- Keep user-facing wording at `bounded live recovery`.
- Keep the claim lane honest about still being guided and not yet fully
  unguided.
- Prefer structural exact bounds over semantic shortcuts.
- Trust stored artifacts over terminal impressions.
