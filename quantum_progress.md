# Quantum Progress

Last updated: 2026-03-16

This file is the forward-facing status snapshot for
[`quantum_improvement_plan.md`](./quantum_improvement_plan.md). It should
answer two questions:

- where the remaining late-step cost still is
- what the next pass should try to remove

## Current Position

- `strict_canon_guarded` remains the authoritative executable lane
- guarded and realistic shadow still preserve the accepted 15-step sequence
- the current realistic engine already has enough structure to build on:
  online prefix expansion, exact signature-keyed memoization, exact small-tree
  and terminal-prefix pruning, exact late terminal-rank pruning, and
  persisted timing/memory telemetry

## What The Latest Evidence Says

Use `runs/codex-terminal-bound-reuse` as the current reference artifact set.

Late-step facts that matter for forward work:

- step 10 already has one exact queue-entry prune:
  `incremental_partial_prefix_bound_prunes = 1`
- step 11 shows current partial-prefix reuse is real but still narrow:
  `incremental_partial_prefix_bound_hits = 1`,
  `incremental_terminal_prefix_bar_prunes = 1`, and
  `full_telescopes_evaluated = 1`
- steps 13 to 15 now show exact late terminal-rank pruning after retained
  terminal-prefix materialization:
  `incremental_terminal_rank_prunes = 1` and
  `full_telescopes_evaluated = 1` on each of steps 13, 14, and 15
- steps 13 and 15 now also reuse the exact one-clause-short bar decision on
  the later enqueue path instead of replaying the same terminal-summary lookup:
  `incremental_partial_prefix_bound_hits = 1` and
  `incremental_terminal_prefix_completion_hits = 2`
- steps 13 and 15 now also shrink the late retained prefix surface:
  `prefix_states_explored = 2` and
  `prefix_frontier_hot_states = 1`
- step 15 still shows terminal reuse and connectivity reuse are working:
  `incremental_terminal_prefix_completion_hits = 2`,
  `incremental_connectivity_shortcuts = 2`, and
  `incremental_connectivity_fallbacks = 0`
- the new terminal-prefix partial-bound reuse removes one exact summary replay,
  but the surviving late root-to-terminal lane still does the same number of
  exact bar checks:
  step 15 still has `incremental_partial_prefix_bound_checks = 3` and
  `prefix_states_explored = 2`
- current late-step wall-clock is already low in absolute terms, so the next
  pass should keep optimizing exact work counts first and timing second

## Active Gaps

### 1. Exact Partial-Prefix Bounds Still Fire After Too Much Prefix Work

The eager exact terminal-rank prune now removes the dominated late terminal
prefix before it enters the retained frontier, and the terminal-prefix
partial-bound reuse now removes one exact summary replay. But the surviving
root and winning terminal prefix still require the same exact bar-clearability
check surface.

Next target:

- find a sound exact bound that fires before terminal-prefix completion
  materialization on steps 13 to 15 without guessed `nu_upper` logic

Evidence to improve against:

- steps 13 and 15 still explore 2 prefix states
- step 15 still does `incremental_partial_prefix_bound_checks = 3`
- the late winning lane still needs a retained terminal-prefix summary hit:
  step 15 still does `incremental_terminal_prefix_completion_hits = 2`

### 2. Non-Family Admissibility Reuse Still Arrives Too Late

Family-shaped filtering, terminal admissibility, trivial-derivability, and
historical-reanchor reuse now clear obvious cases. The new terminal-prefix
partial-bound reuse removes one exact late replay, but the eager late
terminal-rank prune still fires only after exact terminal completion summaries
already exist for multiple plausible continuations.

Next target:

- add another exact `PrefixSignature`-keyed admissibility or dominance summary
  that rejects late continuations before full terminal completion summaries are
  built

Evidence to improve against:

- step 15 still needs terminal completion reuse hits to finish cleanly:
  `incremental_terminal_prefix_completion_hits = 2`
- steps 13 and 15 still build exact terminal completion summaries for multiple
  plausible late continuations before the new rank prune fires

### 3. Search Order Has Not Yet Been Retuned From The New Evidence

The continuation-aware queue order is only a first pass. It has not yet been
retuned using the latest timing, memory, and prune counters after the newer
memoization layers landed. The new terminal-rank prune also makes it clearer
which late prefixes are worth evaluating first.

Next target:

- use current artifacts to explain which late prefixes are still expensive
  relative to payoff, then retune deterministic order

## Immediate Next Steps

1. Audit the remaining late-step prefixes in the latest realistic artifacts,
   especially the step-13 and step-15 root-to-terminal surfaces that still
   explore 2 prefix states and spend 3 exact bound checks after the new
   terminal-prefix partial-bound reuse removed one summary replay.
2. Choose one of:
   - a broader exact partial-prefix bound
   - a broader non-family admissibility summary
3. Land the smallest exact change that removes another real late-step prefix
   state or exact terminal-summary replay, not just a final dominated
   telescope evaluation.
4. Re-run realistic shadow through step 15 and compare:
   - `prefix_states_explored`
   - `full_telescopes_evaluated`
   - `incremental_terminal_rank_prunes`
   - relevant cache/prune counters
   - timing/memory telemetry
5. Only then retune ordering.

## Things Not Worth Re-Planning

- adding a prefix frontier
- re-landing the current signature/memo/filter infrastructure
- changing guarded acceptance semantics
- speculative interference or grammar work before the bound/admissibility gaps
  move

## Guardrails

- no floats on the hot path
- no stochastic acceptance in guarded mode
- no semantic labels in core search/eval
- no accelerator path that bypasses exact check/eval/minimality/acceptance
- no over-claim that realistic shadow is already an open-ended online frontier
  explorer

## Verification Baseline

- `cargo test -p pen-type -p pen-search -p pen-cli -p pen-store`
- `cargo run -p pen-cli -- run --config configs/strict_canon_guarded.toml --root runs --run-id codex-eager-terminal-rank-guarded --until-step 15`
- `cargo run -p pen-cli -- run --config configs/realistic_frontier_shadow.toml --root runs --run-id codex-eager-terminal-rank --until-step 15`
