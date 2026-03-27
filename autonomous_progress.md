# Autonomous Claim Lane Progress

Last updated: 2026-03-27

This file is the live status read for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next
execution slice, [autonomous_plan.md](autonomous_plan.md) for the staged path
to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the
final gate.

## Current Status

- `desktop_claim_shadow` is not signoff-ready.
- The following claim-lane wins are landed in code:
  - delayed materialization
  - the incumbent-primary remaining-one fast path
  - the one-pass `structural_nu` summary-build fast path
  - the algebraic `nu` ceiling patch
  - the family-agnostic claim terminal-admissibility shortcut
  - the exact non-allocating connectivity summary scan
  - the terminal-only cached parent connectivity decision
  - the remaining-one aggregation-side accept-rank short-circuit that now
    skips full `AcceptRank` construction for bar-clearing claim candidates
    whose primary `(overshoot, kappa)` rank is already worse than the best
    primary rank seen in the same summary group
- The following claim-only slices were implemented, measured on stored short
  reruns, and then dropped from code after failing keep:
  - context-equivalence quotient:
    `runs/codex-claim-release-step4-context-equivalence-v1`
  - frontier-pop incumbent ordering:
    `runs/codex-claim-release-step4-incumbent-ordering-v1`
  - exact-two-step local ordering:
    `runs/codex-claim-release-step4-local-two-step-order-v2`
  - proof-close handoff ordering:
    `runs/codex-claim-release-step4-proof-close-handoff-v1`
  - post-plateau summary-skip:
    `runs/codex-claim-release-step4-post-plateau-v1`
  - post-plateau materialize-side gate:
    `runs/codex-claim-release-step4-post-plateau-materialize-v1`
  - post-plateau summary-cache reuse:
    `runs/codex-claim-release-step4-post-plateau-summary-cache-v3`
  - expr-keyed terminal clause hot-path profile cache:
    `runs/codex-claim-release-step4-kernel-connectivity-v3`
  - clause-side terminal connectivity profile precompute:
    `runs/codex-claim-release-step4-kernel-connectivity-v4`
  - terminal-candidate tuple remap and allocation cut:
    `runs/codex-claim-release-step4-terminal-candidate-prep-v1`
- The step-`4` kernel is still measurable from stored evidence:
  - valid diagnostic split:
    `runs/codex-claim-release-step4-kernel-profile-v2`
  - ignore as invalid local diagnostic:
    `runs/codex-claim-release-step4-kernel-profile-v1`
- The most recent stored short rerun in this turn earned keep:
  `runs/codex-claim-release-step4-kernel-aggregation-v1`
- The current short step-`4` baseline is now
  `runs/codex-claim-release-step4-kernel-aggregation-v1`.
- The previous short step-`4` baseline is now
  `runs/codex-claim-release-step4-kernel-connectivity-v2`.
- The older short step-`4` baseline remains
  `runs/codex-claim-release-step4-kernel-connectivity-v1`.
- The current full-profile iteration baseline is now
  `runs/codex-claim-release-full-kernel-aggregation-v1`.
- The previous full-profile iteration baseline is now
  `runs/codex-claim-release-full-nu-profile-v1`.
- The claim lane is still compute-bound in step `4` on the intended profile.
  A new stored full-profile rerun moved materially farther than the prior full
  baseline with observed RSS still controlled, but it did not reach step `5`,
  and it proved the next honest move is another narrow short step-`4` slice
  rather than compare, benchmark, certification, or breadth closeout.

## Latest Evidence

- Diagnostic kernel split:
  `runs/codex-claim-release-step4-kernel-profile-v2`
  - At the honest retained plateau `39 groups / 144845 candidates`, the
    measured hot step-`4` sub-phases at `prefix_states_explored = 44` were:
    - admissibility `= 679889 ms`
    - connectivity `= 492575 ms`
    - aggregation `= 118953 ms`
    - exact `nu` `= 74386 ms`
  - That rerun was diagnostic-only and slower overall, but it identified the
    real dominant cost on the live plateau.
- Previous kept short baseline:
  `runs/codex-claim-release-step4-kernel-connectivity-v2`
  - It kept the same honest retained plateau:
    - retained prefix cache `= 39 groups / 144845 candidates` at `24/43/44`
    - `terminal_summary_first_plateau_activation_prefix_state = 24`
    - `terminal_summary_plateau_activations = 97234` at `44`
  - It reuses the cached parent legality summary for the terminal-only
    connectivity or reanchor decision instead of cloning and extending the
    full child legality summary for each last-clause check.
  - Matched against `runs/codex-claim-release-step4-kernel-connectivity-v1`:
    - at `prefix_states_explored = 24`:
      `elapsed_millis = 551825` instead of `692343`,
      `terminal_summary_build_millis = 495256` instead of `635477`,
      `terminal_summary_connectivity_millis = 95969` instead of `222604`
    - at `prefix_states_explored = 43`:
      `elapsed_millis = 998555` instead of `1245950`,
      `terminal_summary_build_millis = 901994` instead of `1145519`,
      `terminal_summary_connectivity_millis = 178000` instead of `399280`
    - at `prefix_states_explored = 44`:
      `elapsed_millis = 1020529` instead of `1273659`,
      `terminal_summary_build_millis = 921924` instead of `1170875`,
      `terminal_summary_connectivity_millis = 182453` instead of `408582`
  - Residual costs stayed controlled:
    - aggregation `= 68266/119561/121524`
    - exact `nu` `= 39523/73348/74489`
    - `terminal_materialize_millis = 327`
    - fallback connectivity `= 0`
