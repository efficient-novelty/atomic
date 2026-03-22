# Current Claim Lane

Read this file when the task touches `desktop_claim_shadow`, claim-lane
telemetry, claim-lane narratives, or the autonomy-certification roadmap.

## Current Truth

- `desktop_claim_shadow` exists as a separate profile and config family.
- The lane is not yet certified and should still be described with the safer
  `bounded live recovery` wording.
- Claim admissibility now uses structural claim debt and anchor hints, without
  named-family focus progression.
- Claim late expansion now uses a claim-specific late surface with structural
  mutators landed for kappa `4-9`, while stored breadth/floor evidence on that
  widened lane is still open.
- Claim bucket scheduling now uses a structural-generic taxonomy derived from
  prefix-local syntax and runtime evidence.
- The current mixed state is recorded explicitly in run metadata:
  - `guidance_style = claim_debt_guided`
  - `late_expansion_policy = claim_generic`
  - `bucket_policy = structural_generic`
- `run.json` now captures CPU, worker-count, build-profile, target, git,
  `Cargo.lock`, and binary fingerprints for claim certification, and the smoke
  certification path now passes the manifest-completeness gate.
- `pen-cli run` and `pen-cli resume` now write `run.json`, step summaries,
  step checkpoints, frontier snapshots, claim narratives/events, and failure
  status incrementally, so failed long claim runs remain auditable from disk.
- claim runs now record observed process RSS alongside governor-accounted RSS
  in stored step pressure data, so the model gap is visible from artifacts.
- claim runs now also emit `step_live_checkpoint` telemetry and
  `reports/steps/step-XX-live.ndjson` artifacts for steps 4-5, exposing
  observed process RSS, raw catalog widths, frontier queue size, prefix-cache
  size, legality-cache size, and whether late claim widening gates are active
  while the step is still in flight.
- claim auto-worker resolution is now memory-aware on
  `desktop_claim_shadow`, and claim proof-close now drops cached evaluated
  terminal payloads after ranking so the live prefix cache stays smaller.
- claim proof-close now also releases processed retained prefix groups once
  exact certification starts, so the live prefix cache can shrink during
  proof-close instead of carrying already-closed groups to step end.
- claim terminal-prefix materialization now also consumes cached exact
  completion summaries from the legality cache after reuse, so claim runs stop
  holding both the legality-cache payload and the retained prefix-group copy of
  the same exact terminal surface.
- claim terminal-prefix materialization now also has a direct compact path when
  no cached completion summary exists, so claim runs no longer build and then
  immediately re-walk a full terminal evaluation vector just to recover the
  same retained candidates and counts.
- cloned claim prefix signatures now share their serialized exact payload
  across frontier and legality-cache copies, so the same hot-path signature no
  longer duplicates that full prefix string into every cloned cache key.
- claim online frontier work items now reuse the shared clause catalog when no
  prefix-local active-window filter applies, so claim discovery no longer
  clones the full next-clause list into every queued frontier item.
- claim online frontier work items now also reuse that same shared serialized
  prefix key for deterministic ordering, instead of allocating a second copy of
  the serialized prefix string for the queue order key.
- claim terminal-clause filtering now also accepts the shared clause slice
  directly instead of first materializing a fresh `Vec<&ClauseRec>` at every
  terminal-prefix check, so the hot claim release path avoids that per-prefix
  allocation even when the claim lane has no active terminal filter to apply.
- `scripts/compare_runs.py` now audits claim-policy honesty, exact-screen
  reason coverage, prune-class coverage, narrative artifacts, and whether the
  stored run reaches the step-15 claim signoff surface.
- `scripts/certify_claim_lane.py` now emits a stored pass/fail certificate from
  claim artifacts and currently fails honestly on missing breadth, missing
  step-15 parity evidence, and the still-missing full-profile stored claim
  bundle on the intended auto-worker desktop config.
- `scripts/benchmark_claim_lane.py` now aggregates stored claim runs into a
  benchmark bundle with runtime percentiles, parity counts, breadth-floor hit
  counts, and manifest snapshots; it still needs a real full-profile claim
  bundle before those numbers can justify a stronger sentence.
- the repo-level autonomy docs now treat claim-policy separation and failed-run
  evidence preservation as baseline; the live bottleneck is full-profile
  claim-run memory stability on the intended auto-worker profile

## Current Operational Blockers

- the widened claim band `9` still needs stored breadth/floor evidence on the
  claim lane itself
- claim-path parity still needs stored signoff evidence even though direct
  exact prefix-completion behavior is now rechecked by tests under the new
  structural-generic scheduler surface
- a full `desktop_claim_shadow_1h` auto-worker run still aborts before
  step-15 completion on the disclosed machine; the latest attempt failed with
  `memory allocation of 1212416 bytes failed`
