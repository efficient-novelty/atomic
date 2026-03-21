# Autonomous Claim Lane Plan

Last updated: 2026-03-21
Status: active

## Objective

Turn `desktop_claim_shadow` from a mixed-mode scaffold into a claim-specific
lane that can support a certified desktop appendix.

Until the certification gate passes, keep the paper wording in the safer
`bounded live recovery` form. The remaining problem is no longer profile
plumbing or admissibility naming. The remaining problem is that the claim lane
still inherits realistic late expansion, semantic-family scheduling, and
incomplete certification evidence after the admissibility step.

## Current Baseline

What is already true and should now be treated as baseline, not as open work:

- `desktop_claim_shadow` exists as its own profile/config family
- claim runs persist their own narrative and policy metadata
- claim admissibility is now driven by structural claim debt and anchor hints
- claim mode no longer requires a named `focus_family`
- stored policy metadata is intentionally mixed and honest:
  - `guidance_style = claim_debt_guided`
  - `late_expansion_policy = claim_generic`
  - `bucket_policy = structural_generic`

What this means operationally:

- the admissibility split is no longer the bottleneck
- claim mode no longer routes through inherited realistic late-family surfaces
- the scheduler/bucket coupling is now removed on the claim path
- the next meaningful work is to broaden the later claim-generic bands,
  recheck exact prefix completion under the new claim scheduler surface, and
  then bind the paper sentence to stored evidence

## What Still Blocks The Stronger Claim

- later claim bands `7-9` are still thin reference-first surfaces rather than
  richer claim-specific mutator packs
- claim-path parity and exact prefix/completion behavior still need a direct
  recheck under the structural-generic bucket surface
- breadth widening and within-step retuning are not yet claim-specific enough
  to support an autonomy claim
- manifest, benchmark, compare, and certification surfaces are not yet strong
  enough to bind the paper sentence to stored evidence

## Success Condition

This plan is complete only when a stored `desktop_claim_shadow` evidence bundle
shows all of the following at the same time:

- accepted hashes match the guarded reference through step `15`
- `guidance_style == claim_debt_guided`
- `late_expansion_policy == claim_generic`
- `bucket_policy == structural_generic`
- the claim path has no silent realistic-shadow, demo-only, guarded, or replay
  fallback
- early breadth gates pass from stored evidence
- late floor gates pass without inflating `full_telescopes_evaluated` beyond a
  moderate range
- exact-screen reasons and prune classes are complete in stored artifacts
- manifest and benchmark data are complete enough for a paper appendix
- the certification script emits a passing claim certificate

Only after those conditions pass should the paper wording move beyond
`bounded live recovery`.

## Execution Priorities

1. Remove realistic late-surface inheritance.
2. Replace semantic-family bucket scheduling.
3. Earn honest breadth and shape-changing replanning.
4. Harden reporting, provenance, benchmarking, and certification.

## 1. Claim-Generic Late Expansion

Why this is next:

- the claim lane is still not end-to-end claim-specific after admissibility
- the most direct honesty gap is the remaining claim-to-realistic mapping in
  enumeration and prefix memoization

Files:

- `crates/pen-search/src/enumerate.rs`
- `crates/pen-search/src/prefix_memo.rs`
- new claim mutator module if needed:
  - `crates/pen-search/src/claim_mutators.rs`
  - or `crates/pen-search/src/generic_mutators.rs`

Concrete tasks:

- add a claim-specific late expansion policy instead of routing claim mode
  through `LateFamilySurface::RealisticShadow`
- let `EnumerationContext::from_admissibility()` carry claim-specific policy
  information
- stop treating `DesktopClaimShadow` as realistic in prefix filtering and
  terminal completion summaries
- replace claim-path calls to named late helpers with generic structural
  mutators
- keep guarded, realistic, and demo behavior unchanged

Claim mutator target by kappa band:

- kappa `4`: bridge, reanchor, and modal-lift variants
- kappa `5-6`: support-form, bridge-head, and mixed shell-shape variants
- kappa `7`: operator-style wrappers and shell mixtures
- kappa `8`: modal-temporal exchange and history-reanchor mixtures
- kappa `9`: binder-heavy and higher-order structural variants

Hard rule for claim mode:

- do not call named relaxed helpers on the claim path
- do not call named family clause builders on the claim path
- do not call any `demo_*_clauses()` helpers on the claim path

Done when:

- claim mode no longer reaches realistic-shadow late-family helpers
- prefix memo no longer treats claim mode as realistic late surface
- `late_expansion_policy` can honestly switch to `claim_generic`

## 2. Structural Bucket Scheduling

Why this comes second:

- once late expansion is claim-specific, the next inherited semantic coupling
  is the bucket taxonomy itself
- stored evidence should explain claim scheduling without semantic-family names

Files:

- `crates/pen-search/src/engine.rs`
- optionally a later split to `crates/pen-search/src/scheduler.rs`

Concrete tasks:

- define a neutral bucket taxonomy from runtime-local structural features
- compute claim bucket keys from syntax and prefix-local evidence only
- persist neutral labels for claim runs
- keep semantic-family buckets unchanged for realistic and demo lanes

Suggested neutral axes:

- `OperatorCount`
- `ModalTemporalMix`
- `ReferenceSupport`
- `BinderDepth`
- `BridgeDensity`
- `Locality`
- `Width`
- `ReanchorPresence`

Done when:

- claim-run artifacts no longer serialize semantic-family bucket names
- scheduler order for the claim lane can be explained entirely from
  runtime-local evidence
- `bucket_policy` can honestly switch to `structural_generic`

## 3. Honest Breadth And Replanning

Why this comes third:

- claim-specific mutators and buckets are not enough if stored evidence still
  shows narrow or time-shifted search
- the claim must be earned by the actual search mass and the actual replanning
  behavior in the artifacts

Files:

- `crates/pen-search/src/enumerate.rs`
- `crates/pen-search/src/engine.rs`
- possibly:
  - `crates/pen-search/src/diversify.rs`
  - `crates/pen-search/src/frontier.rs`
  - `crates/pen-search/src/scheduler.rs`
- `configs/desktop_claim_shadow_*.toml`

Concrete tasks:

- restore honest early raw-surface accounting on steps `1-4`
- widen steps `5-15` through generic mutators, not named family surfaces
- make retuning able to change search shape, not just move time between phases
- persist retune action and reason into step artifacts and narrative output
- strengthen prefix bounds in parallel so full terminal work stays moderate

Breadth gates to earn:

- step `1` reports `2144`
- steps `1-4` fit the shared early budget honestly from stored counts
- default late floors stay at least:
  - step `10` generated `>= 500`
  - step `11` generated `>= 800`
  - step `12` generated `>= 1200`
  - step `13` generated `>= 2200`
  - step `14` generated `>= 3500`
  - step `15` generated `>= 5000`

Done when:

- early breadth and late floors are achieved from stored evidence
- retune logs explain real search-shape changes
- `full_telescopes_evaluated` stays moderate while floors improve

## 4. Evidence, Provenance, And Certification

Why this is last:

- stronger paper wording should be bound to a reproducible bundle, not to prose
- once the search path is claim-specific, the remaining job is to make that
  path auditable and repeatable

Files:

- `crates/pen-cli/src/cmd_run.rs`
- `crates/pen-cli/src/report.rs`
- `crates/pen-cli/src/narrative.rs`
- `scripts/compare_runs.py`
- new benchmark harness:
  - `scripts/bench_claim.py`
  - or `xtask bench-claim`
- new certification gate:
  - `scripts/certify_claim_lane.py`

Concrete tasks:

- keep exact-screen reasons and prune classes visible in claim reports and
  compare output
- harden manifests with hardware, build, git, and binary fingerprint data
- add a repeatable benchmark harness for the claim config
- add a certification script that checks hashes, metadata, no fallback,
  breadth/floor gates, runtime threshold, reason coverage, and manifest
  completeness

Required outputs:

- `claim_certificate.json`
- compact human-readable summary
- repeatable benchmark bundle suitable for a paper appendix

Done when:

- one command produces an appendix-ready claim bundle
- the certification script passes on the intended claim configuration
- the paper wording is tied to the stored certificate instead of free prose

## Working Rules For Remaining Claim-Lane PRs

- preserve existing behavior for `strict_canon_guarded`,
  `realistic_frontier_shadow`, and `demo_breadth_shadow`
- keep policy metadata honest and update one label only when the underlying
  behavior has actually changed
- prefer runtime-local structural features over semantic-family names
- do not lower floors or hide misses to make the claim look cleaner
- do not spend time on broad `Demo*` renaming cleanup before certification

## Out Of Scope Until Certification

- broad renaming of every `Demo*` helper or type
- stronger user-facing language such as `unguided`
- any paper claim that outruns the stored certification bundle
- relaxing breadth floors to make the claim easier

## Immediate Next Step

Implement the remaining late-surface and evidence follow-up directly:

1. broaden the later `kappa 7-9` claim-generic bands without reintroducing
   realistic-shadow or demo-only helpers
2. recheck prefix-summary pruning and terminal completion exactness under the
   structural-generic claim scheduler surface
3. then earn the late breadth floors and certification bundle on the updated
   claim lane
