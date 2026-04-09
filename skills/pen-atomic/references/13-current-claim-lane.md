# Current Claim Lane

Read this file when the task touches `desktop_claim_shadow`, claim-lane
telemetry, claim-lane narratives, or the autonomy-certification roadmap.

## Contract

- This file owns stable claim-lane context, vocabulary, and invariants.
- Do not store live run IDs, current blocker counts, or the active probe here.
- Use [../../autonomous_progress.md](../../autonomous_progress.md) for the live
  snapshot, [../../autonomous_next_steps.md](../../autonomous_next_steps.md)
  for the active work order, [../../autonomous_plan.md](../../autonomous_plan.md)
  for phases, [../../autonomous_checklist.md](../../autonomous_checklist.md)
  for binary gates, and [../../autonomous_ledger.md](../../autonomous_ledger.md)
  for experiment history.

## Stable Lane Truths

- `desktop_claim_shadow` exists as a separate profile and config family.
- The lane is still pre-certification and should stay at the safer
  `bounded live recovery` wording until a stored certificate passes.
- Claim policy metadata is already real and should stay honest:
  - `guidance_style = claim_debt_guided`
  - `late_expansion_policy = claim_generic`
  - `bucket_policy = structural_generic`
- Claim-specific compare, benchmark, and certification tooling are the
  official evidence surfaces for signoff:
  - `scripts/compare_runs.py`
  - `scripts/benchmark_claim_lane.py`
  - `scripts/certify_claim_lane.py`
- Claim runs persist durable evidence incrementally:
  - `run.json` with build/git/binary fingerprints
  - step summaries and step checkpoints
  - frontier snapshots
  - narratives and events
  - `step_live_checkpoint` telemetry and `reports/steps/step-XX-live.ndjson`
- Claim proof-close and materialization already include the landed memory
  behavior:
  - evaluated terminal payloads drop after ranking
  - processed retained prefix groups release once certification starts
  - legality-cache completion summaries are reused
  - uncached compact materialization is direct rather than rebuild-and-rewalk
  - frontier items reuse the shared clause catalog and serialized prefix key
- Claim-lane work must stay separate from demo-only behavior even when probes
  temporarily reuse demo-family clauses under test-only overrides.

## Stable Working Invariants

- Prefer stored evidence over terminal impressions.
- Keep the accepted path fixed until stored evidence clearly replaces it.
- Keep step `15` local repair ahead of a step-`1` theory pass unless a newer
  rerun changes the diagnosis.
- Do not use stronger wording such as `unguided` before certification passes.
- Do not treat the lane as family-agnostic end to end while stored breadth is
  still open.
- Keep stronger-than-canonical lifted terminals fenced unless new stored
  evidence explicitly proves they are part of the canonical path.
- Keep PowerShell assumptions explicit in workflow notes and avoid POSIX-style
  command chaining in examples.

## Operational Memory Map

- [../../autonomous_progress.md](../../autonomous_progress.md)
  Live operational snapshot: canonical bundle, current counters, current
  diagnosis.
- [../../autonomous_next_steps.md](../../autonomous_next_steps.md)
  Single active work order for the next slice.
- [../../autonomous_plan.md](../../autonomous_plan.md)
  Medium-horizon phases and exit criteria.
- [../../autonomous_checklist.md](../../autonomous_checklist.md)
  Binary signoff gates only.
- [../../autonomous_ledger.md](../../autonomous_ledger.md)
  Append-only probe history and decisions.

## First Reads

