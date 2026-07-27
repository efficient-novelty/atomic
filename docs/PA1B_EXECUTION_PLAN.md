# PA-1b execution plan — open-competition audit

**Plan date:** 2026-07-27. **Scope:** PA-1b only. **Governing brief:**
`docs/pa1_provenance_audit_plan.md`. **Required result:**
`docs/PA1_OPEN_COMPETITION_RESULT.md`.

## 1. Question and pass condition

The audit asks whether any certified run in the repository put two or more real,
lawfully generated candidates before a target-independent admission and
selection law, accounted for the complete outcome-relevant pool, and produced a
lawful winner. A large raw count is not by itself evidence of open competition.

A row is positive only if all four conditions hold:

1. **Open surface:** at least two distinct expressions can reach the governing
   law without a reference literal, position singleton, or named-candidate
   designation deciding their eligibility.
2. **Lawful candidates:** the competitors are members of the run's actual raw
   grammar and pass the ordinary checker and the adopted admissibility law.
3. **Complete selection input:** the run either evaluates the complete lawful
   pool or excludes every omitted member by a sound, replayed bound. Resource
   shaping, a hand-registered witness table, or an unexecuted count is not a
   complete selection input.
4. **Autonomous winner:** the winner is unique under a target-independent
   selector. A reference-derived rank, canonical/hash tie break introduced to
   choose one member of a lawful tie, or explicit selection by name does not
   qualify.

If a row satisfies all four conditions, the report must name it as the
program's strongest autonomy evidence. If no row does, the report must state
verbatim:

> No certified run in the record contains a genuinely open competition with a lawful winner.

Open lawful cones without a winner remain positive evidence of gate openness
and must be reported separately rather than collapsed into the global result.

## 2. Evidence inventory and execution method

| Audit family | Certified surfaces to cover | Primary method | Instrumented check |
|---|---|---|---|
| Claim lane | Step 1 raw 2,144 surface; Step 15 5,000 floor / 7,211 reported prefixes | stored run summaries, config, comparison certificate, source and Git chronology | in-memory current-source Step 1 and Step 15 homologous reruns; no run artifact rewritten |
| Reselection v2 | Stages 1–8, including Stage 4's four-cone | frozen program and burn plus exact selector code | deterministic program/burn replay where current bindings permit; T-BF1 direct cone recomputation as the tie control |
| Reselection v3 | recomputed Stages 8–15 and discharger census | frozen program and burn plus Guarded enumeration path | deterministic replay where current bindings permit |
| T-BF1 | Stage 4 four-candidate cone | sealed certificate and direct re-enumeration code | reissue and replay the four-cone in memory |
| BI-1 | enacted branch stages 5–15; three alternate branches stages 5–8 | sweep manifest, all four branch certificates, branch code path | code-path and artifact analysis; record current-reissuance drift rather than treating it as success |
| BI-1b | enacted replay; three alternate resumptions stages 8–15 | sweep manifest, all four branch certificates, regression disclosure | code-path and artifact analysis; distinguish consumed sealed Stage 8 from fresh later enumeration |
| Step 16 | unclamped lane attempt, count diagnostic, corrected adversarial table, automaton/semantic exhaustion, certified halt, SH-1 continuation | artifacts plus generic Guarded code path | safe count-only geometry, corrected 17-shape gate replay, and certificate replays; never materialize the known infeasible full surface |

Every historical record lacking a `run_id` receives a stable audit key based on
its artifact path and digest. Every row records whether its evidence is a
historical artifact replay, a current homologous rerun, a count-only
calculation, an authored witness replay, or code-path analysis.

## 3. Execution phases

### Phase A — freeze the inputs without editing them

1. Inventory all certified artifacts and relevant source files.
2. Record whole-file BLAKE3 bindings and embedded result digests.
3. Record the dirty worktree before edits and exclude unrelated user changes
   from the PA-1b diff.
4. Run all probes in memory or against newly created PA-1b outputs. No sealed
   input is an output path.

### Phase B — re-derive the censuses

1. Parse the claim Step 1 and Step 15 funnel counters at their actual layers:
   raw prefixes, checker-valid terminals, hard admission, exact pruning,
   heuristic drops, full evaluation, and winner count.
2. Parse every reached v2/v3 stage and record enumeration, admission,
   canonical-deduplication, discharger/clearing, and winner counts.
3. Recompute T-BF1 Stage 4 directly and compare all four candidate hashes,
   `(κ, ν)` pairs, and the no-selection outcome with the sealed certificate.
4. Parse all BI-1 and BI-1b branch stage rows. List every enacted and
   never-enacted branch explicitly; do not summarize away the Stage 8 stop and
   successor reclassification.
5. Recompute the corrected Step 16 adversarial gate traces. Separate raw-surface
   members, lawfully rejected expressions, lawful clearers, impossible authored
   diagnostics, and any selected candidate.

### Phase C — audit pinning and selection

For each row, trace:

1. the enumerator entry point and candidate origin;
2. the checker and admissibility path;
3. every target/reference/position-dependent predicate;
4. every exact prune and every non-sound resource or heuristic drop;
5. the selector's complete input pool and tie-break fields;
6. whether a winner was actually selected, and whether that selection follows
   from the target-independent law.

