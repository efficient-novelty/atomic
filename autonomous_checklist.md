# Autonomous Claim Lane Checklist

Last updated: 2026-03-21

This checklist is the signoff gate for the stronger sentence that the PEN
axioms discover the current 15-step sequence `unguided` and `autonomously` on
a disclosed desktop configuration.

Use this file as a completion checklist, not as a history log. Items should
remain unchecked until the condition is true from stored evidence on the claim
lane itself.

## Exit Rule

Do not use `unguided`, `autonomous recovery`, or stronger paper wording until
every section below is closed and a passing claim certificate exists.

## Current Open Numbers

- No stored claim-lane signoff bundle yet.
- `guidance_style`, `late_expansion_policy`, and `bucket_policy` are now
  claim-specific:
  - `late_expansion_policy = claim_generic`
  - `bucket_policy = structural_generic`
- `kappa 7-9` now use claim-specific later-band mutator packs in code/tests.
- Accepted-hash parity through step `15` and stored breadth evidence are still
  open claim-lane gates.
- Minimum breadth floors that must be earned honestly on the claim lane:
  - step `1` generated raw `= 2144`
  - step `10` generated `>= 500`
  - step `11` generated `>= 800`
  - step `12` generated `>= 1200`
  - step `13` generated `>= 2200`
  - step `14` generated `>= 3500`
  - step `15` generated `>= 5000`

## 1. Claim-Generic Late Expansion

- [x] Add a claim-specific late expansion policy in
      `crates/pen-search/src/enumerate.rs`.
- [x] Stop mapping `DesktopClaimShadow` to
      `LateFamilySurface::RealisticShadow` in
      `EnumerationContext::from_admissibility()`.
- [x] Stop mapping claim mode to realistic late-family behavior in
      `crates/pen-search/src/prefix_memo.rs`.
- [x] Add claim-generic mutators for kappa `4` bridge, reanchor, and
      modal-lift variants.
- [x] Add claim-generic mutators for kappa `5-6` support-form, bridge-head,
      and shell-shape variants.
- [x] Add claim-generic mutators for kappa `7` operator-style wrapper and
      shell-mixture variants.
- [x] Add claim-generic mutators for kappa `8` modal-temporal exchange and
      history-reanchor variants.
- [x] Add claim-generic mutators for kappa `9` higher-order and binder-heavy
      variants.
- [ ] Ensure the claim path does not call:
      `relaxed_axiomatic_bridge_clause()`,
      `relaxed_modal_shell_clause()`,
      `relaxed_connection_shell_clause()`,
      `relaxed_curvature_shell_clause()`,
      `operator_bundle_family_clauses()`,
      `hilbert_functional_family_clauses()`,
      `temporal_shell_family_clauses()`,
      or any `demo_*_clauses()` helpers.
- [x] Add tests proving the claim lane no longer reaches realistic late-family
      helpers.
- [x] Add tests proving the claim lane no longer reaches demo late-family
      helpers.
- [ ] Preserve accepted-hash parity through step `15` while switching the claim
      late path.
- [x] Switch `late_expansion_policy` to `claim_generic` only after the code
      path is genuinely claim-specific.

Done when:

- claim enumeration and prefix filtering are no longer inherited from realistic
  shadow
- claim runs still match the guarded accepted hashes through step `15`
- stored metadata can honestly say `late_expansion_policy = claim_generic`

## 2. Claim-Specific Prefix Filtering And Exact Completion

- [x] Recheck prefix-summary pruning logic under the claim-generic mutator
      surface.
- [x] Confirm terminal-prefix completion summaries still match direct exact
      assessment under the claim policy.
- [x] Confirm cached prefix summaries remain deterministic and parity-safe
      after removing realistic late-family inheritance.
- [ ] Persist any new claim-specific exact-screen reasons or prefix-prune
      distinctions needed to explain the new late path.

Done when:

- prefix filtering remains exact and deterministic on the claim path
- parity is preserved without relying on realistic-family assumptions

## 3. Structural Bucket Scheduling

- [x] Define a neutral claim bucket taxonomy from runtime-local structural
      features only.
- [x] Replace semantic-family bucket labels for claim runs in
      `crates/pen-search/src/engine.rs`.
- [x] Ensure claim bucket keys can be computed from syntax, prefix-local
      evidence, and local runtime state only.
- [x] Keep existing bucket behavior unchanged for guarded, realistic, and demo
      lanes.
- [x] Update stored claim artifacts so they no longer serialize semantic-family
      names such as `TemporalShell`, `Hilbert`, `Curvature`, or
      `Differential`.
- [ ] Update any reporting or compare logic that currently assumes the old
      semantic-family bucket names for claim runs.
- [x] Switch `bucket_policy` to `structural_generic` only after the live
      scheduler and stored evidence both use the neutral taxonomy.

Done when:

- claim scheduler order can be explained entirely from runtime-local structural
  evidence
- claim artifacts no longer serialize semantic-family bucket names
- stored metadata can honestly say `bucket_policy = structural_generic`

## 4. Honest Breadth And Search Mass

- [ ] Restore claim-lane step `1` raw generated count to exactly `2144`.
- [ ] Reconfirm that claim-lane steps `1-4` fit honestly inside the shared
      early budget on the disclosed machine.
- [ ] Ensure early breadth claims come from stored generated counts, not from
      config intent.
