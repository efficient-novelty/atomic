# Step 4 Bottleneck Problem Statement

Reviewed inputs:

- `autonomous_progress.md`
- `autonomous_plan.md`
- `autonomous_next_steps.md`
- `autonomous_checklist.md`
- `crates/pen-search/src/engine.rs`
- `crates/pen-search/src/prefix_memo.rs`

## Executive Summary

Step `4` is no longer blocked by the old early allocator or RSS cliff. The
landed memory and frontier compactions moved that wall. The honest blocker is
now throughput in the remaining-one compact summary kernel used by
`desktop_claim_shadow`.

The kept short baseline
`runs/codex-claim-release-step4-kernel-aggregation-v1` holds the honest
`39 groups / 144845 candidates` retained surface at checkpoints `24/43/44/54`.
The kept full-profile baseline
`runs/codex-claim-release-full-kernel-aggregation-v1` proved that this is not
the terminal full-profile shape: the intended profile later reopens to
`40 groups / 147639 candidates` at `74` and then `41 groups / 154842 candidates`
at `140`. That means the real target is not only the early plateau. Any keep
patch must preserve the early `39/144845` surface and still help, or at least
not regress, the reopened late surface.

The current late diagnostic
`runs/codex-claim-release-step4-kernel-late-profile-v1` shows that the largest
measured share of `terminal_summary_build_*` at the reopened surface is still
aggregation work inside the summary kernel, followed by connectivity, clause
filtering, and exact `nu`. The remaining unattributed tail is real, but it is
not the first wall to attack.

The failed summary-bookkeeping slice
`runs/codex-claim-release-step4-kernel-summary-bookkeeping-v1` is the clearest
proof of the present bottleneck. It improved elapsed time materially on both
the matched early surface and the reopened late surface, but it still regressed
too much on total `terminal_summary_build_*` at `24/43/44/54`. In other words:
some bookkeeping overhead is real, but the core wall is still inside the
measured summary kernel itself.

## Formal Problem Statement

We need one narrow change inside the remaining-one exact-screen pipeline that:

- lowers both wall clock and `terminal_summary_build_*` at `24/43/44/54`
  versus `runs/codex-claim-release-step4-kernel-aggregation-v1`
- keeps the same honest retained-prefix surface
- still preserves the reopened `40/147639` read at `74/76`
- preserves exact winner selection and exact tie-break truth
- avoids broad rewrites, lossy surrogate ordering keys, or cost shifts that
  only make elapsed look better while leaving measured summary-build worse

More concretely, the task is to remove work that is already charged inside
`compute_terminal_prefix_completion_summary_from_candidates`, not merely move
diagnostic or discovery-count bookkeeping elsewhere.

## Evidence From Stored Runs

### Kept Short Baseline

- Run:
  `runs/codex-claim-release-step4-kernel-aggregation-v1`
- Honest retained shape:
  `39 groups / 144845 candidates`
- Matched checkpoints:
  - `24`: `elapsed_millis = 549630`,
    `terminal_summary_build_millis = 492524`
  - `43`: `elapsed_millis = 990480`,
    `terminal_summary_build_millis = 892772`
  - `44`: `elapsed_millis = 1012067`,
    `terminal_summary_build_millis = 912271`
  - `54`: `elapsed_millis = 1247600`,
    `terminal_summary_build_millis = 1126754`

### Kept Full-Profile Baseline

- Run:
  `runs/codex-claim-release-full-kernel-aggregation-v1`
- It later reopened beyond the early plateau:
  - `74`: `40 groups / 147639 candidates`
  - `140`: `41 groups / 154842 candidates`
- It never reached step `5`.
- The last stored step-`4` checkpoint was `163` with:
  - `elapsed_millis = 3942636`
  - `observed_process_rss_bytes = 632197120`
  - `frontier_queue_len = 2612`
  - `terminal_summary_build_millis = 3584914`

### Late Diagnostic

- Run:
  `runs/codex-claim-release-step4-kernel-late-profile-v1`
- At `76` on the reopened surface:
  - `terminal_summary_build_micros = 1839910636`
  - aggregation `= 469431036 us`
  - connectivity `= 416766880 us`
  - clause filtering `= 352203534 us`
  - exact `nu` `= 267574759 us`
  - unexplained tail `= 333934427 us` (`18.15%`)
