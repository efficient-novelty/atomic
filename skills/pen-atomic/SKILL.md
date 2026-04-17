---
name: pen-atomic
description: Current-state architecture and donor guide for the `pen-atomic` Rust workspace. Use when working on live strict search, `realistic_frontier_shadow`, `demo_breadth_shadow`, `desktop_claim_shadow`, MBTT/kernel design, admissibility, exact selection, reporting, checkpoints, Agda export, or when you need to reconcile current Rust behavior with donor theory and Haskell provenance.
---

# PEN Atomic

## Overview

Use this skill as a routing and stable-context guide for the repository as it
exists today, not as a live experiment log.

Treat these as stable repo truths:

- `pen-cli run` and `pen-cli resume` perform live atomic strict search through
  step `15`.
- `strict_canon_guarded` remains the authoritative executable lane.
- `realistic_frontier_shadow` is a live comparison-backed lane with real
  online prefix search, persisted frontier evidence, and detailed reporting.
- `demo_breadth_shadow` is a comparison-backed child of realistic shadow with
  runnable `5m`, `10m`, and `15m` profiles plus stored narrative/event
  artifacts.
- `desktop_claim_shadow` is a separate claim-lane scaffold with its own
  configs, narratives, policy metadata, compare/certification/benchmark
  tooling, manifest provenance, and failure-safe persistence.
- Claim-lane mechanisms such as claim-debt admissibility, a claim-generic late
  surface, structural-generic bucket scheduling, later `kappa 7-9` mutators,
  and claim-path exactness rechecks are landed in code and tests.
- The focused-three no-bound flat-shell exact-bound gate is pinned too:
  baseline `exact_partial_prefix_bound_decision(...)` caches
  `CannotClearBar` on those `3` residues with the same compact dead
  `3`-generated / `0`-admitted no-bound summary, while the pair-cell
  remaining-one summary override only reclassifies that gate to `Unknown`
  without changing the underlying dead summary.
- Claim artifacts, manifests, live checkpoints, and certification surfaces are
  real; the lane now has a stored compare / benchmark / certify-passing bundle,
  and any stronger user-facing claim must be tied directly to that stored
  certificate and disclosed desktop bundle.
- Focused current-machine Rust verification was rerun and remains green on
  the claim-profile dispatch/accessor layer, the claim-open-band route
  selector, the live caller/work-item/proof-close/demo-closure path, the
  qualification-triad and focused-three no-bound flat-shell controls, and the
  CLI/report/narrative/resume closure surface; direct script execution still
  awaits a `python`, `python3`, `py`, or `uv` launcher, but native
  `pen-cli compare-claim-lane`, `certify-claim-lane`, and
  `benchmark-claim-lane` now cover current-head claim evidence refresh on this
  machine.
- Current post-probe stored rerun evidence now has both a previous audited
  head and a current certified head: the earlier audited bundle keeps the
  step-`15` repair explicit on newer code, and the current stored rerun on
  current head preserves accepted-hash parity through step `15`, keeps the
  repaired stored step-`15` surface at `7211 / 553 / 2052`, restores stored
  step `1` to `2144 / 1285 / 1 / 475`, and carries ready native compare,
  benchmark, and certificate artifacts beside that rerun.
- The repaired step-`15` survivor accounting is now stored as well as
  regression-backed too: demo buckets collapse on the repaired current head to
  one live temporal `small_cluster` at `2052 / 522 / 522 / 0`, one
  zero-admitted `broad` bucket at `3600 / 0 / 0 / 0`, and one fenced `single`
  pocket at `0 / 0 / 0 / 3 / 1`; the residual proof-close pressure is
  localized to three exact claim families, and family-local same-primary
  relief only trades those prunes for extra fully scored non-winning
  reference terminals.
- The repaired-head partial-prefix wall is regression-backed too: the
  unchanged clean `553` wall now localizes to exactly eight queued frontier
  remainder branches, with the direct top-level `reference` remainder at
  `241 / 199 / 42`, the top-level `claim_eventual_domain` remainder at
  `156 / 126 / 30`, the six `claim_flat_domain` descendant branches carrying
  the other `156`, and queued branches `0 .. 3` contributing none on the
  repaired head; inside that direct `reference` remainder, mismatch `1`
  carries `177 / 145 / 32`, mismatch `2` carries `50 / 42 / 8`, mismatch `3`
  carries `14 / 12 / 2`, and the largest single remaining-two pairing stays
  `reference / demo_flat_codomain = 61`; inside that pairing, the next exact
  slice is now narrower too at remaining-two clause-`4`
  `claim_next_bridge = 33` versus clause-`4` `reference = 28`, while the
  whole remaining-three spill is another `12` captures on clause-`4`
  `reference`. That `claim_next_bridge = 33` side is now explicit too as tied
  clause-`2`
  `claim_flat_domain = 12` and
  `claim_sharp_codomain = 12`
  ahead of clause-`2` `reference = 9`; the first repaired-head exact
  claim-pair reopening under that side is frozen as a tradeoff control at
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
  `small_cluster = 2076 / 530 / 530 / 0`,
  while the exact clause-`2` `reference = 9` sheet under that same side is
  now frozen too as the first non-widening safe side-control at
  `generated_raw_prefixes = 7149`,
  `partial_prefix_bar_failure = 551`,
  direct `reference` remainder `= 239`,
  `reference / demo_flat_codomain = 57`,
  slot split `33 / 24 / 14`,
  and
  `small_cluster = 2040 / 518 / 518 / 0`.
  That safe control trims only the clause-`4` `reference` companion while
  leaving the live clause-`4` `claim_next_bridge = 33` blocker untouched.
  The attempted landing extension there is now frozen too: reopening that
  same exact clause-`2` `reference` sheet together with its clause-`4`
  `reference` companion is completely neutral and relands the same
  `7149 / 551 / 2040 / 518 / 518 / 0` surface, with the same
  `33 / 24 / 14` slot split and the same
  `12 / 12 / 9`,
  `9 / 9 / 6`,
  and
  `5 / 5 / 4`
  clause-`2` grid across
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
  remaining-three spill.
- The reverted clause-`1` exact-pocket family is now regression-backed too:
  the `demo_eventually_codomain` exact-pocket reland and the widened
  clause-`0` claim-flat `demo_flat_codomain` exact-pocket reland both
  collapse to the same widening negative control at `4466 / 626` with the
  same `3156 / 526 / 526 / 0` `small_cluster`, the same fenced `single`
  pocket, and the same disconnected zero-admitted shell at `2562 / 7686`.
- The representative mismatch-`0` `reference` clause-`5` remaining-one slice
  is now regression-backed too: the pair-cell clause-`2` split, both
  claim-side clause-`6` sibling sets, both clause-`6` `reference`
  clause-`3` branches, and both joint-continuation delta probes all stay on
  the already-pinned `4343 / 552 / 2268-2270` or neutral surfaces, so that
  slice is frozen as geometry rather than a fresh repair class.
- The live step-`15` caller setup is now regression-backed too: on the actual
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
- The first search-side claim-family-summary / work-item layer above those
  accessors is now regression-backed too: on the actual live step-`15`
  claim prefix, `PrefixFamilySummary::for_admissibility(...)` short-circuits
  on `focus_family = None`, so `PrefixLegalityCache` stores no family
  filter, `family_option_count()` stays absent, and
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
  step-`15` closure line on the drifted claim run, so
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
- The live docs now mark the top-level compare summary / signoff consumer in
  `scripts/compare_runs.py` as regression-backed too:
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
- Focused current-machine Rust verification now passes for the matching
  `pen-type`, `pen-search`, and `pen-cli` claim surfaces via
  `cargo test -p pen-type desktop_claim_shadow_step_`,
  `cargo test -p pen-search desktop_claim_shadow_step_fifteen_enumeration_context_derivation_stays_claim_generic_until_the_mode_changes -- --nocapture`,
  `cargo test -p pen-search live_reference_reintroduced_temporal_focus -- --nocapture`,
  `cargo test -p pen-search live_reference_late_family_surface_override -- --nocapture`,
  `cargo test -p pen-search claim_generic_kappa_eight_catalog_adds_modal_temporal_exchange_variants -- --nocapture`,
  `cargo test -p pen-search current_claim_step_fifteen_live_reference_`,
  `cargo test -p pen-search current_claim_step_fifteen_live_reference_proof_close_ -- --nocapture`,
  `cargo test -p pen-search current_claim_step_fifteen_live_reference_top_level_demo_ -- --nocapture`,
  `cargo test -p pen-search current_claim_step_fifteen_live_caller_setup_stays_on_default_claim_generic_profile`,
  `cargo test -p pen-search current_claim_step_fifteen_live_work_item_keeps_the_default_claim_profile_family_agnostic`,
  `cargo test -p pen-search claim_step_fifteen_family_summary_stays_disabled_until_a_focus_family_is_reintroduced`,
  `cargo test -p pen-search claim_open_band_terminal_clause_filter_requires_default_claim_generic_route`,
  `cargo test -p pen-search current_claim_step_fifteen_frontier_queue_pops_the_live_reference_work_item_before_the_tighter_focused_three_prefixes_on_bit_cost`,
  `cargo test -p pen-search current_claim_step_fifteen_remaining_two_`,
  `cargo test -p pen-search current_claim_step_fifteen_reference_reference_tail_`,
  `cargo test -p pen-search current_claim_step_fifteen_clause_one_demo_flat_codomain_`,
  and `cargo test -p pen-cli current_claim_step_fifteen_`, while direct
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
- The dominant pair above that localized frontier family is now the landed
  local repair too: direct `pen-search` coverage pins the combined top-level
  `reference + claim_eventual_domain` ancestry replay at
  `generated_raw_prefixes = 7211` with
  `small_cluster = 2052 / 522 / 522 / 0`, and the non-test
  `DesktopClaimShadow` exact-two-step path now matches that same widened
  surface while still preserving `partial_prefix_bar_failure = 553`,
  accepted `103 / 8`, the fenced `single` pocket, and zero fully scored
  lifted `89 / 8`; keep the historical `4331` local control surfaces pinned
  in tests and move the next honest slice to stored evidence rather than to
  more local scout geometry.
