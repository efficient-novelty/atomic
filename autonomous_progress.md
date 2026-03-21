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
- the lane is still mixed-mode after admissibility:
  - `late_expansion_policy = realistic_shadow_inherited`
  - `bucket_policy = semantic_family_runtime_local`
- the lane is not yet certification-ready and it is still not honest to call
  it `unguided`

## Current Read

- The main blocker is no longer admissibility tuning.
- The next blocker is late expansion and prefix filtering: claim mode still
  routes through realistic late-surface machinery in
  `crates/pen-search/src/enumerate.rs` and
  `crates/pen-search/src/prefix_memo.rs`.
- The next blocker after that is scheduler taxonomy: claim-run artifacts still
  carry semantic-family bucket names.
- Reporting already carries much of the exact-screen reason plumbing, but the
  compare, provenance, benchmark, and certification surfaces are still not
  strong enough for the paper sentence.

## Immediate Next Slice

1. Add a claim-specific late expansion policy and claim-generic mutators.
2. Stop treating claim mode as `LateFamilySurface::RealisticShadow` in
   enumeration and prefix filtering.
3. Preserve accepted-hash parity while switching `late_expansion_policy` to
   `claim_generic`.
4. Only after that, replace semantic-family buckets and switch
   `bucket_policy` to `structural_generic`.

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
