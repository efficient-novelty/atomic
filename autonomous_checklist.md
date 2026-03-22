# Autonomous Claim Lane Checklist

Last updated: 2026-03-22

This checklist is the live signoff gate for the stronger sentence that the PEN
axioms discover the current 15-step sequence on the disclosed desktop
configuration.

This file intentionally lists only open work. Completed historical rollout
items belong in baseline docs, not here.

## Exit Rule

Do not use `unguided`, `autonomous recovery`, or stronger paper wording until
every section below is closed and a passing claim certificate exists.

## Current Open Numbers

- No stored full-profile claim-lane signoff bundle yet.
- The intended `desktop_claim_shadow_1h` auto-worker run still aborts before
  step-15 completion with `memory allocation of 1212416 bytes failed`.
- Failed long runs now leave auditable `run.json`, step, checkpoint, frontier,
  and claim-narrative artifacts; the open issue is finishing the full profile,
  not preserving the failure bundle.
- Claim step artifacts now persist observed process RSS and the gap versus the
  governor-accounted RSS model, but no full-profile stored claim run exists yet
  to interpret those numbers on the disclosed machine.
- Claim terminal-prefix materialization now drops duplicated exact completion
  payloads from the legality cache after reuse, but no full-profile stored
  claim run exists yet to show whether that closes enough of the live RSS gap
  on the disclosed machine.
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

## 1. Memory Stability On The Intended Claim Profile

- [ ] Capture and review the observed process RSS versus governor-accounted RSS
      gap from a stored `desktop_claim_shadow` run now that claim artifacts
      persist both numbers.
- [ ] Verify that the new memory-aware auto-worker cap is sufficient for the
      intended `desktop_claim_shadow_1h` profile on the disclosed machine.
- [ ] Verify that claim-lane proof-close cache compaction removes the remaining
      live allocation spike; claim materialization now also drops duplicated
      legality-cache terminal payloads after reuse, but the intended profile
      still needs a stored rerun to prove that the combined compaction is
      sufficient.
- [ ] Use the new `step_live_checkpoint` telemetry plus
      `reports/steps/step-XX-live.ndjson` artifacts to pinpoint whether step-4
      and step-5 claim growth is coming from raw catalog expansion,
      legality-cache residency, prefix-cache residency, or proof-close queue
      buildup.
- [ ] Reduce or cap any remaining worker scratch/frontier residency enough for
      the intended `desktop_claim_shadow_1h` profile to complete on the
      disclosed machine.
- [ ] Spill or compact earlier again if the new claim-only memory controls are
      still insufficient before the run reaches step-15 completion.
- [ ] Eliminate the `memory allocation of 1212416 bytes failed` abort on the
      disclosed desktop under the intended full profile.

Done when:

- the intended full-profile claim run completes without allocator abort
- pressure/governor evidence explains the observed memory behavior honestly

## 2. Stored Claim Evidence

- [ ] Preserve accepted-hash parity through step `15` on the stabilized claim
      lane.
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
- [ ] Keep `full_telescopes_evaluated` within a certified moderate threshold
      while closing those floors.
- [ ] Persist any remaining claim-specific exact-screen reasons or prefix-prune
      distinctions needed to explain the live lane.
- [ ] Persist complete `exact_screen_reason_counts` for claim runs.
- [ ] Persist complete `prune_class_counts` for claim runs.
- [ ] Ensure late-step summaries expose generated, hard-admissible,
      exact-screened, exact-screen-pruned by reason, heuristic-dropped, and
      fully-evaluated totals.
- [ ] Ensure every prune is labeled as sound, quotient/dedupe, or heuristic.
- [ ] Ensure claim artifacts make guarded, replay, realistic-shadow, or
      demo-only fallback impossible to miss.

Done when:

- parity and breadth both pass from stored claim-lane evidence
- a reviewer can inspect the stored bundle and understand what the lane did

## 3. Benchmark And Certification Bundle

- [ ] Check the claim path used no silent guarded, replay, realistic-shadow, or
      demo-only fallback on the intended stored bundle.
- [x] Add a repeatable benchmark harness for the intended claim config
      (`scripts/benchmark_claim_lane.py`).
- [ ] Record benchmark median wall time.
- [ ] Record benchmark p90 wall time.
- [ ] Record benchmark max wall time.
- [ ] Record parity success count.
- [ ] Record floor-hit count.
- [ ] Record manifest snapshot alongside the benchmark outputs.
- [ ] Freeze a certified runtime threshold for the claim sentence.
- [ ] Pass the certification gate on the intended claim-lane evidence bundle.
- [ ] Store one canonical claim-lane run directory from the disclosed desktop
      configuration.
- [ ] Store one compare report against the guarded reference for that run.
- [ ] Store one benchmark bundle for that same configuration.
- [ ] Store one passing `claim_certificate.json`.
- [ ] Link the canonical claim bundle from the repo-level autonomy docs.

Done when:

- another reviewer can open one stable claim bundle and audit the whole claim
  end to end

## 4. Language Gate

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
