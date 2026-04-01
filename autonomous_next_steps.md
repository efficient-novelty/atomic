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

- The follow-on survivor-sketch bookkeeping slice is now landed on top of the
  broadened incumbent-relevant sketch coverage, the tiered `lib_refs` work,
  the explicit no-miss plateau-kernel split, and the mandatory
  `TerminalClauseNuFacts` winner.
  Compact claim summaries still keep a survivor sketch for competition-allowed
  bar-clearers that can still beat the current incumbent even when multiple
  primary ranks are live, and still reuse it during materialization without
  waking the dormant general cached-summary reopen path, but the hot compact
  summary path now threads borrowed primary-rank refs through best-rank
  tracking and sketch append instead of cloning the same primary-rank payload
  twice per bar-clearer.
- Claim-focused validation stayed green:
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-search cached_terminal_prefix_rank_summary_prunes_without_reopening_completion_summary`
  - `cargo test -p pen-search take_terminal_prefix_completion_summary_removes_cached_payload_after_reuse`
- The release replay harness stayed parity-clean on the stored plateau
  fixtures.
- A follow-on local-only attempt then hoisted the focus-aligned competition
  gate plus the compact/full payload-mode branch once per
  `compute_terminal_prefix_completion_summary_from_candidates(...)` call, but
  that code was dropped after replay validation.
- The immediate pre-slice local reread was `130405 us` total:
  - `24`: `28907`
  - `74`: `47093`
  - `140`: `17896`
  - `332`: `18018`
  - `335`: `18491`
- Warm rereads with the hoist landed worse at `136040 us`, `137054 us`, and
  `140843 us` total:
  - `136040`: `27713 / 50699 / 18734 / 19345 / 19549`
  - `137054`: `25619 / 50318 / 22239 / 19997 / 18881`
  - `140843`: `28737 / 49206 / 20016 / 20591 / 22293`
  These are `24 / 74 / 140 / 332 / 335` in order.
- The dropped hoist therefore did not re-earn the checked-in `123544 us`
  total and did not beat the immediate pre-slice local reread honestly, so the
  benchmark artifact stayed unchanged and no new intended-profile rerun was
  launched.
- The plateau-kernel split remains the only honest short-loop win so far.

## Do This Next

### 1. Reopen Code Work Now

Do not run another intended-profile continuation first.
The next move is a follow-on code slice on top of the newly landed tiered-
`lib_refs`, plateau-kernel, and broadened survivor-sketch work while keeping
`prefix-local-score-v1` as the long-run reference.

### 2. Implement The Next Slice In This Order

1. Keep the new multi-primary/incumbent-relevant survivor-sketch coverage plus
   the new borrowed-primary-rank bookkeeping reuse, but reduce a different
   remaining compact-summary per-admitted cost.
   Stay inside `compute_terminal_prefix_completion_summary_from_candidates(...)`
   and collapse the open-band compact summary bookkeeping further so replay can
   at least re-earn the checked-in `123544 us` total or otherwise show fewer
   exact-`nu` evaluations on the stored plateau fixtures, but do not retry the
   dropped focus-aligned competition-gate/payload-mode hoist first.
   Do not wake the dormant general cached-summary reopen machinery first.

### 3. Validation Loop For Each New Slice

For the next attempts, validate in this order:

1. Rerun only the claim-focused tests needed by the changed code.
2. Refresh the replay-harness read in release mode.
3. Confirm replay parity plus either fewer exact-`nu` evaluations or lower
   measured replay time on the stored plateau fixtures.
   Do not keep a slice on one noisy reread alone; require it to beat the
   immediate pre-slice local replay read honestly as well.
4. Only launch a new intended-profile rerun after that replay gate improves,
   then stop it after `20` minutes max.
5. Compare the next local slice first to the checked-in replay gate, then
   compare any resulting rerun against the current 20-minute target from
   `plateau-kernel-split-v1`.

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
- the dropped focus-aligned competition-gate/payload-mode hoist
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