- Live claim-lane counters, active hypotheses, current blockers, and probe
  history no longer live in this skill file. Read the autonomous docs for that
  operating state:
  [../../autonomous_progress.md](../../autonomous_progress.md),
  [../../autonomous_next_steps.md](../../autonomous_next_steps.md),
  [../../autonomous_plan.md](../../autonomous_plan.md),
  [../../autonomous_checklist.md](../../autonomous_checklist.md), and
  [../../autonomous_ledger.md](../../autonomous_ledger.md).
- Keep this skill file focused on stable repository truths; current probe
  outcomes belong in the autonomous docs rather than here.
- User-facing wording for the certified bundle must now be tied directly to the
  stored certificate and disclosed desktop bundle.
- The accepted executable late-step canon is the current Rust truth, including
  step `15` / `DCT` at `nu = 103`

Treat these as still incomplete:

- `pen-store` is still partly contract-first
- the anti-junk frontier engine is not yet the full long-range design
- the Agda bridge is still lighter than the final proof-facing target

The current architecture focus is split between three active tracks:

- stronger exact late-step pruning and ordering on
  `realistic_frontier_shadow`
- honest breadth, budget, and evidence surfacing on `demo_breadth_shadow`
- rerun-confirmed step-`15` repair, stored breadth repair, and certification
  work on
  `desktop_claim_shadow`

## Current-State References

Read only the track-specific detail you need:

- For the current realistic-shadow capability inventory, evidence snapshot, and
  remaining search gap, read
  [references/11-current-realistic-shadow.md](references/11-current-realistic-shadow.md).
- For the current demo-lane stable mechanisms, evidence baselines, and signoff
  status, read
  [references/12-current-demo-lane.md](references/12-current-demo-lane.md).
- For the current claim-lane scaffold state, read
  [references/13-current-claim-lane.md](references/13-current-claim-lane.md).
- For live demo-lane targets and signoff criteria, read
  [../../demo_lane_progress.md](../../demo_lane_progress.md),
  [../../demo_lane_plan.md](../../demo_lane_plan.md), and
  [../../demo_lane_checklist.md](../../demo_lane_checklist.md).
- For the active autonomy workstream, read
  [../../autonomous_progress.md](../../autonomous_progress.md) for the live
  snapshot,
  [../../autonomous_next_steps.md](../../autonomous_next_steps.md) for the
  active work order,
  [../../autonomous_plan.md](../../autonomous_plan.md) for medium-horizon
  phases,
  [../../autonomous_checklist.md](../../autonomous_checklist.md) for binary
  signoff gates, and
  [../../autonomous_ledger.md](../../autonomous_ledger.md) for append-only
  probe history.

Start with the current architecture doc before diving into donor material:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [../../overall_plan.md](../../overall_plan.md)
- [../../quantum_progress.md](../../quantum_progress.md)

## Working Rules

1. Treat the current Rust workspace and its tests as the source of truth for
   present executable behavior.
2. Use [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md) first when you need
   current crate boundaries, runtime flow, reporting surface, or the honesty
   boundary.
3. Treat `engine/src/*.hs` as donor provenance for unresolved semantics, not as
   automatically newer or truer than the current Rust implementation.
4. Keep the hot path name-free. Semantic labels belong in CLI reporting and
   Agda export layers only.
5. Separate three targets the repo still risks conflating:
   - the current strict executable Rust canon
   - paper-facing or aspirational targets
   - older historical 16-step or alternate late-shell variants
6. Do not mistake compatibility structs, manifest schemas, or placeholder
   storage modules for fully implemented runtime behavior.
7. Do not regress the repo back toward replay-or-template logic when the
   current bounded live atomic lane already works through step `15`.
8. Use the operational repo docs for live open work and use the bundled
   references for stable current-state detail.
9. Assume the local shell is Windows PowerShell unless the current task proves
   otherwise. Prefer one command per shell call and avoid POSIX-style chaining
   such as `&&`, especially for `git add` / `git commit` / `git push`.
10. Prefer narrow, staged edits over oversized multi-file or multi-hunk
    patches. If a large doc or instruction update starts failing to apply,
    split it into smaller targeted changes instead of forcing one huge patch.

## Session Cleanup

Before ending a session:

1. Check for residual processes started during the session, especially
   `cargo`, `pen-cli.exe`, long-running `cargo test`, `cargo run`, benchmark,
   export, and validation commands.
2. Do not assume a timed-out Codex command killed its child processes.
   Inspect the live process table and confirm whether the spawned workload is
   still running.
3. If a residual process was started for the current session and is no longer
   needed, terminate both the child workload and its wrapper process.
4. Do not terminate unrelated user-owned workloads. If ownership is unclear,
   inspect the command line, start time, parent process, and output path before
   deciding.
5. Report any residual processes found or terminated in the final handoff so
   the session closes with an explicit cleanup status.

## First Reads

For most tasks, read in this order:

1. [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
2. [../../overall_plan.md](../../overall_plan.md)
3. One of these track-specific current-state references, depending on the task:
   - [references/11-current-realistic-shadow.md](references/11-current-realistic-shadow.md)
   - [references/12-current-demo-lane.md](references/12-current-demo-lane.md)
   - [references/13-current-claim-lane.md](references/13-current-claim-lane.md)
4. [../../quantum_progress.md](../../quantum_progress.md) when the task
   touches prefix search, bounds, caching, or realistic late-step tuning
5. [../../demo_lane_progress.md](../../demo_lane_progress.md) and
   [../../demo_lane_plan.md](../../demo_lane_plan.md) when the task touches
   `demo_breadth_shadow`
6. [../../autonomous_progress.md](../../autonomous_progress.md),
   [../../autonomous_next_steps.md](../../autonomous_next_steps.md),
   [../../autonomous_plan.md](../../autonomous_plan.md), and
   [../../autonomous_ledger.md](../../autonomous_ledger.md) when the task
   touches `desktop_claim_shadow`
7. [theory/README.md](theory/README.md) when you need the theorem or manuscript
   map

Then branch based on the task.

## Reference Map

- Read [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md) for the current crate
  graph, runtime flow, reporting architecture, artifact model, and honesty
  boundary.
- Read [../../README.md](../../README.md) for the current user-facing commands
  and smoke-test commands.
- Read [../../overall_plan.md](../../overall_plan.md) for current completion
  status, deliverables, and immediate next priorities.
- Read [../../quantum_progress.md](../../quantum_progress.md) for the current
  delta between the quantum improvement plan and the live Rust codebase.
- Read [references/11-current-realistic-shadow.md](references/11-current-realistic-shadow.md)
  for the current realistic-shadow capability inventory, evidence snapshot, and
  remaining search gap.
- Read [../../demo_lane_progress.md](../../demo_lane_progress.md),
  [../../demo_lane_plan.md](../../demo_lane_plan.md), and
  [../../demo_lane_checklist.md](../../demo_lane_checklist.md) for live
  `demo_breadth_shadow` status, targets, and signoff tasks.
- Read [references/12-current-demo-lane.md](references/12-current-demo-lane.md)
  for the stable current demo-lane mechanisms and evidence baselines that
  should remain true while later demo-lane changes move.
- Read [references/13-current-claim-lane.md](references/13-current-claim-lane.md)
  plus [../../autonomous_progress.md](../../autonomous_progress.md),
  [../../autonomous_next_steps.md](../../autonomous_next_steps.md),
  [../../autonomous_plan.md](../../autonomous_plan.md),
  [../../autonomous_checklist.md](../../autonomous_checklist.md), and
  [../../autonomous_ledger.md](../../autonomous_ledger.md) for the current
  claim-lane mixed state, active work order, signoff gates, and probe
  history.
- Read [theory/README.md](theory/README.md) when you need the theorem map or
  manuscript map.
- Read [theory/genesis.md](theory/genesis.md) when you need the exact strict
  `15`-step target.
- Read [theory/pen-model.md](theory/pen-model.md),
  [theory/coherence-and-scaling.md](theory/coherence-and-scaling.md), and
  [theory/novelty-selection-and-rejection.md](theory/novelty-selection-and-rejection.md)
  for the mathematical contract behind the search objective.
- Read [theory/late-framework-abstraction.md](theory/late-framework-abstraction.md)
  and [theory/terminal-dct.md](theory/terminal-dct.md) before making claims
  about steps `10` to `15`.
- Read [references/01-project-brief.md](references/01-project-brief.md) for
  scope, donor priorities, and stale-doc warnings.
- Read [references/02-target-sequence.md](references/02-target-sequence.md) for
  the canonical `15`-step target, current strict values, and bar arithmetic.
- Read [references/03-repo-donor-map.md](references/03-repo-donor-map.md) when
  deciding what to port, what to demote to reporting, and what not to copy.
- Read [references/04-mbtt-kernel.md](references/04-mbtt-kernel.md) before
  touching ASTs, encoding, canonicalization, telescope storage, or capability
  flags.
- Read [references/05-search-and-selection.md](references/05-search-and-selection.md)
  before implementing enumeration, admissibility, ranking, minimality, or
  resume behavior.
- Read [references/06-atomic-research-lessons.md](references/06-atomic-research-lessons.md)
  before designing any wider anti-junk or frontier-retention story.
- Read [references/07-agda-validation.md](references/07-agda-validation.md)
  before building export payloads, witness modules, or verification scripts.
- Read [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)
  when you need test or CI contracts, evidence requirements, or
  non-interference rules.
- Read [references/09-rust-rewrite-blueprint.md](references/09-rust-rewrite-blueprint.md)
  when working on schemas, checkpoints, memory limits, or unfinished runtime
  infrastructure.
- Read [references/10-open-questions.md](references/10-open-questions.md)
  before making decisions that could accidentally lock in the wrong semantics.

## Task Routing

### If you are working on current runtime architecture

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [../../overall_plan.md](../../overall_plan.md)
- [references/09-rust-rewrite-blueprint.md](references/09-rust-rewrite-blueprint.md)

Focus on:

- crate boundaries
- runtime and artifact flow
- what is already real versus still scaffolded
- how not to over-claim unfinished resume/storage/governor features

### If you are defining or changing the Rust core

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [theory/pen-model.md](theory/pen-model.md)
- [theory/coherence-and-scaling.md](theory/coherence-and-scaling.md)
- [references/04-mbtt-kernel.md](references/04-mbtt-kernel.md)
- [references/03-repo-donor-map.md](references/03-repo-donor-map.md)

Focus on:

- frozen atom schema
- telescope and library representation
- exact `kappa` and canonicalization contracts
- stable IDs, hashes, and checkpoint compatibility

### If you are implementing the search loop

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [../../quantum_progress.md](../../quantum_progress.md)
- [references/11-current-realistic-shadow.md](references/11-current-realistic-shadow.md)
- [theory/genesis.md](theory/genesis.md)
- [theory/novelty-selection-and-rejection.md](theory/novelty-selection-and-rejection.md)
- [references/05-search-and-selection.md](references/05-search-and-selection.md)
- [references/06-atomic-research-lessons.md](references/06-atomic-research-lessons.md)
- [references/02-target-sequence.md](references/02-target-sequence.md)

Focus on:

- exact-band search and bar semantics
- admissibility from structural debt, not names
- deterministic dedupe and SCC minimality
- the remaining difference between the current realistic online prefix engine
  and the still-missing stronger sound bounds plus broader non-family
  admissibility reuse
- using `PrefixSignature`, memo counters, timing telemetry, and frontier
  memory evidence to drive the next realistic late-step improvements

### If you are working on `demo_breadth_shadow`

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [references/12-current-demo-lane.md](references/12-current-demo-lane.md)
- [../../demo_lane_progress.md](../../demo_lane_progress.md)
- [../../demo_lane_plan.md](../../demo_lane_plan.md)
- [../../demo_lane_checklist.md](../../demo_lane_checklist.md)
- [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)

Focus on:

- keeping `demo_breadth_shadow` comparison-backed rather than authoritative
- treating breadth as real generated or exactly screened search mass, not
  inflated full-evaluation counts
- preserving the explicit phase machine
  `Scout -> BreadthHarvest -> Materialize -> ProofClose -> Seal`
- persisting honest demo evidence in step summaries, narratives, and event
  streams rather than reconstructing it from debug text
- holding the closed early and late signoff baselines without regressing
  accepted parity, narrative/event coverage, or the honesty boundary

### If you are working on `desktop_claim_shadow`

Read:

- [references/13-current-claim-lane.md](references/13-current-claim-lane.md)
- [../../autonomous_progress.md](../../autonomous_progress.md)
- [../../autonomous_next_steps.md](../../autonomous_next_steps.md)
- [../../autonomous_plan.md](../../autonomous_plan.md)
- [../../autonomous_checklist.md](../../autonomous_checklist.md)
- [../../autonomous_ledger.md](../../autonomous_ledger.md)
- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)

Focus on:

- keeping the claim lane separate from demo-only behavior
- use [../../autonomous_progress.md](../../autonomous_progress.md) as the
  canonical home for live counters, blocker anatomy, and the current
  operating position
- use [../../autonomous_next_steps.md](../../autonomous_next_steps.md) as the
  only home for the active slice and explicit "not the next move" guidance
- use [../../autonomous_ledger.md](../../autonomous_ledger.md) for probe
  history, negative controls, and why earlier ideas were ruled out
- treat the exhausted mismatch-`1` `reference + demo_flat_codomain` ladder as
  a live autonomy detail owned by the autonomous docs rather than by this
  skill file; read [../../autonomous_progress.md](../../autonomous_progress.md)
  and [../../autonomous_next_steps.md](../../autonomous_next_steps.md) for the
  current off-branch priority order before opening another late-step probe
- use [../../autonomous_plan.md](../../autonomous_plan.md) for
  medium-horizon sequencing and exit criteria rather than re-explaining the
  current slice
- keep certification language honest: stronger wording is now allowed only when
  it is tied directly to the stored certificate and disclosed desktop bundle
- use `scripts/compare_runs.py`, `scripts/benchmark_claim_lane.py`, and
  `scripts/certify_claim_lane.py` as the evidence surfaces before moving the
  search code again
- use the current canonical certified rerun plus earlier stored evidence and
  `run.json` build fingerprints to keep post-certification step-`15`
  hardening ahead of reopening step-`1`, storage, or tooling work
- treat the step-`15` partial-prefix wall, the dominant clause-`0` / clause-`1`
  pairings, and the live clause-`4` split as live diagnosis owned by
  [../../autonomous_progress.md](../../autonomous_progress.md)
- once the live docs mark the representative mismatch-`0` parent-route,
  qualification-family, clause-`4`, and clause-`2` selector lattice as spent,
  treat `engine.rs` exact screening as the first fresh lever:
  `screen_prefix_for_frontier(...)`,
  `exact_partial_prefix_bound_decision(...)`,
  `exact_terminal_prefix_bound_decision(...)`, and
  `claim_try_summary_prune_before_materialization(...)`; once the live docs
  also mark that post-pop collapse/summary handoff as spent on the live
  seven-clause reference prefix, move above it to
  `materialize_remaining_one_prefix_group(...)` and
  `materialize_terminal_prefix_group(...)`; once the live docs also mark that
  remaining-one materialization handoff as spent on the live reference prefix,
  move above it again to the first post-materialize retained-bound /
  incumbent-rank / candidate-cache consumer:
  `record_demo_bucket_surface(...)`,
  `record_demo_bucket_exact_screened(...)`,
  `terminal_prefix_rank_prune_count(...)`,
  `best_prefix_group_accept_rank(...)`,
  `cache_evaluated_terminal_prefix_group_candidates(...)`, and
  `PrefixCache::record_group_with_surface_counts(...)`; the live docs now also
  mark that candidate-cache handoff, the first proof-close intake, and the
  proof-close candidate-order / candidate-level incumbent gate, and the
  proof-close candidate evaluation / rank-recomputation / minimality path, and
  the first post-evaluation fully scored bucket / proof-close full-eval
  accounting consumer, and the top-level full-telescope / demo-funnel
  accounting consumer, and the top-level demo-closure consumer, and the
  report-side demo-closure handoff, and the first report/narrative closure
  render consumer, and the first caller/output emission layer as spent on the
  live reference prefix, and the caller-side narrative toggle /
  emission-dispatch layer plus the first compare-side demo-phase consumer as
  spent too, move above them to the top-level compare summary / signoff
  consumer in `scripts/compare_runs.py` beginning at
  `build_summary(...)`,
  `signoff_summary(...)`,
  `render_claim_lane_audit(...)`,
  `render_text_summary(...)`, and the top-level
  `Comparison Signoff:` / `claim lane audit:` lines
- treat the active slice and the exact next move as owned by
  [../../autonomous_next_steps.md](../../autonomous_next_steps.md), not by
  this skill file
- if the live docs already mark both first-sibling exact-screen claim-side
  sheets on
  `claim_eventual_domain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`
  as spent through their exact-screen boundary, neutral summary-relief, and
  dead clause-`6` follow-ons, route to the next sibling exact-screen pair on
  the same `claim_next_codomain` cell instead of reopening either spent
  first-sibling sheet as if it were fresh leverage