- New kept short baseline:
  `runs/codex-claim-release-step4-kernel-aggregation-v1`
  - It keeps the same honest retained plateau:
    - retained prefix cache `= 39 groups / 144845 candidates` at `24/43/44`
    - `terminal_summary_first_plateau_activation_prefix_state = 24`
    - `terminal_summary_plateau_activations = 97234` at `44`
  - It keeps the connectivity-side cached parent-summary reuse and now also
    stops constructing full `AcceptRank` values for bar-clearing claim
    candidates once the group has already seen a strictly better primary
    `(overshoot, kappa)` rank.
  - Matched against the prior kept short baseline
    `runs/codex-claim-release-step4-kernel-connectivity-v2`:
    - at `prefix_states_explored = 24`:
      `elapsed_millis = 549630` instead of `551825`,
      `terminal_summary_build_millis = 492524` instead of `495256`,
      `terminal_summary_connectivity_millis = 88989` instead of `95969`
    - at `prefix_states_explored = 43`:
      `elapsed_millis = 990480` instead of `998555`,
      `terminal_summary_build_millis = 892772` instead of `901994`,
      `terminal_summary_connectivity_millis = 164940` instead of `178000`
    - at `prefix_states_explored = 44`:
      `elapsed_millis = 1012067` instead of `1020529`,
      `terminal_summary_build_millis = 912271` instead of `921924`,
      `terminal_summary_connectivity_millis = 169227` instead of `182453`
  - Residual costs also stayed controlled or improved:
    - aggregation fell to `67567/118700/120643` from `68266/119561/121524`
    - exact `nu` stayed flat at `39527/73331/74471`
    - `terminal_materialize_millis = 325` instead of `327`
    - fallback connectivity stayed `0`
  - The rerun was allowed to continue past the matched plateau checkpoints
    under a bounded session timeout, then the still-live `pen-cli` process was
    terminated manually after the stored short comparison surface had been
    secured through `prefix_states_explored = 54`.
- New intended full-profile rerun:
  `runs/codex-claim-release-full-kernel-aggregation-v1`
  - It materially improved the old intended full-profile baseline
    `runs/codex-claim-release-full-nu-profile-v1` at the matched early
    checkpoints while preserving the same real search policy and worker shape:
    - at `prefix_states_explored = 24`:
      `elapsed_millis = 564414` instead of `898646`,
      `terminal_summary_build_millis = 506830` instead of `836247`,
      observed RSS `= 228831232` instead of `228790272`
    - at `prefix_states_explored = 43`:
      `elapsed_millis = 1014969` instead of `1629553`,
      `terminal_summary_build_millis = 916413` instead of `1520539`,
      observed RSS `= 282046464` instead of `284622848`
    - at `prefix_states_explored = 44`:
      `elapsed_millis = 1036933` instead of `1667689`,
      `terminal_summary_build_millis = 936329` instead of `1556452`,
      observed RSS `= 284889088` instead of `287059968`
    - at `prefix_states_explored = 54`:
      `elapsed_millis = 1274136` instead of `2142900`,
      `terminal_summary_build_millis = 1152279` instead of `2008719`,
      observed RSS `= 313511936` instead of `315887616`
  - At those matched checkpoints it also stayed close to the kept short
    baseline `runs/codex-claim-release-step4-kernel-aggregation-v1`, which
    shows that the winning kernel-aggregation shape survived on the real
    intended profile with only modest full-profile overhead.
  - The old honest retained plateau did not remain terminal on the real
    full-profile rerun:
    - `39 groups / 144845 candidates` first activated at
      `prefix_states_explored = 24` and held through `73`
    - `40 groups / 147639 candidates` first appeared at `74`
    - `41 groups / 154842 candidates` first appeared at `140`
  - The rerun never reached step `5`; there is no stored
    `reports/steps/step-05-live.ndjson`.
  - The last stored step-`4` checkpoint before manual termination was
    `prefix_states_explored = 163` with:
    - `elapsed_millis = 3942636`
    - `observed_process_rss_bytes = 632197120`
    - `frontier_queue_len = 2612`
    - legality summaries `= 762359`
    - `terminal_summary_build_millis = 3584914`
    - `terminal_summary_connectivity_millis = 696824`
    - `terminal_summary_aggregation_millis = 483012`
    - `terminal_summary_exact_nu_millis = 311183`
  - So the new intended-profile read moved far enough to invalidate the old
    `39/144845` "final plateau" read, but it still exposes no later step wall:
    the live blocker remains step-`4` summary compute, especially connectivity
    first and aggregation second, with exact `nu` still behind them.
