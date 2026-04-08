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
- reopen the exact claim-pair clause-`4` `reference` side first; that narrower
  relocalization now only reproduces the older broader clause-`4`
  `reference`-sheet tradeoff and belongs in the autonomy ledger rather than as
  a fresh lead
- split the exact claim-pair clause-`4` `claim_next_bridge` half into the
  claim-flat or claim-sharp single-sheet relocalizations first; those smaller
  tradeoff controls now also belong in the autonomy ledger rather than as a
  fresh lead
- reopen runtime-only step-`4` throughput work first unless a newer rerun
  proves the remaining misses are runtime fallout
- reland ruled-out clause-`4` / clause-`5` or same-primary probes without new
  evidence
- claim certification or stronger language before the stored certificate says
  so
