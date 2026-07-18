# IP-1 and TDC-1: post-SH-1 experiment plan

**Date:** 2026-07-18. **Status:** frozen before IP-1/TDC-1 engine outputs;
A5 adjudication remains explicit and unresolved. Builds only on
frozen artifacts: the T4 blind law (freeze `4b408c8`), the SH-1 burn record
(`docs/SH1_EXPERIMENT_RESULT.md`), the Step-16 dual certificate
(`docs/CERTIFIED_HALT_15.md`), and the Guard-Rail theorem. Naming
provisional. Nothing here revives SH-1: SH-1b (host-capacity gate) and the
theta/torsion arithmetic remain dead; no staircase iteration appears below.

**What the burn established, used as premises here:**

- **B1.** Honest law-level credit = number of normalized families proved in
  Marg₂ carrying distinct EGP anchors; 1+d² is only the conditional raw
  ceiling of the single-constructor beta/Kan basis (T4, frozen).
- **B2.** Missing normalization/EGP evidence ⇒ novelty *undefined* ⇒
  candidate *unrankable* — not zero, not placeholder (T4, frozen).
- **B3.** The shipped structural continuation is a self-feeding P5 chain
  (ν_n = ν_{n−1} + 4 from step 18): reference re-priced as value, iterated
  (T1 trace, burned artifact).
- **B4.** The rankable raw surface is already swept: nine-class certified
  maxima 8/11/14 at κ = 2/3/4, against clearing thresholds 19/28/37
  (dual certificate, Agda-checked).
- **B5.** No public token constructors exist: the shallow AST cannot mint
  the evidence B1 requires (certified sidecar, shipped).

---

## Track A — IP-1: the Certification Boundary Theorem

**Claim to derive.** Within the frozen system (raw caps 2 ≤ κ ≤ 4, r ≤ 2,
d ≤ 1, six nodes; T4 law; sealed B₁₅):

> Every raw-admitted Step-16 candidate is either (i) rankable, with
> certified ν ≤ 14 < Bar₁₆·κ for its κ, or (ii) unrankable, with no
> constructible path to the evidence acceptance would require. Therefore
> **no sixteenth step is certifiably clearing: Genesis cannot justify its
> own extension.**

This is the *operational halt* — weaker than the semantic halt (a stronger
system might later certify a candidate), but a theorem rather than a
conjecture, and available without the typed kernel.

**The one free choice, flagged for adjudication (do not smuggle).**
The theorem needs a selection-semantics clause not currently frozen:

- **A5 (open-band burden).** On a debt-free field, acceptance requires a
  *certified* clearing: unrankable candidates cannot be accepted.

Justification with Guard-Rail pedigree: every Genesis acceptance carried a
binding justification — at steps 4–15 it was debt discharge, verified by
the O-ladder (admissibility was the proof). At 16, with O = ∅, valuation is
the only justification channel left, so valuation must carry proof; an
uncertified score is not a justification but a claim. B3 is the exhibit for
why unranked structural scoring must never carry acceptance. A5 is
nevertheless a **new clause**: Halvor adopts or rejects it explicitly; IP-1
is conditional on it and says so in its statement.

**Engine work (small).**

- **A-1.** Per-class completeness sweep: every raw signature class maps to
  either an EGP verdict (rankable) or a named required-token failure
  (unrankable). Mostly assembled from `certified_novelty.rs` + the
  step16 automaton; the new content is the *exhaustiveness* of the
  class-to-verdict map over the counted surface, replay-checked.
- **A-2.** Emit `docs/ip1_certification_boundary.json`: class table,
  verdicts, thresholds, and the A5-conditionality flag. Mutation falsifiers:
  flipping any verdict, token outcome, or completeness flag must invalidate.

**Falsifiers.**

- **F-IP1.** A raw class found with neither an EGP verdict nor a
  required-token failure → sweep incomplete; theorem blocked until closed.
- **F-IP2.** A public route to any required token within the frozen AST →
  B5 false; theorem dead as stated.
- **F-IP3.** A5 rejected on adjudication → IP-1 is reduced from theorem to
  observation; record and stop (no reformulation of A5 to rescue it).

**Book consequence if it lands.** The halt chapter's honest statement
becomes: *the two laws build for fifteen steps and then provably lose the
ability to certify further building — decidability terminates at 15.* The
semantic remainder (could a stronger system continue?) is exactly Track B.

## Track B — TDC-1: the typed d = 4 certificate