- if the live docs also mark the first clause-`0` sibling exact-screen pair on
  `claim_flat_domain / claim_next_codomain / claim_next_bridge / claim_next_codomain`
  as spent through its clause-`2` split, both claim-side exact-screen
  boundaries, neutral summary-relief, and dead clause-`6` follow-ons, route
  to the first clause-`1` sibling exact-screen pair on the same clause-`0`
  `claim_flat_domain` family,
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`,
  instead of reopening that spent `claim_next_codomain` pair as if it were
  fresh leverage
- keep the next repair above exhausted clause-`5` tail probes and above
  demo-only reopenings and above another exact claim-pair clause-`4`
  `reference`-side relocalization pass; that narrower reference-side tradeoff
  now only reproduces the broader clause-`4` `reference`-sheet result, so the
  skill should route you to the live docs rather than restating their full
  diagnostic ledger
- do not bypass `exact_partial_prefix_bound_decision(...)` for the whole
  remaining-two mismatch-`0` claim-domain tier; that parent-level release is
  now a ruled-out widening control owned by the autonomous docs and ledger, so
  read those files before touching exact-bound relief on the current step-`15`
  wall
- keep the next repair above the broader remaining-two exact partial-prefix
  relief on the whole mismatch-`0` clause-`4` `claim_next_bridge` half; that
  probe now lands only a broader `4763 / 517 / 2271` tradeoff with canonical
  `103 / 8` and a fenced `single` pocket, but it widens
  `small_cluster` to `3456 / 522 / 522 / 0` and equalizes every live
  mismatch-`0` clause-`4` / clause-`5` cell to `36`, so route to the live
  docs before reopening that broad exact-screen release as if it were the
  landing
- keep the next repair above the narrower remaining-two exact partial-prefix
  clause-`5` splits below that same `claim_next_bridge` half too; once the
  live docs pin `claim_flat_codomain`, `claim_next_codomain`, and
  `reference` at the same `4475 / 541 / 2271` smaller tradeoff with widened
  `small_cluster = 3240 / 522 / 522 / 0`, a fenced `single` pocket, the same
  `42 -> 40` pair contraction, and the same `24 / 18 -> 22 / 18`
  clause-`4` split, route there instead of reopening clause-`5` identity as
  if one remaining-two exact-screen cell were newly privileged
- keep the next repair above the narrower exact-screen clause-`0` /
  clause-`1` pair splits inside the representative `claim_next_codomain`
  cell too; once the live docs pin all six pair probes there at the same
  `4355 / 551 / 2271` smaller tradeoff with fixed zero-admitted captures
  `2271`, widened `small_cluster = 3150 / 522 / 522 / 0`, a fenced
  `single` pocket, a targeted `42 -> 40` pair contraction, a targeted
  `24 / 18 -> 22 / 18` clause-`4` split, and a targeted `48 -> 46`
  `claim_next_bridge` plus `claim_next_codomain` bucket contraction, route
  there instead of reopening exact-screen pair identity as if one
  representative pair were newly privileged
- keep the next repair above the representative exact-screen pair-cell
  clause-`2` split inside that same `claim_next_codomain` cell too; once the
  live docs pin the `claim_flat_domain` and `claim_sharp_codomain` sheets at
  the same `4343 / 552 / 2271` smaller tradeoff with first-mismatch counts
  `311 / 177 / 50 / 14`, `small_cluster generated = 3141`, and a targeted
  `23 / 18` clause-`4` split plus `47` active clause-`5` bucket, while the
  sibling `reference` sheet is neutral on the untouched
  `4331 / 553 / 2271` baseline, route there instead of reopening exact-screen
  clause-`2` identity as if one claim-side sheet were newly privileged
- keep the next repair above the representative exact-screen
  `claim_flat_domain` clause-`6` split inside that same `claim_next_codomain`
  cell too; once the live docs pin that claim-flat clause-`2` tradeoff to
  exactly one removed six-clause exact-prune capture with no introduced
  capture or pruned-prefix family, clause `6` is still out of scope at that
  exact-screen boundary, so route there instead of reopening exact-screen
  clause-`6` identity as if one continuation were newly privileged
- keep the next repair above remaining-one exact-summary relief beneath that
  released exact-screen representative `claim_flat_domain` plus clause-`5`
  `claim_next_codomain` parent too; once the live docs pin the stacked
  follow-on as a neutral control on the same `4343 / 552 / 2271` shell with
  first-mismatch counts `311 / 177 / 50 / 14`,
  `small_cluster generated = 3141`, the same zero-admitted exact-prune
  family `((0, None, None), 2271)`, and no released remaining-one
  pruned-terminal group, route there instead of reopening that summary-relief
  slice as if it were fresh leverage
- keep the next repair above direct remaining-one completion or terminal
  identity beneath that same released exact-screen representative
  `claim_flat_domain` plus clause-`5` `claim_next_codomain` parent too; once
  the live docs pin all three clause-`6` continuations at the same dead
  `3`-generated / `0`-admitted completion summary with no bound, no
  best-rank profile, no survivor sketch, the same
  `reference / eventual_lift / next_lift` `NeedsFallback` trio, and the same
  clause-`2` blocker with `max_lib_ref = 10`, route there instead of
  reopening that claim-flat clause-`6` path as if one continuation were newly
  privileged
- keep the next repair above the representative exact-screen
  `claim_sharp_codomain` clause-`6` split inside that same
  `claim_next_codomain` cell too; once the live docs pin that claim-sharp
  clause-`2` tradeoff to exactly one removed six-clause exact-prune capture
  with no introduced capture or pruned-prefix family, clause `6` is still out
  of scope at that exact-screen boundary, so route there instead of reopening
  exact-screen clause-`6` identity on the claim-sharp sheet as if one
  continuation were newly privileged
- keep the next repair above remaining-one exact-summary relief beneath that
  released exact-screen representative `claim_sharp_codomain` plus
  clause-`5` `claim_next_codomain` parent too; once the live docs pin the
  stacked follow-on as a neutral control on the same `4343 / 552 / 2271`
  shell with first-mismatch counts `311 / 177 / 50 / 14`,
  `small_cluster generated = 3141`, the same zero-admitted exact-prune
  family `((0, None, None), 2271)`, and no released remaining-one
  pruned-terminal group, route there instead of reopening that claim-sharp
  summary-relief slice as if it were fresh leverage
- keep the next repair above direct remaining-one completion or terminal
  identity beneath that same released exact-screen representative
  `claim_sharp_codomain` plus clause-`5` `claim_next_codomain` parent too;
  once the live docs pin all three clause-`6` continuations at the same dead
  `3`-generated / `0`-admitted completion summary with no bound, no
  best-rank profile, no survivor sketch, the same
  `reference / eventual_lift / next_lift` `NeedsFallback` trio, and the same
  clause-`2` blocker with `max_lib_ref = 10`, route there instead of
  reopening that claim-sharp clause-`6` path as if one continuation were
  newly privileged
- once the live docs pin both representative exact-screen claim-side sheets
  and their deeper remaining-one follow-ons beneath
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  claim_next_codomain`, keep the next repair above that spent representative
  pair and route to a sibling exact-screen pair on the same
  `claim_next_codomain` cell instead of reopening either claim-side sheet,
  the neutral `reference` sheet, or the same remaining-one follow-ons as if
  they were fresh leverage
- once the live docs also pin the first sibling exact-screen pair's
  clause-`2` split beneath
  `claim_eventual_domain / claim_sharp_codomain / claim_next_bridge /
  claim_next_codomain`, keep the next repair above that spent sibling
  clause-`2` axis too and route below the representative
  `claim_flat_domain` sheet on that sibling pair instead of reopening the
  sibling `claim_sharp_codomain` sheet, the neutral `reference` sheet, or
  the same remaining-one follow-ons on the spent representative pair as if
  they were fresh leverage
- once the live docs also pin that first sibling exact-screen
  `claim_flat_domain` sheet and its deeper remaining-one follow-ons as spent
  beneath `claim_eventual_domain / claim_sharp_codomain / claim_next_bridge /
  claim_next_codomain`, keep the next repair above that exhausted sibling
  claim-flat branch too and route to the sibling
  `claim_sharp_codomain` sheet on the same pair instead of reopening the
  neutral `reference` sheet, the same claim-flat follow-ons, or the spent
  representative pair as if they were fresh leverage
