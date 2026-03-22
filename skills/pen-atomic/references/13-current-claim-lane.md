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
- cloned claim prefix signatures now share their serialized exact payload
  across frontier and legality-cache copies, so the same hot-path signature no
  longer duplicates that full prefix string into every cloned cache key.
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
- that same rerun later showed a step-`4` live checkpoint around `639 MiB`
  observed RSS after about `510.5s` with `13` retained prefix-cache groups and
  `10193` legality summaries while step `4` was still running, so mid-step
  memory collapse now shows up in stored evidence even though the early spike
  and full-profile stability gate remain open
- benchmark evidence is still too weak for a passing claim certificate

## Immediate Next Slice

1. Rerun the intended `desktop_claim_shadow_1h` profile on the disclosed
   machine and inspect the stored RSS-gap data after the latest
   legality-cache compaction plus processed-prefix-group release change.
2. Once that bundle exists, run the compare, benchmark, and certification
   scripts against it.
3. Then close the remaining breadth/floor and parity misses.

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
- focus next on memory stability, then stored breadth/parity/certification

Do not:

- claim that the whole lane is already family-agnostic end-to-end
- switch `bucket_policy` early
- spend time reopening already-landed claim-policy split work unless a memory
  or evidence bug forces it
- call the lane `unguided` yet
