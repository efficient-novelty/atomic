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

What is still not true:

- claim mode does not yet use claim-debt admissibility
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

Current claim-lane policy metadata is intentionally honest:

- `guidance_style = legacy_family_guided`
- `late_expansion_policy = realistic_shadow_inherited`
- `bucket_policy = semantic_family_runtime_local`

Those values are scaffolding truths, not the final target. They should switch
to `claim_debt_guided`, `claim_generic`, and `structural_generic` only when
the underlying behavior actually changes.

## Verification

Passing checks from this slice:

- `cargo test -p pen-store --lib`
- `cargo test -p pen-search --lib`
- `cargo test -p pen-cli claim_run_writes_policy_metadata_and_claim_narrative`
- `cargo test -p pen-cli claim_narrative_uses_claim_headline`

## Current Read Of The Lane

- The lane split is real enough that later claim-specific work no longer has
  to be tangled into `demo_breadth_shadow`.
- The current claim profile still inherits realistic late-search behavior at
  the admissibility and late-surface level.
- That inheritance is now visible in stored metadata instead of being hidden.
- The next meaningful work should change real behavior, not just names.

## Next Actions

1. Split admissibility so `DesktopClaimShadow` stops using named-family focus
   progression.
2. Replace claim-path named late clause builders with generic structural
   mutators.
3. Replace claim-path semantic-family bucket labels with structural-generic
   bucket scheduling.
4. Only after those land, switch the claim-lane policy metadata to the target
   `claim_debt_guided` / `claim_generic` / `structural_generic` values.

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` behavior unchanged while the claim lane evolves.
- Do not claim autonomy from policy labels alone.
- Do not mark the lane `unguided` before the certification gate exists and
  passes.
