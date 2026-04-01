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

- The follow-on survivor-sketch bookkeeping slice remains landed on top of the
  broadened incumbent-relevant sketch coverage, the tiered `lib_refs` work,
  the explicit no-miss plateau-kernel split, and the mandatory
  `TerminalClauseNuFacts` winner.
  Compact claim summaries still keep a survivor sketch for competition-allowed
  bar-clearers that can still beat the current incumbent even when multiple
  primary ranks are live, and still reuse it during materialization without
  waking the dormant general cached-summary reopen path, while the hot compact
  summary path still threads borrowed primary-rank refs through best-rank
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
- Two later local-only compact-summary bookkeeping slices then also stayed
  parity-clean in the same claim-focused tests and replay parity, but both
  landed worse and were dropped:
  - a shared compact sketch/best-primary bookkeeping fold across the compact
    summary paths
  - a claim-open-band compact local-state hoist for
    `bound / admitted count / best primary / best rank / sketch` bookkeeping
- The shared compact-bookkeeping fold rereads landed `140129 us`,
  `140565 us`, and `145083 us` total:
  - `140129`: `27648 / 50765 / 20329 / 20751 / 20636`
  - `140565`: `26793 / 58976 / 18071 / 18207 / 18518`
  - `145083`: `31456 / 56870 / 19093 / 18553 / 19111`
- The claim-open-band compact local-state hoist rereads landed `129362 us`,
  `133001 us`, and `164098 us` total:
  - `129362`: `26190 / 45671 / 18214 / 18308 / 20979`
  - `133001`: `24988 / 49889 / 19082 / 18876 / 20166`
  - `164098`: `47503 / 54272 / 21662 / 19399 / 21262`
  These are `24 / 74 / 140 / 332 / 335` in order for each total above.
- Neither later slice re-earned the checked-in `123544 us` total, so both
  code paths were reverted, the benchmark artifact stayed unchanged, and no
  new intended-profile rerun was launched.
- A later narrow slice then cached terminal-clause bit cost inside
  `TerminalClauseNuFacts`, backfilled older replay fixtures on load, and
  reused that sidecar across the remaining-one plateau-kernel and compact
  admitted bit-`kappa` bookkeeping paths instead of recursively walking the
  same clause expr on every admitted candidate.
- Claim-focused validation stayed green:
  - `cargo test -p pen-eval terminal_clause_nu_facts_backfill_missing_bit_cost`
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-search cached_terminal_prefix_rank_summary_prunes_without_reopening_completion_summary`
  - `cargo test -p pen-search take_terminal_prefix_completion_summary_removes_cached_payload_after_reuse`
- The release replay harness stayed parity-clean on the stored plateau
  fixtures while the slice was in place.
- The immediate pre-slice local release reread was `134660 us` total:
  - `24`: `30164`
  - `74`: `48407`
  - `140`: `18293`
  - `332`: `18854`
  - `335`: `18942`
- Warm rereads with the bit-cost sidecar reuse landed `133228 us`,
  `124028 us`, `122830 us`, and `122493 us` total:
  - `133228`: `26828 / 49463 / 18923 / 18241 / 19773`
  - `124028`: `24874 / 45415 / 17527 / 18007 / 18205`
  - `122830`: `26271 / 43593 / 17247 / 17245 / 18474`
  - `122493`: `25108 / 43553 / 18100 / 17228 / 18504`
  These are `24 / 74 / 140 / 332 / 335` in order.
- The refreshed stored benchmark artifact was then rewritten from a later
  under-gate reread at `123148 us` total:
  - `123148`: `25589 / 43365 / 18070 / 17325 / 18799`
- Additional spot-check rereads still bounced around the gate at `124012 us`
  and `126456 us`, so the win is narrow and noisy, but the slice re-earned the
  checked-in `123544 us` total more than once and also beat the immediate
  pre-slice local release reread honestly.
- The plateau-kernel split therefore remains the stored 20-minute run to beat,
  but the lane has now earned another capped intended-profile rerun.

## Do This Next

### 1. Spend The Replay Win Now

Do not reopen another speculative code slice first.
The next move is one capped intended-profile rerun on top of the newly landed
tiered-`lib_refs`, plateau-kernel, broadened survivor-sketch, borrowed-
primary-rank, and clause-bit-cost sidecar work while keeping
`prefix-local-score-v1` as the long-run reference.

### 2. Run The Next Validation Slice In This Order

1. Launch the intended `desktop_claim_shadow` profile on the current binary.
2. Stop the rerun after `20` minutes max.
3. Record the nearest stored step-`4` checkpoint to `1200000 ms`.
4. Compare that checkpoint first against the current short-loop target from
   `plateau-kernel-split-v1`, then keep `prefix-local-score-v1` as the long-
   run continuation reference behind it.
5. If the rerun does not beat the current short-loop target honestly, return
   to code work with another narrow per-admitted compact-summary cost slice
   inside `compute_terminal_prefix_completion_summary_from_candidates(...)`,
   but do not retry the dropped focus-aligned
   competition-gate/payload-mode hoist, the dropped shared
   compact-bookkeeping fold, or the dropped claim-open-band compact local-
   state hoist first.
6. Do not wake the dormant general cached-summary reopen machinery first.

### 3. Keep Rule For The Next Rerun

Treat the next rerun as real progress only if the stored `20`-minute
checkpoint beats the current short-loop target honestly.

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

- another long intended-profile continuation beyond the `20`-minute cap
- another fresh code slice before spending the newly re-earned capped rerun
- cached-summary reopen wake-up work
- the dropped focus-aligned competition-gate/payload-mode hoist
- the dropped shared compact-bookkeeping fold
- the dropped claim-open-band compact local-state hoist
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
