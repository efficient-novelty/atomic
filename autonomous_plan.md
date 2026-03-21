# Autonomous Claim Lane Plan

Last updated: 2026-03-21
Status: proposed

## Objective

Build a new claim-bearing Rust lane, tentatively named
`desktop_claim_shadow`, whose end-state is strong enough to support the
statement that the PEN axioms recover the current 15-step sequence
autonomously on a disclosed desktop configuration.

The immediate implementation goal is narrower than the final paper claim:

- separate the claim lane from `demo_breadth_shadow` instead of continuing to
  patch demo-only scaffolding
- keep the current paper wording in the safer `bounded live recovery` form
  until the certification gate passes
- preserve the repo's already-landed strength that the hot path is structural
  and name-free
- remove the remaining trajectory-shaped behavior from the claim lane's
  proposal, admissibility, scheduling, and evidence pipeline

The key point is that the remaining problem is no longer primarily
"make scoring less biased." The repo is already clear that the hot path is
structural and name-free. The remaining work is to ensure that the new claim
lane does not depend on named family progression, named late-family clause
builders, named semantic bucket labels, or demo-only reporting shortcuts.

## Why A Separate Lane

The current repo state is already explicit:

- `strict_canon_guarded` is the authoritative lane
- `realistic_frontier_shadow` is comparison-backed
- `demo_breadth_shadow` is an experimental comparison-backed scaffold that
  still reuses realistic-shadow search semantics

That makes `demo_breadth_shadow` the wrong place to accumulate a claim-bearing
story. The claim lane should begin as a separate profile with separate
telemetry, separate configs, and separate certification, even if it initially
borrows some of the same infrastructure.

Naming guidance:

- use `desktop_claim_shadow` for the new profile name
- do not use `unguided` in code or docs yet
- earn stronger wording only after the certification step passes

## Non-Negotiable Constraints

- Existing behavior for `strict_canon_guarded`,
  `realistic_frontier_shadow`, and `demo_breadth_shadow` must remain unchanged
  during the initial claim-lane rollout.
- The claim lane must not silently fall back to guarded, replay, or demo-only
  behavior and then still present itself as autonomous.
- Stored artifacts must say which guidance, late-expansion, and bucket policies
  were actually used.
- Breadth claims must stay honest: do not claim widened search if the stored
  evidence shows only narrow live exploration.
- Do not lower existing late-step floor expectations to make the claim easier.
- Do not spend time on broad `Demo*` renaming cleanup until the claim lane
  actually works end-to-end.

## Working Baseline

What the repo can already honestly say:

- the hot path is structural and name-free
- the guarded lane recovers the full accepted 15-step corpus
- comparison-backed lanes already have a documented honesty boundary
- step summaries, run manifests, narrative artifacts, and compare tooling exist
  and can be extended rather than invented from scratch

What still blocks the stronger autonomy claim:

- admissibility still walks named-family progression in the live logic
- late clause generation still reaches named family builders and demo-specific
  clause generators
- scheduler buckets still serialize semantic-family names
- within-step retuning is mostly time shifting, not real replanning
- stored evidence is still missing some reason-code and provenance details
- there is not yet a dedicated claim certificate that binds the paper wording
  to stored evidence

## Success Condition

This plan is complete only when a stored `desktop_claim_shadow` evidence bundle
shows all of the following at the same time:

- accepted hashes match the guarded reference through step `15`
- claim-lane metadata records `guidance_style = claim_debt_guided`
- claim-lane metadata records `late_expansion_policy = claim_generic`
- claim-lane metadata records `bucket_policy = structural_generic`
- early breadth gates pass honestly from stored evidence
- late floor gates pass without inflating full telescope work beyond a moderate
  range
- exact-screen reasons and prune classes are complete in stored artifacts
- manifest/provenance data are complete enough for a paper appendix
- the certification script emits a passing claim certificate