- once the live docs also pin the first clause-`0` sibling exact-screen
  `claim_sharp_codomain` sheet and its deeper remaining-one follow-ons as
  spent beneath `claim_flat_domain / claim_next_codomain / claim_next_bridge /
  claim_next_codomain`, keep the next repair above that exhausted
  `claim_next_codomain` clause-`1` branch on the `claim_flat_domain`
  clause-`0` family too and route to the first clause-`1` sibling exact-screen
  pair there,
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`,
  instead of reopening the spent `claim_eventual_domain` trio or the spent
  first clause-`0` sibling pair as if they were fresh leverage
- once the live docs also pin that first clause-`1` sibling exact-screen pair
  at the same smaller clause-`2` tradeoff lattice and pin the first
  clause-`1` sibling `claim_flat_domain` sheet's exact-screen boundary to one
  released six-clause parent with clause `6` still out of scope, and also pin
  the stacked remaining-one exact-summary relief beneath that released
  claim-flat parent as a neutral control on the same `4343 / 552 / 2271`
  shell with no released remaining-one pruned-terminal group, keep the next
  repair above that spent pair-level and summary-prune lattice too; once the
  live docs also pin direct remaining-one completion beneath that released
  claim-flat parent as the same dead `3`-generated / `0`-admitted
  clause-`6` triad with only the nonlive
  `reference / eventual_lift / next_lift` terminal trio, route sideways to
  the sibling `claim_sharp_codomain` sheet instead of reopening the spent
  claim-flat parent, the neutral `reference` sheet, or the spent first
  clause-`0` sibling pair as if they were fresh leverage
- once the live docs also pin that first clause-`1` sibling exact-screen
  `claim_sharp_codomain` sheet and its deeper remaining-one follow-ons as
  spent beneath `claim_flat_domain / claim_sharp_codomain / claim_next_bridge /
  claim_next_codomain`, keep the next repair above that exhausted
  `claim_sharp_codomain` branch too; once the live docs also pin the next
  clause-`1` sibling exact-screen pair on that same clause-`0`
  `claim_flat_domain` family,
  `claim_flat_domain / reference / claim_next_bridge / claim_next_codomain`,
  at the same smaller clause-`2` tradeoff lattice with the targeted pair at
  `41` while the sibling `reference` sheet stays neutral at `42`, and once
  the live docs also pin that pair's representative `claim_flat_domain`
  sheet through its same-boundary one-parent reland, neutral remaining-one
  exact-summary follow-on, and dead clause-`6` completion split, and once
  the live docs also pin that pair's sibling `claim_sharp_codomain` sheet
  through its matching same-boundary one-parent reland, neutral
  remaining-one exact-summary follow-on, and dead clause-`6` completion
  split, route sideways to the sibling active mismatch-`0` clause-`5`
  `reference` family, and once the live docs also pin that sibling
  reference-family representative `claim_flat_domain` sheet's clause-`6`
  `claim_next_codomain`, `claim_sharp_codomain`, and `reference`
  continuations at the same `4343 / 552` shell with
  `small_cluster = 3141 / 522 / 522 / 0`, representative clause-`2` spread
  `14 / 15 / 12`, representative clause-`4` split `23 / 18`, the active
  `claim_next_bridge` plus clause-`5` `reference` bucket at `47`, and only
  a tiny deeper zero-admitted tail delta `2270 / 2269 / 2268`, route below
  the marginally best clause-`6` `reference` continuation on that same
  sheet instead of reopening the spent clause-`6` siblings there, any spent
  exact-screen pair-cell split, claim-side sheet, or neutral `reference`
  sheet on that exhausted
  `claim_next_codomain` exact-screen cell
- keep the next repair above the broad clause-`1` `demo_flat_codomain`
  reopening across the full mismatch-`0` claim-domain surface; that control is
  now a widening negative result owned by
  [../../autonomous_ledger.md](../../autonomous_ledger.md), so route to the
  live docs before reopening that surface again
- keep the next repair above the narrower mismatch-`0` clause-`4`
  `claim_next_bridge`-side relocalization too; that smaller negative control
  now also belongs to [../../autonomous_ledger.md](../../autonomous_ledger.md),
  so route to the live docs before retrying that same connectivity-only slice
- keep the next repair above the narrower mismatch-`0` clause-`4`
  `reference`-side relocalization too; that sharper negative control now also
  belongs to [../../autonomous_ledger.md](../../autonomous_ledger.md), so
  route to the live docs before retrying that same connectivity-only slice
- keep the next repair above the representative mismatch-`0` claim-side
  clause-`2` sheet splits on the parent-route, active-window, and
  self-contained shells too; if the live docs already pin those sheet splits
  at unsafe `4379 / 549` controls, with the parent-route `reference` sheet
  only changing the noncanonical winner to `74 / 8` and the alternate
  `reference` sheet reopening a two-retained unsafe control, route there
  instead of treating clause-`2` identity as fresh leverage
- keep the next repair above the spent representative mismatch-`0`
  connectivity lattice as a whole; once parent-route, alternate
  qualification-family, clause-`4`, and clause-`2` relands are all frozen,
  route back to the live autonomy docs and the exact-screen engine path
  instead of inventing another connectivity-family retry here
- keep the next repair above remaining-one exact-summary relief on the
  mismatch-`0` clause-`4` `reference` plus clause-`5` `reference` tail too;
  that deeper negative control now also belongs to
  [../../autonomous_ledger.md](../../autonomous_ledger.md), so route to the
  live docs before retrying that same narrow reference-tail slice
- treat the whole mismatch-`0` clause-`4` `claim_next_bridge`-half
  remaining-one exact-summary tradeoff as live autonomy detail too; it now
  narrows the wall but still widens `small_cluster`, so route to the live
  docs before retrying that whole half and localize within its clause-`5`
  cells if that is still the active slice
- treat the narrower mismatch-`0` clause-`4` `claim_next_bridge` plus
  clause-`5` `claim_flat_codomain` and clause-`5` `reference` exact-summary
  tradeoffs as live autonomy detail too; they are smaller tradeoff controls,
  while the sibling clause-`5` `claim_next_codomain` cell is inert, so route
  to the live docs before retrying any of those whole-cell slices
- treat the deeper pair-cell subprobes below those active mismatch-`0`
  clause-`5` `claim_flat_codomain / reference` cells as live autonomy detail
  too; they are now symmetric smaller tradeoff controls across all `12`
  live pairings, so route to the live docs before retrying that pair-cell
  scope instead of assuming one pairing or active clause-`5` family is newly
  privileged
- treat the representative clause-`2` sheet subprobes below that pair-cell as
  live autonomy detail too; the two claim-side sheets are now matching smaller
  tradeoff controls while the reference sheet is neutral, so route to the live
  docs before retrying clause-`2` identity instead of assuming one sheet is
  newly privileged
- treat the sibling active mismatch-`0` clause-`5` `reference` family's
  representative clause-`2` split as live autonomy detail too; if the live
  docs already localize it to the same `4343 / 552 / 2268` smaller tradeoff
  shell with `small_cluster = 3141 / 522 / 522 / 0` on the two claim-side
  sheets and a neutral `4331 / 553 / 2271` sibling `reference` sheet, route
  there instead of retrying clause-`5` family identity one layer deeper as if
  it were fresh leverage
- treat the representative `claim_flat_domain` clause-`2` sheet's clause-`6`
  continuations as live autonomy detail too; the `claim_next_codomain`,
  `claim_sharp_codomain`, and `reference` continuations now all reland the
  same matched smaller tradeoff shell and differ only by a tiny deeper
  zero-admitted tail delta, so route to the live docs before retrying
  clause-`6` identity as if it were fresh leverage
- treat the deeper representative mismatch-`0` claim-side clause-`6`
  `reference` union as live autonomy detail too; if the live docs already pin
  that cross-sheet union at the same `4355 / 551 / 2265` pair-cell shell with
  `small_cluster = 3150 / 522 / 522 / 0`, plus the same zero-admitted
  `((0, None, None), 2265)` remaining-one family, the full `+18`
  `small_cluster` widening localized to six released `3`-generated /
  `0`-admitted `NeedsFallback` groups, and no cached bound or hidden live
  pocket, route there instead of reopening a claim-side union as if it were a
  fresh repair class
- treat the representative mismatch-`0` claim-side parent-route probes as
  live autonomy detail too; if the live docs already pin both scoped
  historical-reanchor routes on the active clause-`5`
  `claim_flat_codomain / reference` families at the same unsafe
  `4427 / 545 / 2247` negative-control shell with noncanonical `60 / 8`,
  `incumbent_dominance = 117`,
  `small_cluster = 2931 / 455 / 455 / 115`, a reopened `single` bucket, and a
  delta localized only to the four targeted claim-side remaining-two parent
  cells plus their `24` matching remaining-one pruned prefixes on the chosen
  active clause-`5` bucket, route there instead of reopening either
  parent-route sibling as if it were a fresh repair class
- treat the representative mismatch-`0` claim-side parent-route
  clause-`4` `reference` split as live autonomy detail too; if the live docs
  already pin both active clause-`5` `claim_flat_codomain / reference` probes
  at the same unsafe `4391 / 557 / 2259` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 113`, `small_cluster = 2871 / 435 / 435 / 111`, the
  same reopened `single` bucket, a shifted `19 / 19 / 12` clause-`2` spread,
  a shifted `24 / 26` clause-`4` split, and a delta that only swaps two
  remaining-three clause-`4` `reference` parent captures into four targeted
  remaining-two clause-`4` `reference` capture families plus `12` removed
  pruned prefixes on the chosen active clause-`5` bucket, route there instead
  of reopening that narrower route split as if it were a fresh repair class
- treat the representative mismatch-`0` claim-side parent-route
  clause-`4` `claim_next_bridge` split as live autonomy detail too; if the
  live docs already pin both active clause-`5`
  `claim_flat_codomain / reference` probes at the broader unsafe
  `4427 / 545 / 2271` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 117`, `small_cluster = 2931 / 455 / 455 / 115`, the
  same reopened `single` bucket, the same `15 / 15 / 12` clause-`2` spread,
  the same `24 / 18` clause-`4` split, and the same four-cell plus
  `24`-pruned-prefix targeted delta on the chosen active clause-`5` bucket,
  route there instead of reopening that broader route split as if it were a
  fresh repair class
- treat the first alternate representative mismatch-`0` claim-side
  active-window qualification family as live autonomy detail too; if the live
  docs already pin both active clause-`5`
  `claim_flat_codomain / reference` probes at the same unsafe
  `4427 / 545 / 2247` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same four-cell plus `24`-pruned-prefix targeted delta,
  route there instead of reopening that alternate qualification family as if
  it were a fresh repair class
- treat the next alternate representative mismatch-`0` claim-side
  self-contained qualification family as live autonomy detail too; if the
  live docs already pin both active clause-`5`
  `claim_flat_codomain / reference` probes at the same unsafe
  `4427 / 545 / 2247` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same four-cell plus `24`-pruned-prefix targeted delta,
  route there instead of reopening that alternate qualification family as if
  it were a fresh repair class
- treat the matched alternate active-window and self-contained clause-`4`
  `claim_next_bridge` splits as live autonomy detail too; if the live docs
  already pin both active clause-`5` `claim_flat_codomain / reference`
  probes at the broader unsafe `4427 / 545 / 2271` shell with noncanonical
  `60 / 8`, `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, the same `15 / 15 / 12` clause-`2` spread, the same
  `24 / 18` clause-`4` split, and the same four-cell plus
  `24`-pruned-prefix targeted delta, route there instead of reopening those
  broader branch splits as if they were a fresh repair class
