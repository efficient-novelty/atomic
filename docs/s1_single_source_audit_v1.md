# S1: Does the Public Interface Supply Its Own Valuation?

**Date:** 2026-07-25.
**Verdict:** **S1 is false as the synthesis currently stands.** Two sealed
clauses supply a valuation independently of the dagger, and both name the Born
weight. The blocker is one clause of an axiom, the repair is a two-word
restatement, and the theory has an independent reason to want that repair — as
written, the clause presupposes a quantity the dependency chain derives
downstream of it.
**Status of the change:** **not made.** This is an axiom change and belongs to
whoever owns the physical sealing clauses.
**Context:** `docs/r1_public_layer_typing_v1.md` (which raised S1),
`docs/target1_ledger_invariance_joint_v1.md`, ledger event 009.
**Register independence:** unaffected by the standing provenance disclosure.

---

## 1. What S1 claimed and what the audit found

> **S1 (single source).** The public interface introduces no scalar-producing
> structure beyond what the dagger already supplies.

The test is empirical, not conceptual: read the sealed clauses of the public
interface and see whether any of them produces a scalar from a state. I audited
Internal Record Actualization (four clauses), the public-event extractor
definition, and Public Confluence.

**Exactly two hits, both naming Born.**

Internal Record Actualization, clause 1:

> `r_x` is correlated with the realized alternative by the **Born-weighted
> internal dynamics**

Public-event extractor, admissibility condition:

> The value ... is defined only when the record is constitutively admissible,
> **Born-supported**, decodable at the declared tolerance, ...

A weighting is precisely a scalar-producing structure on states. So the public
interface, as sealed, supplies one — and it does not get it from the dagger. S1
is false.

---

## 2. Two readings, and both are fatal to S1

The phrase "Born-weighted" admits two readings. They differ in *why* S1 fails,
not in whether it does, and the difference matters for the repair.

**Reading A — it imports the standard quantum rule.** Then Internal Record
Actualization is borrowing a valuation from outside the theory's own derivation
as a modelling assumption. S1 is straightforwardly false: there are two
scalar-formers, the dagger's pairing and the imported rule, and nothing forces
them to agree. This is the countermodel's situation exactly — `Mat_{ℝ≥0}` also
has a dagger and an independently supplied weighting, and they disagree.

**Reading B — it forward-references the theory's own Born weight theorem.** Then
Internal Record Actualization is not positing a valuation but consuming one. S1
would survive. But the clause then cannot be used to *establish* S1 without
begging the question, and worse, it is out of order: see §3.

Neither reading lets S1 stand on the current text.

---

## 3. A circularity that exists independently of S1

This is the more important finding, and it does not depend on anything in the
environment-completion programme.

The synthesis's own dependency chain reads

```
DCT + Closed(U) + IRA + PC + Q_p   ⟹   observer-indexed public physics
```

so Internal Record Actualization, Public Confluence and the extractor are
**inputs** to the public-physics layer. The Born weight theorem lives inside that
layer — it is derived in the Quantum Theory and Born Weights section from phased
proof families and the requirement that the public scalar be phase-neutral.

But Internal Record Actualization, an input, names the Born weighting in its
first clause. So on Reading B the chain has an input presupposing an output of
what it feeds. On Reading A the axiom imports a rule the theory elsewhere
undertakes to derive, which is not circular but is a silent bridge in a clause
presented as constitutive.

Either way one clause should move. This would be worth fixing if S1 had never
been raised.

---

## 4. The repair

Restate the two clauses weight-neutrally, so that they constrain records to
track *whatever* weighting the layer supplies, rather than naming it:

> **Internal Record Actualization, clause 1 (proposed).** `r_x` is correlated
> with the realized alternative by the sealed internal dynamics and its own
> public weighting.

> **Public-event extractor (proposed).** ... defined only when the record is
> constitutively admissible, **supported by the public weighting**, decodable at
> the declared tolerance, ...

This is a two-word change in each case and it removes content rather than adding
it, which is the right direction for an axiom. Nothing downstream that actually
uses Born weights loses anything, because the weighting is still there — it is
simply no longer *named* by a clause that precedes its derivation.

**This is an axiom change and I have not made it.** It touches the physical
sealing clauses, which are the most load-bearing text in the synthesis, and the
choice between "the axiom should be weight-neutral" and "the axiom is entitled
to import the Born rule as a declared bridge" is a decision about what kind of
theory this is. Both are defensible. Only the first makes S1 true.