Only after those conditions pass should the paper wording move from
`bounded live recovery` to a stronger claim about autonomous recovery under the
certified claim lane.

## Execution Order

1. Create the separate claim-lane profile and telemetry surface first.
2. Split admissibility so claim mode can bypass named-family progression.
3. Replace claim-mode named clause builders with generic structural mutators.
4. Replace claim-mode named bucket scheduling with structural bucket scheduling.
5. Restore honest breadth, widen late surfaces, and make reserve retuning into
   actual replanning.
6. Complete reporting, provenance, benchmarking, and the certification gate.

## Workstreams

### 1. Profile Plumbing

Create a separate claim lane before changing behavior. This keeps every later
change decoupled from demo-only policy and lets stored artifacts identify claim
runs explicitly.

Files:

- `crates/pen-search/src/config.rs`
- `crates/pen-type/src/admissibility.rs`
- `crates/pen-cli/src/cmd_run.rs`
- `configs/`
- new root/docs planning files

Planned changes:

- add `SearchProfile::DesktopClaimShadow`
- add `AdmissibilityMode::DesktopClaimShadow`
- add configs:
  - `configs/desktop_claim_shadow_smoke.toml`
  - `configs/desktop_claim_shadow_1h.toml`
  - `configs/desktop_claim_shadow_10h.toml`
- generalize `terminal_narrative_config()` so terminal narrative/progress
  output is not hard-wired to `DemoBreadthShadow`
- add follow-on tracking docs:
  - `claim_lane_plan.md`
  - `claim_lane_checklist.md`
- add config tests analogous to the existing demo-profile coverage
- extend `run.json` / telemetry with explicit claim-lane policy markers:
  - `guidance_style = "claim_debt_guided"`
  - `late_expansion_policy = "claim_generic"`
  - `bucket_policy = "structural_generic"`

Done when:

- the new profile runs end-to-end
- old lanes behave exactly as before
- claim-run artifacts explicitly record the claim guidance and scheduler policy

### 2. Admissibility Split

This is the main structural blocker. The current admissibility path still
walks an ordered named-family chain through
`focus_family_from_debt() -> focus_policy_for_mode() -> clause_band_for_mode()`
and uses named package checks in `StructuralDebt`.

Files:

- `crates/pen-type/src/admissibility.rs`
- `crates/pen-type/src/obligations.rs`

Planned changes:

- add a claim-only guidance path instead of deleting the legacy path
- introduce a guidance discriminator such as:

```rust
enum GuidanceStyle {
    LegacyFamilyGuided,
    ClaimDebtGuided,
}

struct ClaimDebtAxes {
    kappa_min: u16,
    kappa_max: u16,
    path_pressure: u8,
    trunc_pressure: u8,
    coupling_pressure: u8,
    support_pressure: u8,
    modal_pressure: u8,
    temporal_pressure: u8,
    reanchor_pressure: u8,
    closure_pressure: u8,
}
```

- in claim mode, bypass:
  - `focus_family_from_debt()`
  - `focus_policy_for_mode()`
  - family-specific branches inside `clause_band_for_mode()`
- add claim-only helpers in `obligations.rs`, for example:
  - `claim_debt_axes()`
  - `claim_clause_band_hint()`
  - `claim_anchor_policy()`
  - `claim_exact_cap_hint()`
- derive claim axes from structural counts that already exist or can be
  computed from `StructuralDebt`, including:
  - path depth
  - truncation
  - active entries and exports
  - constructor pressure
  - modal and temporal usage
  - coupled differential counts
  - reanchor availability

Migration rule:

- keep the existing family-guided logic unchanged for
  `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow`
- route only `DesktopClaimShadow` through the new claim-debt path initially

Tests to add:

- claim mode never sets a `focus_family`
- claim mode still opens kappa bands when structural axes justify them
- legacy modes still produce identical admissibility results

Done when:

- `DesktopClaimShadow` no longer depends on named-family progression in
  admissibility
