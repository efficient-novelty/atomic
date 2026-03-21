# Autonomous Claim Lane Progress

Last updated: 2026-03-21

This file tracks the live implementation status of the claim-bearing
`desktop_claim_shadow` lane described in [autonomous_plan.md](autonomous_plan.md).

## Objective Status

The claim lane is now separated at the profile/config/reporting level, but it
is not yet certified and it is not yet honest to call it `unguided`.

What is true now:

- `desktop_claim_shadow` exists as a distinct search profile
- claim configs exist for `smoke`, `1h`, and `10h`
- claim runs can execute end-to-end and persist their own run metadata
- claim runs can write terminal and persisted narrative artifacts
- `run.json` and `telemetry.ndjson` now record explicit search-policy metadata
- claim mode now derives admissibility from structural claim debt instead of
  named-family focus progression

What is still not true:

- claim mode does not yet use generic claim mutators
- claim mode does not yet use structural-generic bucket scheduling
- the lane is not yet certification-ready for the stronger paper sentence

## Landed In This Slice

Completed work:

- added `SearchProfile::DesktopClaimShadow`
- added `AdmissibilityMode::DesktopClaimShadow`
- added:
  - `configs/desktop_claim_shadow_smoke.toml`
  - `configs/desktop_claim_shadow_1h.toml`
  - `configs/desktop_claim_shadow_10h.toml`
- generalized persisted and terminal narrative support so it is no longer
  limited to `demo_breadth_shadow`
- added claim-lane narrative and run tests
- extended `run.json` with a `search_policy` block
- extended telemetry `run_started` payloads with the same `search_policy` block
- updated `scripts/compare_runs.py` so claim runs are checked for narrative
  artifact completeness
- added claim-debt admissibility helpers in `pen-type` based on structural
  debt axes and claim anchor hints
- routed `DesktopClaimShadow` through the claim-debt admissibility path
- removed named-family focus requirements from claim-mode admissibility while
  preserving legacy behavior for guarded and shadow lanes
- switched claim-run `guidance_style` metadata to `claim_debt_guided`

Current claim-lane policy metadata is intentionally honest:

- `guidance_style = claim_debt_guided`
- `late_expansion_policy = realistic_shadow_inherited`
- `bucket_policy = semantic_family_runtime_local`

Those values are scaffolding truths, not the final target. `guidance_style`
now reflects the landed claim-debt admissibility split, while
`late_expansion_policy` and `bucket_policy` stay on the inherited realistic and
semantic-family values until those behaviors actually change.

## Verification

Passing checks from this slice:

- `cargo test -p pen-store --lib`
- `cargo test -p pen-search --lib`
- `cargo test -p pen-cli claim_run_writes_policy_metadata_and_claim_narrative`
- `cargo test -p pen-cli claim_narrative_uses_claim_headline`
- `cargo test -p pen-type --lib`

## Current Read Of The Lane

- The lane split is real enough that later claim-specific work no longer has
  to be tangled into `demo_breadth_shadow`.
- The claim profile is now structurally guided at the admissibility layer, but
  it still inherits realistic late-search behavior after that point.
- The remaining inherited late-surface and bucket behavior is visible in stored
  metadata instead of being hidden.
- The next meaningful work should change real behavior, not just names.

## Next Actions

1. Replace claim-path named late clause builders with generic structural
   mutators.
2. Replace claim-path semantic-family bucket labels with structural-generic
   bucket scheduling.
3. Add certification-facing reporting and compare coverage for the new
   claim-debt admissibility reasons and provenance.
4. Only after the late-expansion and bucket work land, switch the remaining
   claim-lane policy metadata to `claim_generic` and `structural_generic`.

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` behavior unchanged while the claim lane evolves.
- Do not claim autonomy from policy labels alone.
- Do not mark the lane `unguided` before the certification gate exists and
  passes.
