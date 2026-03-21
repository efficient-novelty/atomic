# Current Claim Lane

Read this file when the task touches `desktop_claim_shadow`, claim-lane
telemetry, claim-lane narratives, or the autonomy-certification roadmap.

## Current Truth

- `desktop_claim_shadow` exists as a separate profile and config family.
- The lane is not yet certified and should still be described with the safer
  `bounded live recovery` wording.
- Claim admissibility now uses structural claim debt and anchor hints, without
  named-family focus progression.
- Claim late expansion now uses a claim-specific late surface with the first
  structural mutators landed for kappa `4-6`.
- Claim bucket scheduling now uses a structural-generic taxonomy derived from
  prefix-local syntax and runtime evidence.
- The current mixed state is recorded explicitly in run metadata:
  - `guidance_style = claim_debt_guided`
  - `late_expansion_policy = claim_generic`
  - `bucket_policy = structural_generic`

## Current Operational Blockers

- later claim bands `7-9` are still conservative reference-first surfaces
  rather than richer claim-specific mutator packs
- claim-path parity and exact prefix-completion behavior still need a direct
  recheck under the new structural-generic scheduler surface
- provenance, benchmark, compare, and certification surfaces are still not
  strong enough for the stronger paper sentence

## Immediate Next Slice

1. Broaden the later `kappa 7-9` claim-generic bands without regressing
   accepted parity.
2. Recheck prefix-summary pruning and terminal completion exactness under the
   structural-generic claim scheduler.
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
