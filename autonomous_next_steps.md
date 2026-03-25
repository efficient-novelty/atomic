# Autonomous Claim Lane: Next Operational Slice

This note is the exact work order for the next `desktop_claim_shadow` slice.
Assume delayed materialization, the incumbent-primary remaining-one fast path,
the one-pass `structural_nu` summary-build fast path, and the algebraic `nu`
ceiling patch are already landed.
The current short step-`4` baseline is
`runs/codex-claim-release-step4-algebraic-v1`.
The current full-profile baseline remains
`runs/codex-claim-release-full-nu-profile-v1`.

## Goal

Cut repeated remaining-one exact-summary work in step `4` by collapsing
structurally equivalent typing-context states before
`compute_terminal_prefix_completion_summary_from_candidates` runs.

## Why This Is The Next Slice

- The short rerun on the algebraic binary improved the narrow
  `until_step = 4` profile, so that patch can stay landed.
- The intended-profile rerun on that same binary did not reproduce the gain at
  the matching `43/52/59` checkpoints.
- `remaining_one_algebraic_prunes` stayed `0` through
  `prefix_states_explored = 105` at `4271.3s`, so the new ceiling is
  effectively inert on the real profile.
- The retained prefix-cache shape stayed flat at the matching checkpoints while
  legality summaries and exact summary time kept growing.
- The wall is still compute in step `4`, not allocator or RSS pressure.
- The next honest win is therefore quotienting repeated remaining-one states,
  not reopening memory compaction or another blind timing-only rerun.

## Current Comparison Targets

Use these as the numbers to beat:

- `runs/codex-claim-release-step4-algebraic-v1`
  - `prefix_states_explored = 43` at `1629.3s`
  - `prefix_states_explored = 52` at `1975.0s`
  - `prefix_states_explored = 59` at `2252.6s`
  - `terminal_summary_build_millis = 2111246` at the `59`-state checkpoint
  - retained prefix cache stayed `39 groups / 144845 candidates`
  - legality summaries stayed `205199`, `246986`, and `279487`
- `runs/codex-claim-release-full-nu-profile-v1`
  - `prefix_states_explored = 43` at `1629.6s`
  - `prefix_states_explored = 52` at `2039.7s`
  - `prefix_states_explored = 59` at `2367.7s`
  - frontier queue length `= 2716`
  - legality summaries `= 279487`
  - retained prefix cache `= 39 groups / 144845 candidates`
  - `remaining_one_rank_prunes_pre_materialize = 273957`
  - `terminal_summary_build_millis = 2221499`
  - `terminal_materialize_millis = 460`
  - observed RSS `~ 316 MiB`
- `runs/codex-claim-release-full-algebraic-v1`
  - `prefix_states_explored = 43` at `1703.4s`
  - `prefix_states_explored = 52` at `2081.4s`
  - `prefix_states_explored = 59` at `2369.7s`
  - `remaining_one_algebraic_prunes = 0` at the matching checkpoints and at
    the manual stop
  - manual stop at `prefix_states_explored = 105` / `4271.3s`
  - frontier queue length `= 2670`
  - legality summaries `= 493065`
  - retained prefix cache `= 40 groups / 147639 candidates`
  - `terminal_summary_build_millis = 4027549`
  - observed RSS `~ 451 MiB`

## Do This Next

### 1. Patch One Narrow Remaining-One Context-Equivalence Quotient

Preferred form:

- key the quotient by normalized exported type context plus the active
  structural obligation surface, not by exact AST prefix history
- reuse exact summaries only when the quotient key preserves admissibility,
  retained-prefix honesty, and winner determinism
- keep queue order deterministic within quotient buckets
- keep the write surface narrow to the remaining-one exact-summary path first

Reject as first moves:

- memory compaction
- frontier queue rewrites
- more algebraic ceiling variants
- certification or benchmark work
- widening-band retuning

### 2. Add Telemetry And Tests For The Quotient Surface

Before trusting live runs, make the new slice measurable.

Preferred telemetry additions:

- `remaining_one_context_equivalence_hits`
- `remaining_one_context_equivalence_misses`
- `remaining_one_context_equivalence_reused_summaries`

Required tests:

- one targeted test that two distinct remaining-one prefixes with the same
  normalized exported type context reuse the same exact summary
- one targeted test that inequivalent contexts do not collapse
- one regression that proves the kept reference winner still survives

### 3. Re-Earn One Stored Release `until_step = 4` Rerun

Run the same claim profile shape derived from
`configs/desktop_claim_shadow_1h.toml` with:

- `until_step = 4`
- release build
- live checkpoint persistence left on

Read the stored artifacts, not terminal output.

### 4. Decide Keep Or Drop From Stored Telemetry

Keep the quotient only if the new `reports/steps/step-04-live.ndjson` shows at
least one of these against the current baselines:

- lower `terminal_summary_build_millis` at matching checkpoints
- fewer legality summaries at matching retained-prefix shape
- materially more context-equivalence reuse hits
- the same wall clock reaches materially farther
- faster frontier drainage without weakening retained prefix-cache shape

Drop the patch if the rerun does not show one of those.

### 5. Only Then Rerun The Real Profile

Only after the short step-`4` rerun earns keep:

- rerun `desktop_claim_shadow_1h`
- inspect the stored full-profile artifacts
- decide whether step `4` still blocks or whether a later phase becomes the
  honest next wall

## What To Read After The Rerun

Open `reports/steps/step-04-live.ndjson` and answer:

- did `terminal_summary_build_millis` fall at matching checkpoints?
- did legality summaries grow more slowly at the same retained-prefix shape?
- did the context-equivalence counters fire materially?
- did the same wall clock reach farther than the current baseline?
- did frontier queue length drop faster?
- did retained prefix-cache shape stay honest?
- did the hot cost move somewhere new?

## Regression Set

Run these after each search-code change:

```bash
cargo test -p pen-search claim_
cargo test -p pen-search online_work_items_
cargo test -p pen-search prefix_queue_prefers_nearer_terminal_and_tighter_cached_continuations
cargo test -p pen-search terminal_clause_filter_skips_inadmissible_last_clauses_before_connectivity
```

Add the new targeted quotient tests before trusting live runs.

Do not block on `cargo test -p pen-search --lib` being fully green right now;
the broader tree still stops at
`engine::tests::demo_late_surface_carries_through_live_config_runs`.

## Stop Condition For This Note

Rewrite this file as soon as one new stored rerun shows one of these is true:

- exact summary build is no longer the next honest win
- step `4` is no longer the dominant blocker
- the context-equivalence slice is exhausted and the next real move is queue
  or incumbent work