Git chronology is used only for causal direction. It cannot turn a replay into
a discovery run. A current rerun of a historical dirty-tree claim artifact is
labelled homologous, not bit-identical.

### Phase D — ship replayable output

1. Add `crates/pen-search/src/pa1_open_competition_v1.rs` with typed audit rows,
   input bindings, safe probes, deterministic derivation, rendering, and
   create-new output.
2. Add `crates/pen-search/examples/pa1_open_competition_v1.rs` with
   `create-new` and `replay` modes.
3. Emit `docs/pa1_open_competition_v1.json` and
   `docs/PA1_OPEN_COMPETITION_RESULT.md` using create-new semantics.
4. Make replay reject:
   - any payload digest mismatch;
   - unknown or dropped JSON fields;
   - input-binding drift;
   - missing or reordered audit rows;
   - a rehashed mutation that differs from deterministic reissuance;
   - a report that differs byte-for-byte from deterministic rendering.

### Phase E — verification and registration

1. Run focused PA-1b unit tests, including ordinary mutation, rehashed mutation,
   unknown-field, report-drift, and positive-classification guard tests.
2. Run the existing focused claim, T-BF1, Step 16, reselection, and branch
   regression tests used by the audit.
3. Run `cargo test -p pen-search --lib` if the focused suite leaves no known
   infeasible test path; otherwise document the exact bounded verification set
   and reason.
4. Replay the emitted JSON/report from a fresh process.
5. Recheck `git diff` and input hashes, then append one deliverable event to
   `docs/XF_LEDGER.md` without rewriting its existing dirty content.

## 4. Falsifier controls

| PA-1 falsifier | Control |
|---|---|
| F-PA1 — unsupported grade | PA-1b assigns no PA-1c grades; every competition classification carries census and code-path evidence |
| F-PA2 — narrative ground | the certificate schema has no narrative, schedule, milestone, or publication field |
| F-PA3 — sealed artifact touched | probes are read-only and outputs use create-new paths; sealed inputs are digest-bound |
| F-PA4 — unflattering cell suppressed | every requested run family, all four BI roots, every reached stage, failed/OOM attempts, and zero-winner rows are explicit |
| F-PA5 — disclosure weakened | `docs/provenance_disclosure_v1.md` is a bound input and is not edited or superseded by PA-1b alone |

## 5. Known limits that must remain visible

- The certified claim-lane v15 artifact records a dirty source tree. Its exact
  historical source is not reconstructible from the commit alone; current
  reruns are corroborating homologous probes, not bit-replays.
- The original full Step 16 materializing attempts exhausted memory. PA-1b does
  not repeat a known unsafe allocation and does not convert an OOM into a halt
  verdict.
- BI-1b already records current deterministic reissuance drift for prerequisites.
  Frozen-testimony authentication and current reissuance are reported as
  different evidence classes.
- PA-1b decides competition and winner autonomy only. It does not assign the
  PA-1c discovery/verification/unearned grades and does not weaken the standing
  disclosure.

## 6. Execution record

Execution on 2026-07-27 produced 20 explicit audit rows and zero rows that
satisfy the complete PA-1b predicate. The safe current-source probes reproduced
the claim-lane Step 1 census, reproduced the T-BF1 four-way Stage 4 tie, replayed
the corrected Step 16 authored witnesses, and replayed the certified-halt
mutation guard.

Two bounded current-source Step 15 attempts (two minutes and three minutes)
were terminated without an emitted artifact. The audit therefore retains the
frozen 7,211 prefix-event count and source-path analysis rather than presenting
an interrupted computation as exhaustive. The historical full Step 16
materialization was not repeated because its recorded multi-gigabyte OOM path
is unsafe; exact semantic-exhaustion counts and the corrected authored-witness
probe are recorded separately.

Nine targeted tests passed: four PA-1b issue/replay/mutation/report tests, the
claim Step 1 catalog test, the T-BF1 tie test, the corrected Step 16 witness
test, the semantic-exhaustion regression, and the certified-halt mutation test.
The legacy BI branch-cone test failed at its known current-reissuance
prerequisite drift (`frozen v3 artifact failed independent replay: audit differs
from independent v3 reissuance`). This is recorded as current-vs-frozen
evidence, not suppressed or recast as a branch result. The example target
passes `cargo check`, and both new Rust files pass a direct `rustfmt --check`.
The full 939-test library suite was not run because it contains known
long-running and intentionally infeasible branch paths; the bounded set above
directly covers the PA-1b implementation and its independent evidence probes.

## 7. Completion checklist

- [x] All requested certified surfaces represented.
- [x] Safe instrumented probes executed.
- [x] Code-path and Git causal analysis recorded where rerun is impossible.
- [x] Mutation-guarded JSON emitted and replayed.
- [x] Required Markdown result emitted and replayed.
- [x] No sealed input changed.
- [x] Focused and crate-level verification recorded.
- [x] Deliverable ledger event appended.