---

## 5. What S1 buys once the repair is made

With no valuation in the public-interface clauses, the dagger is the only
structure converting a state into a scalar, and the chain from the R1 note runs:

| step | consequence |
|---|---|
| S1 + dagger | the public value of a state factors through `ψ†ψ` |
| hence | the public layer is typed over self-paired objects (**R1**) |
| hence | the discard is the canonical pairing (**R3**), and it is monoidal (**Q1a**) |
| hence | `Causal(P, ε) = Isom(P)` (**ledger invariance**, event 009's counted premise) |
| hence | the environment completion is the free seal — **target 3 closes** behind target 1's dagger fragment alone |
| bonus | Internal Record Actualization's "Born-weighted" is **recovered as a theorem** rather than posited |

**Honest scoping on that bonus, because it is easy to overstate.** The dagger
route forces the public weight to be a *function of the self-pairing* `ψ†ψ`. It
does not by itself fix which function: `(ψ†ψ)^k` is available for any `k`, since
scalars multiply in any monoidal category. So this route delivers the Born
**form** and not the Born **exponent**.

Fixing the exponent is the separate route campaign amendment 1 item B.5
registered — orthogonal additivity over public channels plus continuity and
normalization, giving Cauchy additivity and hence the square. The two are
complementary and should be reported as such: the dagger gives the pairing, the
additivity argument gives the power. Neither needs the phase-neutrality premise
that the currently printed derivation uses, which is what keeps target 2 out of
the chain.

---

## 6. What I checked and cleared

For completeness, the clauses that do **not** supply a valuation, so the
finding is exactly two hits and not more:

| clause | verdict |
|---|---|
| IRA 2 — recoverable by an admissible decoder "to the declared tolerance" | **clear.** A tolerance is a threshold on a valuation; it presupposes one but produces none |
| IRA 3 — lock-in until replaced by a lossless declared sufficient statistic | **clear.** Losslessness is relative to retained claims, not a scalar on states |
| IRA 4 — comparisons preserve the record up to observer-code error | **clear.** Threshold again |
| `Q_p` — "inside the active record-capacity bound" | **clear.** Capacity is a property of the observer, not a valuation of states |
| Public Confluence — compatible, nondemolition refinement, public operational equivalence | **clear throughout.** No valuation anywhere in the axiom |

Public Confluence is entirely valuation-free, which is consistent with the
structural point recorded in event 009 — it is a conditional about the existence
of refinements and cannot force an equation.

---

## 7. Falsification

- **The audit missed a clause.** Kill by exhibiting a sealed clause of the
  public interface, outside the five cleared in §6, that produces a scalar from
  a state.
- **Reading A is the intended one and is defended.** If the theory means to
  import the Born rule as a declared bridge in Internal Record Actualization,
  then S1 is false permanently, ledger invariance stays a counted premise with
  no route, and event 009's registration stands unchanged. This is a coherent
  position; it just costs the derivation.
- **The repair breaks something downstream.** Any result that needs Internal
  Record Actualization to name *Born* specifically, rather than to name the
  layer's weighting, would break. I found none, but I did not audit the book
  appendices.
- **`(ψ†ψ)^k` cannot be excluded** even after the B.5 route is executed, in
  which case the Born exponent remains open and only the form is derived. This
  does not affect R1 or ledger invariance, both of which need the form only.

---

## 8. Summary

| item | status |
|---|---|
| S1 as stated | **false** on the current text — two clauses supply a valuation |
| Number of offending clauses | **exactly two**, both naming Born; five related clauses audited and cleared |
| Is the failure structural or textual? | **textual**, and repairable by a two-word weight-neutral restatement in each |
| Independent reason to repair | **yes** — as written, an input to the dependency chain presupposes an output of the layer it feeds |
| Change made? | **No.** Axiom change; flagged for decision |
| If repaired | S1 holds; R1, R3, Q1(a) and ledger invariance all follow; target 3 closes behind target 1's dagger fragment alone; "Born-weighted" upgrades from posited to derived, in form though not in exponent |
| If not repaired | ledger invariance remains a counted premise with no route, and event 009 stands unchanged |

One line: **S1 fails on two words, and those two words are also a circularity in
the theory's own dependency chain — the fix is the same fix.**
