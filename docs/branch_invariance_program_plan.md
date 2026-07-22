# BI-1 — Branch-invariance program: does the cone halt?

**Date:** 2026-07-22. **Status:** brief frozen before any BI engine
output exists. Discharges the standing obligation registered in the
R-T3 Option-B adoption (`docs/r_t3_stage4_adjudication.md`, F-R3-B1):
until this program speaks, every halt, ledger, O(16), and T-BF claim
is branch-indexed.

**Standing:** construction and measurement task. Admissible context:
the adopted rule ledger (21 items, in particular two-register gating,
the tie-resolution protocol, `future-hole-hypothesis-definition-v1`,
`dependent-ambient-context-v1`); the R-T1 distinctness certificate;
the authoritative R-T2 certificate
(`docs/r_t2_future_hole_confluence_v2_dependent_context_v2.json`); the
E-5 dependent-context certificate
(`docs/schema2_e5_future_hole_finale_v2_dependent_context.json`) — as
regression reference only, see F-BI1; the certified v3 history and its
burn. Forbidden in all law-level code paths: any bar as gate
(diagnostic recording permitted), enumeration/hash order as selector,
any enacted-branch outcome as steering or acceptance input on a
non-enacted branch, any appeal to desired invariance.

## Mission

The four Stage-4 acts — `20167267`, `43a0ed70`, `4b2211ec`,
`b4f821d9`, with certified key classes {`20167267`, `b4f821d9`} and
{`43a0ed70`, `4b2211ec`} — define the cone. The enacted branch is the
one whose act matches the certified v3 history by sealed digest; the
other three have never been run. For each non-enacted branch,
construct the continuation under the adopted laws and measure, at
pre-registered granularities, whether the enacted branch's law-level
claims hold there too. The program ends in a cone report assigning an
invariance verdict per granularity level, or in named lawful stops.

Two registered routes; either or both may be pursued:

- **Route E (execution).** Per-branch continuation + E-5-class
  certification, as specified below. The default route.
- **Route T (theorem).** A transport theorem proving invariance at
  some granularity for the whole cone without per-branch execution.
  Note the ceiling: R-T2 already refuted scheme-set equivalence at
  Stage 5, so no Route-T claim may target granularity G1; a lawful
  Route-T result proves G2 or coarser, and its proof may not repair
  or re-quotient the R-T2 comparison (F-R3-2).

## Pre-registered granularity ladder (verdicts issued per level)

- **G1 — scheme identity.** Successor scheme sets equal under the
  certified family quotient. **Already refuted** (R-T2); recorded for
  completeness; no BI task re-litigates it.
- **G2 — obligation profile.** Per-stage demand structure equivalent:
  same number of live obligations per window, discharged totally at
  each stage, O-ladder shape preserved (one demand per stage from the
  branch point to halt).
- **G3 — numeric ledger.** Per-stage (κ, ν) equal to the enacted
  branch's certified vector from Stage 5 onward; hence equal Σν and
  equal diagnostic bar trajectory.
- **G4 — halt.** The branch reaches O(n+1) = ∅ at n = 15 with a
  debt-free instance ledger and F1 excluded (its own E-5-class
  certificate).

The levels are logically independent measurements except where
implication is proved: a G3 pass does not assume or imply G2; each is
measured, not inferred. Divergence at one level does not stop
measurement at the others (F-BI3).

## Tasks

- **BI-0 (regression rung — gate for everything).** Run the full BI
  pipeline on the *enacted* branch first. It must re-earn the
  certified v3 vector, the discharger census, and the E-5
  dependent-context results exactly. If the pipeline cannot reproduce
  the world it is auditing, its verdicts on other branches are void —
  fix the machinery, never the history.
- **BI-1 (continuations).** For each non-enacted branch: replace the
  Stage-4 act in the prefix (create-new; the sealed history is never
  edited), and run the continuation under two-register gating: at
  guarded stages acceptance = total discharge of the live demand with
  typed provenance; bars recorded as diagnostics only. Run to halt
  (O = ∅) or to a lawful stop. Per-stage outputs: live demand,
  admitted cone size, discharger count, winner digest, (κ, ν),
  diagnostic bar. Suggested order: one branch per key class first
  (economy probe), then the remaining two; scheduling is free,
  verdicts require all four or a Route-T theorem.
- **BI-2 (per-branch finale).** For each branch that halts: an
  E-5-class execution on its completed library — A3 inventory, unary
  and structural registrations with totality theorems under the
  dependent-context rule, membership partition, semantic O(halt+1),
  F1 disposition. Same issuer discipline, create-new, per branch.
