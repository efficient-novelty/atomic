# Current Claim Lane

Read this file when the task touches `desktop_claim_shadow`, claim-lane
telemetry, claim-lane narratives, or the autonomy-certification roadmap.

## Current Truth

- `desktop_claim_shadow` exists as a separate profile and config family.
- The lane is not yet certified and should still be described with the safer
  `bounded live recovery` wording.
- Claim admissibility now uses structural claim debt and anchor hints, without
  named-family focus progression.
- The lane still reuses realistic-shadow late-surface behavior and
  semantic-family bucket scheduling after admissibility.
- The current mixed state is recorded explicitly in run metadata:
  - `guidance_style = claim_debt_guided`
  - `late_expansion_policy = realistic_shadow_inherited`
  - `bucket_policy = semantic_family_runtime_local`

## Current Operational Blockers

- claim mode still maps to `LateFamilySurface::RealisticShadow` in enumeration
  and prefix memoization
- claim enumeration still depends on named relaxed/family helpers rather than a
  claim-generic mutator surface
- claim-run artifacts still serialize semantic-family bucket names
- provenance, benchmark, compare, and certification surfaces are still not
  strong enough for the stronger paper sentence

## Immediate Next Slice

1. Add a claim-specific late expansion policy.
2. Stop routing claim mode through realistic late-family surfaces.
3. Land the first claim-generic mutators for kappa `4-6`.
4. Switch `late_expansion_policy` only after that code path is real.

## First Reads

- [../../autonomous_plan.md](../../autonomous_plan.md)
- [../../autonomous_progress.md](../../autonomous_progress.md)
- [../../README.md](../../README.md)
- [../../docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)

## Do And Do Not

Do:

- treat the current claim lane as a mixed-mode scaffold with honest metadata
- preserve existing guarded, realistic, and demo behavior while the claim lane
  changes
- prefer structural explanations over family-name explanations in new claim
  code
- focus next on enumeration, prefix memo, scheduling, and certification

Do not:

- claim that the whole lane is already family-agnostic end-to-end
- switch `late_expansion_policy` or `bucket_policy` early
- call the lane `unguided` yet