- the repo can now store the observed-versus-accounted RSS gap for claim steps,
  and the new step-live checkpoint path can now show which in-memory structures
  are growing before acceptance, but there is still no full-profile stored run
  showing whether the new worker cap and combined proof-close group release
  plus prefix-cache and legality-cache compaction fully remove the live spike
- a 2026-03-22 claim smoke rerun reached step `4` and recorded about
  `3.30 GiB` observed RSS after `14.9s` with `2775` frontier groups,
  `5550` legality summaries, `5084` partial-prefix-bound entries, and
  `0` retained prefix-cache groups, so the early spike is still in
  discovery/frontier/legality growth before proof-close on that partial run
- a follow-up 2026-03-22 smoke rerun (`codex-claim-shared-signature-v1`)
  kept the comparable early step-`4` checkpoint at about `3.06 GiB` observed
  RSS after `13.2s`, only about `6.6 MiB` below the prior comparable
  checkpoint, so sharing cloned signature payloads is real but not the main
  memory fix
- a newer 2026-03-22 smoke rerun (`codex-claim-frontier-catalog-reuse-v1`)
  removed that startup cliff from stored evidence: the old `13.2s` /
  `3.06 GiB` checkpoint no longer appears, and the first stored step-`4`
  frontier-progress checkpoint now lands at about `66.4 MiB` observed RSS
  after `422.9s` with `2774` frontier groups, `10193` legality summaries,
  `5084` partial-prefix-bound entries, and `13` retained prefix-cache groups,
  suggesting the old spike was largely frontier queue residency from cloning
  the full next-clause catalog into each queued item
- a 2026-03-22 optimized step-5-limited rerun (`codex-claim-release-step5-v1`)
  then showed that the release claim binary no longer has an early RSS crisis
  on step `4`: after `1777.1s` it was still only about `167.1 MiB` observed
  RSS while enumerating `310916028` candidates and exploring `16` prefix
  states, so the hot blocker there is now exact remaining-two throughput
- a follow-up optimized step-`4` rerun after the new compact claim
  materialization fast path (`codex-claim-release-step4-fastpath-v2`) reached
  the same hot discovery checkpoints about `12-14%` sooner than
  `codex-claim-release-step5-v1` while keeping observed RSS below about
  `89.6 MiB`, but the intended full profile still lacks a stored completion
  bundle
- a newer optimized step-`4` rerun with the slice-based terminal clause path
  (`codex-claim-release-filter-slice-v1a`) reached those same hot checkpoints
  another about `18-20%` sooner than
  `codex-claim-release-step4-fastpath-v2` while keeping observed RSS below
  about `84.0 MiB` through prefix state `7`, so the latest evidence says the
  lane should now be rerun full-profile before reopening another speculative
  step-`4` rewrite
- a follow-up intended-profile rerun (`codex-claim-release-full-v1a`) then ran
  the full `desktop_claim_shadow_1h` shape on that newer binary and did not
  re-hit the old allocator abort before an external timeout after `3844.7s`;
  by then step `4` had explored `43` prefix states, enumerated `848047359`
  candidates, kept the frontier queue at `2732`, and held observed RSS to
  about `278.2 MiB`, so the current blocker is still step-`4` throughput and
  frontier drainage rather than the old early RSS cliff
- benchmark evidence is still too weak for a passing claim certificate

## Immediate Next Slice

1. Inspect `codex-claim-release-full-v1a` and use its stored step-live data to
   identify the next narrow exact remaining-two throughput fix inside step `4`.
2. Rerun the intended `desktop_claim_shadow_1h` profile on the disclosed
   machine on the next optimized binary and inspect the stored RSS-gap data
   again.
3. Once a full-profile bundle exists, run the compare, benchmark, and
   certification scripts against it, then close the remaining breadth/floor
   and parity misses.

## First Reads

- [../../autonomous_plan.md](../../autonomous_plan.md)
- [../../autonomous_progress.md](../../autonomous_progress.md)
- [../../autonomous_checklist.md](../../autonomous_checklist.md)
- [../../README.md](../../README.md)
- [../../docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)

## Do And Do Not

Do:

- treat the current claim lane as a mixed-mode scaffold with honest metadata
- preserve existing guarded, realistic, and demo behavior while the claim lane
  changes
- prefer structural explanations over family-name explanations in new claim
  code
- assume the local shell is Windows PowerShell and avoid shell chaining such
  as `&&`; use separate commands for staging, commit, and push work
- keep claim-lane edits narrow and staged; split very large file updates into
  smaller targeted patches when the first broad patch does not land cleanly
- focus next on release-build step-`4` throughput and full-profile completion,
  then stored breadth/parity/certification

Do not:

- claim that the whole lane is already family-agnostic end-to-end
- switch `bucket_policy` early
- spend time reopening already-landed claim-policy split work unless a memory
  or evidence bug forces it
- call the lane `unguided` yet
