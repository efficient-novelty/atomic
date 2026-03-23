# Autonomous claim lane: next step-4 moves

This note is for the local coding agent working on `desktop_claim_shadow`.

## Operating read

The current deadlock is **not** a late-band claim mutator problem.

At step `4`, the claim lane is still in the early open-band regime:

- `strict_admissibility_for_mode(..., DesktopClaimShadow)` uses `focus_family = None`
- step `4` is `min_clause_kappa = 3`, `max_clause_kappa = 3`
- claim widening bands `7/8/9` are not active yet when `library_size = 3`

That means the hot cost is the **engine/prefix-memo exact remaining-two path**, not the late-band claim-generic enumerator.

The strongest signal from `autonomous_progress.md` is this:

- the old queue-startup memory cliff is largely gone
- later step-`4` checkpoints plateau in retained prefix groups/candidates
- legality summaries keep rising anyway
- so the remaining wall time is being spent on **exact terminal completion on surfaces that are no longer changing the retained frontier**

That points to earlier incumbent/bar pruning, cheaper cached-summary reuse, and less queue churn.

---

## Do not spend the next slice on these

1. **Do not reopen memory compaction first.** The recent evidence says step `4` is now mostly throughput-bound, not early-RSS-bound.
2. **Do not narrow claim-generic late bands `7/8/9`.** They are not the step-`4` bottleneck.
3. **Do not chase worker-count tuning first.** The hot discovery path here is mostly engine-side exact screening, not frontier planning.
4. **Do not try a broad architecture rewrite first.** Prefer narrow throughput fixes that can be A/B tested on `until_step = 4`.

---

## Recommended implementation order

## 1. Add one narrow telemetry pass before the next rewrite

### Goal

Make the next patch decision from stored step-`4` evidence instead of intuition.

### Touch

- `crates/pen-search/src/engine.rs`
- `reports/steps/step-04-live.ndjson` emission path

### Add counters/timers for the exact remaining-two path

Inside the live step-`4` discovery checkpoints, add counters for:

- `remaining_one_prefixes_seen`
- `remaining_one_cached_bound_hits`
- `remaining_one_cached_rank_prunes`
- `remaining_one_materialized`
- `remaining_one_materialized_from_cached_summary`
- `remaining_one_materialized_compact_direct`
- `remaining_one_bar_prunes_pre_materialize`
- `remaining_one_rank_prunes_pre_materialize`
- `remaining_one_rank_prunes_post_materialize`
- `remaining_one_unknown_bound_budget_exhaustions`

Add wall-clock accumulators for:

- `prepare_exact_two_step_terminal_surface_millis`
- `exact_partial_prefix_bound_millis`
- `terminal_summary_build_millis`
- `terminal_materialize_millis`
- `candidate_sort_millis`
- `candidate_eval_minimality_millis`
- `frontier_sort_pop_millis`

### Why first

Right now the progress doc says the likely remaining stories are:

- earlier incumbent pruning
- broader exact bound screening
- or some still-untracked frontier drain cost

These counters will separate those quickly.

### Acceptance

After one `until_step = 4` release rerun, the live NDJSON should make it obvious which of these dominates:

- cached-summary misses vs hits
- bound-unknowns vs bound-prunes
- materialization time vs frontier-pop time
- pre-materialize vs post-materialize rank pruning

---

## 2. Add a claim-only **cached rank pre-prune** before materialization

### Why this is my top code change

The strongest direct match to the current deadlock is:

> later step-`4` time is still going into already-dominated surfaces

You already have the pieces for a cheap prune:

- `cached_terminal_prefix_rank_prune_count(...)`
- `PrefixLegalityCache::terminal_prefix_rank_summary(...)`
- claim mode already stores exact completion summaries for many queued `remaining_clause_slots == 1` prefixes
- claim mode currently **does not** use the cached-rank path before materializing the group

### Current issue

In both of these places, claim mode materializes a terminal prefix group **before** checking whether the cached best rank can already be pruned by the current incumbent:

- the `remaining_clause_slots == 1` branch in the main discovery loop
- `process_prepared_exact_two_step_terminal_surface(...)`

That is late. By the time `group.best_accept_rank` is available, you have already paid the materialization cost.

### Patch

Before `materialize_terminal_prefix_group(...)`, add a claim-only fast path:

```rust
if should_compact_terminal_prefix_group_candidates(admissibility.mode) {
    if let Some(pruned) = cached_terminal_prefix_rank_prune_count(
        &work_item.signature,
        discovery.terminal_rank_incumbent.as_ref(),
        &mut discovery.prefix_legality_cache,
    ) {
        discovery.terminal_rank_prunes += pruned;
        // record bucket prune stats if available
        // optionally drop cached completion payload here
        continue;
    }
}
```

