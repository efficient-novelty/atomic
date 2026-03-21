# Autonomous Claim Lane Progress

Last updated: 2026-03-21

This file tracks the live operational state of `desktop_claim_shadow`. It is
intentionally short and forward-looking; use
[autonomous_plan.md](autonomous_plan.md) for the remaining work sequence.

## Snapshot

- `desktop_claim_shadow` is a distinct profile with its own configs,
  narratives, and run metadata
- claim admissibility is now structurally guided:
  - `guidance_style = claim_debt_guided`
  - claim mode does not use a named `focus_family`
- claim late expansion is now claim-specific:
  - `late_expansion_policy = claim_generic`
- the lane is still mixed-mode at the scheduler/reporting layer:
  - `bucket_policy = semantic_family_runtime_local`
- the lane is not yet certification-ready and it is still not honest to call
  it `unguided`

## Current Read

- The main blocker is no longer admissibility tuning or inherited late-surface
  routing.
- Claim mode now has its own `ClaimGeneric` late surface in
  `crates/pen-search/src/enumerate.rs` and
  `crates/pen-search/src/prefix_memo.rs`.
- The first claim-generic mutator pass is now real for kappa `4-6`, while the
  later `7-9` claim bands are still conservative reference-first surfaces.
- The next blocker is scheduler taxonomy: claim-run artifacts still carry
  semantic-family bucket names.
- Reporting already carries much of the exact-screen reason plumbing, but the
  compare, provenance, benchmark, and certification surfaces are still not
  strong enough for the paper sentence.

## Immediate Next Slice

1. Replace semantic-family claim buckets with runtime-local structural bucket
   labels and ordering.
2. Preserve accepted-hash parity while switching `bucket_policy` to
   `structural_generic`.
3. Broaden the later `kappa 7-9` claim-generic bands without reintroducing
   realistic-shadow fallback.
4. Then earn the breadth/floor evidence and certification bundle on top of the
   claim-generic lane.

## After That

- earn honest breadth and floor evidence under the claim-specific mutator path
- make retuning change search shape, not just time allocation
- harden manifest, benchmarking, compare output, and certification

## Verification Baseline

- `cargo test -p pen-type --lib`
- `cargo test -p pen-search --lib`
- `cargo test -p pen-cli claim_run_writes_policy_metadata_and_claim_narrative`

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` behavior unchanged while the claim lane evolves.
- Do not switch policy metadata ahead of the real behavior.
- Do not claim autonomy from labels alone.
- Do not mark the lane `unguided` before the certification gate exists and
  passes.