- existing profiles still pass unchanged

### 3. Generic Structural Mutators

Enumeration is still strongly family-shaped. `EnumerationContext` carries
family-tuned booleans, `from_admissibility()` copies family expectations into
live enumeration policy, and `late_clause_options()` still dispatches into
named relaxed, family, and demo clause builders. That must stop in claim mode.

Files:

- `crates/pen-search/src/enumerate.rs`
- new module:
  - `crates/pen-search/src/claim_mutators.rs`
  - or `crates/pen-search/src/generic_mutators.rs`

Planned changes:

- keep legacy fields in `EnumerationContext` for current lanes
- add a claim-only late-expansion path such as:

```rust
enum LateExpansionPolicy {
    None,
    RealisticShadow,
    DemoBreadthShadow,
    ClaimGeneric(ClaimExpansionPolicy),
}

struct ClaimExpansionPolicy {
    max_binder_nesting: u8,
    max_operator_mix: u8,
    max_reference_reanchors: u8,
    max_bridge_heads: u8,
    max_union_width: u8,
    allow_modal: bool,
    allow_temporal: bool,
    allow_history_reanchor: bool,
}
```

- in claim mode, let `EnumerationContext::from_admissibility()` populate
  `LateExpansionPolicy::ClaimGeneric(...)`
- replace claim-mode `late_clause_options()` dispatch with generic structural
  mutators instead of named family builders

Suggested generic mutators:

- `enumerate_bridge_variants()`
- `enumerate_binder_shape_variants()`
- `enumerate_reference_reanchors()`
- `enumerate_support_form_variants()`
- `enumerate_operator_mix_variants()`
- `enumerate_modal_temporal_exchange_variants()`
- `enumerate_union_cluster_variants()`

Claim-mode kappa mapping:

- kappa `4`: bridge variants plus modal-lift and reanchor shapes
- kappa `5-6`: support-form, connection-style, and curvature-style shapes
- kappa `7`: operator-style wrappers and mixed shell heads
- kappa `8`: modal-temporal exchange and shell mixtures
- kappa `9`: higher-order functional and binder-heavy variants

Hard rule for claim mode:

- do not call `relaxed_axiomatic_bridge_clause()`
- do not call `relaxed_modal_shell_clause()`
- do not call `relaxed_connection_shell_clause()`
- do not call `relaxed_curvature_shell_clause()`
- do not call `operator_bundle_family_clauses()`
- do not call `hilbert_functional_family_clauses()`
- do not call `temporal_shell_family_clauses()`
- do not call any `demo_*_clauses()` functions

Tests to add:

- unit tests for each claim-mode kappa band
- a test that the claim lane never uses the demo late-family surface
- parity smoke tests that accepted hashes remain unchanged on the known path

Done when:

- the claim profile can generate late candidates through generic mutators alone
- no named family builders are reachable from the claim path

### 4. Structural Bucket Scheduling

The scheduler has already improved, but the bucket taxonomy still uses semantic
family names such as `TemporalShell`, `Hilbert`, `Curvature`, and
`Differential`. For the claim lane, the live scheduler and stored reports
should avoid those names entirely.

Files:

- `crates/pen-search/src/engine.rs`
- optionally later split to `scheduler.rs`

Planned changes:

- add a claim-only neutral bucket taxonomy such as:
  - `OperatorCount`
  - `ModalTemporalMix`
  - `ReferenceSupport`
  - `BinderDepth`
  - `BridgeDensity`
  - `Locality`
  - `Width`
  - `ReanchorPresence`
- compute claim bucket keys from syntax and runtime-local prefix features only
- keep existing demo bucket families for demo mode
- persist claim bucket labels under neutral names, for example:
  - `k8/modal-temporal-mix/library-backed/broad`
  - not `k8/temporal_shell/...`

Done when:

- claim-run artifacts no longer serialize semantic-family bucket names
- scheduler order for the claim lane can be explained entirely from
  runtime-local evidence