- [ ] Raise claim-lane step `10` generated count to at least `500`.
- [ ] Raise claim-lane step `11` generated count to at least `800`.
- [ ] Raise claim-lane step `12` generated count to at least `1200`.
- [ ] Raise claim-lane step `13` generated count to at least `2200`.
- [ ] Raise claim-lane step `14` generated count to at least `3500`.
- [ ] Raise claim-lane step `15` generated count to at least `5000`.
- [ ] Preserve accepted-hash parity through step `15` while closing the late
      floors.
- [ ] Keep `full_telescopes_evaluated` within a certified moderate threshold
      while closing those floors.
- [ ] Ensure widened breadth comes from claim-generic mutators, not from named
      realistic or demo family surfaces.

Done when:

- early and late breadth floors are closed from stored claim-lane evidence
- accepted parity still holds
- the claim is not being carried by inflated full evaluation counts

## 5. Replanning Instead Of Time Shifting

- [ ] Extend within-step feedback so it can express real search bottlenecks,
      not just timing pressure.
- [ ] Add retune actions that can change search shape, including mutation
      breadth, reanchor quotas, bucket priority, prefix bounds, and full-eval
      caps.
- [ ] Persist the chosen retune action and reason into step artifacts.
- [ ] Surface the same retune action and reason in narrative/debug reporting.
- [ ] Demonstrate on stored claim runs that replanning can change the search
      shape itself instead of only moving time between phases.

Done when:

- claim-lane retuning is inspectably shape-changing
- every retune has a stored reason and action record

## 6. Reporting, Reason Codes, And Artifact Honesty

- [ ] Persist complete `exact_screen_reason_counts` for claim runs.
- [ ] Persist complete `prune_class_counts` for claim runs.
- [ ] Ensure late-step summaries expose:
      generated,
      hard-admissible,
      exact-screened,
      exact-screen-pruned by reason,
      heuristic-dropped,
      and fully-evaluated totals.
- [ ] Ensure every prune is labeled as sound, quotient/dedupe, or heuristic.
- [ ] Ensure `run.json`, `telemetry.ndjson`, and step narratives record the
      actual claim policies used.
- [ ] Ensure claim artifacts make any guarded, replay, realistic-shadow, or
      demo-only fallback impossible to miss.
- [x] Update `scripts/compare_runs.py` so claim runs are checked for exact
      reason completeness and policy honesty, not just parity.

Done when:

- a reviewer can inspect the stored claim bundle and see exactly why candidates
  were pruned and which live policies were used

## 7. Provenance And Benchmark Appendix

- [ ] Extend the run manifest with CPU model or brand.
- [ ] Extend the run manifest with physical core count.
- [ ] Extend the run manifest with RAM.
- [ ] Extend the run manifest with resolved worker count.
- [ ] Extend the run manifest with build profile.
- [ ] Extend the run manifest with target triple.
- [ ] Extend the run manifest with `target-cpu` when available.
- [ ] Extend the run manifest with git commit SHA.
- [ ] Extend the run manifest with dirty-tree flag.
- [ ] Extend the run manifest with `Cargo.lock` hash.
- [ ] Extend the run manifest with binary SHA256.
- [ ] Replace any fixed compat tags used for the claim appendix with
      source-derived fingerprints.
- [ ] Add a repeatable benchmark harness for the intended claim config.
- [ ] Record benchmark median wall time.
- [ ] Record benchmark p90 wall time.
- [ ] Record benchmark max wall time.
- [ ] Record parity success count.
- [ ] Record floor-hit count.
- [ ] Record manifest snapshot alongside the benchmark outputs.
- [ ] Freeze a certified runtime threshold for the claim sentence.

Done when:

- one command produces an appendix-ready benchmark and provenance bundle on the
  disclosed machine

## 8. Certification Gate

- [x] Add `scripts/certify_claim_lane.py`.
- [x] Check accepted hashes match guarded through step `15`.
- [x] Check `guidance_style == claim_debt_guided`.
- [x] Check `late_expansion_policy == claim_generic`.
- [x] Check `bucket_policy == structural_generic`.
- [ ] Check the claim path used no silent guarded, replay, realistic-shadow, or
      demo-only fallback.
- [x] Check early breadth gates from stored evidence.
- [x] Check late generated floors from stored evidence.
- [x] Check the certified runtime threshold.
- [x] Check exact-screen reason completeness.
- [x] Check prune-class completeness.
- [x] Check manifest completeness.
- [x] Emit `claim_certificate.json`.
- [x] Emit a compact human-readable certification summary.
- [ ] Pass the certification gate on the intended claim-lane evidence bundle.

Done when:

- the certification script passes without manual caveats on the disclosed
  desktop bundle

## 9. Final Signoff Bundle

- [ ] Store one canonical claim-lane run directory from the disclosed desktop
      configuration.
- [ ] Store one compare report against the guarded reference for that run.
- [ ] Store one benchmark bundle for that same configuration.
- [ ] Store one passing `claim_certificate.json`.
- [ ] Link the canonical claim bundle from the repo-level autonomy docs.

Done when:

- another reviewer can open one stable claim bundle and audit the whole claim
  end to end

## 10. Language Gate

- [ ] Keep user-facing and paper-facing wording at `bounded live recovery`
      until every section above is closed.
- [ ] Update the paper sentence only after the certification gate passes.
- [ ] Do not use the word `unguided` anywhere user-facing before the passing
      certificate exists.
- [ ] Ensure the stronger sentence is explicitly tied to the stored claim
      certificate and disclosed desktop bundle.

Done when:

- the stronger claim appears only after the technical and evidence gates have
  already been satisfied
