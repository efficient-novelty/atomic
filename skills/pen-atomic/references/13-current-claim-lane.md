# Current Claim Lane

Read this file when the task touches `desktop_claim_shadow`, claim-lane
telemetry, claim-lane narratives, or the autonomy-certification roadmap.

## Current Truth

- `desktop_claim_shadow` now exists as a separate profile and config family.
- The lane is not yet certified and should still be described with the safer
  `bounded live recovery` wording.
- The lane now uses claim-debt admissibility derived from structural debt and
  claim anchor hints, without named-family focus progression.
- The lane still reuses realistic-shadow late-surface and bucket behavior where
  claim-specific logic has not landed yet.
- The current mixed state is recorded explicitly in run metadata:
  - `guidance_style = claim_debt_guided`
  - `late_expansion_policy = realistic_shadow_inherited`
  - `bucket_policy = semantic_family_runtime_local`

## What Landed

- separate `desktop_claim_shadow` profile wiring
- separate claim configs for `smoke`, `1h`, and `10h`
- claim narrative and event artifact support
- claim-lane `run.json` and telemetry policy metadata
- compare-tool narrative artifact checks for claim runs
- claim-debt admissibility helpers and claim-only anchor policy wiring
- claim-mode admissibility with `focus_family = None`

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
- remember that only the admissibility layer is claim-specific so far

Do not:

- claim that the whole lane is already family-agnostic end-to-end
- switch `late_expansion_policy` or `bucket_policy` to `claim_generic` or
  `structural_generic` before the code actually behaves that way
- call the lane `unguided` yet
