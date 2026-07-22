# Step-16 dual certificate

**Date:** 2026-07-18. **Status:** the finite implication and audit boundary are
implemented, but the mathematical global-halt theorem remains conditional on
typed semantic certificates that the present shallow AST cannot construct.

**Selective-Law version note.** The immutable EGP-v1 burned reselection is
`docs/semantic_reselection.json`: it first diverges at the Stage-1 score and
halts at Stage 2. It is evidence against EGP-v1, not evidence against every
typed Selective Law. The fresh-declaration/generative-basis EGP-v2 law and its
score-blind Stage 1--4 gate were preregistered separately in
`docs/EGP_V2_BOOTSTRAP_PROGRAM.md`. The burned result is recorded in
`docs/EGP_V2_BOOTSTRAP_RESULT.md`: Stages 1 and 2 re-enact with score `1`, but
the run halts at Stage 3 because its one-atom completion has density `1` below
the revised bar `4/3`. The bootstrap gate fails; there is still no revised
Bar16 or global-halt conclusion.

## Result

The machine check now reports three different propositions separately:

| proposition | result | scope |
| --- | --- | --- |
| Shipped structural evaluator at Step 16 | **SAT** | unconditional; four concrete raw-surface witnesses replay every implemented gate |
| New proof-carrying support-local calculus at Step 16 | **UNSAT** | unconditional for the calculus as defined in code |
| Intended semantic Genesis sequence halts after Step 15 | **not yet proved** | requires anchored orbit extraction, historical reselection, internality witnesses, and Step-16 exhaustion |

The distinction is part of the serialized certificate. In particular,
`original_global_halt_proven` is `false`; the checker cannot convert a
conditional theorem into a global claim.

## 1. Permanent shipped-evaluator falsifiers

`halting_probe.rs` and `step16_automaton.rs` replay these exact, raw-generable,
admitted, shallow-type-correct, connected, semantically minimal clearers:

| witness | kappa | `(nu_G,nu_C,nu_H)` | `nu` | `rho` | semantic defect exposed |
| --- | ---: | ---: | ---: | ---: | --- |
| `hit_no_formation_d1` | 2 | `(0,17,2)` | 19 | `19/2` | whole-library HIT credit without formation or typed eliminator |
| `temporal_polymorphic_kappa2` | 2 | `(0,32,0)` | 32 | `16` | whole-library temporal multiplier without typed polymorphism/naturality |
| `axiomatic_single_l15_kappa3` | 3 | `(1,106,0)` | 107 | `107/3` | graph premise holds, but irreducible API and typed lift are absent |
| `axiomatic_inheritance_kappa3` | 3 | `(1,107,0)` | 108 | `36` | direct imports `{L14,L15}` have no reachability-dominant member |

This is a complete SAT proof because one replayed witness would suffice. It
is not an exhaustive maximum calculation for the shipped evaluator. The
finite-signature certificate says so with
`exhaustive_telescope_quotient=false`, `global_maximum_certified=false`, and
`unsat_certificate_available=false`.

The expression automaton nevertheless closes a useful engineering gap. It
counts the actual position-specific scopes `2 + position` through six nodes,
uses checked `u128` arithmetic, reproduces the terminal catalog widths
`910182`, `1207872`, and `1559142`, and bisimulates the concrete enumerator on
a materializable small cap. Its digest and full-definition replay reject a
mutated witness, count, or completeness flag.

## 2. Proof-carrying semantic boundary

`certified_novelty.rs` is a sidecar evaluator; it does not silently alter the
historical `structural_nu` implementation. It normalizes direct support,
ambient parameters, prior fresh heads, and stuck fresh-head eliminators.

Declarations have two routes:

- A transparent declaration needs a private, candidate-and-realizer-bound
  typed-elaboration token. Canonical syntax equality alone cannot mint it.
- An opaque declaration explicitly asserts a D3 fresh kernel. That assertion
  can earn only the independently bounded local kernel credit.

The three historical amplification routes additionally require private
semantic tokens that the shallow AST cannot construct:

- **H-form:** a real `Formation` clause, exact beta/Kan basis, and a typed
  eliminator token.
- **P5:** an irreducible minimal complete API, unique reachability-dominant
  direct import, exact bridges/local lifts, and a typed lift/eliminator token.
- **Synthesis:** exact detected sites by direct support, a typed polymorphic
  eliminator token, and a naturality token.

No public token constructors exist. A future dependent elaborator/reduction
kernel must be connected to issue them. This prevents an AST pattern from
manufacturing a semantic theorem.

## 3. Exact corrected-calculus bound

The frozen surface caps are derived without consulting the bar:

```text
2 <= kappa <= 4
r <= 2                    (only L14 and L15 are raw leaves)
d <= 1
max expression nodes = 6
```

