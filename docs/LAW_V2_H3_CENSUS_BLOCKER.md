# Law V2 H3-to-Stage-4 census blocker

**Date:** 2026-07-28
**Status:** `blocked_pending_versioned_scheme_calculus_adoption`

The registered Law-V2A three-act bootstrap now replays successfully, but the
first guarded demand census cannot yet be computed with law-level authority.
The blocker is not a search limit or a missing Rust loop. The adopted sources
do not define the complete stage-generic relation that generates
\(C(S_3,S_2)\).

## What is already fixed

The current contract supplies:

- the width-two equation
  \(O(n+1)=C(S_n,S_{n-1})\setminus D(B_n)\);
- future holes as dependent open judgments with declared motives and
  per-registration total-specialization obligations;
- the adopted `schema2-operational-domain-v1` bounds for the intended
  \(\mathsf{NfSch}_2(H)\) carrier: alphabet from the sealed boundary, semantic
  depth two, and context/binder/dimension maxima from the sealed record; and
- a clean, anonymous, kernel-replayed Law-V2A bootstrap.

See:

- [the normative appendix](app_a_two_laws_formal_axioms.tex);
- [the future-hole adjudication](future_hole_definition_adjudication.md);
- [the operational-domain adjudication](schema2_operational_domain_adjudication.md);
- [the executable plan](autonomous_genesis_plan.md); and
- [the certificate boundary](LAW_V2_CERTIFICATE_BOUNDARY.md).

Those rules delimit possible typed schemas and explain how an already
registered scheme specializes. They do not determine which schemas are
authoritative demands of a particular active window.

## The missing formal rule

No adopted source currently provides all of the following:

1. an exact versioned constructor set for the demand-scheme calculus;
2. typing premises and conclusions for every constructor;
3. the seed relation from the two sealed window events and current public
   boundary;
4. the support-touching predicate that grants a generated scheme jurisdiction
   in the active window;
5. the closure order and fixed-point rule;
6. the family, substitution, naturality, and equivalence quotient used during
   generation;
7. an equation identifying the generated result with
   \(C(S_n,S_{n-1})\); and
8. an induction principle from which total specialization, equivariance,
   weakening, locality, expiration, and generator exhaustiveness can be
   proved.

The constructor names in the plan (`FormationUse`,
`IntroductionElimination`, `ComputationClosure`, `NaturalitySquare`,
`AdjointMate`, `SupportAction`, `CoherenceCell`, and
`WindowCompatibility`) are introduced only “for example.” They have no
adopted inference rules. Turning those names into rules in production code
would therefore add a new law rather than implement an existing one.

## Why the apparent shortcuts are invalid

- Treating every bounded \(\mathsf{NfSch}_2(H_3)\) member as a demand would add
  an unadopted equation \(C=\mathsf{NfSch}_2\).
- Treating the current three native `pen-demand` constructors as complete
  would confuse verification of caller-supplied schemes with exhaustive
  generation.
- Treating the absence of registered schemes as an empty census would produce
  a false halt: absence of input is not extraction completeness.
- Importing the historical A3 `former_eliminator` record or exact prefix
  machinery would violate the oracle firewall. Its published exhaustiveness
  is relative to the authored historical grammar, and its absolute semantic
  exhaustiveness is explicitly false.
- Choosing finite caps for convenience cannot repair the gap. The adopted
  operational-domain rule permits only bounds derived from the sealed record
  and separately requires normalization, quotient, and enumeration proofs.

The current operational-domain replay confirms the remaining proof gaps:
[T-D2-1 v3](T_D2_1_OPERATIONAL_DOMAIN_V3_RESULT.md) reports context
finiteness, typed-normalization totality, and natural-family quotient
decidability/finiteness as false, and
[T-D2-2 v2](T_D2_2_OPERATIONAL_SURJECTIVITY_V2_RESULT.md) stops before
surjectivity.

## Exact condition for resuming

Implementation can resume after a versioned, adopted scheme-calculus contract
freezes items 1–8 above and states whether it succeeds or replaces the
existing future-hole and operational-domain adjudications.

The first code milestone after that adoption is:

1. encode the adopted constructors and seed relation without stage or semantic
   labels;
2. generate and intrinsically register the complete \(H_3\) scheme set;
3. prove total specialization by induction on scheme derivations;
4. enumerate the history-derived GF2 carrier and prove finite normalization
   and quotient decisions;
5. prove generator coverage, weakening, locality, and expiration;
6. compute \(C(S_3,S_2)\setminus D(B_3)\); and
7. issue either a verified live-orbit ledger or `Unknown`, never an inferred
   empty census.

Until then the lawful engine outcome is `Unknown`. No Stage-4 multiplicity,
blockage, or halt follows.
