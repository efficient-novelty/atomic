# Current Claim Lane

Read this file when the task touches `desktop_claim_shadow`, claim-lane
telemetry, claim-lane narratives, or the autonomy-certification roadmap.

## Current Truth

- `desktop_claim_shadow` now exists as a separate profile and config family.
- The lane is not yet certified and should still be described with the safer
  `bounded live recovery` wording.
- The lane currently reuses realistic-shadow admissibility and late-surface
  behavior where claim-specific logic has not landed yet.
- That inherited behavior is now recorded explicitly in run metadata:
  - `guidance_style = legacy_family_guided`
  - `late_expansion_policy = realistic_shadow_inherited`
  - `bucket_policy = semantic_family_runtime_local`

## What Landed

- separate `desktop_claim_shadow` profile wiring
- separate claim configs for `smoke`, `1h`, and `10h`
- claim narrative and event artifact support
- claim-lane `run.json` and telemetry policy metadata
- compare-tool narrative artifact checks for claim runs

## What Is Still Open

- claim-debt admissibility
- generic claim mutators
- structural-generic bucket scheduling
- exact certification gate
- benchmark and provenance hardening for the paper appendix

## First Reads

- [../../autonomous_plan.md](../../autonomous_plan.md)
- [../../autonomous_progress.md](../../autonomous_progress.md)
- [../../README.md](../../README.md)
- [../../docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)

## Do And Do Not

Do:

- treat the current claim lane as a scaffold with honest metadata
- preserve existing guarded, realistic, and demo behavior while the claim lane
  changes
- prefer structural explanations over family-name explanations in new claim
  code

Do not:

- claim that the lane is already family-agnostic
- switch policy metadata to `claim_debt_guided`, `claim_generic`, or
  `structural_generic` before the code actually behaves that way
- call the lane `unguided` yet
