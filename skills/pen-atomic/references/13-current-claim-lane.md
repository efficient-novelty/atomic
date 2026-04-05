# Current Claim Lane

Read this file when the task touches `desktop_claim_shadow`, claim-lane
telemetry, claim-lane narratives, or the autonomy-certification roadmap.

## Current Truth

- `desktop_claim_shadow` exists as a separate profile and config family.
- The lane is not yet certified and should still be described with the safer
  `bounded live recovery` wording.
- The current canonical stored claim bundle is clean-tree completed `v9`:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v9`.
- Stored compare, certification, and benchmark outputs now exist beside that
  `v9` bundle, and they already treat accepted-hash parity, claim-policy
  honesty, fallback honesty, narrative/event completeness, exact-screen
  reason coverage, prune-class coverage, and manifest completeness as earned.
- Claim admissibility now uses structural claim debt and anchor hints, without
  named-family focus progression.
- Claim late expansion now uses a claim-specific late surface with structural
  mutators landed for kappa `4-9`, while stored breadth/floor evidence on that
  widened lane is still open.
- Claim bucket scheduling now uses a structural-generic taxonomy derived from
  prefix-local syntax and runtime evidence.
- The current mixed state is recorded explicitly in run metadata:
  - `guidance_style = claim_debt_guided`
  - `late_expansion_policy = claim_generic`
  - `bucket_policy = structural_generic`
- `run.json` now captures CPU, worker-count, build-profile, target, git,
  `Cargo.lock`, and binary fingerprints for claim certification, and the smoke
  certification path now passes the manifest-completeness gate.
- `pen-cli run` and `pen-cli resume` now write `run.json`, step summaries,
  step checkpoints, frontier snapshots, claim narratives/events, and failure
  status incrementally, so failed long claim runs remain auditable from disk.
- claim runs now record observed process RSS alongside governor-accounted RSS
  in stored step pressure data, so the model gap is visible from artifacts.
- claim runs now also emit `step_live_checkpoint` telemetry and
  `reports/steps/step-XX-live.ndjson` artifacts for steps 4-5, exposing
  observed process RSS, raw catalog widths, frontier queue size, prefix-cache
  size, legality-cache size, and whether late claim widening gates are active
  while the step is still in flight.
- claim auto-worker resolution is now memory-aware on
  `desktop_claim_shadow`, and claim proof-close now drops cached evaluated
  terminal payloads after ranking so the live prefix cache stays smaller.
- claim proof-close now also releases processed retained prefix groups once
  exact certification starts, so the live prefix cache can shrink during
  proof-close instead of carrying already-closed groups to step end.
- claim terminal-prefix materialization now also consumes cached exact
  completion summaries from the legality cache after reuse, so claim runs stop
  holding both the legality-cache payload and the retained prefix-group copy of
  the same exact terminal surface.
- claim terminal-prefix materialization now also has a direct compact path when
  no cached completion summary exists, so claim runs no longer build and then
  immediately re-walk a full terminal evaluation vector just to recover the
  same retained candidates and counts.
- cloned claim prefix signatures now share their serialized exact payload
  across frontier and legality-cache copies, so the same hot-path signature no
  longer duplicates that full prefix string into every cloned cache key.
- claim online frontier work items now reuse the shared clause catalog when no
  prefix-local active-window filter applies, so claim discovery no longer
  clones the full next-clause list into every queued frontier item.
- claim online frontier work items now also reuse that same shared serialized
  prefix key for deterministic ordering, instead of allocating a second copy of
  the serialized prefix string for the queue order key.
- claim terminal-clause filtering now also accepts the shared clause slice
  directly instead of first materializing a fresh `Vec<&ClauseRec>` at every
  terminal-prefix check, so the hot claim release path avoids that per-prefix
  allocation even when the claim lane has no active terminal filter to apply.
- claim exact remaining-two loops now also reuse one scratch terminal
  telescope plus the precomputed prefix bit cost across bound checks,
  completion-summary builds, and compact materialization, so the hot claim
  step-`4` path stops cloning the full prefix telescope for every admitted
  last-clause candidate.
- claim compact discovery now also skips full evaluation for terminal
  candidates that are already below bar or no longer beat the current
  incumbent, so the hot claim step-`4` path stops spending discovery time on
  proof-close work that is already known to be non-winning.
- `scripts/compare_runs.py` now audits claim-policy honesty, exact-screen
  reason coverage, prune-class coverage, narrative artifacts, and whether the
  stored run reaches the step-15 claim signoff surface.
- `scripts/certify_claim_lane.py` now emits a stored pass/fail certificate from
  claim artifacts and currently fails honestly on missing breadth, missing
  generated-floor evidence; the current `v9` certificate still flags
  `early_breadth` plus `late_generated_floors`.
- `scripts/benchmark_claim_lane.py` now aggregates stored claim runs into a
  benchmark bundle with runtime percentiles, parity counts, breadth-floor hit
  counts, and manifest snapshots; it still needs a breadth-clean stored claim
  bundle before those numbers can justify a stronger sentence.
- The current stored breadth snapshot on canonical `v9` is:
  - step `1`: `546 / 2144`
  - step `10`: `1428 / 500`
  - step `11`: `1338 / 800`
  - step `12`: `1338 / 1200`
  - step `13`: `123 / 2200`
  - step `14`: `12027 / 3500`
  - step `15`: `1794 / 5000`
- A fresh stored rerun stack has now consumed the guarded local step-`11`
  breadth repair:
  - the connected claim step-`11` surface now holds
    `kappa 5 = 243`, `kappa 6 = 729` (total `972`)
  - local exact-screen connectivity rejections there are now `0`
  - the guarded accepted step-`11` shell stays fixed locally
  - dirty-tree `v7` first re-earned stored step `11` but reopened accepted-
    hash parity at step `12`
  - the narrow step-`12` same-primary selector repair now lands on top of that
    rerun read
  - clean-tree `v9` restores accepted-hash parity through step `15` while
    keeping stored step `11` and step `12` both as breadth hits
- the repo-level autonomy docs now treat claim-policy separation, failed-run
  evidence preservation, and one parity-clean full-profile stored bundle as
  baseline; the live blocker is diagnosing the remaining stored breadth misses
  on the canonical chain, not reopening another step-`4` survival pass first

## Current Operational Blockers

- the lane still does not have a signoff-ready certified bundle even though
  stored `v9` now passes accepted-hash parity and the compare/certification/
  benchmark infrastructure is live
- stored breadth still fails honestly on the canonical chain:
  - early breadth still misses at step `1` (`546` versus `2144`)
  - late generated floors still miss at step `13` (`123` versus `2200`) and
    step `15` (`1794` versus `5000`)
  - stored breadth already hits at step `10`, step `11`, step `12`, and
    step `14`
- step `13` is now the earliest remaining stored late-floor miss on the clean
  canonical bundle
- the canonical repaired late chain must stay frozen while breadth is
  re-earned:
  - step `12` should keep the guarded `34 / 6` continuation
  - step `13` should stay at `[3,1,3,3,1,1,1]` / `27` / `123`
  - step `14` should stay at `19683` / `12027`
  - step `15` should stay on `DCT 103 / 8 / 1794`
- step `1` remains a separate stored early breadth blocker even if late-step
  repairs continue to land
- benchmark evidence is still too weak for a passing claim certificate until a
  fresh stored bundle closes the remaining breadth failures without losing
  accepted-hash parity

## Immediate Next Slice

1. Freeze clean-tree completed `v9` as the current canonical stored claim
   bundle, keep clean `v6` as the pre-step-`11` breadth baseline, and keep
   completed `v5` as the pre-parity reference surface.
2. Hold the current local guardrails green before reopening any new theory:
   the step-`11` connected surface should stay at `243 + 729 = 972`, the
   guarded step-`11` shell should stay accepted, the repaired step-`12`
   `34 / 6` continuation should stay fixed, and the current
   step-`13..15` surfaces should stay frozen on the canonical branch.
3. Start from clean-tree `v9` and resume diagnosis from stored step `13` as
   the earliest remaining late-floor miss on that canonical bundle.
4. Keep stored step `15` and step `1` in view beside that work:
   `1794 / 5000` and `546 / 2144`.
5. Do not reopen another stored step-`11` rerun first now that clean-tree
   `v9` already re-earns stored step `11 = 1338 / 800`.

## First Reads

- [../../autonomous_plan.md](../../autonomous_plan.md)
- [../../autonomous_next_steps.md](../../autonomous_next_steps.md)
- [../../autonomous_progress.md](../../autonomous_progress.md)
- [../../autonomous_checklist.md](../../autonomous_checklist.md)
- [../../README.md](../../README.md)
- [../../docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)

## Do And Do Not

Do:

- treat the current claim lane as a mixed-mode scaffold with honest metadata
- preserve existing guarded, realistic, and demo behavior while the claim lane
  changes
- prefer structural explanations over family-name explanations in new claim
  code
- assume the local shell is Windows PowerShell and avoid shell chaining such
  as `&&`; use separate commands for staging, commit, and push work
- keep claim-lane edits narrow and staged; split very large file updates into
  smaller targeted patches when the first broad patch does not land cleanly
- focus next on diagnosing the remaining stored breadth misses on the clean
  canonical bundle, keeping step `13` / step `15` / step `1` in view, and
  keeping the repaired local step-`11` / step-`12` / step-`13..15`
  guardrails fixed

Do not:

- claim that the whole lane is already family-agnostic end-to-end
- switch `bucket_policy` early
- spend time reopening already-landed claim-policy split work unless a memory
  or evidence bug forces it
- reopen another local step-`11` selector or raw-connectivity theory first:
  the current local breadth repair is already landed and guarded
- reopen another runtime-only step-`4` micro-optimization first unless a fresh
  stored rerun proves the remaining misses are really runtime fallout
- reland the rejected global band-`7` widening or the rejected late reanchor /
  early bridge expansions first while stored breadth is still open
- call the lane `unguided` yet
