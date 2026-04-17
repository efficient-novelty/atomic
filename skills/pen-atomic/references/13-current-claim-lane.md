# Current Claim Lane

Read this file when the task touches `desktop_claim_shadow`, claim-lane
telemetry, claim-lane narratives, or the autonomy-certification roadmap.

## Contract

- This file owns stable claim-lane context, vocabulary, and invariants.
- Do not store live run IDs, current blocker counts, or the active probe here.
- Use [../../autonomous_progress.md](../../autonomous_progress.md) for the live
  snapshot, [../../autonomous_next_steps.md](../../autonomous_next_steps.md)
  for the active work order, [../../autonomous_plan.md](../../autonomous_plan.md)
  for phases, [../../autonomous_checklist.md](../../autonomous_checklist.md)
  for binary gates, and [../../autonomous_ledger.md](../../autonomous_ledger.md)
  for experiment history.

## Stable Lane Truths

- `desktop_claim_shadow` exists as a separate profile and config family.
- The lane now has a stored compare / benchmark / certify-passing bundle.
  Any stronger wording must be tied directly to that stored certificate and
  disclosed desktop bundle.
- Claim policy metadata is already real and should stay honest:
  - `guidance_style = claim_debt_guided`
  - `late_expansion_policy = claim_generic`
  - `bucket_policy = structural_generic`
- Claim-specific compare, benchmark, and certification tooling are the
  official evidence surfaces for signoff:
  - `scripts/compare_runs.py`
  - `scripts/benchmark_claim_lane.py`
  - `scripts/certify_claim_lane.py`
- Those evidence surfaces now also have native `pen-cli` entry points for
  current-head refresh:
  - `pen-cli compare-claim-lane`
  - `pen-cli benchmark-claim-lane`
  - `pen-cli certify-claim-lane`
- Direct script execution is still blocked on this machine because there is no
  runnable `python`, `python3`, `py`, or `uv` launcher here, but the native
  `pen-cli` commands now cover local evidence refresh.
- Claim runs persist durable evidence incrementally:
  - `run.json` with build/git/binary fingerprints
  - step summaries and step checkpoints
  - frontier snapshots
  - narratives and events
  - `step_live_checkpoint` telemetry and `reports/steps/step-XX-live.ndjson`
- Claim proof-close and materialization already include the landed memory
  behavior:
  - evaluated terminal payloads drop after ranking
  - processed retained prefix groups release once certification starts
  - legality-cache completion summaries are reused
  - uncached compact materialization is direct rather than rebuild-and-rewalk
  - frontier items reuse the shared clause catalog and serialized prefix key
- Claim-lane work must stay separate from demo-only behavior even when probes
  temporarily reuse demo-family clauses under test-only overrides.
- The current post-probe evidence now has two layers: a previous audited head
  already refreshed compare, benchmark, and certification on newer code and
  re-confirmed that step `15`, not step `1`, was the first stored breadth
  miss, while the current certified stored head preserves accepted-hash parity
  through step `15`, keeps the repaired `7211 / 553 / 2052` late surface,
  restores stored step `1` to `2144 / 1285 / 1 / 475`, and now carries ready
  native compare, benchmark, and certificate artifacts beside the rerun.
- The first current-head step-`1` follow-on is no longer only local evidence:
  `DesktopClaimShadow` config runs borrow the shared claim/demo exhaustive
  path only at step `1`, the claim smoke config restores the full step-`1`
  raw catalog surface to `2144`, guarded accepted parity stays green through
  step `8`, step `5+` keeps the existing non-budgeted claim path, and the
  stored certified head now reflects that same step-`1` repair. The next
  honest move is post-certification local step-`15` hardening below the clean
  `partial_prefix_bar_failure = 553` wall rather than more step-`1` geometry
  or tooling work.
- The report-side closure handoff is no longer the blocker: direct `pen-cli`
  claim coverage now keeps the stored step-`15` `demo_closure` surface intact
  across step summaries, step narratives, `latest.debug.txt`,
  finalize-success / finalize-failed output, and completed `resume(...)`
  output even when the in-memory `demo_funnel` surface is intentionally
  drifted.
- The first post-reference exact-two-step local repair is now landed on the
  current head too: outside tests, `DesktopClaimShadow` step `15` widens only
  descendants of the top-level queued `reference` and top-level queued
  `claim_eventual_domain` remainders to the demo late-family surface,
  matching the regression-backed `7211` local generation /
  `2052 / 522 / 522 / 0` `small_cluster` surface while preserving
  `partial_prefix_bar_failure = 553`, canonical accepted `103 / 8`, the
  fenced `single` pocket, and zero fully scored lifted `89 / 8`.
- The repaired step-`15` survivor accounting is regression-backed and stored
  too: demo buckets now collapse on the repaired current head to one live
  temporal `small_cluster` at `2052 / 522 / 522 / 0`, one zero-admitted
  `broad` bucket at `3600 / 0 / 0 / 0`, and one fenced `single` pocket at
  `0 / 0 / 0 / 3 / 1`; the residual proof-close pressure is localized to
  three exact claim families
  `clause-0 claim_flat_domain`,
  `clause-2 claim_flat_domain plus anchor-11 exact-argument`, and
  `clause-5 claim_flat_codomain`, and family-local same-primary relief only
  trades those prunes for extra fully scored non-winning reference terminals.
- The repaired-head partial-prefix wall is regression-backed too: the
  unchanged clean `553` wall now localizes to exactly queued frontier
  remainder branches `4 .. 11`, with branch totals
  `4 / 5 = 7`,
  `6 / 7 = 19`,
  `8 / 9 = 52`,
  `10 = 156`,
  and
  `11 = 241`; queued branches `0 .. 3` contribute none, and the largest
  single live blocker is the direct top-level `reference` remainder at
  `241 = 199` remaining-two plus `42` remaining-three. Inside that direct
  `reference` remainder, mismatch `1` now carries
  `177 = 145` remaining-two plus `32` remaining-three, mismatch `2` carries
  `50 = 42 + 8`, mismatch `3` carries `14 = 12 + 2`, and the largest single
  remaining-two pairing stays `reference / demo_flat_codomain = 61`. Inside
  that pairing, the repaired head now localizes one layer deeper too:
  remaining-two clause-`4` `claim_next_bridge = 33`,
  remaining-two clause-`4` `reference = 28`, and the whole remaining-three
  spill is another `12` captures that stay entirely on clause-`4`
  `reference`. That `claim_next_bridge = 33` side is now explicit too at
  clause-`2`
  `claim_flat_domain = 12`,
  `claim_sharp_codomain = 12`,
  and
  `reference = 9`;
  the first repaired-head exact claim-pair reopening under that side is
  frozen as a tradeoff control at
  `generated_raw_prefixes = 7485`,
  `partial_prefix_bar_failure = 539`,
  direct `reference` remainder `= 227`,
  `reference / demo_flat_codomain = 45`,
  slot split `27 / 18 / 14`,
  and
  `small_cluster = 2112 / 542 / 542 / 0`,
  while the tied repaired-head exact `claim_flat_domain` and exact
  `claim_sharp_codomain` single-sheet reopenings under that same side are
  frozen as matched smaller tradeoff controls at
  `generated_raw_prefixes = 7317`,
  `partial_prefix_bar_failure = 545`,
  direct `reference` remainder `= 233`,
  `reference / demo_flat_codomain = 51`,
  slot split `30 / 21 / 14`,
  and
  `small_cluster = 2076 / 530 / 530 / 0`;
  the exact clause-`2` `reference = 9` sheet under that same side is now
  frozen too as the first non-widening safe side-control at
  `generated_raw_prefixes = 7149`,
  `partial_prefix_bar_failure = 551`,
  direct `reference` remainder `= 239`,
  `reference / demo_flat_codomain = 57`,
  slot split `33 / 24 / 14`,
  and
  `small_cluster = 2040 / 518 / 518 / 0`.
  It trims only the clause-`4` `reference` companion while leaving the live
  clause-`4` `claim_next_bridge = 33` blocker untouched. That attempted
  landing extension is now frozen too: reopening that same exact clause-`2`
  `reference` sheet together with its clause-`4` `reference` companion is
  completely neutral and relands the same
  `7149 / 551 / 2040 / 518 / 518 / 0`
  surface, with the same direct `reference` remainder `239`, the same
  targeted pairing `57`, and the same `33 / 24 / 14` slot split. That neutral
  result also pins the exact-reference-sheet grid itself:
  remaining-two clause-`4` `claim_next_bridge` stays
  `12 / 12 / 9`,
  remaining-two clause-`4` `reference` stays
  `9 / 9 / 6`,
  and the remaining-three clause-`4` `reference` spill stays
  `5 / 5 / 4`
  across clause-`2`
  `claim_flat_domain / claim_sharp_codomain / reference`.
  The first exact `claim_flat_domain` and exact `claim_sharp_codomain`
  reopenings under that broader clause-`4` `reference` companion are now
  frozen too as spill-only tradeoff controls at
  `7236 / 550 / 2058 / 524 / 524 / 0`;
  they keep the remaining-two clause-`4` `reference` row pinned at
  `9 / 9 / 6`
  and trim only one remaining-three spill capture to
  `4 / 5 / 4`
  or
  `5 / 4 / 4`.
  The next honest move is therefore the smaller exact `reference = 6` slice
  below that same clause-`4` `reference` companion before the whole
  remaining-three spill or any higher-branch family.
- The reverted clause-`1` exact-pocket family is regression-backed too:
  the `demo_eventually_codomain` exact-pocket reland and the widened
  clause-`0` claim-flat `demo_flat_codomain` exact-pocket reland both
  collapse to the same widening negative control at `4466 / 626` with the
  same `3156 / 526 / 526 / 0` `small_cluster`, the same fenced `single`
  pocket, and the same disconnected zero-admitted shell at `2562 / 7686`.
- The representative mismatch-`0` `reference` clause-`5` remaining-one slice
  is regression-backed too: the pair-cell clause-`2` split, both claim-side
  clause-`6` sibling sets, both clause-`6` `reference` clause-`3` branches,
  and both joint-continuation delta probes all keep the same already-pinned
  `4343 / 552 / 2268-2270` or neutral surfaces, so that slice is frozen as
  geometry rather than as a fresh repair class.
- The live step-`15` caller setup is regression-backed too: on the actual
  step-`14` claim prefix, `DesktopClaimShadow` keeps no named focus family,
  keeps `PackagePolicies::default()`, keeps `historical_anchor_ref = 10`,
  keeps the `ClaimGeneric` late surface at `kappa = 8`, and now pins the
  first temporal-shell claim-debt snapshot too:
  `anchor_policy = Modal` with only `temporal_shell` package pressure and
  `claim_debt_axes = 8 / 8` carrying `coupling = 2`, `support = 2`,
  `temporal = 1`, `reanchor = 2`, and `closure = 3`.
- On the canonical seven-clause reference prefix, the default claim-open-band
  route now keeps the full
  `reference + eventual_lift + next_lift` claim-generic `kappa = 8` trio
  live.
- The raw claim-generic band-`8` family emitter / late clause selector is now
  regression-backed too: on the live step-`15` claim library the raw
  `kappa = 8` catalog stays uniformly `3 / 3 / 3 / 3 / 3 / 3 / 3 / 3` for
  exactly `6561` raw telescopes, `late_clause_options(..., 8)` stays
  identical to `claim_generic_band8_clauses(...)` at every live position
  under `LateFamilySurface::ClaimGeneric`, the live remaining-one selector
  emits the same sorted `reference + eventual_lift + next_lift` trio
  directly, the realistic control stays only
  `1 / 1 / 1 / 1 / 2 / 1 / 1 / 1` with `2` raw telescopes, and changing only
  `LateFamilySurface` to `RealisticShadow` or `DemoBreadthShadow` collapses
  or widens that same terminal selector to the lone reference clause or the
  five-clause demo family instead.
- That default claim route is now derivation-backed too: on the same live
  step-`14` claim library, the raw structural debt still resolves to
  `TemporalShell`, but `DesktopClaimShadow` step `15` bypasses that
  structural focus because `claim_guarded_early_focus_family(...)` only
  applies on steps `4 ..= 8`; it dispatches through
  `claim_strict_admissibility(...)` instead, preserving the same
  `kappa = 8`, modal/temporal budget, and modal anchor `10` while keeping
  `focus_family = None` and `PackagePolicies::default()`.
- That claim-profile helper layer is now regression-backed too:
  `claim_debt_axes_for_step(14, ...)` promotes the divergent operator-bundle
  claim snapshot from raw `7 / 7` to `9 / 9`, while on the actual live
  step-`14` claim library `claim_debt_axes_for_step(15, ...)` keeps the raw
  temporal-shell `8 / 8` axes unchanged and
  `claim_historical_anchor_ref(...)` keeps the modal anchor `10`.
- The modal/temporal inclusion layer above those helpers is now
  regression-backed too: on the divergent step-`13` operator-bundle claim
  library, `claim_include_modal(...)` and `claim_include_temporal(...)` both
  stay closed after the step-`14` `9 / 9` band promotion, while on the
  actual live step-`14` claim library the step-`15` temporal-shell `8 / 8`
  snapshot opens both helpers because `temporal_pressure = 1` even though
  `modal_pressure = 0`.
- The claim-profile sizing helpers above that inclusion layer are now
  regression-backed too: on both the divergent step-`13` operator-bundle
  control after the step-`14` `9 / 9` promotion and the actual live
  step-`15` temporal-shell `8 / 8` claim profile,
  `claim_max_expr_nodes(...)` relands the same wide `7`-node envelope while
  `claim_max_path_dimension(...)` stays closed at `0`.
