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
- The current post-probe canonical rerun has already refreshed compare,
  benchmark, and certification on newer code and re-confirmed that step `15`,
  not step `1`, is the first stored breadth miss.

## Stable Working Invariants

- Prefer stored evidence over terminal impressions.
- Keep the accepted path fixed until stored evidence clearly replaces it.
- The post-probe rerun has now reconfirmed the same step-`15` miss on newer
  code, so keep step-`15` repair work ahead of step-`1` theory work unless a
  later stored bundle changes the diagnosis.
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
- `run.json` and step summaries when the task touches stored provenance or the
  ordering between a rerun-backed step-`15` reset and a later step-`1`
  reopening

## Do And Do Not

Do:

- keep the claim lane separate from demo-only behavior
- preserve existing guarded, realistic, and demo behavior while the claim lane
  changes
- keep claim-lane edits narrow and staged
- use the certificate first when you need stored breadth anatomy
- compare the earlier and current canonical reruns plus `run.json` build
  fingerprints before reopening step `1`, and if the newer rerun reconfirms
  the same breadth-only miss, keep step-`15` repair ahead of step `1`
- use the autonomous files as intended instead of restating live state here

Do not:

- restate live counters in this file
- treat negative controls as if they were still open hypotheses
- spend another turn on rerun-vs-step-`1` ordering after the current
  canonical rerun has already reconfirmed the same breadth-only miss on newer
  code
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
- reopen the sibling active mismatch-`0` clause-`5` `reference` family's
  representative pair-cell clause-`2` identity split first; if the live docs
  already localize it to the same `4343 / 552 / 2268` smaller tradeoff shell
  on the two claim-side sheets with the sibling reference sheet neutral at
  `4331 / 553 / 2271`, that axis also belongs to the autonomy ledger rather
  than as a fresh lead
- reopen the representative `claim_flat_domain` clause-`2` sheet's clause-`6`
  identity split first; those `claim_next_codomain`,
  `claim_sharp_codomain`, and `reference` continuations are now matching
  smaller tradeoff controls too, so that axis also belongs to the autonomy
  ledger rather than as a fresh lead
- reopen the deeper representative mismatch-`0` claim-side clause-`6`
  `reference` union first either; if the live docs already pin that
  cross-sheet union at the same `4355 / 551 / 2265` smaller pair-cell
  tradeoff with `small_cluster = 3150 / 522 / 522 / 0`, the same
  zero-admitted `((0, None, None), 2265)` remaining-one family, the full
  `+18` `small_cluster` widening localized to six released
  `3`-generated / `0`-admitted `NeedsFallback` groups, no cached bound, and
  no hidden live pocket, that union also belongs to the autonomy ledger
  rather than as a fresh lead
- reopen the representative mismatch-`0` claim-side parent-route first
  either; if the live docs already pin both scoped historical-reanchor routes
  on the active clause-`5` `claim_flat_codomain / reference` families at the
  same unsafe `4427 / 545 / 2247` negative-control shell with noncanonical
  `60 / 8`, `incumbent_dominance = 117`,
  `small_cluster = 2931 / 455 / 455 / 115`, a reopened `single` bucket, and
  a delta localized only to the four targeted claim-side remaining-two parent
  cells plus their `24` matching remaining-one pruned prefixes on the chosen
  active clause-`5` bucket, those parent-route probes also belong to the
  autonomy ledger rather than as fresh repair leads
- reopen the first alternate representative mismatch-`0` claim-side
  active-window qualification family first either; if the live docs already
  pin both active clause-`5` `claim_flat_codomain / reference` probes at the
  same unsafe `4427 / 545 / 2247` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same four targeted remaining-two parent cells plus their
  `24` matching remaining-one pruned prefixes, that alternate qualification
  family also belongs to the autonomy ledger rather than as a fresh repair
  lead
