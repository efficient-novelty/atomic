# Autonomous Claim Lane Plan

Last updated: 2026-03-22
Status: active

## Objective

Stabilize `desktop_claim_shadow` so the intended full-profile claim run can
finish on the disclosed desktop configuration, preserve usable artifacts when
it does not finish, and then earn a passing certification bundle from stored
evidence.

Until that certification gate passes, keep the paper wording in the safer
`bounded live recovery` form.

## Current Baseline

These are now baseline truths, not forward work:

- `desktop_claim_shadow` is a distinct claim-lane profile/config family
- claim admissibility is structurally guided:
  - `guidance_style = claim_debt_guided`
- claim late expansion is claim-specific:
  - `late_expansion_policy = claim_generic`
- claim bucket scheduling is claim-specific:
  - `bucket_policy = structural_generic`
- claim-path `kappa 4-9` mutator packs and exactness rechecks are landed in
  code/tests
- `scripts/compare_runs.py` and `scripts/certify_claim_lane.py` exist and now
  audit claim-policy honesty, fallback evidence, reason/prune completeness,
  breadth gates, runtime threshold, and manifest completeness
- `run.json` now records CPU, worker-count, build-profile, target, git,
  `Cargo.lock`, and binary fingerprints needed for certification
- `pen-cli run` and `pen-cli resume` now persist `run.json`, step summaries,
  step checkpoints, frontier snapshots, claim narratives/events, and failure
  status incrementally so failed long runs remain auditable
- claim auto-worker resolution is now memory-aware for
  `desktop_claim_shadow`
- claim step artifacts now persist observed process RSS alongside the
  governor-accounted RSS model
- claim proof-close now drops cached evaluated terminal-prefix payloads after
  ranking so the live claim cache stays smaller
- claim proof-close now also releases processed retained prefix groups once
  certification starts, so the live claim cache can shrink during proof-close
  instead of holding already-certified groups until the end of the step
- claim terminal-prefix materialization now consumes cached exact completion
  summaries from the legality cache after reuse, so claim runs stop holding
  both copies of the same exact terminal surface
- claim prefix signatures now share their serialized exact payload across
  cloned frontier and legality-cache keys, so cloned hot-path signatures no
  longer copy the same full prefix string into every map entry

## Active Blocker

The main blocker is no longer claim-specific policy plumbing. The main blocker
is that the intended full-profile claim run is not yet operationally stable on
the disclosed machine:

- `desktop_claim_shadow_1h` with `workers = "auto"` still aborts before
  step-15 completion with `memory allocation of 1212416 bytes failed`
- failed runs now leave an auditable bundle, but there is still no stored
  full-profile step-15 claim run on the disclosed machine
- the 2026-03-22 `codex-claim-shared-signature-v1` smoke rerun showed that
  sharing cloned prefix-signature payloads only trimmed the comparable early
  step-4 observed RSS checkpoint by about `6.6 MiB`, so the dominant remaining
  spike is still in discovery/frontier/legality growth beyond duplicated
  signature-key storage
- without a stored bundle, parity, breadth, fallback honesty, and runtime
  certification cannot advance from code/tests to real evidence

## Success Condition

This plan is complete only when a stored `desktop_claim_shadow` evidence bundle
shows all of the following at the same time:

- the intended full-profile claim run completes without allocator abort on the
  disclosed machine
- failed or interrupted claim runs still leave enough manifest and step data to
  audit what happened
- accepted hashes match the guarded reference through step `15`
- the claim path has no silent guarded, replay, realistic-shadow, or demo-only
  fallback
- early breadth gates pass from stored evidence
- late floors pass from stored evidence without inflating
  `full_telescopes_evaluated` beyond a moderate range
- exact-screen reasons and prune classes are complete in stored artifacts
- benchmark and manifest data are complete enough for an appendix bundle
- the certification script emits a passing claim certificate

Only after those conditions pass should the paper wording move beyond
`bounded live recovery`.

## Execution Priorities

1. Preserve failure evidence instead of losing the whole run.
2. Make claim-run memory use predictable and bounded.
3. Re-earn stored parity and breadth on the stabilized claim lane.
4. Freeze benchmarking and certification on the intended profile.

## 1. Stabilize Memory And Worker Use

Why this is second:

- the intended claim profile now fails before certification can even begin
- the next job is to make full-profile claim execution finish, not to keep
  widening the claim lane blindly

Files:

- `crates/pen-cli/src/cmd_run.rs`
- `crates/pen-search/src/engine.rs`
- `crates/pen-search/src/diversify.rs`
- `crates/pen-search/src/frontier.rs`
- `crates/pen-search/src/scheduler.rs`
- `crates/pen-store/src/memory.rs`
- `crates/pen-store/src/spill.rs`
- `configs/desktop_claim_shadow_*.toml`

Concrete tasks:

- capture and review the newly stored observed-versus-accounted RSS gap from a
  full claim run on the disclosed machine