- Incremental `54 -> 76` still had aggregation first:
  - aggregation `+141716373 us`
  - connectivity `+124894828 us`
  - clause filtering `+107776335 us`
  - exact `nu` `+80574865 us`

### Latest Failed Slice

- Run:
  `runs/codex-claim-release-step4-kernel-summary-bookkeeping-v1`
- It preserved the honest shapes at `24/43/44/54/74/76`, including the
  reopened `40/147639` surface at `74/76`.
- It improved elapsed materially, but it failed keep because the total
  summary-build wall got worse on the matched early surface.

| Checkpoint | Kept baseline elapsed / summary-build | Summary-bookkeeping elapsed / summary-build |
| --- | --- | --- |
| `24` | `549630 / 492524` | `533912 / 529427` |
| `43` | `990480 / 892772` | `960978 / 955495` |
| `44` | `1012067 / 912271` | `981674 / 976142` |
| `54` | `1247600 / 1126754` | `1207236 / 1201231` |

At the reopened surface it was much better than the late diagnostic on elapsed
and total summary-build, but it still trailed the kept full-profile aggregation
baseline on total summary-build by roughly `70-73s` at `74/76`.

The lesson is important: bookkeeping-only cleanup can help elapsed, but it has
not yet moved the honest measured summary kernel enough to replace the kept
baseline.

### Current Code Under Test

The working tree currently contains one narrow loop-hoist hypothesis in
`crates/pen-search/src/engine.rs`. It precomputes
`focus_aligned_competitors_only = demo_focus_aligned_competitors_only(...)`
once per prefix summary and then uses
`admitted_terminal_completion_can_compete_for_acceptance(...)` inside the
per-admitted loop instead of recomputing the prefix-wide gate every time.

There is a matching in-progress run directory,
`runs/codex-claim-release-step4-kernel-competition-hoist-v1`, but it is not
yet an honest keep or drop read. The latest stored live artifact has only
reached `prefix_states_explored = 15`, far short of the required
`24/43/44/54/74/76` checkpoints. As of that latest stored point:

- `elapsed_millis = 349425`
- `terminal_summary_build_millis = 345621`
- `terminal_summary_aggregation_millis = 86745`
- `terminal_summary_connectivity_millis = 78781`
- `terminal_summary_exact_nu_millis = 50793`
- `terminal_prefix_clause_filter_millis = 66038`

That confirms the same general shape, but it is still only early evidence.

## Code-Path Analysis

The hot path lives in the claim-only remaining-one exact screen inside
`crates/pen-search/src/engine.rs`.

At a high level:

1. The step-`4` discovery loop pops a frontier prefix.
2. When only one clause slot remains, the claim path first tries
   `claim_try_summary_prune_before_materialization(...)`.
3. That path builds or reuses a compact
   `TerminalPrefixCompletionSummary` for the prefix.
4. The summary is used to:
   - update discovery counters
   - prove `bound.can_clear_bar(objective_bar)` or prune the whole group
   - compare the best reachable rank against the incumbent or prune the whole
     group
5. Only if the group survives does the code materialize terminal candidates.

The core summary builder is
`compute_terminal_prefix_completion_summary_from_candidates(...)`. Its loop is
the step-`4` wall because it executes, per surviving terminal clause:

- cached connectivity lookup
- optional fallback connectivity check
- admissibility decision or cached admissibility reuse
- admissibility diagnostics bookkeeping
- exact `structural_nu(...)`
- bit-cost computation and `PrefixBound` merge
- admitted-candidate counting
- competition gating
- primary-rank comparison
- sometimes full `AcceptRank` construction

This work is multiplicative over a very large remaining-one surface. The late
diagnostic shows that even after earlier wins, the reopened surface still spends
most of its measured time in this kernel.

There is also an important secondary effect: the compact claim path does not
store full `TerminalPrefixClauseEvaluation` payloads when it builds the summary.
If a group survives summary pruning, `materialize_terminal_prefix_group_compact`
replays much of the per-clause work in order to rebuild retained telescopes and
their exact acceptance ranks. That duplication is real cost, but the stored run
evidence says the dominant wall is already inside the summary-build pass itself.

## Rust-Like Pseudocode For The Current Approach

High-level remaining-one screening path:

```rust
fn screen_remaining_one_prefix(prefix: PrefixWorkItem, discovery: &mut Discovery) -> Outcome {
    if algebraic_nu_ceiling_cannot_clear_bar(prefix) {
        cache.store_partial_prefix_bound(prefix.signature, CannotClearBar);
        return PrunedByBar;
    }

    if cache.terminal_prefix_bound_summary(prefix.signature).is_none() {
        let terminal_clauses =
            terminal_prefix_clause_candidates(step, library, admissibility, prefix.signature);

        let summary = compute_terminal_prefix_completion_summary_from_candidates(
            step,
            library,
            admissibility,
            objective_bar,
            nu_history,
            prefix.signature,
            prefix.telescope,
            TerminalPrefixSummaryPayload::Compact,
            terminal_clauses,
            incumbent_rank,
            cache,
            telemetry,
        );

        cache.store_terminal_prefix_completion_summary(prefix.signature, summary);
    }

    let summary = cache.take_terminal_prefix_completion_summary(prefix.signature)?;
    record_terminal_prefix_summary_discovery_counts(discovery, &summary);

    if summary.bound.is_none() || !summary.bound.unwrap().can_clear_bar(objective_bar) {
        return PrunedByBar;
    }

    if summary.best_rank_cannot_beat(incumbent_rank) {
        return PrunedByIncumbent;
    }

    // Compact claim mode does not keep full evaluation payloads, so a surviving
    // group is materialized with a second per-clause pass.
    let group = materialize_terminal_prefix_group_compact(prefix, discovery)?;
    retain_or_prune_after_materialization(group, discovery)
}
```

Current summary kernel, including the new loop-hoist shape:

```rust
fn compute_terminal_prefix_completion_summary_from_candidates(...) -> Summary {
    let mut summary = Summary::default_compact_or_full(payload);
    let clause_kappa_used = prefix_len + 1;
    let prefix_bit_cost = prefix_telescope.bit_cost();
    let mut scratch = terminal_prefix_scratch_telescope(prefix_telescope);

    // Current in-progress hoist: prefix-wide competition policy is computed once.
    let focus_aligned_competitors_only =
        demo_focus_aligned_competitors_only(prefix_signature, admissibility, cache);

    for (clause, cached_admissibility) in terminal_clauses {
        let connectivity = cache.terminal_connectivity(prefix_signature, library, clause)?;
        summary.generated_candidate_count += 1;

        if connectivity == PruneDisconnected {
            maybe_record_disconnected(payload, &mut summary);
            continue;
        }

        let telescope = load_terminal_clause_into_scratch(&mut scratch, prefix_len, clause);
        if connectivity == NeedsFallback && !passes_connectivity(library, telescope) {
            maybe_record_disconnected(payload, &mut summary);
            continue;
        }

        let admissibility_decision = cached_admissibility.unwrap_or_else(|| {
            assess_strict_admissibility(step_index, library, telescope, admissibility)
        });
        summary.admissibility_diagnostics.record(&admissibility_decision);

        if !admissibility_decision.is_admitted() {
            maybe_record_rejection(payload, &mut summary, admissibility_decision);
            continue;
        }

        let exact_nu = structural_nu(telescope, library, nu_history).total as u16;
        let bit_kappa_used = terminal_prefix_completion_bit_cost(prefix_bit_cost, clause);
        summary.bound.absorb(PrefixBound::singleton(
            exact_nu,
            clause_kappa_used,
            bit_kappa_used,
        ));
        summary.admitted_candidate_count += 1;

        if admitted_terminal_completion_can_compete_for_acceptance(
            focus_aligned_competitors_only,
            &admissibility_decision,
        ) {
            if let Some(primary_rank) =
                terminal_prefix_primary_rank(objective_bar, exact_nu, clause_kappa_used)
            {
                let should_build_full_rank =
                    update_best_primary_rank_if_better_or_tied(&mut summary, &primary_rank);

                if should_build_full_rank
                    && primary_rank_could_still_beat_incumbent(&primary_rank, incumbent_rank)
                {
                    if let Some(rank) = acceptance_rank_for_telescope(
                        objective_bar,
                        telescope,
                        exact_nu,
                        bit_kappa_used,
                        clause_kappa_used,
                    ) {
                        update_best_accept_rank_if_better(&mut summary, rank);
                    }
                }
            }
        }

        maybe_record_admitted_completion(payload, &mut summary, telescope, exact_nu, bit_kappa_used);
    }

    summary
}
```