### 5. Honest Breadth Restoration

The current demo docs already identify the honest breadth goals. The claim lane
should adopt those as minimum evidence gates rather than weakening them.

Files:

- `crates/pen-search/src/enumerate.rs`
- `crates/pen-search/src/engine.rs`
- possibly:
  - `crates/pen-search/src/scheduler.rs`
  - `crates/pen-search/src/diversify.rs`
  - `crates/pen-search/src/frontier.rs`
- `configs/desktop_claim_shadow_*.toml`

Planned changes:

- restore full candidate-list generation on steps `1-4` wherever affordable
- preserve honest raw-surface accounting
- validate early exhaustiveness from stored evidence, not from config intent
- widen steps `5-9` by increasing:
  - kappa range
  - support forms
  - bridge-head variety
- widen steps `10-12` by increasing:
  - binder nesting
  - reference patterns
  - reanchors
  - union width
  - mixed support shapes
- widen steps `13-15` by increasing:
  - operator, modal, and temporal mixtures
  - historical reanchors
  - clause unions
  - positional filters
- do the widening through generic mutators, not named late families
- strengthen prefix bounds in parallel so full terminal work stays moderate

Baseline floors for the claim lane:

- use the current demo floors as the minimum baseline in
  `configs/desktop_claim_shadow_10h.toml`
- do not lower them to make the claim easier
- carry the current default-floor expectations for late steps `10-15`:
  - step `10` generated `>= 500`
  - step `11` generated `>= 800`
  - step `12` generated `>= 1200`
  - step `13` generated `>= 2200`
  - step `14` generated `>= 3500`
  - step `15` generated `>= 5000`

Done when:

- step `1` reports `2144`
- steps `1-4` honestly fit the shared early budget on the disclosed machine
- steps `10-15` consistently clear the baseline floors
- `full_telescopes_evaluated` stays moderate

### 6. Replanning Instead Of Time Shifting

Current within-step reserve retuning is still too narrow. The existing
`DemoBudgetFeedback` and retune action set mostly move milliseconds between
discovery and proof-close. The claim lane needs actual closure-aware replanning
that can change the search shape itself.

Files:

- `crates/pen-search/src/engine.rs`

Planned changes:

- extend feedback beyond the current small boolean set into explicit
  bottleneck kinds:
  - `LateFloorMiss`
  - `ClosureDeficit`
  - `ExactScreenBottleneck`
  - `BucketStarvation`
  - `FullEvalPressure`
- extend retune actions:
  - `BorrowFromProofClose`
  - `ReturnToProofClose`
  - `RaiseMutationBreadth`
  - `RaiseReanchorQuota`
  - `ShiftBucketPriority`
  - `TightenPrefixBound`
  - `ReduceFullEvalCap`
  - `PauseProofClose`
- persist the chosen retune action and reason into step artifacts and
  narrative output

Done when:

- the claim lane can change search shape, not just time allocation
- every retune has a stored reason

### 7. Exact-Screen Reason Taxonomy

The repo is already close here, but the claim lane needs a complete, explicit
taxonomy that survives into reports and compare output.

Files:

- `crates/pen-search/src/engine.rs`
- `crates/pen-cli/src/report.rs`
- `scripts/compare_runs.py`

Planned changes:

- add exact-screen reason classes such as:

```rust
enum ExactScreenReasonClass {
    PartialPrefixBarFailure,
    TerminalPrefixCompletionFailure,
    IncumbentDominance,
    ExactLegalityConnectivityFailure,
}
```

- extend stored step stats with:
  - `exact_screen_reason_counts`
  - `prune_class_counts`
- update standard and debug reporting plus step-summary JSON
- update `compare_runs.py` to show exact-screen reason totals the same way it
  already shows floor status and funnel counts

Done when:

- every late step reports:
  - generated
  - hard-admissible
  - exact-screened
  - exact-screen-pruned by reason
  - heuristic-dropped
  - fully-evaluated
