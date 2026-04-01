# Autonomous Claim Lane: Next Operational Slice

Last updated: 2026-04-01

This note is the exact next work order for `desktop_claim_shadow`.

## Keep Fixed

- Keep the current short baseline:
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- Keep the current long-run reference to beat:
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`
- Keep the current best short-loop checkpoint to beat:
  `runs/codex-claim-release-full-aggregation-open-band-plateau-kernel-split-v1`
- Keep the replay harness corpus and benchmark files:
  - `tests/fixtures/claim_runtime/remaining_one_plateau_fixtures.json`
  - `tests/fixtures/claim_runtime/remaining_one_plateau_benchmark.json`
- Keep the current 20-minute intended-profile validation target from the stored
  step-`4` stream of `plateau-kernel-split-v1`:
  - `elapsed_millis = 1191562`
  - `prefix_states_explored = 124`
  - `prefix_cache_groups = 40`
  - `prefix_cache_candidates = 109690`
  - `frontier_queue_len = 2651`
  - RSS `= 495325184` bytes
  - `terminal_summary_build_millis = 1183359`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`

## Current Read

- The explicit no-miss plateau-kernel split is now landed on top of the
  mandatory-`TerminalClauseNuFacts` winner.
  Remaining-one exact bound screening, compact summary build, and compact
  materialization now route `KeepWithoutFallback` candidates through one shared
  plateau kernel and reserve the general fallback kernel for the candidates
  that really need extra connectivity or admissibility work.
- The release replay harness stayed parity-clean on the stored plateau
  fixtures.
- The checked-in five-surface replay benchmark improved overall from
  `147912 us` to `145248 us`.
  It improved surfaces `74`, `332`, and `335`, but regressed surfaces `24` and
  `140`, so the slice is a real local win overall, not a uniform win at every
  stored surface.
- The capped intended-profile contender
  `runs/codex-claim-release-full-aggregation-open-band-plateau-kernel-split-v1`
  was manually stopped during step `4` at the `20` minute cap.
  The authoritative stored read is
  `reports/steps/step-04-live.ndjson`.
- Its nearest stored checkpoint to `20` minutes was:
  - `elapsed_millis = 1191562`
  - `prefix_states_explored = 124`
  - `prefix_cache_groups = 40`
  - `prefix_cache_candidates = 109690`
  - `frontier_queue_len = 2651`
  - `terminal_summary_build_millis = 1183359`
  - RSS `= 495325184` bytes
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`
- That contender kept the same retained-prefix surface and beat the old
  `prefix-local-score-v1` 20-minute gate honestly because it explored
  `124` prefixes instead of `123` while keeping fallback connectivity and
  admissibility checks at zero.
- The next loop is still a short intended-profile optimization loop with a
  hard `20` minute cap.
  One short-loop win is not enough yet to reopen a longer continuation read.

## Do This Next

### 1. Reopen Code Work Now

Do not run another uncapped intended-profile continuation first.
The next move is the next code slice on top of the newly landed plateau-kernel
split while keeping `prefix-local-score-v1` as the long-run reference.

### 2. Implement The Next Slice In This Order

1. Compress `lib_refs` inside
   `SingleClauseStructuralNuContext`.
   Preferred order:
   - inline small array for the common case
   - dense bitset after a threshold
   - sorted boxed slices only where serialization or debug parity needs them

2. If second-pass duplication still matters after that slice, prefer a tiny
   survivor sketch.
   Carry only the clause refs and facts needed for the best primary rank and
   tie-break-relevant survivors.
   Do not wake the dormant general cached-summary reopen machinery first.

### 3. Validation Loop For Each New Slice

For the next attempts, validate in this order:

1. Rerun only the claim-focused tests needed by the changed code.
2. Refresh the replay-harness read in release mode.
3. Confirm replay parity plus either fewer exact-`nu` evaluations or lower
   measured replay time on the stored plateau fixtures.
4. Launch a new intended-profile rerun and stop it after `20` minutes max.
5. Compare the nearest stored step-`4` checkpoint to the current 20-minute
   target from `plateau-kernel-split-v1`.

### 4. Short-Loop Beat Rule

Treat a new slice as a real short-loop win only if its 20-minute stored read
beats the current target honestly.

Minimum comparison fields:

- `prefix_states_explored`
- `prefix_cache_groups`
- `prefix_cache_candidates`
- `frontier_queue_len`
- `observed_process_rss_bytes`
- `terminal_summary_build_millis`
- `terminal_summary_admissibility_checks`
- `terminal_summary_fallback_connectivity_checks`

Interpretation rule:

- More explored prefixes by `20` minutes is the main signal.
- Keep the retained-prefix story honest.
- Do not accept a "win" that only comes from waking fallback connectivity or
  admissibility work on what should still be no-miss surfaces.
- Do not expect the next few slices to beat the full `1095`-prefix stop yet.
  The immediate goal is repeated honest improvement over the current
  `124`-prefix short-loop checkpoint.

### 5. When To Allow Another Long Read

Only allow a longer intended-profile continuation again after repeated
20-minute wins show that the lane has materially improved.

Until then:

- stop new intended-profile contenders at `20` minutes
- record their nearest stored 20-minute checkpoint
- keep `prefix-local-score-v1` as the long-run reference to beat

## Do Not Reopen First

- another long intended-profile rerun with no new code
- cached-summary reopen wake-up work
- contender-rank helper rewrites
- connectivity-first or cache-first rewrites
- deterministic batched parallel reduction
- broad metadata-only or bookkeeping-only cleanup
- another timing-only slice with no new runtime question

## Keep Or Branch Decision

- Stay on runtime work while the intended profile still stalls in step `4`.
- Keep `prefix-local-score-v1` as the long-run reference until a later slice
  beats the new `124`-prefix short-loop gate repeatedly and then eventually
  beats the older longer continuation honestly.
- Treat the stopped `1095`-prefix read as the current long-run continuation
  reference, but treat the `plateau-kernel-split-v1` 20-minute checkpoint as
  the next validation gate.
- Branch to parity, breadth, compare, benchmark, and certification work only
  after later runtime slices move the intended profile far enough that longer
  reads become worth reopening.