- [../../docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [../../autonomous_progress.md](../../autonomous_progress.md)
- [../../autonomous_next_steps.md](../../autonomous_next_steps.md)
- [../../autonomous_plan.md](../../autonomous_plan.md)
- [../../autonomous_checklist.md](../../autonomous_checklist.md)
- [../../autonomous_ledger.md](../../autonomous_ledger.md)
- [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)

## Evidence Surfaces

Use these before changing search code:

- `scripts/certify_claim_lane.py` for stored pass/fail plus failing-step
  breadth anatomy
- `scripts/compare_runs.py` for parity and artifact honesty
- `scripts/benchmark_claim_lane.py` for stored runtime and breadth summaries
- `reports/steps/step-15-live.ndjson` when the task touches late-step pressure
- `run.json` and step summaries when the task touches stored provenance

## Do And Do Not

Do:

- keep the claim lane separate from demo-only behavior
- preserve existing guarded, realistic, and demo behavior while the claim lane
  changes
- keep claim-lane edits narrow and staged
- use the certificate first when you need stored breadth anatomy
- use the autonomous files as intended instead of restating live state here

Do not:

- restate live counters in this file
- treat negative controls as if they were still open hypotheses
- reopen or re-summarize the exhausted mismatch-`1`
  `reference + demo_flat_codomain` ladder first; the live off-branch priority
  order now belongs in the autonomous docs rather than in this stable
  reference
- reopen the exact claim-pair clause-`4` `reference` side first; that narrower
  relocalization now only reproduces the older broader clause-`4`
  `reference`-sheet tradeoff and belongs in the autonomy ledger rather than as
  a fresh lead
- split the exact claim-pair clause-`4` `claim_next_bridge` half into the
  claim-flat or claim-sharp single-sheet relocalizations first; those smaller
  tradeoff controls now also belong in the autonomy ledger rather than as a
  fresh lead
- reopen the broad clause-`1` `demo_flat_codomain` reopening across the full
  mismatch-`0` claim-domain surface first; that control is now a widening
  negative result owned by the autonomy ledger rather than a fresh lead
- reopen the narrower clause-`4` `claim_next_bridge`-side relocalization under
  that mismatch-`0` surface first; that smaller negative result also belongs in
  the autonomy ledger rather than as a fresh lead
- reopen the narrower clause-`4` `reference`-side relocalization under that
  same mismatch-`0` surface first; that sharper negative result now also
  belongs in the autonomy ledger rather than as a fresh lead
- reopen remaining-one exact-summary relief on the mismatch-`0` clause-`4`
  `reference` plus clause-`5` `reference` tail first; that deeper negative
  control now also belongs in the autonomy ledger rather than as a fresh lead
- reopen the whole mismatch-`0` clause-`4` `claim_next_bridge`-half
  remaining-one exact-summary relief as if it were already the landed repair;
  that wall-narrowing tradeoff still belongs in the autonomy docs and ledger
  rather than as a stable resolved lead
- reopen the whole-cell mismatch-`0` clause-`4` `claim_next_bridge` plus
  clause-`5` `claim_flat_codomain` or clause-`5` `reference` exact-summary
  tradeoffs first; those smaller tradeoff controls now also belong in the
  autonomy ledger rather than as stable resolved leads
- reopen the pair-cell subprobes below those active clause-`5`
  `claim_flat_codomain / reference` cells first; those deeper relands are now
  also symmetric smaller tradeoff controls owned by the autonomy ledger rather
  than fresh leads
- reopen the representative pair-cell clause-`2` identity split first; those
  two claim-side clause-`2` sheets are now matching smaller tradeoff controls
  while the sibling reference sheet is neutral, so that axis also belongs to
  the autonomy ledger rather than as a fresh lead
- reopen the representative `claim_flat_domain` clause-`2` sheet's clause-`6`
  identity split first; those `claim_next_codomain`,
  `claim_sharp_codomain`, and `reference` continuations are now matching
  smaller tradeoff controls too, so that axis also belongs to the autonomy
  ledger rather than as a fresh lead
- reopen the representative `claim_flat_domain` clause-`2` sheet's
  marginally best clause-`6` `reference` continuation through either
  individual clause-`3` `claim_flat_argument` or `claim_eventual_argument`
  branch first; those branches are now individually neutral controls on the
  untouched `4331 / 553 / 2271` baseline, and the broader
  `4343 / 552 / 2268` clause-`6` `reference` tradeoff only appears when both
  reopen together, so that axis also belongs to the autonomy ledger rather
  than as a fresh lead
- reopen that whole joint clause-`3` continuation first either; that broader
  clause-`6` `reference` tradeoff is now localized to one remaining-two
  parent capture plus its three clause-`6` remaining-one child
  continuations, so the live autonomy docs should own the finer completion
  partition beneath that parent/child shell rather than this stable reference
- reopen that representative claim-flat parent/child shell through another
  clause-`3` / clause-`6` child-identity pass first either; those six child
  continuations now only expose matched dead `3`-generated / `0`-admitted
  completion summaries with the same nonlive open-band structural terminal
  trio, so route to the live autonomy docs before spending another turn on the
  exhausted claim-flat shell
- reopen the representative `claim_sharp_codomain` clause-`2` sheet's
  clause-`6` identity split first either; its `claim_next_codomain`,
  `claim_sharp_codomain`, and `reference` continuations now also reland the
  same matched smaller tradeoff shell and differ only by a tiny deeper
  zero-admitted tail delta, so that axis also belongs to the autonomy ledger
  rather than as a fresh lead
- reopen the representative claim-safe mismatch-`1` clause-`4`
  `demo_sharp_codomain` or `demo_sharp_bridge` side on either exact
  claim-safe pair first; those `reference / claim_next_codomain` and
  `reference / claim_sharp_codomain` relands now belong to the autonomy
  ledger as matching smaller negative controls rather than as fresh leads
- reopen the representative claim-safe exact-pair clause-`2` split first
  either; on the chosen `reference / claim_next_codomain / demo_sharp_codomain`
  cell, the two claim-side sheets are matched smaller controls while the
  sibling `reference` sheet is only a search-neutral control, so that axis
  also belongs to the autonomy ledger rather than as a fresh lead
- keep the next slice above another mismatch-`0` claim-side identity reland;
  with both representative claim-side sheets now frozen through clause-`6`
  identity, route to the live autonomy docs before spending another turn
  below the mismatch-`0` representative pair unless a reason-level
  connectivity partition under one frozen shell is explicitly promoted
- reopen the mismatch-`0` clause-`4` `claim_next_bridge` plus clause-`5`
  `claim_next_codomain` exact-summary cell first; that sibling is now a
  neutral control owned by the autonomy ledger rather than a fresh lead
- reopen the whole remaining-two mismatch-`0` claim-domain tier by flipping
  parent-level `CannotClearBar` decisions to `Unknown`; that broad exact-bound
  release is now also a ruled-out widening control owned by the autonomy
  ledger rather than a safe local repair
- reopen runtime-only step-`4` throughput work first unless a newer rerun
  proves the remaining misses are runtime fallout
- reland ruled-out clause-`4` / clause-`5` or same-primary probes without new
  evidence
- claim certification or stronger language before the stored certificate says
  so
