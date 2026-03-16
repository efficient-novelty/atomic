# Quantum Improvement Plan

Last updated: 2026-03-16

This file is the working plan for the next realistic-shadow search passes.
It should stay operational: what we are trying to improve next, how we will
measure it, and what should stay deferred.

## Objective

Reduce redundant late-step work in `realistic_frontier_shadow` while
preserving the exact acceptance contract:

- same accepted 15-step sequence in guarded mode
- same accepted 15-step sequence in realistic shadow mode
- exact integer/rational comparisons on the hot path
- deterministic ordering and acceptance
- no semantic labels in core search/eval
- no bypass around exact check/eval/minimality/acceptance

## Working Baseline

Assume the following are already good enough to build on:

- online prefix expansion through clause-position catalogs is real
- prefix work is keyed by strengthened `PrefixSignature`
- realistic shadow already has exact signature-keyed legality/filter/connectivity
  reuse
- realistic shadow already has exact tiny-subtree and one-clause-short
  terminal-prefix bar pruning
- realistic shadow already persists timing and frontier-memory telemetry

Do not spend the next pass re-landing those.

## Main Problem

The remaining cost is concentrated in late-step continuation surfaces that are
too large for the current exact tiny-subtree bound but still structured enough
that full terminal work is wasteful.

The next pass should push exact pruning and exact admissibility reuse earlier on
those surfaces.

## Priority Order

### 1. Broader Exact Partial-Prefix Bounds

Question:
what exact bar-clearability result can be computed before terminal assembly
without inventing unsound `nu_upper` logic?

Work:

- audit late steps 11 to 15 and isolate prefixes that survive current family
  and terminal filters but later collapse to a small exact completion set
- derive an exact earlier bound from the existing scoring/checking pipeline or
  from exact completion summaries already available in the search path
- prune or deprioritize those prefixes before full terminal assembly when the
  bound cannot clear the bar

Done when:

- accepted outputs stay unchanged
- `full_telescopes_evaluated` drops on late realistic steps
- exact-prune counters rise beyond current small-tree and one-clause-short
  terminal-prefix cases

### 2. Broader Non-Family Admissibility Reuse

Question:
what exact summary can reject or admit late continuations before full telescope
assembly when family-shaped summaries are no longer the bottleneck?

Work:

- add exact `PrefixSignature`-keyed summaries for non-family admissibility
  structure
- target steps 11 to 15 where current family, trivial-derivability, and
  connectivity reuse already fire but repeated terminal work still remains
- expose cache-hit and prune counters in stored artifacts

Done when:

- repeated late admissibility work visibly drops in artifacts
- cache-hit counters rise on realistic steps 11 to 15
- accepted outputs stay unchanged

### 3. Search-Order Retune From Evidence

Question:
after stronger bounds and admissibility summaries land, which prefixes should
be explored first?

Work:

- use stored timing and frontier-memory telemetry together with the newer
  prune/hit counters
- retune deterministic prefix ordering only after the stronger bound and
  admissibility layers are stable
- keep order changes shadow-only unless they are obviously contract-preserving

Done when:

- accepted outputs stay unchanged
- late realistic steps do less work or retain better prefixes under the same
  limits

## Deferred Until 1 To 3 Are Stable

- deterministic continuation profile variants
- restricted interference / normalization sidecar
- declarative late-family grammar

## Metrics To Watch

Primary:

- `prefix_states_explored`
- `prefix_states_exact_pruned`
- `full_telescopes_evaluated`
- `incremental_partial_prefix_bound_hits`
- `incremental_partial_prefix_bound_checks`
- `incremental_partial_prefix_bound_prunes`
- `incremental_terminal_prefix_bar_prunes`
- `incremental_terminal_prefix_completion_hits`
- `incremental_terminal_admissibility_hits`
- `incremental_connectivity_shortcuts`
- per-step timing telemetry
- frontier memory high-water bytes

Interpretation:

- more exact-prune counters matter only if accepted outputs stay identical
- cache hits matter only if they replace real repeated late-step work
- timing wins matter most on steps 11 to 15, not in early trivial steps

## Next Pass Checklist

1. Inspect the latest realistic late-step artifacts and name the remaining
   expensive prefixes.
2. Land one exact earlier-pruning or exact admissibility-reuse improvement.
3. Add or update the counters needed to prove the payoff.
4. Re-run guarded and realistic verification through step 15.
5. Update `quantum_progress.md` with only the new forward-relevant facts.

## Non-Negotiables

- no floats on the hot path
- no stochastic acceptance in guarded mode
- no semantic labels in core search/eval
- no accelerator path that bypasses exact check/eval/minimality/acceptance
- no regression of the guarded 15-step canon
- no claim that realistic shadow is already an open-ended anti-junk explorer