- treat the narrower clause-`6` `reference` refinements of those same
  alternate active-window and self-contained families as live autonomy detail
  too; if the live docs already pin `claim_flat_codomain` at unsafe
  noncanonical `60 / 8` with `retained = 4`,
  `incumbent_dominance = 113`,
  `small_cluster = 2904 / 462 / 462 / 109 / 2`, and pin `reference` at the
  same unsafe `60 / 8` with `retained = 2`,
  `incumbent_dominance = 115`, the same
  `2904 / 462 / 462 / 109 / 2` `small_cluster`, no `single` bucket, best
  overshoot `545 / 5278`, and the same four-cell plus `24`-pruned-prefix
  targeted delta, route there instead of reopening either narrower alternate
  selector as if it were a fresh repair class
- treat the narrower claim-flat clause-`3`
  `claim_flat_argument / claim_eventual_argument` refinements of those same
  alternate active-window and self-contained families as live autonomy detail
  too; if the live docs already pin both branches on the active
  `claim_flat_codomain` bucket at unsafe noncanonical `60 / 8` with
  `retained = 2`, `generated = 4379`,
  `partial_prefix_bar_failure = 549`, `incumbent_dominance = 110`,
  zero-admitted captures `2259`,
  `small_cluster = 2880 / 486 / 486 / 108`, the same reopened `single`
  bucket, and the same four-cell plus `12`-pruned-prefix targeted delta,
  route there instead of reopening either narrower alternate clause-`3`
  selector as if it were a fresh repair class
- treat the narrower clause-`6` `reference` refinement of that same
  representative mismatch-`0` claim-side parent-route class as live autonomy
  detail too; if the live docs already pin both active clause-`5`
  `claim_flat_codomain / reference` refinements at unsafe noncanonical
  `74 / 8` with `retained = 1`, the same `4427 / 545 / 2247`,
  `incumbent_dominance = 111`,
  `small_cluster = 2904 / 430 / 430 / 108`, a still-fenced `single` bucket,
  and the same four-cell plus `24`-pruned-prefix delta, route there instead
  of reopening that marginally narrower clause-`6` selector as if it were a
  fresh repair class
- treat recombined representative mismatch-`0` claim-side parent-route plus
  alternate qualification families as live autonomy detail too; if the live
  docs already pin both historical-reanchor plus active-window and
  historical-reanchor plus self-contained hybrids across the active
  clause-`5` `claim_flat_codomain / reference` buckets at the same unsafe
  `4427 / 545 / 2247` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, and the same reopened `single`
  bucket, route there instead of reopening either recombination as if it were
  fresh leverage
- treat the first looser representative mismatch-`0` claim-side
  active-window plus self-contained recombination as live autonomy detail too;
  if the live docs already pin both active clause-`5`
  `claim_flat_codomain / reference` probes at that same unsafe
  `4427 / 545 / 2247` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same four-cell plus `24`-pruned-prefix targeted delta,
  route there instead of reopening that looser recombination as if it were
  fresh leverage
- treat the full representative mismatch-`0` claim-side
  historical-reanchor plus active-window plus self-contained qualification
  triad as live autonomy detail too; if the live docs already pin both active
  clause-`5` `claim_flat_codomain / reference` probes at that same unsafe
  `4427 / 545 / 2247` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same four-cell plus `24`-pruned-prefix targeted delta,
  route there instead of reopening that full qualification triad as if it
  were fresh leverage
- treat the alternate active-window or self-contained clause-`4`
  `reference` split as live autonomy detail too; if the live docs already pin
  both active clause-`5` `claim_flat_codomain / reference` probes at the
  same smaller unsafe `4391 / 557 / 2271` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2880 / 486 / 486 / 108`, the same reopened `single`
  bucket, the same `15 / 15 / 12` clause-`2` spread, the same
  `24 / 18` clause-`4` split, and the same two-remaining-three-to-four-
  remaining-two targeted clause-`4` `reference` delta, route there instead
  of reopening that alternate clause-`4` split as if it were fresh leverage
- treat the representative claim-flat clause-`3` refinement inside that same
  mismatch-`0` claim-side parent-route family as live autonomy detail too; if
  the live docs already pin both `claim_flat_argument /
  claim_eventual_argument` branches on the active `claim_flat_codomain`
  bucket at the same smaller unsafe `4379 / 549 / 2259` shell with
  noncanonical `60 / 8`, `incumbent_dominance = 113`,
  `small_cluster = 2871 / 435 / 435 / 111`, and the same reopened `single`
  bucket, route there instead of reopening clause-`3` identity as if it were
  a fresh repair class
- treat the representative `claim_flat_domain` clause-`2` sheet's marginally
  best clause-`6` `reference` continuation's clause-`3` split as live
  autonomy detail too; the `claim_flat_argument` and
  `claim_eventual_argument` branches are now individually neutral controls on
  the untouched `4331 / 553 / 2271` baseline, and the broader
  `4343 / 552 / 2268` clause-`6` `reference` tradeoff only appears when both
  clause-`3` branches reopen together, so route to the live docs before
  retrying clause-`3` identity as if one branch were newly privileged
- treat that broader joint clause-`3` continuation below the representative
  `claim_flat_domain` clause-`2` sheet's marginally best clause-`6`
  `reference` shell as live autonomy detail too; it is now localized to one
  remaining-two parent capture plus its three clause-`6` remaining-one child
  continuations, so route to the live docs before retrying the whole joint
  continuation and instead localize the next probe to a finer terminal or
  remaining-one completion partition inside that parent/child shell
- treat the sibling active clause-`5` `reference` family's representative
  `claim_flat_domain` marginally best clause-`6` `reference`
  continuation's clause-`3` split as live autonomy detail too; its
  `claim_flat_argument` and `claim_eventual_argument` branches are now
  individually neutral on the untouched `4331 / 553 / 2271` baseline with
  the same `312 / 177 / 50 / 14` first-mismatch counts and the same
  `small_cluster = 3132 / 522 / 522 / 0`, so route to the live docs before
  privileging either branch as if it were newly productive
- treat the broader joint clause-`3` continuation below that same sibling
  active clause-`5` `reference` family shell as live autonomy detail too;
  once those two clause-`3` branches are pinned as individually neutral, the
  next probe should localize the joint continuation beneath the same
  marginally best clause-`6` `reference` shell rather than swapping among
  spent clause-`6` siblings or reopening the representative clause-`2`
  split there
- treat that same sibling active clause-`5` `reference` family's
  representative claim-flat parent/child shell as live autonomy detail too;
  the broader joint clause-`3` continuation now differs from either
  individually neutral clause-`3` branch only by one remaining-two parent
  capture on
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  plus the same three clause-`6` remaining-one child continuations
  `claim_next_codomain / claim_sharp_codomain / reference`, so route to the
  live docs before retrying the whole joint continuation and localize the
  next probe to a finer remaining-one completion or terminal partition inside
  that shell
- treat that same sibling active clause-`5` `reference` family's
  representative claim-flat parent/child shell as live autonomy detail too;
  its six clause-`3` / clause-`6` child continuations now all collapse to
  matched dead `3`-generated / `0`-admitted completion summaries with no
  bound, no survivor sketch, and only the same nonlive
  `reference / eventual_lift / next_lift` open-band structural terminal trio,
  so route to the live docs before spending another turn on that exhausted
  claim-flat shell as if a finer completion or terminal partition were still
  unresolved
- treat that same sibling active clause-`5` `reference` family's
  representative claim-flat dead-child reason split as live autonomy detail
  too; its first finer reason pass is now uniform at clause `2` with the
  same structurally connected but nonqualifying connectivity vector across
  all clause-`3` / clause-`6` / terminal continuations, so route to the live
  docs before reopening that claim-flat shell as if a finer reason partition
  were still unresolved
- once the live docs mark that sibling active clause-`5` `reference`
  family's representative `claim_flat_domain` shell as spent through both
  completion and first finer reason split, move sideways to the sibling
  `claim_sharp_codomain` sheet on
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / reference`
  instead of retrying the spent claim-flat shell again
- treat that same sibling active clause-`5` `reference` family's
  representative `claim_sharp_codomain` clause-`6` continuations as live
  autonomy detail too; they now all reland the same `4343 / 552` shell with
  `small_cluster = 3141 / 522 / 522 / 0`, representative clause-`2` spread
  `15 / 14 / 12`, representative clause-`4` split `23 / 18`, the active
  `claim_next_bridge` plus clause-`5` `reference` bucket at `47`, and only
  a tiny deeper zero-admitted tail delta `2270 / 2269 / 2268`, so route
  below the marginally best clause-`6` `reference` continuation instead of
  reopening the spent clause-`6` siblings there
- treat that same sibling active clause-`5` `reference` family's
  representative `claim_sharp_codomain` marginally best clause-`6`
  `reference` continuation's clause-`3` split as live autonomy detail too;
  its `claim_flat_argument` and `claim_eventual_argument` branches are now
  individually neutral on the untouched `4331 / 553 / 2271` baseline with
  the same `312 / 177 / 50 / 14` first-mismatch counts and the same
  `small_cluster = 3132 / 522 / 522 / 0`, so route to the live docs before
  privileging either branch as if it were newly productive
