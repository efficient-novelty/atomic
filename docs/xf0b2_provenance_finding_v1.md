> **SUPERSEDED by `docs/xf0b2_provenance_finding_v2.md`.** Retained unedited
> because the adversarial verifier audited *this* text; v2 records its
> corrections (three sub-claims overstated, one refuted) and the causal
> direction, which v2 settles and this version left open. Do not cite v1.

# XF-0b2 — Provenance finding: the subject clause is hard-coded in the search's admissibility layer

**Date:** 2026-07-24. **Status:** finding, discovered during R-1; additive to
`docs/xf0b2_r0_dossier_v1.md`. **Origin:** raised by independent armed
analyst B while checking an unrelated claim, then verified mechanically by
the coordinator. **Bearing:** decisive for the export, neutral for the
naming verdict.

## The finding

The exact syntactic shape of the subject clause,
`Pi(Lam(Var 1), Sigma(Var 1, Var 2))`, is written as a **hard-coded literal
in the search engine's own structural-admissibility layer**, at four sites:

| File | Function | Role |
|---|---|---|
| `crates/pen-core/src/library.rs` | `matches_hilbert_functional_shell` | computes the public capability flag `has_hilbert` |
| `crates/pen-type/src/admissibility.rs` | `matches_hilbert_functional_package` | the admissibility predicate for the stage-14 package |
| `crates/pen-search/src/enumerate.rs` | `supports_hilbert_functional_clause_at_position` | **the per-position enumeration gate** |
| `crates/pen-search/src/enumerate.rs` | `supports_hilbert_functional_clause` | the position-agnostic variant |

The third is the one that matters. Its `position == 3` arm is:

```rust
3 => matches!(
    expr,
    Expr::Pi(domain, codomain)
        if matches!(domain.as_ref(), Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1)))
            && matches!(
                codomain.as_ref(),
                Expr::Sigma(left, right)
                    if matches!(left.as_ref(), Expr::Var(1))
                        && matches!(right.as_ref(), Expr::Var(2))
            )
),
```

Every leaf is a concrete literal. **The predicate is a syntactic singleton:
exactly one expression satisfies it, and it is the subject family's clause.**

This gate is on the live enumeration path — `enumerate.rs` dispatches
`StructuralFamily::HilbertFunctional` to it directly — and the flag that
turns it on, `require_hilbert_functional_clauses`, is set from
`admissibility.require_hilbert_functional_package`, which the debt system
raises at stage 14 by construction (`demand_completeness.rs`: `14 =>
Some("hilbert_functional")`).

## Mechanical verification

Perturbing clause 3 and re-running the public recognizer
`LibraryEntry::from_telescope(&variant, &library)` over a library built from
steps 1..13 (raw output: `docs/xf0b2_r0_probe_output.txt`, ADDENDUM C):

| Clause 3 replaced by | `has_hilbert` |
|---|---|
| `Pi(Lam(Var 1), Sigma(Var 1, Var 2))` — **the sealed clause** | **true** |
| `Pi(Var 1, Sigma(Var 1, Var 2))` — the eta-collapse | false |
| `Pi(Lam(Var 1), Sigma(Var 1, Var 1))` | false |
| `Pi(Lam(Var 2), Sigma(Var 1, Var 2))` | false |
| `Pi(Lam(Var 1), Sigma(Var 2, Var 1))` | false |
| `Pi(Lam(Var 1), Sigma(Var 1, Var 5))` | false |

Six variants, one accepted. The recognizer admits the sealed clause and
nothing else.

## What this does and does not show

**It does not touch the naming verdict.** Whether mathematics has a name for
`Π(λ_.X, X × Y)` is a question about mathematics, and is unaffected by how
the expression came to sit in a telescope. All seven R-1 contexts returned
NO-NAME on grounds internal to the elaborated content; none of those grounds
is disturbed.

**It does defeat the export as framed.** XF-0b registered df4feb52882e as a
*candidate novel mathematical object* — the study's potential export. That
registration carries an implicit provenance claim: that the search *found*
this object. It did not select it from alternatives. At clause position 3 of
stage 14 the enumerator admits exactly one expression, hand-written in the
admissibility layer, and that expression is the object. A structure that is
a literal in the code that searches for it cannot be reported as a discovery
of the search.

**It is not specific to this family.** All nine positions of the stage-14
gate are pinned the same way, including the two credited siblings the
concordance *named* (`67134aea1691`, the pointed quiver, and `4efc2e4dac7a`,
the coalgebra for the squaring endofunctor). The same architecture appears
for the other late structural families (`operator_bundle`, `temporal_shell`,
and the connection/curvature shells). Early families are looser — the
stage-5 initial-HIT gate admits `App(Univ, Var 1)`, `Var 1`, `PathCon(1)` by
shape rather than by literal — so the pinning tightens with stage depth.

**The causal direction is not established here, and must not be assumed.**
Two readings remain open and this study does not choose between them:

1. **Post-hoc recognizer.** The literals were written after the search
   found the winner, to pin it for replay and regression — the same role
   `Telescope::reference` plays. Innocent, but it still means the *current*
   code cannot rediscover the object from alternatives, so no present run
   can be cited as evidence of discovery.
2. **A priori gate.** The literals were authored before or during the run
   and constrained it. Then the stage-14 content is a transcription of the
   admissibility layer, and the semantic register's stage-14 families
   inherit that provenance.

Settling this needs repository history — the commit order of
`enumerate.rs`'s stage-14 arms against the run that produced the sealed
stage-14 winner. That is a bounded, mechanical question and it is referred
to the campaign as such. **Until it is settled, no stage-14 family may be
described as a search discovery.**

## Consequences registered

1. **The export is withdrawn**, not deferred. Per F-R5 a withdrawn export is
   a success of the discipline, and the ground here is stronger than the
   naming question: the object's provenance does not support the claim the
   registration would make.
2. **`docs/df4feb52_specification_v1.md` is not written.** The plan makes it
   conditional on NO-NAME CONFIRMED, but its purpose is to specify a
   candidate novel object. Writing it now would assert the provenance this
   finding defeats. This note stands in its place.
3. **A campaign-level methods question is opened**, wider than this study:
   *to what extent is the certified register's late-stratum content a
   transcription of hand-authored admissibility literals rather than a
   search result?* This bears on XF-0b's headline (31 of 32 families named)
   and on any future v2 census, and must be disclosed by both.
4. **The XF-0 census result is untouched.** It was blind and never read the
   register; its 3/11 mismatch stands exactly as published.