- The claim-profile truncation/quota layer above that sizing layer is now
  regression-backed too: on both the divergent step-`13` operator-bundle
  control after the step-`14` `9 / 9` promotion and the actual live
  step-`15` temporal-shell `8 / 8` claim profile,
  `claim_include_trunc(...)` stays closed with
  `max_path_dimension = 0` and `truncated_entries = 0`, while
  `quota_per_bucket()` stays wide at `64` by inheriting the raw
  operator-bundle or temporal-shell package pressure.
- The claim-profile package-requirement layer above that truncation/quota
  surface is now regression-backed too: on both the divergent step-`13`
  operator-bundle control after the step-`14` `9 / 9` promotion and the
  actual live step-`15` temporal-shell `8 / 8` claim profile, the raw debt
  snapshot still keeps only the corresponding operator-bundle or
  temporal-shell package pressure, but `claim_strict_admissibility(...)`
  leaves `PackagePolicies::default()` intact and keeps every
  `require_*_package` flag closed on the no-focus claim route.
- The downstream package-policy consumer accessor layer above that helper-
  local package surface is now regression-backed too: on both no-focus claim
  profiles, `required_focus_family()` stays `None` and `policy_for(...)`
  stays `Allow` for every structural family, while the bypassed step-`15`
  structural temporal-shell route still yields no required focus family and
  only `policy_for(TemporalShell) = Prefer`.
- The first search-side family-summary / work-item layer above those accessors
  is now regression-backed too: on the actual live step-`15` claim prefix,
  `PrefixFamilySummary::for_admissibility(...)` short-circuits on
  `focus_family = None`, so `PrefixLegalityCache` stores no family filter,
  `family_option_count()` stays absent, and
  `create_online_prefix_work_item(...)` keeps
  `remaining_family_options = u8::MAX` plus the full step-`8` catalog on the
  default claim-generic route; replaying that same prefix through the
  bypassed temporal-shell focused route re-enables one cached family option
  and the active-window consumer path.
- The frontier ordering / queue-pop consumer above that first work-item layer
  is now regression-backed too: on the live seven-clause reference prefix and
  the already-spent focused-three no-bound flat-shell residues used only as
  comparison geometry, `prefix_frontier_work_key(...)` ties on
  `remaining_clause_slots = 1`,
  `next_clause_count = 3`,
  `remaining_family_options = u8::MAX`, and
  `clause_count = 7`, then still prefers the live reference prefix because
  its `bit_cost = 191` stays below each focused-three residue, so
  `pop_best_prefix(...)` still pops the live reference work item first.
- The focused-three exact-bound gate above that same dead branch is now
  regression-backed too: on those same `3`
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  residues, direct `exact_partial_prefix_bound_decision(...)` replay still
  caches `CannotClearBar` by default while storing the same compact dead
  `3`-generated / `0`-admitted no-bound summary with no best accept primary
  rank and no best accept rank; the pair-cell remaining-one exact-summary
  override only reclassifies that gate to `Unknown` without changing the
  underlying dead summary or caching a stable partial-prefix decision.
- The post-pop remaining-one consumer above that frontier layer is now
  regression-backed too: on the same live seven-clause reference prefix,
  `collapse_single_continuation_chain(...)` is a no-op at
  `remaining_clause_slots = 1`, while
  `claim_try_summary_prune_before_materialization(...)` does not pre-prune
  either and instead caches a compact claim-open-band shell with
  `generated = 3`, `admitted = 1`, a bar-clearing bound, and a single
  survivor whose best accept rank already matches the canonical
  `reference(15)` `103 / 8` completion.
- The remaining-one materialization handoff above that post-pop layer is now
  regression-backed too: on the same live seven-clause reference prefix,
  `materialize_remaining_one_prefix_group(...)` increments the remaining-one
  materialization telemetry and delegates to
  `materialize_terminal_prefix_group(...)`; because the cached compact summary
  there carries no full evaluations and only one survivor sketch,
  materialization reuses that cached summary rather than reopening direct
  compact replay, relanding `generated = 3`, `admitted = 1`, a bar-clearing
  bound, and only the canonical `reference(15)` `103 / 8` survivor.
- The first post-materialize bucket/rank consumer above that handoff is now
  regression-backed too: on that same live reference prefix, summary
  pre-prune has already recorded the live `small_cluster` shell once at
  `3 / 1 / 1`; the post-materialize
  `record_demo_bucket_surface(...)` plus
  `record_demo_bucket_exact_screened(...)` pass lifts that running
  prefix-local total to `6 / 2 / 2`; the retained bound still clears the bar;
  no incumbent rank exists yet; and `best_prefix_group_accept_rank(...)`
  still relands only the canonical `reference(15)` `103 / 8` survivor.
- The candidate-cache handoff above that first post-materialize consumer is
  now regression-backed too: on that same live reference prefix,
  `cache_evaluated_terminal_prefix_group_candidates(...)` sorts the lone
  retained candidate, evaluates it once because no incumbent exists yet,
  seeds `discovery.terminal_rank_incumbent` with the canonical
  `reference(15)` `103 / 8` rank, and then compacts away the evaluated
  payload because `DesktopClaimShadow` stays on the compact claim path;
  `PrefixCache::record_group_with_surface_counts(...)` then records that same
  prefix as a one-survivor compact cache group with `generated = 3`,
  `admitted = 1`, the same bar-clearing bound, and the same canonical best
  accept rank.
- The first proof-close intake above that candidate-cache handoff is now
  regression-backed too: on that same live reference prefix,
  `load_terminal_prefix_group_for_proof_close(...)` immediately takes the
  compact one-survivor group back out of `PrefixCache`; the reloaded group
  still carries `generated = 3`, `admitted = 1`, the same bar-clearing
  bound, and the canonical `reference(15)` `103 / 8` survivor with no
  retained evaluated payload; `demo_bucket_key_for_group(...)` rebuckets that
  intake onto the `single` bucket so the earlier small-cluster same-primary
  relief no longer applies there; and the first intake stays live because
  proof-close starts with a fresh local incumbent rather than reusing the
  earlier candidate-cache incumbent.
- The proof-close candidate-order / candidate-level incumbent gate above that
  intake is now regression-backed too: on that same live reference prefix,
  `sort_terminal_prefix_group_candidates_for_certification(...)` still sees
  only the compact canonical `reference(15)` `103 / 8` survivor, so
  certification sort does not widen or reorder the live one-survivor claim
  group before evaluation; that sorted survivor still carries no retained
  evaluated payload; and the candidate-level
  `accept_rank_can_survive_incumbent(...)` gate stays open only because
  proof-close starts with no local incumbent, since same-primary relief is
  already closed on the rebucketed `single` bucket and the same canonical
  survivor would fail that gate once a same-rank local incumbent exists.
- The proof-close candidate evaluation / rank-recomputation / minimality layer
  above that gate is now regression-backed too: on that same live reference
  prefix, `evaluate_checked_candidate(...)` re-evaluates the compact canonical
  `reference(15)` survivor because proof-close kept no retained evaluated
  payload; `acceptance_rank(...)` recomputes the same canonical `103 / 8`
  accept rank from the fully scored survivor;
  `analyze_semantic_minimality(...)` keeps it semantically minimal with no
  admissible bar-clearing detachable subbundle; and proof-close seeds its
  fresh local incumbent only after that re-evaluation rather than by reusing
  `discovery.terminal_rank_incumbent`.
- The first post-evaluation fully scored bucket / proof-close full-eval
  accounting consumer above that proof-close evaluation layer is now
  regression-backed too: on that same live reference prefix, the rebucketed
  `single` pocket records the lone fully scored canonical `reference(15)`
  replay with `best_overshoot = 115657 / 21112`, the earlier `small_cluster`
  shell keeps zero fully scored candidates, both
  `demo_phase.materialize_full_evals` and
  `demo_phase.proof_close_full_evals` stay `0` on the live non-budgeted claim
  path, and the lone replay is carried instead by top-level
  `full_telescopes_evaluated = demo_funnel.full_telescopes_evaluated = 1`.
- The top-level full-telescope / demo-funnel accounting consumer above that
  post-evaluation layer is now regression-backed too: rebuilding the public
  funnel with `build_demo_funnel_stats(...)` relands the same live step-`15`
  surface on the canonical reference prefix, keeping
  `generated_raw_prefixes = 4331`,
  `full_telescopes_evaluated = 1`,
  `exact_bound_screened = exact_bound_pruned + 1`,
  `bar_clearers = 1`,
  `semantically_minimal_clearers = 1`, and canonical
  `winner_overshoot = 115657 / 21112`.
- The top-level demo-closure consumer above that funnel layer is now
  regression-backed too: rebuilding the public closure with
  `build_demo_closure_stats(...)` relands the same live step-`15` closure
  surface on the canonical reference prefix, keeping
  `frontier_total_seen = exact_bound_screened`,
  `frontier_certified_nonwinning = exact_bound_pruned`,
  `frontier_total_seen = frontier_certified_nonwinning + 1`, and
  `closure_percent = 99`.
- The report-side demo-closure handoff above that public closure builder is
  now regression-backed too: `step_to_report(...)` copies the live
  `DemoClosureStats` straight into `StepReport.search_stats.demo_closure`,
  serialized step reports keep the same step-`15` closure surface at
  `frontier_total_seen = 554`,
  `frontier_certified_nonwinning = 553`,
  `closure_percent = 99`, and `stored_demo_closure(...)` prefers that stored
  surface over any `demo_funnel` fallback whenever those stored counts are
  present.
- The first report/narrative closure render consumer above that handoff is now
  regression-backed too: `render_step_narrative(...)` feeds
  `closure_line(...)` from `stored_demo_closure(...)`, so even a drifted
  `demo_funnel` at `exact_bound_screened = 7` and
  `exact_bound_pruned = 1` still renders the stored
  `frontier_total_seen = 554`,
  `frontier_certified_nonwinning = 553`,
  `closure_percent = 99` surface; `render_debug_report(...)` keeps that same
  stored `demo closure:` line too.
- The first caller/output emission layer above that render consumer is now
  regression-backed too: `render_run_narrative(...)` keeps that same stored
  step-`15` closure line on the drifted claim run,
  `write_step_narrative_artifact(...)` persists it into
  `reports/steps/step-15-narrative.txt`, `render_run_output(...)` appends it
  when narrative output is requested, and `write_latest_reports(...)`
  persists `latest.debug.txt` from `render_debug_report(...)` with the same
  stored `frontier_total_seen = 554`,
  `frontier_certified_nonwinning = 553`,
  `closure_percent = 99` surface.
- The caller-side narrative toggle / emission-dispatch layer above that
  caller/output emission layer is now regression-backed too:
  `terminal_narrative_config(...)` keeps claim narrative output opt-in,
  `RunArtifactWriter::persist_step(...)` preserves the stored
  `frontier_total_seen = 554`,
  `frontier_certified_nonwinning = 553`,
  `closure_percent = 99` surface in drifted step summaries and step
  narratives, `RunArtifactWriter::finalize_success(...)` plus
  `finalize_failed_run(...)` / `RunArtifactWriter::finalize_failed(...)`
  keep that same stored surface in `latest.debug.txt`, and a completed
  `resume(...)` call reuses the stored closure rather than a drifted
  `7 / 1 / 14` fallback.
- The first downstream stored-artifact consumer in `scripts/compare_runs.py`
  above that caller-side dispatch layer is now regression-backed too:
  `has_demo_phase_evidence(...)` stays live on stored-closure evidence alone,
  `demo_phase_entry(...)` keeps the drifted step-`15` funnel on
  `4331 / 7 / 1 / 1 / 115657 / 21112` while preserving stored closure
  `554 / 553 / 99`, and `render_lane_summary(...)` keeps the
  `demo phase latest:` / `demo funnel latest:` lines on that stored `99%`
  closure rather than the drifted `14%` fallback.
- The compare-side audit consumer in `scripts/compare_runs.py` is now
  regression-backed too: `narrative_artifact_entry(...)` relands complete
  claim step artifacts, `build_claim_lane_audit(...)` /
  `summarize_claim_lane_audit(...)` keep a complete honest claim lane at
  `ready`, and `render_lane_summary(...)` keeps the
  `narrative artifacts:` / `claim audit:` / `search policy:` /
  `fallback honesty:` lines aligned with that ready audit surface.
- The top-level compare summary / signoff consumer in
  `scripts/compare_runs.py` is now regression-backed too:
  `build_summary(...)` / `signoff_summary(...)` keep a ready two-lane claim
  comparison on `signoff.status = ready` with
  `signoff.summary = all 2 lanes preserve baseline claim-baseline and the
  step-15 reference claim boundary`, `render_claim_lane_audit(...)` keeps
  `ready (claim-baseline, claim-current)`, and `render_text_summary(...)`
  now surfaces that signoff explanation directly in the top-level
  `Comparison Signoff:` line while preserving the top-level
  `claim lane audit:` line on that ready audit surface even against the known
  drifted `7 / 1 / 14` funnel plus stored closure `554 / 553 / 99`.
- The compare script caller/output layer in `scripts/compare_runs.py` is now
  regression-backed too: direct `unittest` coverage drives `main()` through
  real `--lane` / `--baseline` / `--text-out` / `--json-out` parsing on the
  ready claim-comparison surface, `write_text(...)` creates nested
  `summary.txt` and `summary.json` outputs, `--json-out` preserves
  `baseline_lane = claim-baseline` together with `signoff.status = ready`
  and `claim_lane_audit.status = ready`, and stdout matches the ready
  top-level `Comparison Signoff:` / `claim lane audit:` summary byte-for-byte
  with `--text-out`.