- reopen the next alternate representative mismatch-`0` claim-side
  self-contained qualification family first either; if the live docs already
  pin both active clause-`5` `claim_flat_codomain / reference` probes at the
  same unsafe `4427 / 545 / 2247` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same four targeted remaining-two parent cells plus their
  `24` matching remaining-one pruned prefixes, that alternate qualification
  family also belongs to the autonomy ledger rather than as a fresh repair
  lead
- reopen the narrower clause-`6` `reference` refinement of those same
  alternate active-window or self-contained families first either; if the
  live docs already pin `claim_flat_codomain` at unsafe noncanonical `60 / 8`
  with `retained = 4`, `incumbent_dominance = 113`,
  `small_cluster = 2904 / 462 / 462 / 109 / 2`, and pin `reference` at the
  same unsafe `60 / 8` with `retained = 2`,
  `incumbent_dominance = 115`, the same
  `2904 / 462 / 462 / 109 / 2` `small_cluster`, no `single` bucket, best
  overshoot `545 / 5278`, and the same four targeted remaining-two parent
  cells plus their `24` matching remaining-one pruned prefixes, those
  narrower alternate refinements also belong to the autonomy ledger rather
  than as fresh repair leads
- reopen the narrower clause-`6` `reference` refinement of that same
  representative mismatch-`0` claim-side parent-route first either; if the
  live docs already pin both active clause-`5`
  `claim_flat_codomain / reference` refinements at unsafe noncanonical
  `74 / 8` with `retained = 1`, the same `4427 / 545 / 2247`,
  `incumbent_dominance = 111`,
  `small_cluster = 2904 / 430 / 430 / 108`, a still-fenced `single` bucket,
  and the same four targeted remaining-two parent cells plus their `24`
  matching remaining-one pruned prefixes, that narrower refinement also
  belongs to the autonomy ledger rather than as a fresh repair lead
