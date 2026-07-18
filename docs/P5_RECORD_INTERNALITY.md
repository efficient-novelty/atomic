# P5-record / internality boundary for the Step-16 candidate

**Date:** 2026-07-18. **Candidate:** `axiomatic_inheritance_kappa3`.
**Machine checks:** `crates/pen-eval/src/p5_record.rs` and
`agda/P5RecordBoundary.agda`.

## Result

The candidate is **not in the stated domain of the P5-record theorem**.  Its
direct imports are `{L14,L15}`, and neither import reaches the other in the
accepted Genesis dependency DAG.  There is therefore no unique
reachability-dominant `L_max`.  The evaluator's use of `nu(L15)=103` is a
syntactic `max_ref_nu` convention, not an instance of Telescopic Elimination.

This is a theorem-domain result, not an internality result.  The present MBTT
surface does not determine whether the three displayed clauses are transparent
definitions over `B15` or fresh opaque declarations.  Consequently it proves
neither unconditional `nu=0` nor unconditional `nu=108`.

## 1. The two disjoint theorems

Let `Cl2(B)` denote depth-at-most-two derivation schemas over `B`, quotiented by
the operational definitional equality intended by D1-D3.

The internal branch requires a transparent elaboration witness: every proposed
declaration has a prefix-aware, well-typed closed realizer over `B`; it reduces
under the old operational semantics; and all advertised computation rules are
inherited.  Such a witness supplies inverse maps

```text
Cl2(B)  <->  Cl2(B + X)
```

for weakening and erasure.  Hence

```text
Transparent_B(X)  =>  Internal_B(X)  =>  nu(X | B) = 0.
```

`P5RecordBoundary.agda` packages the inverse laws as
`transparent-conservativity` and derives zero through
`transparent-internality`.  Its `TransparentExtension` argument is deliberate:
the current AST cannot construct that evidence.

The external record branch has different, incompatible premises:

```text
MinimalCompleteAPI_B(X)
and DominantImport_B(X,Lmax)
  => nu_C(X | B) = nu(Lmax) + kappa(X) + (|Refs(X)| - 1).
```

`MinimalCompleteAPI` requires every charged field to be constructively
irreducible.  A transparent clause is excluded by that definition.  P5 values
an already-proved opaque API record; it cannot also prove that the same record
is internal.

## 2. Import-DAG proof

Orient an edge `a -> b` when the accepted telescope at step `a` contains a
direct `Lib(b)` reference.  The relevant reference DAG is

```text
11 -> 10
12 -> 11
13 -> {11,12}
14 -> {11,12,13}
15 -> 10
```

Thus

```text
reach(14) = {14,13,12,11,10}
reach(15) = {15,10}.
```

For a direct-import set `R`, call `m in R` dominant when every `r in R` lies in
`reach(m)`.  This is the operational orientation of the paper's “unique
maximal imported interface” premise; reversing the order changes the word
“maximal,” not the required subsumption relation.

- Step 13 has imports `{11,12}` and unique dominant import `12`.
- Step 14 has imports `{11,12,13}` and unique dominant import `13`.
- The Step-16 candidate has imports `{14,15}` and no dominant import.

The candidate's local `Var` chain makes a cone that mentions both siblings; it
does not prove that either imported interface subsumes the other.  In
particular, `App(Lib14,Var1)` is not a typed, fully faithful transport with
inverse laws.  The shallow checker does not even infer its dependent type.

The Rust audit computes transitive closure from the accepted telescopes and
refuses to evaluate the conditional P5 equation without exactly one dominant
import.  The Agda proof independently tabulates the small closure and proves
`no-survivor-dominant : SurvivorDominant -> Empty`.  It also constructs the
Step-13 and Step-14 dominance witnesses, so the audit is a conservativity test,
not a blanket ban on late Axiomatic packages.

## 3. Why internality is still underdetermined

The three candidate clauses are

```text
Formation     Pi(Lib15, Var1)
Formation     Sigma(Var1, Var1)
Introduction App(Lib14, Var1)
```

`ClauseRec` stores only a role and an expression.  It stores no declaration
name, definition body, opacity flag, type derivation, reduction trace, or
structured-equivalence certificate.  The current checker validates reference
and scope bounds, not dependent typing.  Canonicalization is structural, and
the “trivially derivable” predicate recognizes only a few surface forms.

The identical AST therefore admits two completions consistent with every
current check:

1. **Transparent:** the expressions elaborate to old terms and add no neutral
   heads.  Then the candidate is redundant and `nu=0`.
2. **Opaque:** the expressions are signatures for fresh abstract fields.  Then
   they may be irreducible, but P5 still needs a dominant-import/subsumption
   witness before its displayed closed form applies.

Univalence does not choose between these completions.  An equivalence of
carrier types does not make a fresh opaque operation judgmentally reducible.

## 4. Consequence for the halt claim

The two-import raw-generable engine survivor is no longer a **theorem-certified
P5 clearer**: its recorded `nu=108` applies P5 outside the theorem's graph
premise.  Under a proof-carrying admissibility policy it would remain
uncertified until it supplied either transparent elaboration evidence or a
Minimal Complete API plus import-subsumption evidence.

A second raw-generable Axiomatic witness imports only `L15`, so its graph
premise does hold. It still cannot claim P5 credit from syntax alone: the
proof-carrying layer also requires an irreducible minimal complete API and a
private typed lift/eliminator token. The shallow AST supplies neither. Thus
the graph audit rejects the two-import witness and the typed boundary rejects
unsupported inheritance for both witnesses.

That does not yet prove that Genesis stops at fifteen.  Removing an unsupported
positive score is not an upper bound on the candidate's true novelty; there is
no `Transparent_B15(candidate)` witness, and Step-16 enumeration remains
non-exhaustive.  The strongest honest conclusion is therefore:

```text
not P5-certified(candidate)
and
Transparent_B15(candidate) -> nu(candidate | B15) = 0,
```

with the antecedent of the second line still open.

## 5. Reproduction

```powershell
cargo test -p pen-eval p5_record --lib
agda -i agda agda/P5RecordBoundary.agda
cargo test -p pen-search t1_adversarial_probe_records_only_raw_minimal_clearing_survivors --lib
```