## Approaches Already Tried

### Landed And Still Kept

These are already part of the current baseline and should stay landed:

- delayed materialization
- the incumbent-primary remaining-one fast path
- the one-pass `structural_nu` summary-build fast path
- the algebraic `nu` ceiling patch
- the family-agnostic claim terminal-admissibility shortcut
- the exact non-allocating connectivity summary scan
- the terminal-only cached parent connectivity decision
- the aggregation-side accept-rank short-circuit that skips full
  `AcceptRank` construction for primary-dominated bar-clearers
- the higher-fidelity late-surface timing accumulation

### Tried And Dropped

- Ordering, reuse, cache, and post-plateau variants:
  they did not engage honestly, found no real reuse, or only shifted cost.
- Connectivity-side cache and precompute variants (`kernel-connectivity-v3`,
  `kernel-connectivity-v4`):
  they attacked real work but still regressed.
- Terminal candidate prep and remap (`terminal-candidate-prep-v1`):
  it exposed real prep cost but did not improve the kept surface.
- Exact primary-rank bookkeeping rewrite
  (`kernel-rank-bookkeeping-v1`):
  honest shape, slower clock, dropped.
- Bound-merge bookkeeping rewrite (`kernel-bound-merge-v1`):
  `PrefixBound` churn was real, but not dominant enough to beat the kept short
  baseline on `24/43/44/54`.
- Lazy incumbent-tie `AcceptRank` deferral (`kernel-lazy-acceptrank-v1`):
  exact tie-break work was too small a share of the remaining wall.
- Summary batching (`kernel-summary-batching-v1`):
  helped the reopened `74/76` read, but not enough at the honest early
  checkpoints.
- Summary bookkeeping (`kernel-summary-bookkeeping-v1`):
  improved elapsed on both surfaces, but still regressed total
  `terminal_summary_build_*` too much early.
- Bar-clear threshold bookkeeping rewrite (`kernel-summary-threshold-v1`):
  better aggregation microtime, but still no keep on elapsed plus total
  summary-build.

The pattern across these failures is consistent: local bookkeeping wins exist,
but the dominant wall remains per-admitted work inside the measured summary
kernel.

## Planned Next Steps

1. Keep these reference surfaces fixed while evaluating any new cut:
   - `runs/codex-claim-release-step4-kernel-aggregation-v1`
   - `runs/codex-claim-release-full-kernel-aggregation-v1`
   - `runs/codex-claim-release-step4-kernel-late-profile-v1`
2. Finish the current short rerun for the loop-hoist hypothesis
   `runs/codex-claim-release-step4-kernel-competition-hoist-v1` until it
   captures at least `24/43/44/54/74/76`.
3. Keep the hoist only if it:
   - preserves the honest `39/144845` surface at `24/43/44/54`
   - lowers both elapsed and `terminal_summary_build_*` there
   - and does not lose the reopened `40/147639` read at `74/76`
4. If this hoist fails, stay inside
   `compute_terminal_prefix_completion_summary_from_candidates(...)` and remove
   one more loop-invariant or per-admitted constant from the aggregation block.
5. Do not spend the next turn on:
   - another connectivity-side rewrite
   - another clause-filter-side rewrite
   - another exact-`nu` cleanup first
   - another retry of `kernel-rank-bookkeeping-v1`
   - another retry of `kernel-bound-merge-v1`
   - another retry of `kernel-lazy-acceptrank-v1`
   - another retry of `kernel-summary-batching-v1`
   - another retry of `kernel-summary-bookkeeping-v1`
   - another retry of `kernel-summary-threshold-v1`
   - a lossy hash or surrogate key in place of the exact tie-break
6. Validation for each narrow runtime slice should stay minimal:
   - `cargo test -p pen-search claim_`
   - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`

## Bottom Line

The step-`4` problem is now much narrower than it was earlier in the lane.
Memory is no longer the lead blocker. The honest wall is the remaining-one
summary kernel on the reopened intended-profile surface. The kept code already
removed several obvious costs, including some `AcceptRank` construction. The
failed bookkeeping slice proved that non-kernel overhead exists but did not fix
the measured wall. The current best next move is therefore exactly what the
docs already point to and what the working tree now implements: hoist or
precompute loop-invariant aggregation-side work inside the compact summary
kernel, then judge it only from stored `24/43/44/54/74/76` evidence.