- Dropped short rerun:
  `runs/codex-claim-release-step4-kernel-connectivity-v3`
  - It tried caching per-clause hot-path terminal
    check or connectivity profiles keyed by clause `expr`.
  - It kept the same honest plateau, but at the matched `44` checkpoint it
    slowed wall clock to `1081589` from `1020529` and raised
    `terminal_summary_build_millis` to `978181` from `921924`, so it was
    dropped from code.
- Dropped short rerun:
  `runs/codex-claim-release-step4-kernel-connectivity-v4`
  - It tried precomputing a clause-side terminal connectivity profile beside
    the terminal candidate list and reusing that profile inside the
    remaining-one connectivity decision.
  - It kept the same honest plateau and collapsed the measured connectivity
    counter, but still regressed at the matched `44` checkpoint to
    `elapsed_millis = 1115276` and
    `terminal_summary_build_millis = 1002528`, so it was dropped from code.
- Dropped short rerun:
  `runs/codex-claim-release-step4-terminal-candidate-prep-v1`
  - It added one narrow pre-summary candidate-preparation counter and tried one
    narrow throughput cut that removed the extra filtered-candidate remap and
    tuple allocation before the first connectivity check.
  - It kept the same honest retained plateau and exposed
    `terminal_summary_candidate_prep_millis = 32904/71577/73974`
    at `24/43/44`, but it still regressed against the then-current kept
    baseline and was dropped from code.
- Re-earned validations in this turn:
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-search online_work_items_`
  - `cargo test -p pen-search prefix_queue_prefers_nearer_terminal_and_tighter_cached_continuations`
  - `cargo test -p pen-search terminal_clause_filter_skips_inadmissible_last_clauses_before_connectivity`
  - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`

## Current Read

- The recent loop of failed experiments is now anchored by two real keep
  slices:
  `runs/codex-claim-release-step4-kernel-connectivity-v2` and
  `runs/codex-claim-release-step4-kernel-aggregation-v1`.
- The re-earned intended-profile rerun
  `runs/codex-claim-release-full-kernel-aggregation-v1` then proved three
  things from stored artifacts:
  - the winning kernel-aggregation shape survives on the real
    `desktop_claim_shadow_1h` profile and materially improves the old
    full-profile baseline at matched `24/43/44/54` checkpoints
  - the earlier `39 groups / 144845 candidates` plateau is real but no longer
    the full-profile terminal read, because the retained prefix cache later
    reopens to `40/147639` and then `41/154842`
  - no later step wall is exposed yet, because the run still never reaches
    step `5`
- The current honest blocker is therefore still inside step `4` on the
  intended profile:
  connectivity first, aggregation second, exact `nu` behind them, now on the
  later reopened retained-prefix surface rather than only on the first
  `39/144845` plateau.
- Observed RSS remained controlled on the intended profile during the re-earned
  full read, rising gradually to about `632.2 MiB` at the last stored
  checkpoint rather than revisiting the old allocator-failure story.
- The next honest move is now another narrow short step-`4` slice guided by
  the later full-profile retained-prefix growth, not compare, benchmark,
  certification, or breadth/parity closeout work.

## Immediate Order

1. Keep `runs/codex-claim-release-step4-kernel-aggregation-v1` landed in code
   and keep the earlier prep/profile cache experiments dropped.
2. Treat `runs/codex-claim-release-full-kernel-aggregation-v1` as the new
   intended-profile baseline and use its stored step-`4` evidence, not the old
   `full-nu-profile-v1` read, to define the next runtime slice.
3. Re-enter one narrow short step-`4` throughput experiment aimed at the later
   retained-prefix surface that reopens after the old `39/144845` plateau.
4. Return to stored parity, breadth, compare, benchmark, and certification
   work only after a later rerun either reaches step `5` or proves the step-`4`
   wall has finally moved.

## Active Baselines

- Current full-profile baseline:
  `runs/codex-claim-release-full-kernel-aggregation-v1`
- Previous full-profile baseline:
  `runs/codex-claim-release-full-nu-profile-v1`
- Current short step-`4` baseline:
  `runs/codex-claim-release-step4-kernel-aggregation-v1`
- Previous short step-`4` baseline:
  `runs/codex-claim-release-step4-kernel-connectivity-v2`
- Older short step-`4` baseline:
  `runs/codex-claim-release-step4-kernel-connectivity-v1`
- Most recent valid diagnostic:
  `runs/codex-claim-release-step4-kernel-profile-v2`
- Ignore as invalid diagnostic:
  `runs/codex-claim-release-step4-kernel-profile-v1`
- Most recent short evidence that did not advance the current short baseline:
  `runs/codex-claim-release-step4-terminal-candidate-prep-v1`

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` unchanged.
- Keep user-facing wording at `bounded live recovery`.
- Keep the claim lane honest about still being guided and not yet fully
  unguided.
- Prefer exact structural cuts over semantic shortcuts.
- Trust stored artifacts over terminal impressions.