- The stored breadth/runtime/completeness certification consumers in
  `scripts/certify_claim_lane.py` are now regression-backed too: direct
  `unittest` coverage pins the passing helper surface for
  `breadth_check(...)`, `breadth_diagnosis(...)`,
  `runtime_threshold_check(...)`, `completeness_check(...)`, and
  `manifest_completeness_check(...)`, proves that the stored
  `4331 / 5000` step-`15` miss keeps the live `catalog = 64`,
  `widths = [2, 3, 4, 5, 6, 7, 8]`, root seeding `64 / 57 / 7`, and
  `ClaimGeneric` / `Modal@10` / `temporal_shell` claim-step-open surface
  through a real `step-15-live.ndjson` checkpoint, and drives `main()`
  through real CLI parsing on an isolated-failure certificate surface where
  only `early_breadth`, `late_generated_floors`, `runtime_threshold`,
  `exact_screen_reason_completeness`, `prune_class_completeness`, and
  `manifest_completeness` fail while parity, policy, fallback, and narrative
  checks stay green.
- The remaining certification text/output consumer in
  `scripts/certify_claim_lane.py` is now regression-backed too: direct
  `unittest` coverage pins `render_text_summary(...)` on the ready
  `Claim Certification:` surface, suppresses `failing checks:` on the pass
  path, and drives `main()` through real ready-path `--json-out` /
  `--text-out` parsing while preserving `status = ready`,
  `failing_checks = []`, nested certificate artifacts, and stdout parity with
  the written text summary.
- The first reintroduced late temporal-focus replay on the live seven-clause
  reference prefix is now regression-backed too: replaying that same prefix
  with `focus_family = TemporalShell` and
  `package_policies.temporal_shell = Prefer` reactivates one remaining family
  option but keeps no filtered terminal-clause list resident at remaining-one,
  closes `ClaimAdmittedOpenBand`, and still relands the same compact
  `generated = 3`, `admitted = 1`, bar-clearing single-survivor canonical
  `103 / 8` summary with zero pre-materialize terminal-prefix-bar and
  terminal-rank prunes.
- The first caller-level late-family-surface override replay on that same live
  seven-clause reference prefix is now regression-backed too: keeping the same
  live `DesktopClaimShadow` admissibility profile but overriding
  `LateFamilySurface` from `ClaimGeneric` to `DemoBreadthShadow` closes
  `ClaimAdmittedOpenBand`, leaves no cached family summary plus no filtered
  next-clause list on the work item, widens the remaining-one terminal catalog
  from `3` to `5`, and still relands a compact `generated = 5`,
  `admitted = 1`, `open_band_structural = 1`, bar-clearing single-survivor
  canonical `103 / 8` summary with zero pre-materialize terminal-prefix-bar
  and terminal-rank prunes.
- The first enumeration-context constructor above that late-family-surface
  override replay is now regression-backed too:
  `EnumerationContext::from_admissibility(...)` still derives
  `LateFamilySurface::ClaimGeneric` directly from
  `AdmissibilityMode::DesktopClaimShadow` on the live step-`15` claim
  profile; replaying that same live admissibility with a bypassed synthetic
  `focus_family = TemporalShell` plus
  `package_policies.temporal_shell = Prefer` still yields an identical
  enumeration context and the same live
  `reference + eventual_lift + next_lift` remaining-one catalog; only an
  explicit mode change to `DemoBreadthShadow` leaves that default
  claim-generic surface at constructor time.
- Focused current-machine Rust verification was rerun and remains green for
  the matching `pen-type`, `pen-search`, and `pen-cli` claim surfaces under
  the current filters
  `desktop_claim_shadow_step_`,
  `desktop_claim_shadow_step_fifteen_enumeration_context_derivation_stays_claim_generic_until_the_mode_changes`,
  `live_reference_reintroduced_temporal_focus`,
  `live_reference_late_family_surface_override`,
  `claim_generic_kappa_eight_catalog_adds_modal_temporal_exchange_variants`,
  `claim_open_band_terminal_clause_filter_`,
  `claim_step_fifteen_family_summary_stays_disabled_until_a_focus_family_is_reintroduced`,
  `current_claim_step_fifteen_live_`,
  `current_claim_step_fifteen_full_qualification_triad_`,
  `current_claim_step_fifteen_prefix_local_open_band_terminal_filtering_on_the_focused_three_no_bound_flat_shell_prefixes`,
  `current_claim_step_fifteen_focused_three_no_bound_flat_shell_prefixes_`,
  `current_claim_step_fifteen_focused_three_no_bound_flat_shell_pair_cell_summary_override_only_reclassifies_the_exact_partial_prefix_gate_to_unknown`,
  and `current_claim_step_fifteen_`, while direct
  `compare_runs.py` / `certify_claim_lane.py` execution still awaits a
  `python`, `python3`, `py`, or `uv` launcher.
- The first raw-catalog / scout consumer above that live claim-generic
  band-`8` emitter is now regression-backed too: direct `pen-search` coverage
  pins the step-`15` `claim_regular_clause_catalog` checkpoint at raw widths
  `3 / 3 / 3 / 3 / 3 / 3 / 3 / 3`, raw telescopes `6561`, zero generated
  surface, and pre-root-seeding `3 / 0 / 0 / 0`, while the matching
  `claim_root_seeding_summary` checkpoint keeps that same `6561` raw catalog
  and shrinks only to `generated_raw_surface = 3`, `prefixes_created = 3`,
  `frontier_queue_len = 3`, and public root seeding `3 / 0 / 0 / 3`.
- The first in-flight discovery checkpoint above that scout branch is now
  regression-backed too: direct `pen-search` coverage pins
  `current_claim_step_fifteen_live_regular_frontier_progress_checkpoint_keeps_the_same_three_root_shell_before_queue_pop`,
  and the engine now force-emits the first
  `claim_regular_frontier_progress` note even on fast runs while later loop
  emissions stay throttled. That first in-loop checkpoint still keeps the same
  `clause_kappa = 8`, raw widths `3 / 3 / 3 / 3 / 3 / 3 / 3 / 3`, raw
  telescopes `6561`, zero prefix-state exploration, zero exact-screen prunes,
  and the same public root seeding `3 / 0 / 0 / 3` before any queue-pop.
- The next honest slice now moves above the fully pinned compare /
  certification evidence surface, the exhausted broader-backup lattice, the
  whole qualification-triad exact-shell remaining-one lattice, the newly
  frozen pure temporal-focus replay, the newly frozen caller-level
  late-family-surface override replay, the newly frozen constructor-level
  claim-generic derivation, the newly frozen raw claim-generic band-`8`
  emitter / selector, the newly frozen regular-catalog plus root-seeding
  scout branch, the newly frozen first in-loop discovery checkpoint, the
  newly frozen live seven-clause reference-prefix queue-pop / post-pop /
  proof-close stack, and the newly frozen first post-reference
  `claim_regular_frontier_remainder` checkpoint above that canonical branch;
  keep the exact live move in
  [../../autonomous_next_steps.md](../../autonomous_next_steps.md).
- The first post-reference exact-two-step frontier family above that remainder
  checkpoint is now regression-backed too: direct `pen-search` coverage pins
  the exact `12` queued post-reference labels plus frontier work keys, and the
  test-only blanket `DemoBreadthShadow` override across all queued exact-two-
  step children widens the local step-`15` surface from `4331` to `8507`
  while keeping `partial_prefix_bar_failure = 553`, accepted `103 / 8`, no
  fully scored lifted `89 / 8`, the fenced `single` pocket, and
  `small_cluster = 1566 / 522 / 522 / 0`; treat that as proof of breadth
  headroom rather than as the final landed repair.
- That same frontier family is now localized too: direct `pen-search`
  coverage also pins the per-branch scoped replay, so the queued remainder
  branches can be widened one by one without touching unrelated claim
  exact-two-step surfaces; every scoped replay preserves `553`, accepted
  `103 / 8`, the fenced `single` pocket, and zero fully scored lifted
  `89 / 8`, while the dominant localized gains sit on the top-level queued
  `reference` remainder at `5915 / 2538` and the top-level queued
  `claim_eventual_domain` remainder at `5627 / 2646`, and the two queued
  `4`-slot `claim_flat_domain / reference / reference` controls stay dead at
  the baseline `4331 / 3132` surface.
- The combined dominant pair above that localized frontier family is now the
  landed local repair too: direct `pen-search` coverage pins the top-level
  `reference + claim_eventual_domain` ancestry replay at
  `generated_raw_prefixes = 7211` with
  `small_cluster = 2052 / 522 / 522 / 0`, and the non-test
  `DesktopClaimShadow` exact-two-step path now matches that same widened
  surface while still preserving `partial_prefix_bar_failure = 553`,
  accepted `103 / 8`, the fenced `single` pocket, and zero fully scored
  lifted `89 / 8`; keep that pair as the current landed local repair and use
  the earlier scout controls only as regression evidence while stored
  evidence remains open.

## Stable Working Invariants

- Prefer stored evidence over terminal impressions.
- Keep the accepted path fixed until stored evidence clearly replaces it.
- The post-probe rerun has now reconfirmed the same step-`15` miss on newer
  code, so keep step-`15` repair work ahead of step-`1` theory work unless a
  later stored bundle changes the diagnosis.
- Do not reopen the blanket post-reference exact-two-step scout,
  per-branch localization map, or combined dominant-pair scout as the active
  work order now that the ancestry-gated local repair is landed; use them only
  as regression controls until stored evidence says otherwise.
- Do not use stronger wording such as `unguided` before certification passes.
- Do not treat the lane as family-agnostic end to end while stored breadth is
  still open.
- Keep stronger-than-canonical lifted terminals fenced unless new stored
  evidence explicitly proves they are part of the canonical path.
- Keep PowerShell assumptions explicit in workflow notes and avoid POSIX-style
  command chaining in examples.

## Operational Memory Map

- [../../autonomous_progress.md](../../autonomous_progress.md)
  Live operational snapshot: canonical bundle, current counters, current
  diagnosis.
- [../../autonomous_next_steps.md](../../autonomous_next_steps.md)
  Single active work order for the next slice.
- [../../autonomous_plan.md](../../autonomous_plan.md)
  Medium-horizon phases and exit criteria.
- [../../autonomous_checklist.md](../../autonomous_checklist.md)
  Binary signoff gates only.
- [../../autonomous_ledger.md](../../autonomous_ledger.md)
  Append-only probe history and decisions.

## First Reads