- reopen the representative claim-flat clause-`3` refinement inside that same
  mismatch-`0` claim-side parent-route family first either; if the live docs
  already pin both `claim_flat_argument / claim_eventual_argument` branches on
  the active `claim_flat_codomain` bucket at the same smaller unsafe
  `4379 / 549 / 2259` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 113`,
  `small_cluster = 2871 / 435 / 435 / 111`, and the same reopened `single`
  bucket, that clause-`3` refinement also belongs to the autonomy ledger
  rather than as a fresh repair lead
- reopen another deeper remaining-one exact-summary reland inside that same
  representative mismatch-`0` shell first either after the live docs have
  already shown the latest reconstructive control stays on that same
  zero-admitted dead surface; keep the next repair above that remaining-one
  lattice rather than stabilizing another deeper reland here
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
- reopen that representative mismatch-`0` claim-flat dead child through
  another finer reason split first either after the live docs have already
  localized that shell to one uniform clause-`2` blocker plus the same
  nonqualifying connectivity vector across all clause-`3` / clause-`6` /
  terminal continuations; keep that finer reason partition in the autonomy
  docs rather than stabilizing it here
- reopen that representative mismatch-`0` claim-sharp dead child through
  another completion or finer reason split first either after the live docs
  have already localized that shell to the same dead
  `3`-generated / `0`-admitted child summaries and the same uniform
  clause-`2` blocker plus nonqualifying connectivity vector; keep that
  deeper claim-sharp partition in the autonomy docs rather than stabilizing
  it here
- reopen the isolated clause-`1` `demo_flat_codomain` exact-suffix side
  pocket first either after the live docs have already pinned that lone
  reland at `4349 / 556 / 2268` with
  `small_cluster = 3141 / 522 / 522 / 0` and the isolated `single` pocket
  still fenced; keep that looser side-pocket control in the autonomy ledger
  rather than stabilizing it here
- reopen broader mismatch-`0` or claim-safe shells first either after the
  live docs have already localized the promoted `reference / reference` tail
  to mismatch `2 = 42` versus mismatch `3 = 12` with mismatch-`2`
  clause-`4` pressure still concentrated on `claim_next_bridge = 18` and
  `reference = 16`; keep that tail split in the autonomy docs rather than
  stabilizing it here
- reopen the larger mismatch-`2` `reference / reference` clause-`4`
  `claim_next_bridge` or `reference` half first either after the live docs
  have already pinned those halves as tradeoff controls with wider
  `small_cluster`; keep those half-probe outcomes in the autonomy docs rather
  than stabilizing them here
- reopen the tiny mismatch-`2` `reference / reference` clause-`4`
  `demo_sharp_codomain` or `demo_sharp_bridge` pocket first either after the
  live docs have already pinned those pockets as matched smaller tradeoff
  controls; keep those demo-side outcomes in the autonomy docs rather than
  stabilizing them here
- reopen the smaller mismatch-`3` `reference / reference` clause-`4`
  `claim_next_bridge` or `reference` half first either after the live docs
  have already pinned those halves as smaller or sharper tradeoff controls
  with wider `small_cluster`; keep those mismatch-`3` half-probe outcomes in
  the autonomy docs rather than stabilizing them here
- reopen broader mismatch-`0` or claim-safe shells by restating the
  `reference / reference` tail either after the live docs have already marked
  both mismatch-`2` demo-side pockets and both mismatch-`3` halves as spent;
  once the full tail is exhausted, route to the live docs for the next
  broader-backup comparison rather than retelling that tail again
- promote the representative claim-safe claim-side clause-`2` shell ahead of
  the tighter representative mismatch-`0` claim-side clause-`2` shell after
  the live docs have already compared those broader backups; keep that
  ordering in the autonomy docs rather than stabilizing the looser claim-safe
  shell here
- reopen the representative `claim_sharp_codomain` clause-`2` sheet's
  clause-`6` identity split first either; its `claim_next_codomain`,
  `claim_sharp_codomain`, and `reference` continuations now also reland the
  same matched smaller tradeoff shell and differ only by a tiny deeper
  zero-admitted tail delta, so that axis also belongs to the autonomy ledger
  rather than as a fresh lead
- keep the next slice above another representative mismatch-`0` claim-side
  dead-shell reland too; once both representative claim-side sheets are
  frozen through their deeper completion and first finer reason passes, and
  once the deeper claim-side clause-`6` `reference` union is also pinned as a
  reconstructive control, route to the live autonomy docs for the
  post-local-probe fallback instead of reopening either claim-side dead child
  again
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
- reopen the representative claim-safe claim-side clause-`5` / clause-`6`
  identity split first either; beneath that same chosen cell, both claim-side
  sheets now reland the same six dead prefixes and every terminal continuation
  still needs fallback, so that axis also belongs to the autonomy ledger
  rather than as a fresh lead
- reopen the representative claim-safe `claim_flat_domain` dead prefix's
  terminal-family split first either; beneath
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`,
  the six dead prefixes now reland the same `3`-generated / `0`-admitted
  profile with the same `reference / eventual_lift / next_lift` trio, and
  every one of those completed telescopes is still structurally connected but
  outside historical reanchor and active-window qualification, so that axis
  also belongs to the autonomy ledger rather than as a fresh lead
- reopen the representative claim-safe dead prefix's clause-`5` qualifiers
  first either; under the selected
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`
  exact pair, clause-`5` `reference` is the only exact control while
  `claim_flat_codomain / claim_next_codomain / demo_sharp_domain /
  demo_flat_codomain` all stop at `5 / mismatch 5`, and the live dead shell
  itself still uses only the two claim-side dead labels, so that axis also
  belongs to the autonomy ledger rather than as a fresh lead
- reopen a deeper representative claim-safe dead-prefix clause-`6` or
  terminal-family reason pass first either after the live docs have already
  localized that completed shell to the clause-`5` qualification wall; keep
  that finer reason partition in the autonomy docs rather than stabilizing it
  here
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
