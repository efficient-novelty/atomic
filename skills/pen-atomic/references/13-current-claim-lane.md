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
  mutators landed for kappa `4-8`; kappa `9` is still conservative.
- Claim bucket scheduling now uses a structural-generic taxonomy derived from
  prefix-local syntax and runtime evidence.
- The current mixed state is recorded explicitly in run metadata:
  - `guidance_style = claim_debt_guided`
  - `late_expansion_policy = claim_generic`
  - `bucket_policy = structural_generic`

## Current Operational Blockers

- the remaining claim band `9` is still conservative and reference-first
- claim-path parity still needs stored signoff evidence even though direct
  exact prefix-completion behavior is now rechecked by tests under the new
  structural-generic scheduler surface
- provenance, benchmark, compare, and certification surfaces are still not
  strong enough for the stronger paper sentence

## Immediate Next Slice

1. Finish the remaining `kappa 9` claim-generic widening without regressing
   accepted parity.
2. Turn the widened late surface into stored breadth/floor evidence on the
   claim lane itself.
3. Then harden the certification-facing evidence bundle.

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
- focus next on enumeration, prefix memo rechecks, later-band mutators, and
  certification

Do not:

- claim that the whole lane is already family-agnostic end-to-end
- switch `bucket_policy` early
- call the lane `unguided` yet