**Purpose.** Take one candidate class — the formed path package
`[Formation over the window, PathAttach PathCon(4)]`, κ = 2 — from
unrankable to ranked, by building the minimal typed kernel fragment that
class needs. This is the burn's named revival condition executed at its
sharpest point, and the first semantic test of Theorem 17's (V-opq) at the
place it can actually fail.

**Why the arithmetic makes this decisive.** Under B1 the class's law-level
ceiling is 17 + f (17 = 1+d² path-basis ceiling; f = certified formation
families, expected small). The clearing threshold at κ = 2 is ν ≥ 19. The
EGP debt-free envelope is ν ≤ 8. So the certificate lands in one of four
pre-registered zones:

| zone | certified ν | meaning |
| --- | --- | --- |
| Z1 | ≥ 19 | near-saturation is real; continuation revives, earned; Reading A absorbed with typed backing |
| Z2 | 9–18 | halt survives at 16, but the 4κ class-ceiling claim is false → sidecar calculus must be re-derived; margin story moves to the 9κ envelope |
| Z3 | ≤ 8 | (V-opq) holds at the sharpest point; strongest typed support for the halt to date |
| Z4 | undefined | fragment insufficient to decide marginality; unrankability persists; name the next scope increment, no verdict |

Note Z1 requires ≥ 19 of ≤ 17+f families to certify: near-total saturation,
which the Step-15 open-problem record (§6.2, sparse candidates) gives
independent reason to doubt. That expectation is recorded here as *bias
disclosure, not prediction*; the zones above are the registered semantics.

**Domain note (honesty).** d = 4 is outside the frozen raw caps (d ≤ 1), so
TDC-1 does not decide whether step 16 *happens* — under IP-1 it cannot. It
decides whether the completed world *semantically contains* higher-cell
value: the "growth moves inside" question. If constants ever re-enter, it
is through this door — the kernel's actual homotopy content — not through
ledger forcing (that route burned with SH-1).

**Build scope (minimal kernel fragment).**

- **K-1.** Typed elaboration for the one class: contexts, the window
  formation, `PathCon(4)`, over sealed B₁₅. Fill `pen-type`
  `infer/normalize/equality` only to the depth this class's basis needs.
- **K-2.** Beta/Kan basis enumeration at d = 4 and normalization of each
  family to a canonical representative.
- **K-3.** Weakening recognition against B₁₅ for exactly these families
  (old/inherited ⇒ 0 per T4).
- **K-4.** EGP anchor assignment; distinctness proofs; per-family verdict
  marginal / weakening / undefined.
- **K-5.** Certificate emission with replay + mutation falsifiers; the
  ρ-vs-Bar comparison runs as a *separate* step after the certificate is
  frozen (bar-independence).

**Regression gate (non-negotiable).** The same K-1..K-4 machinery, applied
to the historical HIT-class steps in the sealed trace, must rederive their
recorded scores. If the machinery cannot re-earn the world it is auditing,
its Step-16 verdict is void. (This is the per-class instance of the full
semantic re-audit the record already flags as outstanding.)

**Falsifiers.**

- **F-T1.** Historical regression fails → machinery void; fix the
  machinery, never the history.
- **F-T2.** Two normalized families with identical anchors both certified
  marginal → anchor distinctness broken; certificate invalid (thinness
  failure, §S8 of the open-problem record).
- **F-T3.** Basis enumeration exceeds the 1+d² ceiling → T4's frozen law
  contradicted; escalate to T4 re-derivation, do not patch locally.

**Pre-registered follow-up (not in scope).** d = 5 as a separate
registration only if TDC-1 returns Z4, or Z3 with an explicit gap analysis
naming what d = 5 could change. No iteration beyond one dimension per
registration.

## Sequencing and threads

1. **IP-1 first** (days, mostly assembly): it may deliver the operational
   halt as a theorem and restructures the book statement either way. The A5
   adjudication is its gate — that decision is Halvor's alone.
2. **TDC-1 second** (the real build): decides the semantic remainder and is
   the only earned route back to physical content (D2's θ jurisdiction
   stays queued in parallel as a book-side lemma thread; unresolved F-SH5
   carries over unchanged).
3. Contamination register: this plan may be shown to the IP-1 and TDC-1
   build threads (their work is construction, not blind derivation). Any
   future re-derivation of T4-layer valuation law remains blind per the
   standing rule; this document remains a forbidden input there.

**One-line summary.** IP-1 asks: can the world justify a sixteenth step
from inside? (Provably no, if A5 is adopted.) TDC-1 asks: does the finished
world nevertheless contain uncashed higher-dimensional value? (The
certificate answers with a number, and every zone of that number teaches
something registered in advance.)