- every prune is labeled as sound, quotient/dedupe, or heuristic

### 8. Provenance And Benchmark Hardening

Current run-manifest and compat data are useful, but not yet strong enough for
the paper sentence about a standard desktop configuration. The claim lane
should produce an appendix-ready evidence bundle in one repeatable command.

Files:

- `crates/pen-cli/src/cmd_run.rs`
- new harness:
  - `scripts/bench_claim.py`
  - or `xtask bench-claim`

Planned changes:

- extend the manifest with:
  - CPU model or brand
  - physical core count
  - RAM
  - resolved worker count
  - build profile
  - target triple
  - `target-cpu` if available
  - git commit SHA
  - dirty-tree flag
  - `Cargo.lock` hash
  - binary SHA256
- replace fixed tagged compat hashes with source-derived fingerprints
- add a benchmark harness that repeatedly runs the claim config and writes:
  - median wall time
  - p90 wall time
  - max wall time
  - parity success count
  - floor-hit count
  - manifest snapshot

Done when:

- one command produces an evidence bundle suitable for a paper appendix

### 9. Certification Gate

The README already establishes an honesty boundary for comparison-backed
surfaces. The claim lane should not rely on prose alone. It needs a dedicated
certificate that binds the final wording to stored evidence.

Files:

- `scripts/certify_claim_lane.py`

Checks:

- accepted hashes match guarded
- `guidance_style == claim_debt_guided`
- `late_expansion_policy == claim_generic`
- `bucket_policy == structural_generic`
- no silent guarded or replay fallback
- early breadth gates pass
- late floor gates pass
- runtime threshold passes
- reason-code completeness passes
- manifest completeness passes

Outputs:

- `claim_certificate.json`
- compact human-readable summary

Paper wording rule:

- keep the current safer `bounded live recovery` wording until this script
  passes on the disclosed desktop bundle
- only then move to a sentence like:

> On the disclosed desktop configuration, the PEN claim lane recovers the
> 15-step sequence under a family-agnostic structural guidance policy and
> within the certified runtime bound.

Done when:

- the certification script passes on the intended claim configuration
- the paper wording is tied to that stored certificate instead of free prose

## PR-Sized Delivery Plan

Implement the work in five chunks:

1. Profile plumbing only
   Add `DesktopClaimShadow`, configs, narrative/config plumbing, and telemetry
   flags with no intended behavioral change.
2. Admissibility split
   Add `ClaimDebtAxes` and route only claim mode through the family-agnostic
   admissibility path.
3. Generic mutators
   Add `claim_mutators.rs` and remove named relaxed, family, and demo clause
   builders from the claim path.
4. Scheduler, breadth, and replanning
   Add structural buckets, widen honest search surfaces, and make retuning able
   to change search shape.
5. Reporting, provenance, and certification
   Add exact-screen reasons, benchmark harness, manifest hardening, and
   `certify_claim_lane.py`.

## Review Checklist For Every Claim-Lane PR

- Does this change preserve existing behavior for the guarded, realistic, and
  demo lanes?
- Does the claim lane avoid named-family progression where the new code path
  is supposed to be family-agnostic?
- Are stored artifacts explicit about which claim policies were used?
- Does the widening remain honest in stored counts, not just in config intent?
- Did `full_telescopes_evaluated` stay moderate while floors improved?
- Can compare/certify tooling detect failure modes instead of silently masking
  them?

## Out Of Scope Until Certification

- broad renaming of every `Demo*` type or helper
- stronger user-facing language such as `unguided`
- any paper claim that outruns the stored certification bundle
- lowering floors or hiding misses to make the claim look cleaner

## Immediate Next Step

Start with the profile split only. Create `DesktopClaimShadow`, make claim-run
metadata explicit, and keep behavior intentionally close to the current demo or
realistic shadow path until the claim lane has its own admissibility,
enumeration, scheduler, and certification story.