- treat the broader joint clause-`3` continuation below that same sibling
  active clause-`5` `reference` family claim-sharp shell as live autonomy
  detail too; it now differs from either individually neutral clause-`3`
  branch only by one remaining-two parent capture on
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / reference`
  plus the same three clause-`6` remaining-one child continuations
  `claim_next_codomain / claim_sharp_codomain / reference`, so route to the
  live docs before rerunning the whole joint continuation and localize the
  next probe to a finer remaining-one completion or terminal partition
  inside that shell
- treat that same sibling active clause-`5` `reference` family's
  representative claim-sharp parent/child shell as live autonomy detail too;
  its six clause-`3` / clause-`6` child continuations now all collapse to
  matched dead `3`-generated / `0`-admitted completion summaries with no
  bound, no survivor sketch, and only the same nonlive
  `reference / eventual_lift / next_lift` open-band structural terminal trio,
  so route to the live docs before spending another turn on that exhausted
  claim-sharp shell as if a finer completion or terminal partition were
  still unresolved
- treat that same sibling active clause-`5` `reference` family's
  representative claim-sharp dead-child reason split as live autonomy detail
  too; its first finer reason pass is now uniform at clause `2` with the
  same structurally connected but nonqualifying connectivity vector across
  all clause-`3` / clause-`6` / terminal continuations, so route to the live
  docs before reopening that claim-sharp shell as if a finer reason
  partition were still unresolved
- once the live docs mark that sibling active clause-`5` `reference`
  family's representative `claim_sharp_codomain` shell as spent through both
  completion and first finer reason split, move sideways to the first
  sibling exact-screen pair on that same active clause-`5` `reference`
  family, starting with the representative `claim_flat_domain` sheet on
  `claim_eventual_domain / claim_sharp_codomain / claim_flat_domain / claim_next_bridge / reference`,
  instead of retrying the spent representative pair again
- treat that first sibling exact-screen pair on the same active clause-`5`
  `reference` family as live autonomy detail too; its
  `claim_flat_domain` and `claim_sharp_codomain` clause-`2` sheets now each
  land the same smaller `4343 / 552 / 2271` tradeoff with the targeted exact
  pair only at `41` and the localized clause-`2` split only at
  `14 / 15 / 12` or `15 / 14 / 12`, while the sibling `reference` sheet is
  neutral on the untouched `4331 / 553 / 2271` baseline with that pair still
  at `42`, so route to the live docs before reopening the sibling
  claim-sharp or neutral reference sheet as if one were a fresh lead
- treat that same first sibling exact-screen `claim_flat_domain` sheet on the
  active clause-`5` `reference` family as live autonomy detail too; its
  exact-screen delta now removes exactly one six-clause exact-prune capture
  on
  `claim_eventual_domain / claim_sharp_codomain / claim_flat_domain / claim_next_bridge / reference`
  with no new exact-prune family and no deeper pruned-terminal change, so
  route to the live docs before pretending clause-`6` identity is already in
  scope there
- treat the stacked remaining-one exact-summary relief beneath that same
  released first sibling exact-screen `claim_flat_domain` plus clause-`5`
  `reference` parent as live autonomy detail too; it keeps the same
  `4343 / 552` wall, the same `311 / 177 / 50 / 14`, and the same
  `small_cluster = 3141 / 522 / 522 / 0`, but sharpens the deeper
  zero-admitted tail to `2268` and the structurally connected but
  unqualified generated surface to `6804` by shaving exactly three
  remaining-one pruned prefixes, so route to the live docs before moving
  sideways to the sibling `claim_sharp_codomain` sheet and instead localize
  the clause-`6` continuations beneath that sharper `4343 / 552 / 2268`
  shell
- treat that same first sibling exact-screen `claim_flat_domain` plus
  clause-`5` `reference` shell's clause-`6` continuations as live autonomy
  detail too; they now all reland the same matched smaller `4343 / 552`
  shell, keep first-mismatch counts `311 / 177 / 50 / 14`, keep
  `small_cluster = 3141 / 522 / 522 / 0`, keep the localized clause-`2`
  spread at `14 / 15 / 12`, keep the localized clause-`4` split at `23 / 18`,
  keep the active clause-`4` `claim_next_bridge` plus clause-`5`
  `reference` bucket at `47`, and differ only in the deeper zero-admitted
  tail `2270 / 2269 / 2268`, so route below the marginally best clause-`6`
  `reference` continuation instead of reopening the spent
  `claim_next_codomain` or `claim_sharp_codomain` siblings or moving sideways
  to the sibling exact-screen `claim_sharp_codomain` sheet
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
  autonomy detail too; its six clause-`3` / clause-`6` child continuations
  now all collapse to matched dead `3`-generated / `0`-admitted completion
  summaries with no bound, no survivor sketch, and only the same nonlive
  `reference / eventual_lift / next_lift` open-band structural terminal trio,
  so route to the live docs before spending another turn on that exhausted
  exact-screen claim-flat shell as if a finer completion or terminal
  partition were still unresolved
- treat that same first sibling exact-screen `claim_flat_domain` plus
  clause-`5` `reference` shell's dead-child reason split as live autonomy
  detail too; its first finer reason pass is now uniform at clause `2` with
  the same structurally connected but nonqualifying connectivity vector
  across all clause-`3` / clause-`6` / terminal continuations, so route to
  the live docs before reopening that exact-screen claim-flat shell as if a
  finer reason partition were still unresolved
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
  active clause-`5` `reference` family as two claim-side `4343 / 552 / 2271`
  smaller tradeoff controls plus one neutral `reference` sheet on
  `claim_eventual_domain / reference / claim_flat_domain / claim_next_bridge / reference`,
  route below the representative `claim_flat_domain` sheet there instead of
  reopening the sibling `claim_sharp_codomain` or neutral `reference`
  sheets as if one were newly productive
- treat that same next sibling exact-screen `claim_flat_domain` sheet on the
  active clause-`5` `reference` family as live autonomy detail too; its
  exact-screen delta now removes exactly one six-clause exact-prune capture
  on
  `claim_eventual_domain / reference / claim_flat_domain / claim_next_bridge / reference`
  with no new exact-prune family and no deeper pruned-terminal change, and
  stacking remaining-one exact-summary relief beneath that same released
  parent keeps the same `4343 / 552` wall and the same
  `311 / 177 / 50 / 14`, but sharpens the deeper zero-admitted tail to
  `2268` and the structurally connected but unqualified generated surface to
  `6804`
- treat that same next sibling exact-screen `claim_flat_domain` plus
  clause-`5` `reference` shell's clause-`6` and clause-`3` identities as
  live autonomy detail too; its three clause-`6` continuations now reland
  the same matched smaller `4343 / 552` shell and differ only in the deeper
  zero-admitted tail `2270 / 2269 / 2268`, while both
  `claim_flat_argument` and `claim_eventual_argument` branches beneath the
  marginally best clause-`6` `reference` continuation reland the same
  matched smaller `4343 / 552 / 2271` shell, so route there instead of
  privileging either clause label as a fresh repair class
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
- treat that representative claim-flat parent/child shell as live autonomy
  detail too; its six clause-`3` / clause-`6` child continuations now all
  collapse to matched dead `3`-generated / `0`-admitted completion summaries
  with the same nonlive open-band structural terminal trio, so route to the
  live docs before spending another turn on the exhausted claim-flat shell
- treat the representative mismatch-`0` claim-flat dead-child reason split as
  live autonomy detail too; if the live docs have already localized that
  shell's first finer reason pass to one uniform clause-`2` blocker plus the
  same nonqualifying connectivity vector across all clause-`3` / clause-`6` /
  terminal continuations, route there instead of reopening that dead child as
  if another mismatch-`0` identity or reason reland were still fresh
- treat the representative mismatch-`0` claim-sharp dead-child completion and
  reason split as live autonomy detail too; if the live docs have already
  localized that shell to the same dead `3`-generated / `0`-admitted child
  summaries and the same uniform clause-`2` blocker plus nonqualifying
  connectivity vector, route there instead of reopening that sharper dead
  child as if another mismatch-`0` identity or reason reland were still fresh
- treat the isolated clause-`1` `demo_flat_codomain` exact-suffix side
  pocket as live autonomy detail too; if the live docs have already pinned
  that lone reland at `4349 / 556 / 2268` with
  `small_cluster = 3141 / 522 / 522 / 0` and the isolated `single` pocket
  still fenced, route there instead of reopening that side pocket as if it
  were still a fresh repair lead
- treat the promoted `reference / reference` tail split as live autonomy
  detail too; if the live docs have already localized that tail to mismatch
  `2 = 42` versus mismatch `3 = 12` with mismatch-`2` clause-`4` pressure
  still concentrated on `claim_next_bridge = 18` and `reference = 16`, route
  there instead of reopening broader mismatch-`0` or claim-safe shells as if
  the whole tail were still one undifferentiated lead
- treat the larger mismatch-`2` `reference / reference` clause-`4`
  `claim_next_bridge` and `reference` halves as live autonomy detail too; if
  the live docs have already pinned them as wider-`small_cluster` tradeoff
  controls, route there instead of reopening either larger half as if it were
  still a fresh safe repair lead
- treat the tiny mismatch-`2` `reference / reference` clause-`4`
  `demo_sharp_codomain` and `demo_sharp_bridge` pockets as live autonomy
  detail too; if the live docs have already pinned them as matched smaller
  tradeoff controls, route there instead of reopening either demo-side pocket
  as if it were still a fresh repair lead
- treat the smaller mismatch-`3` `reference / reference` clause-`4`
  `claim_next_bridge` and `reference` halves as live autonomy detail too; if
  the live docs have already pinned those halves as smaller or sharper
  tradeoff controls, route there instead of reopening either mismatch-`3`
  half as if it were still a fresh repair lead
- once the live docs say those two mismatch-`2` demo-side pockets and both
  mismatch-`3` halves are spent too, move to the next broader-backup
  comparison before reopening broader mismatch-`0` or claim-safe shells by
  restating the same `reference / reference` tail
- if that broader-backup comparison is already explicit in the live docs,
  keep the tighter representative mismatch-`0` claim-side clause-`2` shell
  ahead of the looser representative claim-safe claim-side shell; do not
  promote the claim-safe backup first unless the mismatch-`0` backup has been
  ruled out again by a later broader comparison rather than by another
  representative dead-shell reland
- treat the representative `claim_sharp_codomain` clause-`2` sheet's
  clause-`6` continuations as live autonomy detail too; the
  `claim_next_codomain`, `claim_sharp_codomain`, and `reference`
  continuations now also reland the same matched smaller tradeoff shell and
  differ only by a tiny deeper zero-admitted tail delta, so route to the
  live docs before retrying clause-`6` identity on the representative
  claim-sharp sheet as if it were fresh leverage
- treat the representative claim-safe mismatch-`1` clause-`4`
  `demo_sharp_codomain` and `demo_sharp_bridge` sides' exact pair-cell
  relands as live autonomy detail too; both sides now reland the same smaller
  negative-control shell on the `reference / claim_next_codomain` and
  `reference / claim_sharp_codomain` pairings, so route to the live docs
  before spending another turn swapping between those claim-safe pairings or
  clause-`4` side siblings as if one were newly privileged
- treat the representative claim-safe exact pair cell's clause-`2` split as
  live autonomy detail too; on the chosen
  `reference / claim_next_codomain / demo_sharp_codomain` cell, the two
  claim-side sheets are matched smaller negative controls while the sibling
  `reference` sheet is only a search-neutral control, so route to the live
  docs before retrying that clause-`2` identity split as if one sheet were
  newly privileged
- treat the representative claim-safe claim-side sheets' clause-`5` /
  clause-`6` shell as live autonomy detail too; beneath that same chosen cell,
  both claim-side sheets now reland the same six dead prefixes and every
  terminal continuation still needs fallback, so route to the live docs before
  retrying clause-`5` or clause-`6` identity there as if one sibling were a
  fresh lead
- treat the representative claim-safe `claim_flat_domain` dead prefix's
  terminal-family partition as live autonomy detail too; beneath
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`,
  all six dead prefixes now reland the same `3`-generated / `0`-admitted
  profile with the same `reference / eventual_lift / next_lift` trio, and
  every one of those completed telescopes is still structurally connected but
  outside historical reanchor and active-window qualification, so route to the
  live docs before retrying terminal-family identity there as if one terminal
  were newly privileged