Do the same for `terminal_prefix` inside `process_prepared_exact_two_step_terminal_surface(...)`.

### Extra improvement

If the cached-rank pre-prune fires in claim mode, immediately discard the cached payload:

- add `drop_terminal_prefix_completion_summary(...)`
- or call `take_terminal_prefix_completion_summary(...)` and drop the result

That keeps claim summaries ephemeral once they are no longer useful.

### Files/functions

- `crates/pen-search/src/engine.rs`
  - main `remaining_clause_slots == 1` branch
  - `process_prepared_exact_two_step_terminal_surface(...)`
  - maybe add helper `claim_try_cached_rank_prune(...)`
- `crates/pen-search/src/prefix_memo.rs`
  - optional: add a drop/remove helper for cached completion payloads

### Tests to add

- `claim_cached_terminal_rank_prunes_before_materialization_in_main_loop`
- `claim_cached_terminal_rank_prunes_before_materialization_in_exact_two_step_path`
- `claim_rank_preprune_drops_cached_completion_payload`

### Success signal

On the next step-`4` rerun:

- `remaining_one_cached_rank_prunes` should be non-trivial
- `remaining_one_materialized` should drop
- late step-`4` frontier queue should drain faster after the incumbent appears

---

## 3. Stop using the all-or-none exact-two handoff

### Current issue

`can_process_exact_two_step_terminal_surface_now(...)` is all-or-none:

- if the slowest child still outranks the frontier head, process all children now
- otherwise queue all children

That is too coarse.

A single late child can force **all** prepared children back onto the frontier, even if many early children should be processed immediately.

### Patch

Replace the all-or-none branch with a stable partition:

1. prepare + sort `terminal_prefixes`
2. compare each child work key against the current frontier head key
3. process the longest in-order prefix of `terminal_prefixes` whose keys still outrank the frontier head
4. exact-bound screen + queue only the remainder

Pseudo-shape:

```rust
let frontier_head_key = frontier.first().map(prefix_frontier_work_key);
let split = terminal_prefixes.partition_point(|item| {
    frontier_head_key.map(|head| prefix_frontier_work_key(item) < head).unwrap_or(true)
});

let (processable_now, deferred) = terminal_prefixes.split_at(split);

// process processable_now in order
// exact-bound screen and queue deferred
```

### Why it likely helps

This should:

- tighten the incumbent earlier
- cut queue churn
- reduce the number of queued `remaining_one` prefixes that later become dominated
- preserve deterministic order because only the prefixes that already outrank the current frontier head are processed in place

### Files/functions

- `crates/pen-search/src/engine.rs`
  - `can_process_exact_two_step_terminal_surface_now(...)`
  - `process_prepared_exact_two_step_terminal_surface(...)`
  - main discovery loop around `prepare_exact_two_step_terminal_surface(...)`

### Tests to add

- `exact_two_step_surface_partially_processes_children_that_outrank_frontier_head`
- `exact_two_step_surface_keeps_deterministic_order_after_split_handoff`

### Success signal

- `frontier_queue_len` should grow more slowly
- `prefix_states_explored` should move farther at equal wall clock
- cached-rank pre-prune should become more useful because the incumbent gets tighter earlier

---

## 4. Add bound-only and rank-only summary accessors so claim cached checks stop cloning full payloads

### Current issue

`cached_terminal_prefix_queue_entry_bound_decision(...)` currently goes through:

- `PrefixLegalityCache::terminal_prefix_completion_summary(...)`

That clones the **entire** cached completion summary payload just to read its `bound`.

For claim mode, that is exactly the wrong tradeoff.

### Patch

Split the accessors in `PrefixLegalityCache`:

- keep the existing full payload accessor for the materialization path
- add cheap accessors that do **not** clone `evaluations`

Suggested additions:

```rust
pub fn terminal_prefix_bound_summary(
    &mut self,
    signature: &PrefixSignature,
) -> Option<Option<PrefixBound>>;

pub fn terminal_prefix_rank_summary(
    &mut self,
    signature: &PrefixSignature,
) -> Option<TerminalPrefixRankSummary>;
```

You already have the rank accessor. Add the bound accessor and use both in claim discovery.

Then change:

- `cached_terminal_prefix_queue_entry_bound_decision(...)`
- the early summary check in `exact_terminal_prefix_bound_decision(...)`

to use the bound-only accessor.

### Why it matters

Even if you do not change the stored summary shape yet, this cuts a lot of avoidable payload cloning.

### Files/functions

- `crates/pen-search/src/prefix_memo.rs`
- `crates/pen-search/src/engine.rs`

### Tests to add

