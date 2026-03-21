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
- `scripts/compare_runs.py` now audits claim-policy honesty, exact-screen
  reason coverage, prune-class coverage, narrative artifacts, and whether the
  stored run reaches the step-15 claim signoff surface.
- `scripts/certify_claim_lane.py` now emits a stored pass/fail certificate from
  claim artifacts and currently fails honestly on missing breadth, missing
  step-15 parity evidence, and the still-missing full-profile stored claim
  bundle on the intended auto-worker desktop config.
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
- benchmark evidence is still too weak for a passing claim certificate

## Immediate Next Slice

1. Make the intended `desktop_claim_shadow_1h` profile memory-stable on the
   disclosed machine.
2. Once that bundle exists, run the compare/certification scripts against it.
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