- [../../docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [../../autonomous_progress.md](../../autonomous_progress.md)
- [../../autonomous_next_steps.md](../../autonomous_next_steps.md)
- [../../autonomous_plan.md](../../autonomous_plan.md)
- [../../autonomous_checklist.md](../../autonomous_checklist.md)
- [../../autonomous_ledger.md](../../autonomous_ledger.md)
- [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)

## Evidence Surfaces

Use these before changing search code:

- `scripts/certify_claim_lane.py` for stored pass/fail plus failing-step
  breadth anatomy
- `scripts/compare_runs.py` for parity and artifact honesty
- `scripts/benchmark_claim_lane.py` for stored runtime and breadth summaries
- `reports/steps/step-15-live.ndjson` when the task touches late-step pressure
- `run.json` and step summaries when the task touches stored provenance or the
  ordering between a rerun-backed step-`15` reset and a later step-`1`
  reopening

## Do And Do Not

Do:

- keep the claim lane separate from demo-only behavior
- preserve existing guarded, realistic, and demo behavior while the claim lane
  changes
- keep claim-lane edits narrow and staged
- use the certificate first when you need stored breadth anatomy
- compare the earlier and current canonical reruns plus `run.json` build
  fingerprints before reopening step `1`, and if the newer rerun reconfirms
  the same breadth-only miss, keep step-`15` repair ahead of step `1`
- use the autonomous files as intended instead of restating live state here
- once the live docs say the representative mismatch-`0` parent-route,
  qualification-family, clause-`4`, and clause-`2` selector lattice is spent,
  start with the step-`15` exact-screen engine path before inventing another
  connectivity family:
  `screen_prefix_for_frontier(...)`,
  `exact_partial_prefix_bound_decision(...)`,
  `exact_terminal_prefix_bound_decision(...)`, and
  `claim_try_summary_prune_before_materialization(...)`; once the live docs
  also mark that post-pop collapse/summary handoff as spent on the live
  seven-clause reference prefix, move above it to
  `materialize_remaining_one_prefix_group(...)` and
  `materialize_terminal_prefix_group(...)`

Do not:

- restate live counters in this file
- treat negative controls as if they were still open hypotheses
- spend another turn on rerun-vs-step-`1` ordering after the current
  canonical rerun has already reconfirmed the same breadth-only miss on newer
  code
- reopen or re-summarize the exhausted mismatch-`1`
  `reference + demo_flat_codomain` ladder first; the live off-branch priority
  order now belongs in the autonomous docs rather than in this stable
  reference
- reopen the exact claim-pair clause-`4` `reference` side first; that narrower
  relocalization now only reproduces the older broader clause-`4`
  `reference`-sheet tradeoff and belongs in the autonomy ledger rather than as
  a fresh lead
- reopen the broader remaining-two exact partial-prefix relief on the whole
  mismatch-`0` clause-`4` `claim_next_bridge` half first either; if the live
  docs already pin that release at canonical `103 / 8` with
  `4763 / 517 / 2271`, a fenced `single` pocket, and widened
  `small_cluster = 3456 / 522 / 522 / 0` while every live mismatch-`0`
  clause-`4` / clause-`5` cell equalizes to `36`, that broader exact-screen
  release also belongs to the autonomy ledger rather than as a fresh repair
  lead
- split the exact claim-pair clause-`4` `claim_next_bridge` half into the
  claim-flat or claim-sharp single-sheet relocalizations first; those smaller
  tradeoff controls now also belong in the autonomy ledger rather than as a
  fresh lead
- reopen the broad clause-`1` `demo_flat_codomain` reopening across the full
  mismatch-`0` claim-domain surface first; that control is now a widening
  negative result owned by the autonomy ledger rather than a fresh lead
- reopen the narrower clause-`4` `claim_next_bridge`-side relocalization under
  that mismatch-`0` surface first; that smaller negative result also belongs in
  the autonomy ledger rather than as a fresh lead
- reopen the narrower clause-`4` `reference`-side relocalization under that
  same mismatch-`0` surface first; that sharper negative result now also
  belongs in the autonomy ledger rather than as a fresh lead
- reopen remaining-one exact-summary relief on the mismatch-`0` clause-`4`
  `reference` plus clause-`5` `reference` tail first; that deeper negative
  control now also belongs in the autonomy ledger rather than as a fresh lead
- reopen the whole mismatch-`0` clause-`4` `claim_next_bridge`-half
  remaining-one exact-summary relief as if it were already the landed repair;
  that wall-narrowing tradeoff still belongs in the autonomy docs and ledger
  rather than as a stable resolved lead
- reopen the narrower remaining-two exact partial-prefix clause-`5`
  `claim_flat_codomain`, `claim_next_codomain`, or `reference` split first;
  if the live docs already pin all three cells at the same
  `4475 / 541 / 2271` smaller tradeoff with widened
  `small_cluster = 3240 / 522 / 522 / 0`, the same fenced `single` pocket,
  the same `42 -> 40` pair contraction, and the same `24 / 18 -> 22 / 18`
  clause-`4` split, those exact-screen cell relands also belong in the
  autonomy ledger rather than as fresh repair leads
- reopen the narrower exact-screen clause-`0` / clause-`1` pair split inside
  the representative `claim_next_codomain` cell first either; if the live
  docs already pin all six pair probes there at the same
  `4355 / 551 / 2271` smaller tradeoff with fixed zero-admitted captures
  `2271`, widened `small_cluster = 3150 / 522 / 522 / 0`, the same fenced
  `single` pocket, a targeted `42 -> 40` pair contraction, a targeted
  `24 / 18 -> 22 / 18` clause-`4` split, and a targeted
  `48 -> 46` `claim_next_bridge` plus `claim_next_codomain` bucket
  contraction, that exact-screen pair lattice also belongs to the autonomy
  ledger rather than as a fresh repair lead
- reopen the representative exact-screen pair-cell clause-`2` split inside
  that same `claim_next_codomain` cell first either; if the live docs
  already pin the `claim_flat_domain` and `claim_sharp_codomain` sheets at
  the same `4343 / 552 / 2271` smaller tradeoff with first-mismatch counts
  `311 / 177 / 50 / 14`, `small_cluster generated = 3141`, and a targeted
  `23 / 18` clause-`4` split plus `47` active clause-`5` bucket, while the
  sibling `reference` sheet is neutral on the untouched
  `4331 / 553 / 2271` baseline, that exact-screen clause-`2` axis also
  belongs to the autonomy ledger rather than as a fresh repair lead
- reopen the representative exact-screen `claim_flat_domain` clause-`6`
  identity split inside that same `claim_next_codomain` cell first either; if
  the live docs already pin that claim-flat clause-`2` tradeoff to exactly
  one removed six-clause exact-prune capture with no introduced capture or
  pruned-prefix family, clause `6` is still out of scope at that
  exact-screen boundary and that axis also belongs to the autonomy ledger
  rather than as a fresh repair lead
- reopen remaining-one exact-summary relief beneath that released exact-screen
  representative `claim_flat_domain` plus clause-`5`
  `claim_next_codomain` parent first either; if the live docs already pin the
  stacked follow-on as a neutral control on the same `4343 / 552 / 2271`
  shell with first-mismatch counts `311 / 177 / 50 / 14`,
  `small_cluster generated = 3141`, the same zero-admitted exact-prune
  family `((0, None, None), 2271)`, and no released remaining-one
  pruned-terminal group, that axis also belongs to the autonomy ledger rather
  than as a fresh repair lead
- reopen direct remaining-one completion or terminal-scope identity beneath
  that same released exact-screen representative `claim_flat_domain` plus
  clause-`5` `claim_next_codomain` parent first either; if the live docs
  already pin all three clause-`6` continuations at the same dead
  `3`-generated / `0`-admitted completion summary with no bound, no
  best-rank profile, no survivor sketch, the same
  `reference / eventual_lift / next_lift` `NeedsFallback` trio, and the same
  clause-`2` blocker with `max_lib_ref = 10`, that claim-flat clause-`6`
  axis also belongs to the autonomy ledger rather than as a fresh repair
  lead
- reopen the representative exact-screen `claim_sharp_codomain` clause-`6`
  identity split inside that same `claim_next_codomain` cell first either; if
  the live docs already pin that claim-sharp clause-`2` tradeoff to exactly
  one removed six-clause exact-prune capture with no introduced capture or
  pruned-prefix family, clause `6` is still out of scope at that exact-screen
  boundary and that claim-sharp clause-`6` axis also belongs to the autonomy
  ledger rather than as a fresh repair lead
- reopen remaining-one exact-summary relief beneath that released exact-screen
  representative `claim_sharp_codomain` plus clause-`5`
  `claim_next_codomain` parent first either; if the live docs already pin the
  stacked follow-on as a neutral control on the same `4343 / 552 / 2271`
  shell with first-mismatch counts `311 / 177 / 50 / 14`,
  `small_cluster generated = 3141`, the same zero-admitted exact-prune
  family `((0, None, None), 2271)`, and no released remaining-one
  pruned-terminal group, that claim-sharp summary-relief axis also belongs to
  the autonomy ledger rather than as a fresh repair lead
- reopen direct remaining-one completion or terminal-scope identity beneath
  that same released exact-screen representative `claim_sharp_codomain` plus
  clause-`5` `claim_next_codomain` parent first either; if the live docs
  already pin all three clause-`6` continuations at the same dead
  `3`-generated / `0`-admitted completion summary with no bound, no
  best-rank profile, no survivor sketch, the same
  `reference / eventual_lift / next_lift` `NeedsFallback` trio, and the same
  clause-`2` blocker with `max_lib_ref = 10`, that claim-sharp clause-`6`
  axis also belongs to the autonomy ledger rather than as a fresh repair
  lead
- reopen the same representative exact-screen pair beneath
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  claim_next_codomain` first either; if the live docs already pin both
  claim-side sheets and their deeper remaining-one follow-ons as spent while
  the sibling `reference` sheet stays neutral, that representative pair also
  belongs to the autonomy ledger rather than as a fresh repair lead, and the
  next honest slice should move to a sibling exact-screen pair on the same
  `claim_next_codomain` cell instead
- reopen the first sibling exact-screen pair's clause-`2` split beneath
  `claim_eventual_domain / claim_sharp_codomain / claim_next_bridge /
  claim_next_codomain` first either; if the live docs already pin the
  `claim_flat_domain` and `claim_sharp_codomain` sheets at the same
  `4343 / 552 / 2271` smaller tradeoff with first-mismatch counts
  `311 / 177 / 50 / 14`, `small_cluster generated = 3141`, and the targeted
  pair at `41`, while the sibling `reference` sheet stays neutral on the
  untouched `4331 / 553 / 2271` baseline with the targeted pair still at
  `42`, that sibling pair clause-`2` axis also belongs to the autonomy
  ledger rather than as a fresh repair lead, and the next honest slice
  should move below the representative `claim_flat_domain` sheet on that
  sibling pair instead
- reopen the first sibling exact-screen `claim_flat_domain` sheet beneath
  `claim_eventual_domain / claim_sharp_codomain / claim_next_bridge /
  claim_next_codomain` first either; if the live docs already pin its
  remaining-two delta to one released six-clause parent, pin its stacked
  remaining-one exact-summary relief as neutral on the same
  `4343 / 552 / 2271` shell, and pin its direct clause-`6` completion split
  to the same dead `3`-generated / `0`-admitted `NeedsFallback` trio, that
  first sibling claim-flat branch also belongs to the autonomy ledger rather
  than as a fresh repair lead, and the next honest slice should move sideways
  to the sibling `claim_sharp_codomain` sheet on that same pair instead
- reopen the first sibling exact-screen `claim_sharp_codomain` sheet beneath
  `claim_eventual_domain / claim_sharp_codomain / claim_next_bridge /
  claim_next_codomain` first either; if the live docs already pin its
  remaining-two delta to one released six-clause parent, pin its stacked
  remaining-one exact-summary relief as neutral on the same
  `4343 / 552 / 2271` shell, and pin its direct clause-`6` completion split
  to the same dead `3`-generated / `0`-admitted `NeedsFallback` trio, that
  first sibling claim-sharp branch also belongs to the autonomy ledger rather
  than as a fresh repair lead, and the next honest slice should move sideways
  to the next sibling exact-screen pair on the same `claim_next_codomain`
  cell instead
- reopen the next sibling exact-screen pair's clause-`2` split beneath
  `claim_eventual_domain / reference / claim_next_bridge / claim_next_codomain`
  first either; if the live docs already pin the `claim_flat_domain` and
  `claim_sharp_codomain` sheets at the same `4343 / 552 / 2271` smaller
  tradeoff with first-mismatch counts `311 / 177 / 50 / 14`,
  `small_cluster generated = 3141`, and the targeted pair at `41`, while the
  sibling `reference` sheet stays neutral on the untouched
  `4331 / 553 / 2271` baseline with the targeted pair still at `42`, that
  next-sibling clause-`2` axis also belongs to the autonomy ledger rather
  than as a fresh repair lead, and the next honest slice should move below
  the next sibling `claim_flat_domain` sheet on that pair instead
- reopen the next sibling exact-screen `claim_flat_domain` sheet beneath
  `claim_eventual_domain / reference / claim_next_bridge / claim_next_codomain`
  first either; if the live docs already pin its remaining-two delta to one
  released six-clause parent, pin its stacked remaining-one exact-summary
  relief as neutral on the same `4343 / 552 / 2271` shell, and pin its
  direct clause-`6` completion split to the same dead
  `3`-generated / `0`-admitted `NeedsFallback` trio, that next-sibling
  claim-flat branch also belongs to the autonomy ledger rather than as a
  fresh repair lead, and the next honest slice should move sideways to the
  sibling `claim_sharp_codomain` sheet on that same pair instead
- reopen the sibling active mismatch-`0` clause-`5` `reference` family's
  representative `claim_flat_domain` clause-`6` siblings beneath
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  first either; if the live docs already pin the `claim_next_codomain`,
  `claim_sharp_codomain`, and `reference` continuations at the same
  `4343 / 552` shell with `small_cluster = 3141 / 522 / 522 / 0`,
  representative clause-`2` spread `14 / 15 / 12`, representative
  clause-`4` split `23 / 18`, the active `claim_next_bridge` plus
  clause-`5` `reference` bucket at `47`, and only a tiny deeper
  zero-admitted tail delta `2270 / 2269 / 2268`, that clause-`6` axis also
  belongs to the autonomy ledger rather than as a fresh repair lead, and
  the next honest slice should move below the marginally best clause-`6`
  `reference` continuation on that same sheet instead
- reopen the first clause-`0` sibling exact-screen `claim_sharp_codomain`
  sheet beneath
  `claim_flat_domain / claim_next_codomain / claim_next_bridge / claim_next_codomain`
  first either; if the live docs already pin its remaining-two delta to one
  released six-clause parent, pin its stacked remaining-one exact-summary
  relief as neutral on the same `4343 / 552 / 2271` shell, and pin its
  direct clause-`6` completion split to the same dead
  `3`-generated / `0`-admitted `NeedsFallback` trio, that first-clause-`0`-
  sibling claim-sharp branch also belongs to the autonomy ledger rather than
  as a fresh repair lead, and the next honest slice should move sideways to
  the first clause-`1` sibling exact-screen pair on the same clause-`0`
  `claim_flat_domain` family,
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`,
  instead
- reopen the first clause-`1` sibling exact-screen pair's clause-`2` split
  beneath
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`
  first either; if the live docs already pin the `claim_flat_domain` and
  `claim_sharp_codomain` sheets at the same `4343 / 552 / 2271` smaller
  tradeoff with first-mismatch counts `311 / 177 / 50 / 14`,
  `small_cluster generated = 3141`, and the targeted pair at `41`, while the
  sibling `reference` sheet stays neutral on the untouched
  `4331 / 553 / 2271` baseline with the targeted pair still at `42`, that
  first-clause-`1`-sibling clause-`2` axis also belongs to the autonomy
  ledger rather than as a fresh repair lead, and the next honest slice
  should move below the representative `claim_flat_domain` sheet on that pair
  instead
- reopen the first clause-`1` sibling exact-screen `claim_flat_domain` sheet
  beneath
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`
  first either; if the live docs already pin its remaining-two delta to one
  released six-clause parent shell, keep clause `6` out of scope at that
  exact-screen boundary and treat that same-boundary claim-flat sheet as
  spent rather than as fresh leverage
- reopen remaining-one exact-summary relief beneath that released first
  clause-`1` sibling exact-screen `claim_flat_domain` plus clause-`5`
  `claim_next_codomain` parent first either; if the live docs already pin the
  stacked follow-on as neutral on the same `4343 / 552 / 2271` shell with
  first-mismatch counts `311 / 177 / 50 / 14`,
  `small_cluster generated = 3141`, the same zero-admitted exact-prune
  family `((0, None, None), 2271)`, the same `6813` structurally connected
  but unqualified generated candidates, and no released remaining-one
  pruned-terminal group, keep that summary-relief axis in the autonomy docs
  and ledger rather than as a fresh repair lead
- reopen direct remaining-one completion or terminal-scope identity beneath
  that same released first clause-`1` sibling exact-screen
  `claim_flat_domain` plus clause-`5` `claim_next_codomain` parent first
  either; if the live docs already pin all three clause-`6` continuations as
  the same dead `3`-generated / `0`-admitted completion summary with
  `matched_clause_count = 2`, `first_mismatch_position = 2`, no bound, no
  best-rank profile, no survivor sketch, and only the same nonlive
  `reference / eventual_lift / next_lift` terminal trio with
  `NeedsFallback` and `max_lib_ref = 10`, route sideways to the sibling
  `claim_sharp_codomain` sheet instead of reopening the spent claim-flat
  parent or the neutral `reference` sheet there as if they were fresh
  leverage
- reopen the first clause-`1` sibling exact-screen `claim_sharp_codomain`
  sheet beneath
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`
  first either; if the live docs already pin its remaining-two delta to one
  released six-clause parent, pin its stacked remaining-one exact-summary
  relief as neutral on the same `4343 / 552 / 2271` shell with the same
  `311 / 177 / 50 / 14`, the same `small_cluster generated = 3141`, the same
  zero-admitted exact-prune family `((0, None, None), 2271)`, and no released
  remaining-one pruned-terminal group, and pin its direct clause-`6`
  completion split to the same dead `3`-generated / `0`-admitted
  `NeedsFallback` trio with `matched_clause_count = 2`,
  `first_mismatch_position = 2`, and `max_lib_ref = 10`, that first
  clause-`1` sibling claim-sharp branch also belongs to the autonomy ledger
  rather than as a fresh repair lead; once the live docs also pin the next
  clause-`1` sibling exact-screen pair on that same clause-`0`
  `claim_flat_domain` family,
  `claim_flat_domain / reference / claim_next_bridge / claim_next_codomain`,
  at the same `4343 / 552 / 2271` smaller clause-`2` tradeoff with the
  targeted pair at `41` while the sibling `reference` sheet stays neutral at
  `42`, and once the live docs also pin that pair's representative
  `claim_flat_domain` sheet through its same-boundary one-parent reland,
  neutral remaining-one exact-summary follow-on, and dead clause-`6`
  completion split, and once the live docs also pin that pair's sibling
  `claim_sharp_codomain` sheet through its matching same-boundary one-parent
  reland, neutral remaining-one exact-summary follow-on, and dead
  clause-`6` completion split, the exact-screen `claim_next_codomain` cell
  also belongs to the autonomy ledger; the next honest slice should move
  sideways to the sibling active mismatch-`0` clause-`5` `reference`
  family, below its representative `claim_flat_domain` clause-`2` sheet at
  finer remaining-one exact-summary scope under
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  instead
- reopen the whole-cell mismatch-`0` clause-`4` `claim_next_bridge` plus
  clause-`5` `claim_flat_codomain` or clause-`5` `reference` exact-summary
  tradeoffs first; those smaller tradeoff controls now also belong in the
  autonomy ledger rather than as stable resolved leads
- reopen the pair-cell subprobes below those active clause-`5`
  `claim_flat_codomain / reference` cells first; those deeper relands are now
  also symmetric smaller tradeoff controls owned by the autonomy ledger rather
  than fresh leads
- reopen the representative pair-cell clause-`2` identity split first; those
  two claim-side clause-`2` sheets are now matching smaller tradeoff controls
  while the sibling reference sheet is neutral, so that axis also belongs to
  the autonomy ledger rather than as a fresh lead
- reopen the sibling active mismatch-`0` clause-`5` `reference` family's
  representative pair-cell clause-`2` identity split first; if the live docs
  already localize it to the same `4343 / 552 / 2268` smaller tradeoff shell
  on the two claim-side sheets with the sibling reference sheet neutral at
  `4331 / 553 / 2271`, that axis also belongs to the autonomy ledger rather
  than as a fresh lead
- reopen the representative `claim_flat_domain` clause-`2` sheet's clause-`6`
  identity split first; those `claim_next_codomain`,
  `claim_sharp_codomain`, and `reference` continuations are now matching
  smaller tradeoff controls too, so that axis also belongs to the autonomy
  ledger rather than as a fresh lead
- reopen the deeper representative mismatch-`0` claim-side clause-`6`
  `reference` union first either; if the live docs already pin that
  cross-sheet union at the same `4355 / 551 / 2265` smaller pair-cell
  tradeoff with `small_cluster = 3150 / 522 / 522 / 0`, the same
  zero-admitted `((0, None, None), 2265)` remaining-one family, the full
  `+18` `small_cluster` widening localized to six released
  `3`-generated / `0`-admitted `NeedsFallback` groups, no cached bound, and
  no hidden live pocket, that union also belongs to the autonomy ledger
  rather than as a fresh lead
- reopen the representative mismatch-`0` claim-side parent-route first
  either; if the live docs already pin both scoped historical-reanchor routes
  on the active clause-`5` `claim_flat_codomain / reference` families at the
  same unsafe `4427 / 545 / 2247` negative-control shell with noncanonical
  `60 / 8`, `incumbent_dominance = 117`,
  `small_cluster = 2931 / 455 / 455 / 115`, a reopened `single` bucket, and
  a delta localized only to the four targeted claim-side remaining-two parent
  cells plus their `24` matching remaining-one pruned prefixes on the chosen
  active clause-`5` bucket, those parent-route probes also belong to the
  autonomy ledger rather than as fresh repair leads
- reopen the representative mismatch-`0` claim-side parent-route clause-`4`
  `reference` split first either; if the live docs already pin both active
  clause-`5` `claim_flat_codomain / reference` probes at the same unsafe
  `4391 / 557 / 2259` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 113`, `small_cluster = 2871 / 435 / 435 / 111`, the
  same reopened `single` bucket, a shifted `19 / 19 / 12` clause-`2` spread,
  a shifted `24 / 26` clause-`4` split, and a delta that only swaps two
  remaining-three clause-`4` `reference` parent captures into four targeted
  remaining-two clause-`4` `reference` capture families plus `12` removed
  pruned prefixes on the chosen active clause-`5` bucket, that narrower route
  split also belongs to the autonomy ledger rather than as a fresh repair
  lead
- reopen the representative mismatch-`0` claim-side parent-route clause-`4`
  `claim_next_bridge` split first either; if the live docs already pin both
  active clause-`5` `claim_flat_codomain / reference` probes at the broader
  unsafe `4427 / 545 / 2271` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 117`, `small_cluster = 2931 / 455 / 455 / 115`, the
  same reopened `single` bucket, the same `15 / 15 / 12` clause-`2` spread,
  the same `24 / 18` clause-`4` split, and the same four-cell plus
  `24`-pruned-prefix targeted delta on the chosen active clause-`5` bucket,
  that broader route split also belongs to the autonomy ledger rather than as
  a fresh repair lead
- reopen representative mismatch-`0` claim-side clause-`2` sheet identity
  first either; if the live docs already pin the parent-route
  `claim_flat_domain / claim_sharp_codomain` sheets at unsafe `4379 / 549`
  with noncanonical `60 / 8`, `retained = 1`, and
  `small_cluster = 2871 / 435 / 435 / 111`, the sibling parent-route
  `reference` sheet at unsafe noncanonical `74 / 8` with the same
  `4379 / 549` and `small_cluster = 2844 / 426 / 426 / 120`, and the
  alternate active-window / self-contained clause-`2` sheets as matched
  smaller unsafe controls with the sibling alternate `reference` sheet
  reopening a two-retained unsafe `60 / 8` at
  `small_cluster = 2844 / 468 / 468 / 127`, that clause-`2` axis also
  belongs to the autonomy ledger rather than as a fresh repair lead
- keep treating the representative mismatch-`0` connectivity overrides as
  regression baselines once that lattice is spent; the next honest slice then
  lives in engine-side exact screening above those frozen controls rather than
  in another route / qualification / clause selector reland
- reopen the first alternate representative mismatch-`0` claim-side
  active-window qualification family first either; if the live docs already
  pin both active clause-`5` `claim_flat_codomain / reference` probes at the
  same unsafe `4427 / 545 / 2247` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same four targeted remaining-two parent cells plus their
  `24` matching remaining-one pruned prefixes, that alternate qualification
  family also belongs to the autonomy ledger rather than as a fresh repair
  lead
- reopen the next alternate representative mismatch-`0` claim-side
  self-contained qualification family first either; if the live docs already
  pin both active clause-`5` `claim_flat_codomain / reference` probes at the
  same unsafe `4427 / 545 / 2247` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same four targeted remaining-two parent cells plus their
  `24` matching remaining-one pruned prefixes, that alternate qualification
  family also belongs to the autonomy ledger rather than as a fresh repair
  lead
- reopen the matched alternate active-window and self-contained clause-`4`
  `claim_next_bridge` splits first either; if the live docs already pin both
  active clause-`5` `claim_flat_codomain / reference` probes at the broader
  unsafe `4427 / 545 / 2271` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`, `small_cluster = 2952 / 558 / 558 / 108`, the
  same reopened `single` bucket, the same `15 / 15 / 12` clause-`2` spread,
  the same `24 / 18` clause-`4` split, and the same four-cell plus
  `24`-pruned-prefix targeted delta, those broader branch splits also belong
  to the autonomy ledger rather than as fresh repair leads
- reopen the narrower clause-`6` `reference` refinement of those same
  alternate active-window or self-contained families first either; if the
  live docs already pin `claim_flat_codomain` at unsafe noncanonical `60 / 8`
  with `retained = 4`, `incumbent_dominance = 113`,
  `small_cluster = 2904 / 462 / 462 / 109 / 2`, and pin `reference` at the
  same unsafe `60 / 8` with `retained = 2`,
  `incumbent_dominance = 115`, the same
  `2904 / 462 / 462 / 109 / 2` `small_cluster`, no `single` bucket, best
  overshoot `545 / 5278`, and the same four targeted remaining-two parent
  cells plus their `24` matching remaining-one pruned prefixes, those
  narrower alternate refinements also belong to the autonomy ledger rather
  than as fresh repair leads
- reopen the narrower claim-flat clause-`3`
  `claim_flat_argument / claim_eventual_argument` refinement of those same
  alternate active-window or self-contained families first either; if the
  live docs already pin both branches on the active `claim_flat_codomain`
  bucket at unsafe noncanonical `60 / 8` with `retained = 2`,
  `generated = 4379`, `partial_prefix_bar_failure = 549`,
  `incumbent_dominance = 110`, zero-admitted captures `2259`,
  `small_cluster = 2880 / 486 / 486 / 108`, the same reopened `single`
  bucket, and the same four targeted remaining-two parent cells plus their
  `12` matching remaining-one pruned prefixes, those narrower alternate
  clause-`3` refinements also belong to the autonomy ledger rather than as
  fresh repair leads
- reopen the narrower clause-`6` `reference` refinement of that same
  representative mismatch-`0` claim-side parent-route first either; if the
  live docs already pin both active clause-`5`
  `claim_flat_codomain / reference` refinements at unsafe noncanonical
  `74 / 8` with `retained = 1`, the same `4427 / 545 / 2247`,
  `incumbent_dominance = 111`,
  `small_cluster = 2904 / 430 / 430 / 108`, a still-fenced `single` bucket,
  and the same four targeted remaining-two parent cells plus their `24`
  matching remaining-one pruned prefixes, that narrower refinement also
  belongs to the autonomy ledger rather than as a fresh repair lead
- reopen recombined representative mismatch-`0` claim-side parent-route plus
  alternate qualification families first either; if the live docs already pin
  both historical-reanchor plus active-window and historical-reanchor plus
  self-contained hybrids across the active clause-`5`
  `claim_flat_codomain / reference` buckets at the same unsafe
  `4427 / 545 / 2247` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, and the same reopened `single`
  bucket, those recombined families also belong to the autonomy ledger rather
  than as fresh repair leads
- reopen the first looser representative mismatch-`0` claim-side
  active-window plus self-contained recombination first either; if the live
  docs already pin both active clause-`5`
  `claim_flat_codomain / reference` probes at that same unsafe
  `4427 / 545 / 2247` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same four targeted remaining-two parent cells plus their
  `24` matching remaining-one pruned prefixes, that looser recombined family
  also belongs to the autonomy ledger rather than as a fresh repair lead
- reopen the full representative mismatch-`0` claim-side
  historical-reanchor plus active-window plus self-contained qualification
  triad first either; if the live docs already pin both active clause-`5`
  `claim_flat_codomain / reference` probes at that same unsafe
  `4427 / 545 / 2247` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same four targeted remaining-two parent cells plus their
  `24` matching remaining-one pruned prefixes, that full qualification triad
  also belongs to the autonomy ledger rather than as a fresh repair lead
- reopen the alternate active-window or self-contained clause-`4`
  `reference` split first either; if the live docs already pin both active
  clause-`5` `claim_flat_codomain / reference` probes at the same smaller
  unsafe `4391 / 557 / 2271` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2880 / 486 / 486 / 108`, the same reopened `single`
  bucket, the same `15 / 15 / 12` clause-`2` spread, the same
  `24 / 18` clause-`4` split, and the same two-remaining-three-to-four-
  remaining-two targeted clause-`4` `reference` delta, that alternate
  clause-`4` split also belongs to the autonomy ledger rather than as a
  fresh repair lead
- reopen the representative claim-flat clause-`3` refinement inside that same
  mismatch-`0` claim-side parent-route family first either; if the live docs
  already pin both `claim_flat_argument / claim_eventual_argument` branches on
  the active `claim_flat_codomain` bucket at the same smaller unsafe
  `4379 / 549 / 2259` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 113`,
  `small_cluster = 2871 / 435 / 435 / 111`, and the same reopened `single`
  bucket, that clause-`3` refinement also belongs to the autonomy ledger
  rather than as a fresh repair lead
- reopen another deeper remaining-one exact-summary reland inside that same
  representative mismatch-`0` shell first either after the live docs have
  already shown the latest reconstructive control stays on that same
  zero-admitted dead surface; keep the next repair above that remaining-one
  lattice rather than stabilizing another deeper reland here
- reopen the representative `claim_flat_domain` clause-`2` sheet's
  marginally best clause-`6` `reference` continuation through either
  individual clause-`3` `claim_flat_argument` or `claim_eventual_argument`
  branch first; those branches are now individually neutral controls on the
  untouched `4331 / 553 / 2271` baseline, and the broader
  `4343 / 552 / 2268` clause-`6` `reference` tradeoff only appears when both
  reopen together, so that axis also belongs to the autonomy ledger rather
  than as a fresh lead
- reopen that whole joint clause-`3` continuation first either; that broader
  clause-`6` `reference` tradeoff is now localized to one remaining-two
  parent capture plus its three clause-`6` remaining-one child
  continuations, so the live autonomy docs should own the finer completion
  partition beneath that parent/child shell rather than this stable reference
- reopen that representative claim-flat parent/child shell through another
  clause-`3` / clause-`6` child-identity pass first either; those six child
  continuations now only expose matched dead `3`-generated / `0`-admitted
  completion summaries with the same nonlive open-band structural terminal
  trio, so route to the live autonomy docs before spending another turn on the
  exhausted claim-flat shell
- reopen that representative mismatch-`0` claim-flat dead child through
  another finer reason split first either after the live docs have already
  localized that shell to one uniform clause-`2` blocker plus the same
  nonqualifying connectivity vector across all clause-`3` / clause-`6` /
  terminal continuations; keep that finer reason partition in the autonomy
  docs rather than stabilizing it here
- reopen that representative mismatch-`0` claim-sharp dead child through
  another completion or finer reason split first either after the live docs
  have already localized that shell to the same dead
  `3`-generated / `0`-admitted child summaries and the same uniform
  clause-`2` blocker plus nonqualifying connectivity vector; keep that
  deeper claim-sharp partition in the autonomy docs rather than stabilizing
  it here
- reopen the isolated clause-`1` `demo_flat_codomain` exact-suffix side
  pocket first either after the live docs have already pinned that lone
  reland at `4349 / 556 / 2268` with
  `small_cluster = 3141 / 522 / 522 / 0` and the isolated `single` pocket
  still fenced; keep that looser side-pocket control in the autonomy ledger
  rather than stabilizing it here
- reopen broader mismatch-`0` or claim-safe shells first either after the
  live docs have already localized the promoted `reference / reference` tail
  to mismatch `2 = 42` versus mismatch `3 = 12` with mismatch-`2`
  clause-`4` pressure still concentrated on `claim_next_bridge = 18` and
  `reference = 16`; keep that tail split in the autonomy docs rather than
  stabilizing it here
- reopen the larger mismatch-`2` `reference / reference` clause-`4`
  `claim_next_bridge` or `reference` half first either after the live docs
  have already pinned those halves as tradeoff controls with wider
  `small_cluster`; keep those half-probe outcomes in the autonomy docs rather
  than stabilizing them here
- reopen the tiny mismatch-`2` `reference / reference` clause-`4`
  `demo_sharp_codomain` or `demo_sharp_bridge` pocket first either after the
  live docs have already pinned those pockets as matched smaller tradeoff
  controls; keep those demo-side outcomes in the autonomy docs rather than
  stabilizing them here
- reopen the smaller mismatch-`3` `reference / reference` clause-`4`
  `claim_next_bridge` or `reference` half first either after the live docs
  have already pinned those halves as smaller or sharper tradeoff controls
  with wider `small_cluster`; keep those mismatch-`3` half-probe outcomes in
  the autonomy docs rather than stabilizing them here
- reopen broader mismatch-`0` or claim-safe shells by restating the
  `reference / reference` tail either after the live docs have already marked
  both mismatch-`2` demo-side pockets and both mismatch-`3` halves as spent;
  once the full tail is exhausted, route to the live docs for the next
  broader-backup comparison rather than retelling that tail again
- promote the representative claim-safe claim-side clause-`2` shell ahead of
  the tighter representative mismatch-`0` claim-side clause-`2` shell after
  the live docs have already compared those broader backups; keep that
  ordering in the autonomy docs rather than stabilizing the looser claim-safe
  shell here
- reopen the representative `claim_sharp_codomain` clause-`2` sheet's
  clause-`6` identity split first either; its `claim_next_codomain`,
  `claim_sharp_codomain`, and `reference` continuations now also reland the
  same matched smaller tradeoff shell and differ only by a tiny deeper
  zero-admitted tail delta, so that axis also belongs to the autonomy ledger
  rather than as a fresh lead
- keep the next slice above another representative mismatch-`0` claim-side
  dead-shell reland too; once both representative claim-side sheets are
  frozen through their deeper completion and first finer reason passes, and
  once the deeper claim-side clause-`6` `reference` union is also pinned as a
  reconstructive control, route to the live autonomy docs for the
  post-local-probe fallback instead of reopening either claim-side dead child
  again
- treat the sibling active clause-`5` `reference` family's representative
  `claim_flat_domain` marginally best clause-`6` `reference`
  continuation's clause-`3` split as live autonomy detail too; if the live
  docs already pin both `claim_flat_argument` and
  `claim_eventual_argument` branches as individually neutral on the untouched
  `4331 / 553 / 2271` baseline with the same
  `312 / 177 / 50 / 14` first-mismatch counts and the same
  `small_cluster = 3132 / 522 / 522 / 0`, route there instead of privileging
  either branch as if it were a fresh repair class
- treat the broader joint clause-`3` continuation below that same sibling
  active clause-`5` `reference` family shell as live autonomy detail too; if
  the live docs already pin those two clause-`3` branches as individually
  neutral, localize the next probe beneath the same marginally best
  clause-`6` `reference` shell rather than swapping among spent clause-`6`
  siblings or reopening the representative clause-`2` split there
- treat that same sibling active clause-`5` `reference` family's
  representative claim-flat parent/child shell as live autonomy detail too;
  if the live docs already localize the broader joint clause-`3`
  continuation to one remaining-two parent capture on
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  plus the same three clause-`6` remaining-one child continuations
  `claim_next_codomain / claim_sharp_codomain / reference`, route there
  instead of retrying the whole joint continuation as if it were still an
  undifferentiated repair class, and localize the next probe to a finer
  remaining-one completion or terminal partition inside that shell
- treat that same sibling active clause-`5` `reference` family's
  representative claim-flat parent/child shell as live autonomy detail too;
  if the live docs already pin all six clause-`3` / clause-`6` child
  continuations there to the same dead `3`-generated / `0`-admitted
  completion summary with no bound, no survivor sketch, and only the same
  nonlive `reference / eventual_lift / next_lift` open-band structural
  terminal trio, route there instead of spending another turn on that
  exhausted claim-flat shell as if a finer completion or terminal partition
  were still unresolved
- treat that same sibling active clause-`5` `reference` family's
  representative claim-flat dead-child reason split as live autonomy detail
  too; if the live docs already pin its first finer reason pass as uniform at
  clause `2` with the same structurally connected but nonqualifying
  connectivity vector across all clause-`3` / clause-`6` / terminal
  continuations, route there instead of reopening that claim-flat shell as if
  a finer reason partition were still unresolved
- once the live docs mark that sibling active clause-`5` `reference`
  family's representative `claim_flat_domain` shell as spent through both
  completion and first finer reason split, move sideways to the sibling
  `claim_sharp_codomain` sheet on
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / reference`
  instead of retrying the spent claim-flat shell again
- treat that same sibling active clause-`5` `reference` family's
  representative `claim_sharp_codomain` clause-`6` continuations as live
  autonomy detail too; if the live docs already pin them at the same
  `4343 / 552` shell with `small_cluster = 3141 / 522 / 522 / 0`,
  representative clause-`2` spread `15 / 14 / 12`, representative
  clause-`4` split `23 / 18`, the active `claim_next_bridge` plus
  clause-`5` `reference` bucket at `47`, and only a tiny deeper
  zero-admitted tail delta `2270 / 2269 / 2268`, route there instead of
  reopening the spent clause-`6` siblings as if one were a fresh repair
  class
- treat that same sibling active clause-`5` `reference` family's
  representative `claim_sharp_codomain` marginally best clause-`6`
  `reference` continuation's clause-`3` split as live autonomy detail too;
  if the live docs already pin both `claim_flat_argument` and
  `claim_eventual_argument` branches as individually neutral on the untouched
  `4331 / 553 / 2271` baseline with the same
  `312 / 177 / 50 / 14` first-mismatch counts and the same
  `small_cluster = 3132 / 522 / 522 / 0`, route there instead of
  privileging either branch as if it were a fresh repair class
- treat the broader joint clause-`3` continuation below that same sibling
  active clause-`5` `reference` family claim-sharp shell as live autonomy
  detail too; if the live docs already localize it to one remaining-two
  parent capture on
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / reference`
  plus the same three clause-`6` remaining-one child continuations
  `claim_next_codomain / claim_sharp_codomain / reference`, route there
  instead of retrying the whole joint continuation as if it were still an
  undifferentiated repair class, and localize the next probe to a finer
  remaining-one completion or terminal partition inside that shell
- treat that same sibling active clause-`5` `reference` family's
  representative claim-sharp parent/child shell as live autonomy detail too;
  if the live docs already pin all six clause-`3` / clause-`6` child
  continuations there to the same dead `3`-generated / `0`-admitted
  completion summary with no bound, no survivor sketch, and only the same
  nonlive `reference / eventual_lift / next_lift` open-band structural
  terminal trio, route there instead of spending another turn on that
  exhausted claim-sharp shell as if a finer completion or terminal partition
  were still unresolved
- treat that same sibling active clause-`5` `reference` family's
  representative claim-sharp dead-child reason split as live autonomy detail
  too; if the live docs already pin its first finer reason pass as uniform at
  clause `2` with the same structurally connected but nonqualifying
  connectivity vector across all clause-`3` / clause-`6` / terminal
  continuations, route there instead of reopening that claim-sharp shell as
  if a finer reason partition were still unresolved
- once the live docs mark that sibling active clause-`5` `reference`
  family's representative `claim_sharp_codomain` shell as spent through both
  completion and first finer reason split, move sideways to the first
  sibling exact-screen pair on that same active clause-`5` `reference`
  family, starting with the representative `claim_flat_domain` sheet on
  `claim_eventual_domain / claim_sharp_codomain / claim_flat_domain / claim_next_bridge / reference`,
  instead of retrying the spent representative pair again
- treat that first sibling exact-screen pair on the same active clause-`5`
  `reference` family as live autonomy detail too; if the live docs already
  pin its `claim_flat_domain` and `claim_sharp_codomain` clause-`2` sheets
  at the same smaller `4343 / 552 / 2271` tradeoff with the targeted exact
  pair only at `41` and the localized clause-`2` split only at
  `14 / 15 / 12` or `15 / 14 / 12`, while the sibling `reference` sheet
  stays neutral on the untouched `4331 / 553 / 2271` baseline with that pair
  still at `42`, route there instead of reopening the sibling claim-sharp or
  neutral reference sheet as if one were a fresh repair class
- treat that same first sibling exact-screen `claim_flat_domain` sheet on the
  active clause-`5` `reference` family as live autonomy detail too; if the
  live docs already localize its exact-screen delta to exactly one released
  six-clause exact-prune capture on
  `claim_eventual_domain / claim_sharp_codomain / claim_flat_domain / claim_next_bridge / reference`
  with no new exact-prune family and no deeper pruned-terminal change, route
  there instead of pretending clause-`6` identity is already in scope
- treat the stacked remaining-one exact-summary relief beneath that same
  released first sibling exact-screen `claim_flat_domain` plus clause-`5`
  `reference` parent as live autonomy detail too; if the live docs already
  pin it at the same `4343 / 552` wall, the same
  `311 / 177 / 50 / 14`, the same
  `small_cluster = 3141 / 522 / 522 / 0`, but with a sharper deeper
  zero-admitted tail `2268` and a sharper structurally connected but
  unqualified generated surface `6804` from shaving exactly three
  remaining-one pruned prefixes, route there instead of moving sideways to
  the sibling `claim_sharp_codomain` sheet and localize the clause-`6`
  continuations beneath that sharper `4343 / 552 / 2268` shell
- treat that same first sibling exact-screen `claim_flat_domain` plus
  clause-`5` `reference` shell's clause-`6` continuations as live autonomy
  detail too; if the live docs already pin them all at the same matched
  smaller `4343 / 552` shell with first-mismatch counts
  `311 / 177 / 50 / 14`, `small_cluster = 3141 / 522 / 522 / 0`, the
  localized clause-`2` spread `14 / 15 / 12`, the localized clause-`4`
  split `23 / 18`, the active clause-`4` `claim_next_bridge` plus
  clause-`5` `reference` bucket at `47`, and only a deeper zero-admitted
  tail delta `2270 / 2269 / 2268`, route there instead of reopening the
  spent `claim_next_codomain` or `claim_sharp_codomain` siblings or moving
  sideways to the sibling exact-screen `claim_sharp_codomain` sheet as if
  either were a fresh repair class
- treat that same first sibling exact-screen `claim_flat_domain` plus
  clause-`5` `reference` shell's marginally best clause-`6` `reference`
  continuation's clause-`3` split as live autonomy detail too; both
  `claim_flat_argument` and `claim_eventual_argument` branches now land the
  same matched smaller `4343 / 552 / 2271` shell with first-mismatch counts
  `311 / 177 / 50 / 14` and `small_cluster = 3141 / 522 / 522 / 0`, so
  route there instead of privileging either branch as a fresh repair class
- treat that same first sibling exact-screen `claim_flat_domain` plus
  clause-`5` `reference` shell's broader joint clause-`3` continuation as
  live autonomy detail too; relative to either matched clause-`3` branch, it
  changes no remaining-two partial-prefix capture and differs only by
  reopening the same three clause-`6` remaining-one continuations
  `claim_next_codomain / claim_sharp_codomain / reference`, so route there
  instead of rerunning the whole joint continuation as if it were still an
  undifferentiated repair class, and localize the next probe to a finer
  remaining-one completion or terminal partition beneath those same three
  clause-`6` continuations
- treat that same first sibling exact-screen `claim_flat_domain` plus
  clause-`5` `reference` shell's localized parent/child shell as live
  autonomy detail too; if the live docs already pin all six
  clause-`3` / clause-`6` child continuations there to the same dead
  `3`-generated / `0`-admitted completion summary with no bound, no
  survivor sketch, and only the same nonlive
  `reference / eventual_lift / next_lift` open-band structural terminal trio,
  route there instead of spending another turn on that exhausted exact-screen
  claim-flat shell as if a finer completion or terminal partition were still
  unresolved
- treat that same first sibling exact-screen `claim_flat_domain` plus
  clause-`5` `reference` shell's dead-child reason split as live autonomy
  detail too; if the live docs already pin its first finer reason pass as
  uniform at clause `2` with the same structurally connected but
  nonqualifying connectivity vector across all clause-`3` / clause-`6` /
  terminal continuations, route there instead of reopening that exact-screen
  claim-flat shell as if a finer reason partition were still unresolved
- once the live docs mark that first sibling exact-screen
  `claim_flat_domain` plus clause-`5` `reference` shell as spent through both
  completion and first finer reason split, move sideways to the sibling
  `claim_sharp_codomain` sheet on
  `claim_eventual_domain / claim_sharp_codomain / claim_sharp_codomain / claim_next_bridge / reference`
  instead of retrying the spent claim-flat shell again
- treat that same first sibling exact-screen `claim_sharp_codomain` sheet on
  the active clause-`5` `reference` family as live autonomy detail too; if
  the live docs already localize its exact-screen delta to exactly one
  released six-clause exact-prune capture on
  `claim_eventual_domain / claim_sharp_codomain / claim_sharp_codomain / claim_next_bridge / reference`
  with no new exact-prune family and no deeper pruned-terminal change, route
  there instead of pretending clause-`6` identity is already in scope
- treat the stacked remaining-one exact-summary relief beneath that same
  released first sibling exact-screen `claim_sharp_codomain` plus clause-`5`
  `reference` parent as live autonomy detail too; if the live docs already
  pin it at the same `4343 / 552` wall, the same
  `311 / 177 / 50 / 14`, the same
  `small_cluster = 3141 / 522 / 522 / 0`, but with a sharper deeper
  zero-admitted tail `2268` and a sharper structurally connected but
  unqualified generated surface `6804` from shaving exactly three
  remaining-one pruned prefixes, route there instead of reopening the spent
  claim-flat shell or the neutral `reference` sheet on that same first
  sibling pair
- treat that same first sibling exact-screen `claim_sharp_codomain` plus
  clause-`5` `reference` shell's clause-`6` continuations as live autonomy
  detail too; if the live docs already pin them all at the same matched
  smaller `4343 / 552` shell with first-mismatch counts
  `311 / 177 / 50 / 14`, `small_cluster = 3141 / 522 / 522 / 0`, the
  localized clause-`2` spread `15 / 14 / 12`, the localized clause-`4`
  split `23 / 18`, the active clause-`4` `claim_next_bridge` plus
  clause-`5` `reference` bucket at `47`, and only a deeper zero-admitted
  tail delta `2270 / 2269 / 2268`, route there instead of reopening the
  spent `claim_next_codomain` or `claim_sharp_codomain` siblings or the
  neutral `reference` clause-`2` sheet as if one were a fresh repair class
- treat that same first sibling exact-screen `claim_sharp_codomain` plus
  clause-`5` `reference` shell's marginally best clause-`6` `reference`
  continuation's clause-`3` split as live autonomy detail too; both
  `claim_flat_argument` and `claim_eventual_argument` branches now land the
  same matched smaller `4343 / 552 / 2271` shell with first-mismatch counts
  `311 / 177 / 50 / 14` and `small_cluster = 3141 / 522 / 522 / 0`, so
  route there instead of privileging either branch as a fresh repair class
- treat that same first sibling exact-screen `claim_sharp_codomain` plus
  clause-`5` `reference` shell's broader joint clause-`3` continuation as
  live autonomy detail too; relative to either matched clause-`3` branch, it
  changes no remaining-two partial-prefix capture and differs only by
  reopening the same three clause-`6` remaining-one continuations
  `claim_next_codomain / claim_sharp_codomain / reference`, so route there
  instead of rerunning the whole joint continuation as if it were still an
  undifferentiated repair class, and localize the next probe to a finer
  remaining-one completion or terminal partition beneath those same three
  clause-`6` continuations
- treat that same first sibling exact-screen `claim_sharp_codomain` plus
  clause-`5` `reference` shell's localized parent/child shell as live
  autonomy detail too; if the live docs already pin all six
  clause-`3` / clause-`6` child continuations there to the same dead
  `3`-generated / `0`-admitted completion summary with no bound, no
  survivor sketch, and only the same nonlive
  `reference / eventual_lift / next_lift` open-band structural terminal trio,
  route there instead of spending another turn on that exhausted exact-screen
  claim-sharp shell as if a finer completion or terminal partition were still
  unresolved
- treat that same first sibling exact-screen `claim_sharp_codomain` plus
  clause-`5` `reference` shell's dead-child reason split as live autonomy
  detail too; if the live docs already pin its first finer reason pass as
  uniform at clause `2` with the same structurally connected but
  nonqualifying connectivity vector across all clause-`3` / clause-`6` /
  terminal continuations, route there instead of reopening that exact-screen
  claim-sharp shell as if a finer reason partition were still unresolved
- once the live docs mark that first sibling exact-screen
  `claim_sharp_codomain` plus clause-`5` `reference` shell as spent through
  both completion and first finer reason split, move sideways to the next
  sibling exact-screen pair on that same active clause-`5` `reference`
  family, starting with the representative `claim_flat_domain` sheet on
  `claim_eventual_domain / reference / claim_flat_domain / claim_next_bridge / reference`,
  instead of retrying the spent first sibling pair again
- once the live docs mark the next sibling exact-screen pair on the same
  active clause-`5` `reference` family as two claim-side
  `4343 / 552 / 2271` smaller tradeoff controls plus one neutral
  `reference` sheet on
  `claim_eventual_domain / reference / claim_flat_domain / claim_next_bridge / reference`,
  route below the representative `claim_flat_domain` sheet there instead of
  reopening the sibling `claim_sharp_codomain` or neutral `reference`
  sheets as if one were a fresh repair class
- treat that same next sibling exact-screen `claim_flat_domain` sheet on the
  active clause-`5` `reference` family as live autonomy detail too; if the
  live docs already localize its exact-screen delta to exactly one released
  six-clause exact-prune capture on
  `claim_eventual_domain / reference / claim_flat_domain / claim_next_bridge / reference`
  with no new exact-prune family and no deeper pruned-terminal change, and
  already pin the stacked remaining-one exact-summary follow-on at the same
  `4343 / 552` wall, the same `311 / 177 / 50 / 14`, and the same
  `small_cluster = 3141 / 522 / 522 / 0`, but with a sharper deeper
  zero-admitted tail `2268` and a sharper structurally connected but
  unqualified generated surface `6804`, route there instead of moving
  sideways too early
- treat that same next sibling exact-screen `claim_flat_domain` plus
  clause-`5` `reference` shell's clause-`6` and clause-`3` identities as
  live autonomy detail too; if the live docs already pin its three
  clause-`6` continuations to the same matched smaller `4343 / 552` shell
  with only the deeper zero-admitted tail delta `2270 / 2269 / 2268`, and
  already pin both `claim_flat_argument` and `claim_eventual_argument`
  branches beneath the marginally best clause-`6` `reference` continuation
  to the same matched smaller `4343 / 552 / 2271` shell, route there instead
  of privileging either clause label as if it were a fresh repair class
- once the live docs also mark that next sibling active `reference`-family
  claim-flat parent as dead across all three clause-`6` completion profiles
  with the same `3`-generated / `0`-admitted summary, the same
  `reference / eventual_lift / next_lift` `NeedsFallback` trio, and the same
  `matched_clause_count = 2` plus `first_mismatch_position = 2`, move
  sideways to the sibling `claim_sharp_codomain` sheet on
  `claim_eventual_domain / reference / claim_sharp_codomain / claim_next_bridge / reference`
  instead of retrying the spent claim-flat shell again
- treat that same next sibling exact-screen `claim_sharp_codomain` sheet on
  the active clause-`5` `reference` family as live autonomy detail too; if
  the live docs already pin its exact-screen delta to exactly one released
  six-clause exact-prune capture on
  `claim_eventual_domain / reference / claim_sharp_codomain / claim_next_bridge / reference`,
  already pin the stacked remaining-one exact-summary follow-on at the same
  `4343 / 552` wall and the same `311 / 177 / 50 / 14` with the same
  `small_cluster = 3141 / 522 / 522 / 0` but the sharper deeper
  zero-admitted tail `2268`, already pin its three clause-`6`
  continuations to the same matched smaller `4343 / 552` shell with only the
  deeper zero-tail delta `2270 / 2269 / 2268`, already pin both
  `claim_flat_argument` and `claim_eventual_argument` branches beneath the
  marginally best clause-`6` `reference` continuation to the same matched
  smaller `4343 / 552 / 2271` shell, and already pin the released parent's
  clause-`6` completion profiles to the same dead `3`-generated /
  `0`-admitted summary with the same
  `reference / eventual_lift / next_lift` `NeedsFallback` trio plus the same
  `matched_clause_count = 2` and `first_mismatch_position = 2`, route there
  instead of reopening the spent claim-flat sibling, privileging the neutral
  `reference` sheet, or bouncing back to the spent higher-level
  `reference`-family shell
- treat that same next sibling exact-screen `claim_sharp_codomain` plus
  clause-`5` `reference` shell's dead-child reason split as live autonomy
  detail too; if the live docs already pin its first finer reason pass as
  uniform at clause `2` with the same structurally connected but
  nonqualifying connectivity vector across all clause-`3` / clause-`6` /
  terminal continuations, route there instead of reopening that exact-screen
  claim-sharp shell as if a finer reason partition were still unresolved
- once the live docs mark the full `claim_eventual_domain / reference` row on
  that active clause-`5` `reference` family as spent through both completion
  and first finer reason split, move sideways to the first clause-`0`
  sibling exact-screen pair on that same active clause-`5` `reference`
  family, starting with the representative `claim_flat_domain` sheet on
  `claim_flat_domain / claim_next_codomain / claim_next_bridge / reference`,
  instead of retrying the exhausted `claim_eventual_domain / reference` row,
  privileging the neutral `reference` sheet, or bouncing back to claim-safe
  controls
- once the live docs mark that first clause-`0` sibling exact-screen pair on
  the active clause-`5` `reference` family as spent through both claim-side
  shells' deeper completion and first finer reason passes, keep that pair
  frozen and move sideways to the first clause-`1` sibling exact-screen pair
  on that same active clause-`5` `reference` family, starting with
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / reference`,
  instead of reopening the spent clause-`0` pair or the exhausted
  `claim_eventual_domain / reference` row
- treat that same next clause-`1` sibling exact-screen pair on the active
  clause-`5` `reference` family as live autonomy detail too; if the live docs
  already pin its clause-`2` split on
  `claim_flat_domain / reference / claim_next_bridge / reference` as two
  claim-side `4343 / 552 / 2271` smaller tradeoff controls with
  first-mismatch `311 / 177 / 50 / 14`,
  `small_cluster = 3141 / 522 / 522 / 0`, localized `14 / 15 / 12` versus
  `15 / 14 / 12`, and a neutral `reference` sheet on `4331 / 553 / 2271`,
  route below the representative `claim_flat_domain` sheet there instead of
  reopening the sibling `claim_sharp_codomain` or neutral `reference` sheet
- treat both claim-side sheets beneath that same next clause-`1` sibling
  pair as live autonomy detail too; if the live docs already pin each sheet
  to the same one-parent exact-screen contraction, the same stacked
  remaining-one relief on `4343 / 552 / 2268`, the same clause-`6` /
  clause-`3` relands, the same dead `3`-generated / `0`-admitted completion
  profile, and the same uniform first finer reason vector
  `(2, Some(2), true, false, false, 10, false, false)` across all `18`
  continuations, keep both sheets frozen instead of reopening either
  claim-side shell or its neutral `reference` control as if it were fresh
  leverage
- once the live docs mark the full next clause-`1` sibling exact-screen pair
  on the active clause-`5` `reference` family as spent through both
  claim-side shells' deeper completion and first finer reason passes, keep
  that full active `reference`-family pair lattice frozen; if the live docs
  also pin it as the same tighter `4343 / 552 / 2268` broader-backup shell
  already seen on the earlier representative mismatch-`0` claim-side backup
  while the looser claim-safe shell still trails at `4347 / 555 / 2277` and
  the dedicated engine regressions already re-confirm that same ordering,
  move to the next broader-backup comparison rather than reopening the spent
  next clause-`1` pair, the spent first clause-`1` pair, the exhausted first
  clause-`0` sibling pair, the exhausted
  `claim_eventual_domain / reference` row, or looser claim-safe controls
- treat the full active clause-`5` `claim_flat_codomain` broader-backup
  lattice as live autonomy detail too; if the live docs already pin all five
  nonrepresentative siblings
  `claim_eventual_domain / claim_sharp_codomain / claim_next_bridge / claim_flat_codomain`
  and
  `claim_eventual_domain / reference / claim_next_bridge / claim_flat_codomain`
  and
  `claim_flat_domain / claim_next_codomain / claim_next_bridge / claim_flat_codomain`
  and
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / claim_flat_codomain`
  and
  `claim_flat_domain / reference / claim_next_bridge / claim_flat_codomain`
  as the same tighter `4343 / 552 / 2268` shell with first-mismatch
  `311 / 177 / 50 / 14` and `small_cluster = 3141 / 522 / 522 / 0`, while
  the looser claim-safe shell still trails at `4347 / 555 / 2277`, keep
  the whole family frozen and move above that fully exhausted
  broader-backup lattice to a different code-side repair family rather than
  reopening looser claim-safe controls or any spent exact-screen sibling
- if those five active `claim_flat_codomain` sibling comparisons are already
  explicit in the live docs, use the dedicated engine regressions as the
  ordering baseline and continue above the exhausted broader-backup lattice
  rather than rechecking the representative claim-side backup, any spent
  `claim_eventual_domain` sibling, the spent first clause-`1`
  `claim_flat_codomain` sibling, the spent first clause-`0`
  `claim_flat_codomain` sibling, the spent next clause-`1`
  `claim_flat_codomain` sibling, or the exhausted next clause-`1`
  `reference`-family reland first
- if the live docs also pin the first combined active-window plus
  representative tighter broader-backup shell, the sibling
  self-contained plus representative tighter broader-backup shell, and the
  looser recombined active-window plus self-contained plus representative
  tighter broader-backup shell on the active clause-`5`
  `claim_flat_codomain` and `reference` families at the same unsafe
  noncanonical `60 / 8` with `4439 / 544 / 2244`,
  `small_cluster = 2961 / 558 / 558 / 108`, the same reopened `single`
  bucket, and only a `43` contraction on the targeted active clause-`4`
  plus clause-`5` bucket, keep that whole combined exact-shell lattice
  frozen as unsafe controls
- if the live docs also pin the full historical-reanchor plus active-window
  plus self-contained plus exact-shell family on those same active
  clause-`5` families at that same unsafe `4439 / 544 / 2244` shell with the
  same reopened `single` bucket, the same targeted `43` active clause-`4`
  plus clause-`5` bucket, and no off-target delta beyond the same targeted
  active bucket, keep that whole qualification-triad exact-shell lattice
  frozen and move above it to a different code-side repair family rather than
  reopening the spent broader-backup lattice or any narrower combined family
  alone
- if the live docs also pin the first pre-materialization
  summary/materialization follow-on above that qualification-triad exact-shell
  lattice on the representative active clause-`5` `reference` claim-flat
  shell as removing `27` baseline remaining-one pruned prefixes while
  pre-pruning only `3` and letting the other `24` materialize the same unsafe
  clause-kappa `8` rank at overshoot `545 / 5278`, with zero terminal-prefix-
  bar and zero terminal-rank prunes, keep that follow-on frozen too and move
  above it rather than reopening the spent broader-backup lattice, any
  narrower combined family, or the pre-materialization branch itself
- if the live docs also pin the first exact-terminal follow-on above that
  pre-materialization branch on the representative active clause-`5`
  `reference` claim-flat shell as splitting the same `27` removed
  remaining-one prefixes into `24` cached `CanClearBar` terminal summaries
  across the same four `6`-prefix labels plus `3` focused no-bound
  `Unknown` groups on
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`,
  keep that exact-terminal follow-on frozen too and move above it rather than
  reopening the spent broader-backup lattice, any narrower combined family,
  the pre-materialization branch, or that exact-terminal branch itself
- if the live docs also pin the first remaining-one rank-profile follow-on
  above that exact-terminal branch on the representative active clause-`5`
  `reference` claim-flat shell as keeping the surviving `24`
  `CanClearBar` groups on the same
  `generated = 3`, `admitted = 3`,
  `best_accept_rank = best_accept_primary_rank = 8 / 545 / 5278` profile
  with `terminal_prefix_rank_prune_count(...) = 0` even when same-primary
  relief is disabled, and as splitting only on compact survivor-sketch
  cardinality `1` versus `3`, keep that rank-profile follow-on frozen too and
  move to compact survivor-sketch / materialization behavior rather than
  reopening same-primary relief, rank-prune logic, the pre-materialization
  branch, or that exact-terminal branch itself
- if the live docs also pin the first compact survivor-sketch /
  materialization follow-on above that same rank-profile branch on the
  representative active clause-`5` `reference` claim-flat shell as
  materializing the same retained candidates from cached survivor sketches and
  the live no-cache compact path across the same `24` `CanClearBar` groups,
  with every group still on `generated = 3`, `admitted = 3`,
  `best_accept_rank = 8 / 545 / 5278`, and the same `1`-versus-`3`
  survivor/materialization split inside each of the same four live
  `6`-prefix labels, keep that materialization follow-on frozen too
- if the live docs also pin the retained-candidate / terminal-family
  follow-on above that same materialization branch as splitting each of the
  same four live `6`-prefix labels into `2` one-survivor
  `claim_next_codomain` continuations, `2` one-survivor
`claim_sharp_codomain` continuations, and `2` three-survivor
`reference` continuations where only clause-`6` `reference` keeps the full
retained `reference + eventual_lift + next_lift` family, keep that
candidate-identity follow-on frozen too; the clause-`6` `reference`
accept-rank / retained-order follow-on is frozen too, with the shared retained
order `reference -> eventual_lift -> next_lift`, the weaker leading
`reference` survivor on `8 / 19563 / 10556`, both lifts on the best primary
`8 / 545 / 5278` tier, and only `next_lift` owning the full best rank; move
above the whole qualification-triad exact-shell remaining-one lattice rather
than reopening same-primary relief, rank-prune logic, the pre-materialization
branch, the exact-terminal branch, materialization parity, or the clause-`6`
controls
- if the live docs also pin the residual proof-close `single` bucket as the
  canonical accepted `reference(15)` completion on the seven-clause reference
  prefix plus only three weaker same-primary `103 / 8` reference-terminal
  siblings on `clause-0 claim_flat_domain`,
  `clause-2 claim_flat_domain plus anchor-11 exact-argument`, and
  `clause-5 claim_flat_codomain`, and exact-family or subset-local
  same-primary relief there only grows the `single` bucket from `1` to `4`
  while leaving `4331 / 553` and
  `small_cluster = 3132 / 522 / 522 / 0` unchanged, keep that proof-close
  family frozen too and move above it rather than reopening same-primary
  relief there
- if the live docs also pin the first fresh code-side family above that
  proof-close pocket as the representative active clause-`5` `reference`
  claim-flat shell's `24` unsafe survivors bypassing
  `terminal_completion_can_compete_for_acceptance(...)` and arriving instead
  through `TerminalPrefixClauseCandidates::ClaimAdmittedOpenBand` as the full
  `reference + eventual_lift + next_lift` trio, with only the focused `3`
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  prefixes still pre-pruning earlier, and the live docs also pin the whole
  terminal-label subset lattice there as unsafe too
  (`reference`-containing subsets relanding unsafe `60 / 8` reference-terminal
  winners on the flat/sharp `reference / reference` shells, single-lift
  subsets shrinking only to `568 / 347 / 7`, and the lift-only pair staying
  unsafe at `616 / 361 / 14`), do not reopen the generic focus-aligned /
  deprioritized competition gate or any whole-label subset first; move instead
  below that subset lattice to the first prefix-local claim-open-band
  filtering / competition pass on the surviving `reference / reference`
  shells; if the live docs also pin that targeted-shell family as unsafe too,
  with local single-label subsets `reference`, `eventual_lift`, and
  `next_lift` plus the lift-only pair rerouting unsafe
  `60 / 8 / 295 / 545 / 5278` reference winners onto the sibling
  `claim_next_bridge / reference` shells while `reference` plus either lift
  relands the original unsafe
  `60 / 8 / 277 / 545 / 5278` `reference / reference` winners, keep that
  targeted-shell family frozen too and move below it to the sibling
  `claim_next_bridge / reference` shells rather than reopening the whole-label
  subset lattice or the spent targeted `reference / reference` shells
- if the live docs also pin the sibling prefix-local claim-open-band pass on
  those same
  `claim_eventual_domain / claim_next_codomain / {claim_flat_domain, claim_sharp_codomain} / claim_next_bridge / reference`
  shells as another unsafe control, with an asymmetric `9 + 6` removed-prefix
  split because the focused `3` no-bound flat-shell groups still live only on
  `claim_flat_domain / claim_next_bridge / reference`, and with every local
  subset rerouting back onto the same unsafe
  `60 / 8 / 277 / 545 / 5278` `reference / reference` winners, keep that
  sibling-shell family frozen too and move below it to the focused `3`
  no-bound flat-shell prefixes on
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  rather than reopening the full `15`-prefix sibling shell, the spent
  targeted `reference / reference` shells, or the whole-label subset lattice
- if the live docs also pin the focused `3` no-bound flat-shell prefix-local
  subset lattice on those same
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  prefixes as another unsafe control, with every local subset still relanding
  the same unsafe `60 / 8 / 277 / 545 / 5278` `reference / reference` winner
  while single-label subsets land `4433 / 544 / 110` with
  `small_cluster = 2952 / 558 / 558 / 108` and
  `single = 3 / 0 / 0 / 2 / 2`, the three pair subsets land
  `4436 / 544 / 110` with the same smaller `2952 / 558 / 558 / 108` shell,
  and only the full trio returns `4439 / 544 / 110`, keep that focused-three
  family frozen too and move below it to the earlier pre-prune /
  exact-terminal path on those same `3` prefixes rather than reopening any
  terminal-label subset, the full sibling shell, the spent targeted
  `reference / reference` shells, or the whole-label subset lattice
- if the live docs also pin that earlier focused-three pre-prune /
  exact-terminal path on those same
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  prefixes as the same dead `3`-generated / `0`-admitted no-bound control,
  with pre-materialization consuming that summary directly, zero
  terminal-prefix-bar prunes, zero terminal-rank prunes, no cached survivor
  sketch, and live exact-terminal replay still staying `Unknown` with no
  retained accept-rank or primary-rank profile, keep that focused-three
  pre-prune / exact-terminal family frozen too and move below it to direct
  connectivity / exact-admissibility makeup on those same `3` prefixes rather
  than reopening any terminal-label subset, the full sibling shell, the spent
  targeted `reference / reference` shells, or the broader exact-shell lattice
- if the live docs also pin that direct connectivity / exact-admissibility
  follow-on on those same
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  prefixes as another dead control, with each full direct terminal summary
  still generating the same `reference + eventual_lift + next_lift` trio but
  all `3` candidates disconnecting before exact admissibility so each prefix
  stays `3` disconnected, `0` admissibility-rejected, and `0` admitted with
  no bound or retained-rank profile, keep that family frozen too and move
  below it to terminal connectivity identity / fallback makeup on those same
  `3` prefixes rather than reopening any terminal-label subset, the full
  sibling shell, the spent targeted `reference / reference` shells, or the
  broader exact-shell lattice
- if the live docs also pin that focused-three terminal connectivity identity /
  fallback makeup on those same
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  prefixes as another dead control, with each prefix still exposing only the
  same `reference + eventual_lift + next_lift` trio, all `9` generated
  candidates first classifying as `NeedsFallback`, none classifying as
  `PruneDisconnected` or `KeepWithoutFallback`, and every fallback failing
  with the same structurally connected but unqualified witness
  `connected = true`,
  `references_active_window = false`,
  `self_contained = false`,
  `max_lib_ref = 10`,
  `historical_reanchor = false`, and if the live docs also pin the
  focused-three historical-reanchor blocker / first-mismatch reason makeup on
  those same `3` prefixes as uniform too, with all `9` completed telescopes
  across those prefixes and their `reference + eventual_lift + next_lift`
  terminal families sharing `matched_clause_count = 2`,
  `first_mismatch_position = 2`, and the same structurally connected but
  unqualified `max_lib_ref = 10` witness, keep that whole branch frozen and
  move above it to a different code-side step-`15` repair family rather than
  reopening terminal-label subsets, the full sibling shell, the spent
  targeted `reference / reference` shells, or the broader exact-shell lattice
- if the live docs also mark the whole claim-open-band terminal-filter branch
  above that same removed-prefix family as spent through the whole-label
  subsets, the targeted `reference / reference` shells, the sibling
  `claim_next_bridge / reference` shells, and the focused `3` no-bound
  flat-shell follow-ons, and if the live docs also pin the route selector
  itself as a caller-level gate that only opens
  `PrefixLegalityCache::filter_claim_open_band_terminal_clauses(...)` and the
  `TerminalPrefixClauseCandidates::ClaimAdmittedOpenBand(...)` split while the
  live path stays on default `DesktopClaimShadow` with
  `focus_family = None`,
  `package_policies = PackagePolicies::default()`, and no late family-surface
  override away from `ClaimGeneric`, and if the live docs also pin that
  caller-side setup itself on the actual step-`15` path with
  `historical_anchor_ref = Some(10)` plus `ClaimGeneric` at `kappa = 8 .. 8`,
  and if the live docs also pin the underlying modal-anchor temporal debt
  snapshot plus the full
  `reference + eventual_lift + next_lift` claim-generic band-`8` trio on the
  canonical seven-clause reference prefix, move above that whole stack to the
  derivation that chooses that default claim-generic path rather than
  reopening any spent subset/filter shell or focused-three continuation
  beneath it
- reopen the mismatch-`0` clause-`4` `claim_next_bridge` plus clause-`5`
  `claim_next_codomain` exact-summary cell first; that sibling is now a
  neutral control owned by the autonomy ledger rather than a fresh lead
- reopen the whole remaining-two mismatch-`0` claim-domain tier by flipping
  parent-level `CannotClearBar` decisions to `Unknown`; that broad exact-bound
  release is now also a ruled-out widening control owned by the autonomy
  ledger rather than a safe local repair
- reopen runtime-only step-`4` throughput work first unless a newer rerun
  proves the remaining misses are runtime fallout
- reland ruled-out clause-`4` / clause-`5` or same-primary probes without new
  evidence
- claim certification or stronger language before the stored certificate says
  so