- `claim_cached_bound_check_does_not_clone_full_completion_payload`
- `claim_cached_rank_check_does_not_reopen_completion_summary`

---

## 5. If the previous two patches are not enough, add a **claim discovery summary** and defer full materialization

### Why this is the highest-upside medium-risk change

The current exact-bound path can still compute and cache a heavy `TerminalPrefixCompletionSummary` even when the prefix is later pruned by bar or incumbent.

That means claim discovery is still paying for a lot of payload it never needs to materialize.

### Better design

Split the current summary into two tiers.

### Tier A: cheap discovery summary

Keep only what discovery needs to prune and queue:

- `bound`
- `best_accept_rank`
- `admitted_candidate_count`
- `generated_candidate_count`
- maybe cheap counters for rejected/disconnected clauses

### Tier B: optional compact materialization payload

For claim mode only, if you want to avoid recomputation later, store at most:

- the **last clause** (or clause index / clause hash), not the full cloned telescope
- `AdmissibilityDecision`
- `exact_nu`
- `bit_kappa_used`

Then reconstruct the full telescope only if the prefix survives to materialization/proof-close.

### Important detail

Do **not** store `completion.telescope.clone()` for every admitted clause in claim discovery if the goal is step-`4` throughput.

That is exactly the kind of work you want to delay.

### Minimal version

If you want the narrowest first pass:

- claim mode stores only Tier A
- claim materialization recomputes compactly on demand with `materialize_terminal_prefix_group_compact(...)`

That is simpler and already likely useful because many prefixes never survive long enough to justify eager payload storage.

### Files/functions

- `crates/pen-search/src/prefix_memo.rs`
  - summary types and cache maps
- `crates/pen-search/src/engine.rs`
  - `exact_terminal_prefix_bound_decision(...)`
  - `compute_terminal_prefix_completion_summary_from_candidates(...)`
  - `materialize_terminal_prefix_group(...)`
  - `materialize_terminal_prefix_group_from_summary(...)`

### Tests to add

- `claim_discovery_summary_preserves_bound_and_best_rank`
- `claim_materialization_from_compact_summary_matches_direct_compact_materialization`
- `claim_discovery_summary_prunes_without_full_payload_storage`

### Success signal

- `legality_cache_entries.terminal_prefix_completions` should stop growing as aggressively
- wall time per `remaining_one` prefix should drop
- late step-`4` RSS should stay flat or improve

---

## 6. Remove the remaining per-prefix allocation churn in the hot terminal loops

### A. Stop allocating a tuple `Vec` for every terminal prefix when the claim path is unfiltered

Right now `terminal_prefix_clause_candidates(...)` returns a `Vec<(&ClauseRec, Option<...>)>` even in the common claim hot path where no terminal clause filter applies.

That is still an allocation per prefix.

### Patch

Change it to something like:

```rust
enum TerminalClauseSource<'a> {
    Unfiltered(&'a [ClauseRec]),
    Filtered(Vec<FilteredTerminalClause<'a>>),
}
```

and iterate directly in the unfiltered case.

### B. Stop serializing full telescopes inside candidate sorting

`sort_terminal_prefix_group_candidates_for_certification(...)` currently uses full telescope JSON serialization as a tiebreak.

For terminal-prefix groups, all candidates share the same prefix and differ only in the last clause.

Use a cheaper tiebreak:

- cached last-clause sort key
- or `expr_sort_key(last_clause)` + clause role + stable JSON of the last clause only

### C. Cache `expr_bit_length` per clause option if profiling points there

`terminal_prefix_completion_bit_cost(...)` recomputes `expr_bit_length(&clause.expr)` repeatedly.

If telemetry says this is hot, make clause catalog entries carry precomputed bit lengths.

### Files/functions

- `crates/pen-search/src/engine.rs`
  - `terminal_prefix_clause_candidates(...)`
  - `sort_terminal_prefix_group_candidates_for_certification(...)`
  - `terminal_prefix_completion_bit_cost(...)`
- `crates/pen-search/src/enumerate.rs`
  - if you choose to enrich `ClauseCatalog`

### Priority

Medium. Do this after cached pre-prunes and split handoff unless the timing counters say allocations dominate.

---

## 7. Only after measuring: add a claim-safe generic terminal filter path

### Current issue

`filter_terminal_clauses(...)` bails out early when there is no family filter.

In claim mode, that means you lose even the cheap claim-safe terminal checks that do **not** depend on a named focus family.

### Narrow version

For `DesktopClaimShadow`, allow a generic path that still performs:

- trivial-derivability rejection
- any claim-open-band admissibility checks that do not require a family mask

### Caution

This may be a real win, but the upside is less certain than the earlier pruning changes. Do not make this the first patch.

### Files/functions