Every one of the nine corrected class ceilings is enumerated for every
`kappa`. The resulting maxima and the independently derived clearing
thresholds are:

| kappa | certified maximum `nu` | maximum `rho` | first integer `nu` that clears | margin |
| ---: | ---: | ---: | ---: | ---: |
| 2 | 8 | 4 | 19 | 11 |
| 3 | 11 | `11/3` | 28 | 17 |
| 4 | 14 | `7/2` | 37 | 23 |

Thus the smallest bar-independent integral envelope is `nu <= 4*kappa`, and
every class lies strictly below

```text
Bar16 = 354333/39040.
```

The checker uses integer cross multiplication for every one of the 27 cases.
It emits a deterministic BLAKE3 digest and re-derives the whole payload from
definitions during replay.

## 4. Independent Agda check

`agda/CertifiedHalt.agda` contains two layers:

1. the general first-fresh-head/support-local finite code, with cardinality
   `k + 2*k*r + k*k` and the conservative Step-16 envelope `14/24/36`;
2. an independent copy of the exact nine-class calculus, whose maxima reduce
   to `8/11/14`.

Agda checks all 27 class cases and the three direct comparisons

```text
39040 * maximum(kappa) < 354333 * kappa.
```

It also keeps the missing semantic theorem visible as
`SupportLocalSemantic`: callers must supply a classifier and prove that it is
injective. No realizer is required, because an upper bound must not force a
sparse candidate to realize every possible interaction tag. The finite
arithmetic is proved; faithful classification of the intended operational
schema quotient is not assumed.

`agda/ProvenanceBound.agda` states the stronger Selective-Law boundary. A
marginal schema must inject into either a charged `(local role, kernel clause)`
tag or a tag attached to a previously live semantic-demand output. The record
also requires a family-indexed `ValidAnchor` proof; an arbitrary injection into
anonymous slots is insufficient. Its debt-free specialization gives
`AtMost Marginal (4*κ)`. The displayed `9*κ` theorem is only a weakening of
that same valid four-role certificate, not an independent nine-role route.

## 5. Conservativity

Running the proof sidecar before and after the reference replay leaves the
fifteen structural records identical:

```text
accepted steps = 15
sum nu          = 359
sum kappa       = 64
replayed Bar16  = 354333/39040
```

The import audit also preserves the historical P5 graph premises: Step 13 has
dominant import `L12`, and Step 14 has dominant import `L13`. This is
implementation conservativity: the shipped Genesis history is unchanged. It
does not claim that all fifteen historical scores have already been rederived
inside the new semantic calculus.

## 6. Remaining theorem obligation

The remaining steps are precise rather than hidden in the arithmetic:

```text
1. Extract all typed natural schema families and individual demand orbits,
   modulo normalization, weakening, and univalent equality.
2. Prove window locality/expiration and the J2/J3 disposition of every orbit.
3. Supply a valid anchored injection for every counted family, or a
   weakening/erasure inverse certificate for the internal branch.
4. Re-run all fifteen candidate cones, orders, winners, and scores under the
   strengthened law; reproducing the old numbers alone does not preserve the
   trace or Bar16.
5. Prove that the resulting Step-16 cone is exhausted by the internal and
   EGP-certified opaque cases.
```

The conditional Rust auditor in `semantic_provenance.rs` makes this boundary
executable without pretending that user-supplied theorem names are checked
proofs. Its generated artifact reproduces the legacy vector
`1,1,2,5,7,8,10,18,17,19,26,34,46,62,103`, but every
`revised_audited_nu` is `null`, semantic O(16) is unknown, and the J2/J3 and
internality flags remain false. Separately, the EGP-v1 full reselection burned
at Stage 2 as described above. EGP-v2 changes the counted object from marginal
families to a certified independent generative basis, so it must produce a new
artifact rather than overwrite either result. No saturation or realization
theorem is needed for the eventual upper bound. Until the obligations above
are discharged, the strongest correct conclusion is:

```text
shipped evaluator: SAT
proof-carrying calculus: UNSAT
intended global halt at fifteen: conditional, not closed
```

## Reproduction

```powershell
cargo test -p pen-eval --lib
cargo test -p pen-search step16_automaton --lib
cargo test -p pen-search certified_halt --lib
cargo test -p pen-search t1_adversarial_probe_records_only_raw_minimal_clearing_survivors --lib
cargo test -p pen-agda --lib
cargo test -p pen-cli --test agda_export
agda -i agda agda/CertifiedHalt.agda
agda -i agda agda/ProvenanceBound.agda
agda -i agda agda/StepWitness.agda
cargo run -p pen-search --release --example genesis_certified_halt -- --out docs/certified_halt_verification.json
cargo run -p pen-eval --example semantic_provenance_audit -- --out docs/semantic_provenance_audit.json
```