- **BI-3 (economy transport, optional).** If two class-mates prove
  equivalent at some granularity stage-by-stage, a transport theorem
  may promote one to class representative *for the granularities
  proven* — never by key-class membership alone (F-BI4).
- **BI-4 (cone report).** Single final artifact: per-branch outcome,
  per-level invariance verdict, first divergence (if any) published
  verbatim with full context, and the disposition of the branch index
  per F-R3-B1: which claims rise to cone-level, which remain indexed.

## Pre-registered stop semantics

- **Multiplicity.** A guarded stage on any branch presenting more than
  one total discharger is a lawful stop for that branch (F-S8-1
  pattern): the tie ladder applies *per branch* — quotient, then
  confluence, then, if genuinely free, R-T3 lands back with the user.
  The cone may be a tree. If so, the invariance question extends to
  the subtree by this same brief's semantics; nothing is improvised
  and nothing is pruned.
- **Deadlock.** A guarded stage whose sole discharger fails typed
  provenance, or a live demand with no discharger, is a lawful halt
  state distinct from debt-freedom (the Stage-8 taxonomy applies). It
  is recorded as the branch's terminus and is itself a G4 verdict:
  that branch does not reach the debt-free halt.
- **Expressivity gap.** Any registration needing semantics beyond the
  adopted closure and the dependent-context rule → named gap and
  versioned adjudication (F-FH4/F-DC pattern); no improvisation.

## Outcome zones (registered before any run)

- **Z-CONE.** All four branches halt debt-free at 15 with G2 and G3
  passes → the cone halts; determinacy returns at law level; the
  branch index becomes recorded gauge; cone-level claims replace
  indexed claims per BI-4.
- **Z-ISO.** All four halt at 15 with G3 pass but G2 divergence
  somewhere (same numbers, different obligations) → isospectral,
  non-isomorphic cone: halt and ledger are cone-level; structure
  claims stay indexed.
- **Z-SPLIT.** Any branch halts elsewhere, deadlocks, or carries a
  different ledger → the Stage-4 fork is observable structure; the
  fifteen-link claim is a fact about our branch; the divergence is
  the theory's first cone-level *prediction of difference*, published
  verbatim.
- **Z-TREE.** Further genuine branching within any branch → the cone
  is a tree; verdicts restate over the tree per the stop semantics;
  no zone is forced.
- **Z-STOP.** A named expressivity gap blocks a branch → partial
  report; the gap gates that branch only.

## Falsifiers

- **F-BI1.** Any non-enacted branch computation consuming enacted
  outcomes (winners, scores, ledgers) as input to acceptance,
  selection, or steering → that branch's run is void. The enacted
  record is admissible in exactly two places: BI-0 regression and
  BI-4 comparison, after branch runs are sealed.
- **F-BI2.** Improvised selection at a multiplicity or deadlock stop →
  void; the stop stands until lawfully resolved.
- **F-BI3.** Suppressing, averaging, or repairing a divergence at any
  granularity → void; divergences are results, published verbatim.
- **F-BI4.** Class-representative substitution without its transport
  theorem → invalid for every claim that relied on it.
- **F-BI5.** BI-0 regression failure → all BI verdicts void until the
  machinery is fixed; the history is never adjusted to pass.
- **F-BI6.** Any artifact of this program publishing an unindexed
  law-level claim before BI-4 issues its disposition → invalid
  (F-R3-B1 restated).

## Deliverables

Create-new result documents and replay-certified JSON artifacts:
`docs/BI_REGRESSION_RESULT.md`, per-branch
`docs/BI_BRANCH_<digest-prefix>_RESULT.md` + certificates, and
`docs/BI_CONE_REPORT.md` + certificate. Mutation falsifiers on every
certificate: flipping any verdict, count, winner digest, or
granularity flag must invalidate replay.

## Relation to the standing program

T-BF2's authorized theorem may execute in parallel (branch-indexed
until BI-4). T-BF1's cone-form successor and T-BF3's branch-relative
form should be drafted after the cone report, which fixes their
quantifiers. The bridge (E-7/E-8) and final certificate remain closed
until BI-4 issues and their cone-level/branch-level split is
statable. WB-1 (windowed-bar hypothesis) remains independent; its
W-T1/W-T2 may run on the enacted branch at any time, and a per-branch
windowed table is a lawful optional diagnostic within BI-4, gating
nothing.