- `crates/pen-search/src/prefix_memo.rs`
  - `filter_terminal_clauses(...)`
  - `terminal_admissibility(...)`

### Good test

- `claim_terminal_filter_reuses_trivial_derivability_without_named_family_filter`

---

## 8. Only if telemetry says frontier drain is still expensive: replace the frontier `Vec` sort/pop loop

### Current issue

`pop_best_prefix(...)` sorts the whole `frontier` vector on every pop.

This is correct but may become expensive if the queue stays in the low-thousands for a long time.

### Patch

Move to one of:

- `BinaryHeap<Reverse<_>>`
- `BTreeSet<_>`
- or a vec kept sorted by binary insertion

### Guardrail

Do not do this unless the new timing counters show `frontier_sort_pop_millis` is a real share of step-`4` wall time.

---

## Concrete patch order I would actually try

### Slice A: fastest high-probability win

1. Add the new step-`4` timers/counters
2. Add claim cached-rank pre-prune before materialization
3. Drop cached completion payload after a claim pre-prune
4. Add bound-only summary accessor to stop cloning full payloads for cached bound checks

### Slice B: next throughput win

5. Replace all-or-none exact-two handoff with partial in-place processing
6. Re-run `until_step = 4` and inspect live NDJSON

### Slice C: bigger win if still stuck

7. Add claim discovery summary / delayed materialization
8. Remove per-prefix tuple vec allocation in unfiltered claim terminal loops
9. Use cheaper candidate sort keys

### Slice D: only if evidence points there

10. Claim-safe generic terminal filter path
11. Frontier priority queue rewrite
12. Incremental signature construction / fewer full telescope serializations

---

## Practical A/B workflow for the agent

### Add a temporary profiling config

Create a local-only config for quick step-`4` A/B work, for example:

- copy `configs/desktop_claim_shadow_1h.toml`
- set `until_step = 4`
- keep `search_profile = "desktop_claim_shadow"`
- keep release build
- keep live checkpoint persistence on

Do **not** weaken the claim lane itself. This is only for iteration speed.

### After each patch

Run one step-`4` release A/B and compare:

- wall-clock at `prefix_states_explored = 5`, `7`, `10`, `20`
- wall-clock at equal `generated_raw_surface`
- `frontier_queue_len`
- `prefix_cache_groups`
- `prefix_cache_candidates`
- `legality_cache_entries.terminal_prefix_completions`
- `legality_cache_entries.partial_prefix_bounds`
- new counters:
  - cached rank pre-prunes
  - cached bound hits
  - pre-materialize bar/rank prunes
  - materialized remaining-one prefixes

### A patch is worth keeping if it does at least one of these

- pushes the same checkpoint materially earlier
- lowers `remaining_one_materialized`
- raises cached pre-prunes materially
- flattens the frontier queue sooner
- makes later step-`4` checkpoints keep moving after the old plateau

---

## Suggested regression set

Run these after each change:

```bash
cargo test -p pen-search claim_
cargo test -p pen-search online_work_items_cache_exact_filtered_next_clauses
cargo test -p pen-search online_work_items_reuse_full_catalog_when_no_filter_applies
cargo test -p pen-search claim_materialization_consumes_cached_terminal_completion_summary
cargo test -p pen-search prefix_queue_prefers_nearer_terminal_and_tighter_cached_continuations
cargo test -p pen-search terminal_clause_filter_skips_inadmissible_last_clauses_before_connectivity
```

Add new targeted tests for any new fast path before relying on live runs.

Do **not** block on `cargo test -p pen-search --lib` being fully green right now; the progress doc already says that broader tree still stops at `engine::tests::demo_late_surface_carries_through_live_config_runs`.

---

## What I would bet on

If I had to bet on the next winning path, I would bet on this combination:

1. **claim cached-rank pre-prune before materialization**
2. **bound-only / rank-only cached-summary access**
3. **partial exact-two in-place processing instead of all-or-none**
4. **then, if needed, claim discovery summary with delayed materialization**

That combination attacks the exact failure story from `autonomous_progress.md` directly:

- too much time in repeated late terminal completion
- too many already-dominated surfaces
- too much queue churn before the incumbent tightens

---

## Stop condition for this slice

Do not move on to compare/benchmark/certify until one stored release run shows that step `4` is no longer the dominant blocker.

A good intermediate target is:

- on the updated `until_step = 4` release A/B, you materially beat the old `codex-claim-release-full-v1a` pace
- queued `remaining_one` prefixes are now getting pruned cheaply
- retained-prefix plateau no longer coincides with heavy later materialization work

Once that happens, re-run the real `desktop_claim_shadow_1h` profile on the disclosed desktop and use the stored artifacts to decide whether the next blocker has finally moved to late steps instead of step `4`.
