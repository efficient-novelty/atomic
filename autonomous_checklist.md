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
- A 2026-03-22 claim smoke rerun reached step `4` and recorded about
  `3.30 GiB` observed RSS after `14.9s` with `2775` frontier groups,
  `5550` legality summaries, `5084` partial-prefix-bound entries, and
  `0` retained prefix-cache groups, so the early spike is still in
  discovery/frontier/legality residency before proof-close on that partial run.
- A follow-up 2026-03-22 smoke rerun (`codex-claim-shared-signature-v1`)
  shared cloned prefix-signature payloads across the frontier and
  legality-cache maps, but the comparable early step-`4` checkpoint still hit
  about `3.06 GiB` observed RSS after `13.2s`, only about `6.6 MiB` below the
  prior comparable checkpoint, so the main discovery/frontier/legality spike
  remains open beyond duplicated signature-key storage.
- A newer 2026-03-22 smoke rerun (`codex-claim-frontier-catalog-reuse-v1`)
  reused the shared clause catalog for unfiltered queued claim frontier items;
  the old `13.2s` / `3.06 GiB` startup checkpoint disappeared, and the first
  stored step-`4` frontier-progress checkpoint landed at about `66.4 MiB`
  observed RSS after `422.9s` with `2774` frontier groups, `10193` legality
  summaries, `5084` partial-prefix-bound entries, and `13` retained
  prefix-cache groups, so the dominant early queue-side spike was
  substantially reduced even though the full-profile lane still lacks a
  completed bundle.
- A 2026-03-22 optimized step-5-limited rerun (`codex-claim-release-step5-v1`)
  kept step `4` under about `167.1 MiB` observed RSS after `1777.1s` while
  enumerating `310916028` candidates and exploring `16` prefix states, so the
  release build is no longer showing an early allocator cliff on step `4`; the
  bottleneck there is now exact remaining-two throughput.
- A follow-up optimized step-`4` rerun after the new compact claim
  materialization fast path (`codex-claim-release-step4-fastpath-v2`) reached
  the same hot discovery checkpoints about `12-14%` sooner than
  `codex-claim-release-step5-v1` while keeping observed RSS below about
  `89.6 MiB`, but the intended full profile still has no stored completion
  bundle yet.
- A newer 2026-03-22 optimized rerun with slice-based terminal clause
  filtering (`codex-claim-release-filter-slice-v1a`) reached those same hot
  step-`4` checkpoints another `18-20%` sooner than
  `codex-claim-release-step4-fastpath-v2`:
  - `prefix_states_explored = 5` landed at `421.9s` versus `515.6s`
  - `prefix_states_explored = 7` landed at `564.1s` versus `701.1s`
  - observed RSS stayed below about `84.0 MiB` through prefix state `7`
  - the intended full profile still lacks a stored completion bundle, so the
    next gate is a full rerun on this newer binary rather than another
    speculative step-`4` rewrite
- A follow-up 2026-03-22 intended-profile rerun
  (`codex-claim-release-full-v1a`) used that newer release binary on
  `configs/desktop_claim_shadow_1h.toml` and did not re-hit the old allocator
  abort before an external shell timeout after `3844.7s`:
  - step `4` had explored `43` prefix states
  - it had enumerated `848047359` candidates
  - frontier queue length was still `2732`
  - legality summaries had risen to `205199`
  - prefix-cache groups were still only `39`
  - observed RSS was about `278.2 MiB`
  - the blocker on the newer binary is therefore still step-`4` throughput
    and frontier drainage, not the old early RSS cliff
- The same `codex-claim-release-full-v1a` step-live stream also showed the
  retained prefix cache flattening after `prefix_states_explored = 24`:
  later checkpoints stayed at `39` groups / `144845` retained candidates while
  legality summaries kept climbing from `140197` to `205199`, so a meaningful
  share of the remaining step-`4` cost is still exact terminal completion on
  surfaces that are no longer adding new retained groups.
- A follow-up 2026-03-22 throughput pass now reuses one scratch terminal
  telescope and the precomputed prefix bit cost across claim exact
  remaining-two bound checks, completion summaries, and compact
  materialization, but no comparable full-profile rerun exists yet to show how
  much wall-clock that removes on the disclosed machine.
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

## 1. Runtime Stability On The Intended Claim Profile

- [ ] Capture and review the observed process RSS versus governor-accounted RSS
      gap from a stored `desktop_claim_shadow` run now that claim artifacts
      persist both numbers.
- [ ] Verify that the new memory-aware auto-worker cap is sufficient for the
      intended `desktop_claim_shadow_1h` profile on the disclosed machine.
- [ ] Verify that claim-lane proof-close and materialization compaction remain
      sufficient on the real full profile; the optimized step-`4` probes no
      longer show the old allocator cliff, but later steps still need a stored
      rerun to prove that they stay inside the same honest memory envelope.
- [ ] Use the new `step_live_checkpoint` telemetry plus
      `reports/steps/step-XX-live.ndjson` artifacts to pinpoint whether step-4
      and step-5 claim growth is coming from raw catalog expansion,
      legality-cache residency, prefix-cache residency, or proof-close queue
      buildup; the 2026-03-22 `codex-claim-frontier-catalog-reuse-v1` smoke
      rerun now suggests the old step-`4` startup cliff was dominated by
      frontier queue residency that cloned the full next-clause catalog into
      each queued item, but step `5` and the intended full profile still need
      stored evidence that isolates any remaining legality/raw-surface or
      later-step residency honestly.
- [ ] Continue reducing step-`4` exact remaining-two runtime on the optimized
      claim binary; the latest release probe is throughput-bound there even
      after the direct compact claim materialization fast path and the new
      slice-based terminal-clause filtering path; the first intended-profile
      rerun on that binary still timed out in step `4` after exploring `43`
      prefix states, and its retained-prefix plateau after prefix state `24`
      suggests later terminal surfaces are still being processed too
      expensively even after the new scratch-telescope reuse pass.
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