- treat the representative claim-safe dead prefix's clause-`5`
  qualification split as live autonomy detail too; under the selected
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`
  exact pair, clause-`5` `reference` is the only exact control while
  `claim_flat_codomain / claim_next_codomain / demo_sharp_domain /
  demo_flat_codomain` all stop at `5 / mismatch 5`, and the live dead shell
  itself still uses only the two claim-side dead labels, so route to the live
  docs before retrying clause-`5` identity there as if a demo-side control
  were still live
- treat any deeper reason-level pass below that representative claim-safe dead
  prefix as live autonomy detail too; if the live docs have already localized
  the completed shell to the clause-`5` qualification wall, route there
  instead of reopening clause-`6` or terminal-family identity as if those
  controls were newly privileged
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
- treat negative controls, tradeoff controls, and probe outcomes as ledger
  entries owned by [../../autonomous_ledger.md](../../autonomous_ledger.md)
- keep the remaining local guardrails green while moving toward
  family-agnostic structural evidence and honest certification language
- keep nearby step-`13` widenings and anchor-`11` side pockets fenced unless a
  new active slice explicitly promotes them in
  [../../autonomous_next_steps.md](../../autonomous_next_steps.md)

### If you are working on reporting or evidence

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [../../overall_plan.md](../../overall_plan.md)
- [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)

Focus on:

- keeping semantic names out of the hot path
- surfacing accepted candidates, near misses, and retained valid candidates
- making inspect/debug output explain current behavior honestly
- not pretending frontier evidence already exists when it does not

### If you are implementing resume or storage

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [references/09-rust-rewrite-blueprint.md](references/09-rust-rewrite-blueprint.md)
- [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)

Focus on:

- step checkpoints as the stable unit
- frontier checkpoints as disposable speed artifacts
- compatibility hashes and migration behavior
- the gap between current manifest contracts and real shard/blob/queue runtime

### If you are implementing Agda verification

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [theory/late-framework-abstraction.md](theory/late-framework-abstraction.md)
- [theory/terminal-dct.md](theory/terminal-dct.md)
- [references/07-agda-validation.md](references/07-agda-validation.md)
- [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)

Focus on:

- deterministic export payloads
- witness and import consistency
- keeping Agda completely outside the hot loop
- strengthening the bridge without changing acceptance truth

### If you are checking whether a design choice is honest

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [references/01-project-brief.md](references/01-project-brief.md)
- [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)
- [references/10-open-questions.md](references/10-open-questions.md)

Reject designs that:

- smuggle semantic names or target IDs into core crates
- let reporting or decoding influence selection
- hide replay or template shortcuts behind supposedly atomic enumeration
- treat placeholder runtime modules as completed storage or resume behavior
- rely on floating-point comparisons for core admissibility or ranking

## Quick Summary

- The current Rust workspace is already the primary executable truth for the
  strict `15`-step lane.
- The main realistic-search gap is stronger sound bound pruning plus broader
  non-family admissibility reuse on top of the already-landed memo/bound path.
- The tracked demo-lane signoff set is currently closed; use the repo-level
  demo-lane docs and `references/12-current-demo-lane.md` as the regression
  baseline for future work.
- The claim lane has real configs, artifacts, compare/certify/benchmark
  tooling, landed claim-specific mechanics, and a current certified stored
  head beyond the previous audited head that preserves the ancestry-gated
  post-reference exact-two-step `7211 / 553 / 2052` step-`15` repair while
  closing stored step `1` at `2144 / 1285 / 1 / 475`; live counters and probe
  history now live in the autonomous docs rather than in this skill file.
- Rust-side claim report and narrative consumers now keep the stored
  step-`15` `demo_closure` surface honest across step reports, latest reports,
  and completed resume output, so report honesty is no longer the blocker.
- Native `pen-cli` compare/certify/benchmark commands now refresh current-head
  claim evidence locally, and the first current-head step-`1` repair is now
  stored rather than local-only: `DesktopClaimShadow` config runs borrow the
  shared claim/demo exhaustive path only at step `1`, restoring
  `generated = 2144` on the smoke config while keeping accepted parity through
  step `8` and leaving step `5+` on the non-budgeted claim path, and the
  certified stored head now reflects that same repair. The next honest
  claim-lane blocker is the branch-localized local step-`15`
  `partial_prefix_bar_failure = 553` wall: all clean captures now sit below
  queued frontier remainder branches `4 .. 11`, and the largest single live
  blocker is now the mismatch-`1` surface inside the direct top-level
  `reference` remainder at `177 / 145 / 32`; within that slice, the first
  narrow pairing to attack is `reference / demo_flat_codomain = 61`, and the
  next exact lead inside it is the remaining-two clause-`4`
  `claim_next_bridge = 33` side. The broadest repaired-head exact claim-pair
  reopening under that side is now frozen as a tradeoff control at
  `7485 / 539 / 2112 / 542 / 542 / 0` with direct `reference` remainder
  `227` and targeted pairing `45`; the tied exact
  `claim_flat_domain` / `claim_sharp_codomain` single-sheet reopenings are
  now frozen too at `7317 / 545 / 2076 / 530 / 530 / 0` with direct
  `reference` remainder `233`, targeted pairing `51`, and slot split
  `30 / 21 / 14`; the exact clause-`2` `reference = 9` sheet is now frozen
  too at `7149 / 551 / 2040 / 518 / 518 / 0` with direct `reference`
  remainder `239`, targeted pairing `57`, and slot split `33 / 24 / 14`.
  That safe control trims only the clause-`4` `reference` companion, not the
  live `claim_next_bridge = 33` blocker, and its attempted landing extension
  is now frozen too: reopening the clause-`4` `reference` companion under the
  same exact reference sheet is completely neutral and relands the same
  `7149 / 551 / 2040 / 518 / 518 / 0` surface. The first exact
  `claim_flat_domain` and exact `claim_sharp_codomain` reopenings under that
  broader clause-`4` `reference` companion are now frozen too as spill-only
  tradeoff controls at `7236 / 550 / 2058 / 524 / 524 / 0`; they keep the
  remaining-two clause-`4` `reference` row pinned at `9 / 9 / 6` and trim
  only one remaining-three spill capture to `4 / 5 / 4` or `5 / 4 / 4`.
  The next move is therefore the smaller exact `reference = 6` slice below
  that same clause-`4` `reference` companion before the whole remaining-three
  spill or another step-`1`, storage, or tooling slice.
- Script execution still has no local `python`, `python3`, `py`, or `uv`
  launcher in common paths or repo virtualenvs, but that no longer blocks
  current-head claim evidence refresh on this machine.
- For current claim-lane status, read
  [references/13-current-claim-lane.md](references/13-current-claim-lane.md),
  [../../autonomous_progress.md](../../autonomous_progress.md),
  [../../autonomous_next_steps.md](../../autonomous_next_steps.md), and
  [../../autonomous_ledger.md](../../autonomous_ledger.md).
- Start with [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md), then load only
  the track-specific references you actually need.