- inspect the new `step_live_checkpoint` telemetry and
  `reports/steps/step-XX-live.ndjson` artifacts for steps 4-5 so partial claim
  bundles can reveal which in-memory structures are growing before acceptance
- verify that the new memory-aware auto-worker cap is sufficient for the
  intended claim profile instead of merely changing the modeled worker count
- reduce or cap any remaining worker scratch, resident cold frontier, and
  checkpoint/spill buffers when the claim lane would otherwise exceed the
  disclosed desktop envelope
- spill and compact earlier again if the new claim-only controls still wait too
  long before allocator pressure turns into failure
- confirm whether any remaining allocator abort comes from tracked frontier
  structures or from another untracked claim-path allocation spike, then close
  the specific gap

Done when:

- the intended `desktop_claim_shadow_1h` profile finishes without allocator
  abort on the disclosed machine
- the claim lane no longer depends on luck in auto-worker selection
- governor/pressure evidence explains the observed memory behavior honestly

## 2. Re-Earn Stored Claim Evidence

Why this is third:

- once the run completes reliably, the real open question returns to stored
  parity, breadth, and fallback honesty
- those claims must be earned from artifacts on the stabilized lane itself

Files:

- `crates/pen-search/src/enumerate.rs`
- `crates/pen-search/src/engine.rs`
- `crates/pen-cli/src/report.rs`
- `crates/pen-cli/src/narrative.rs`
- `configs/desktop_claim_shadow_*.toml`

Concrete tasks:

- preserve accepted-hash parity through step `15` on the stabilized claim lane
- restore honest early raw-surface accounting on steps `1-4`
- close the late generated floors on steps `10-15`
- keep `full_telescopes_evaluated` within a certified moderate threshold
- persist any remaining claim-specific exact-screen or prune distinctions needed
  to explain the live lane honestly
- ensure claim artifacts make fallback impossible to miss

Breadth gates to earn:

- step `1` generated raw `= 2144`
- step `10` generated `>= 500`
- step `11` generated `>= 800`
- step `12` generated `>= 1200`
- step `13` generated `>= 2200`
- step `14` generated `>= 3500`
- step `15` generated `>= 5000`

Done when:

- a stored claim bundle shows parity through step `15`
- stored breadth gates pass honestly on the claim lane itself
- reporting is explicit enough for compare/certification to judge the lane
  without caveats

## 3. Benchmarking And Certification

Why this is last:

- the strong sentence should be tied to one stable stored bundle, not to code
  changes in isolation

Files:

- `scripts/compare_runs.py`
- `scripts/certify_claim_lane.py`
- `scripts/benchmark_claim_lane.py`
- repo-level autonomy docs

Concrete tasks:

- produce one canonical guarded-vs-claim compare report for the stabilized run
- run the landed `scripts/benchmark_claim_lane.py` harness on the intended
  claim config bundle
- record benchmark timing and success/floor-hit counts
- freeze the certified runtime threshold from real claim evidence
- emit and store a passing `claim_certificate.json`
- link the canonical bundle from repo-level autonomy docs

Required outputs:

- canonical claim run directory
- compare report against guarded
- benchmark bundle
- passing `claim_certificate.json`

Done when:

- one command sequence produces an appendix-ready claim bundle on the disclosed
  machine
- the certification script passes on that intended configuration
- the paper wording is tied to the stored certificate instead of free prose

## Working Rules For Remaining Claim-Lane PRs

- preserve existing behavior for `strict_canon_guarded`,
  `realistic_frontier_shadow`, and `demo_breadth_shadow`
- treat completed policy-split work as baseline, not as an excuse to keep
  editing already-landed surfaces
- prefer narrow fixes that explain or bound memory behavior over broad
  speculative rewrites
- do not lower floors or hide misses to make the claim look cleaner
- do not use stronger language such as `unguided` before certification passes

## Immediate Next Step

Use the new memory controls and stored RSS-gap evidence to attack the remaining
live blocker:

1. rerun `desktop_claim_shadow_1h` on the disclosed desktop and inspect the
   stored observed-versus-accounted RSS gap, now including the latest
   legality-cache compaction plus processed-prefix-group release change, plus
   shared cloned prefix-signature payloads
2. use the step-4/5 live-checkpoint stream to separate early discovery growth
   from proof-close retention; the 2026-03-22 smoke rerun already showed
   step `4` at about `3.30 GiB` observed RSS after `14.9s` with `2775`
   frontier groups, `5550` legality summaries, `5084` partial-prefix-bound
   entries, and `0` retained prefix-cache groups, and the follow-up
   `codex-claim-shared-signature-v1` rerun only trimmed the comparable early
   checkpoint by about `6.6 MiB`, so that early spike is still
   discovery/frontier/cache-side rather than proof-close retention or cloned
   signature payload duplication at that point
3. compare that run against guarded from stored artifacts and use that bundle
   to drive the remaining parity, breadth, benchmark, and
   certification fixes
