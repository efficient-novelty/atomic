# SH1 T4 Blind Valuation Specification: Validity Domain of L1

## Decision

The formula

\[
\nu_H(p)=1+d^2
\]

is **not an unconditional novelty multiplier for every syntactically declared
`d`-cell**. It is the cardinality of the single-constructor L1 computation
basis, and it becomes law-level credit only after the typed marginality and
provenance audit certifies every member of that basis.

For one path constructor `p`, put

\[
\mathcal B_d(p)=
\{\beta_p\}\;\sqcup\;
\{K_a:1\le a\le d\}\;\sqcup\;
\{K_{a,b}:1\le a,b\le d,\ a\ne b\}.
\]

Thus `|B_d(p)| = 1 + d + d(d-1) = 1 + d^2`. The strengthened
Selective Law assigns the path/computation contribution

\[
\boxed{
\nu_H^{\mathrm{path}}(p\mid x)
=
\left|[\mathcal B_d(p)]\cap\operatorname{Marg}_2(H;x)\right|
}
\]

only when the complete typed audit and `EGP_H(x)` certificate exist. Here
`[B_d(p)]` means the normalized natural-family classes represented by the L1
basis. The notation `nu_H` in L1 is the path/Kan/computation channel called
`nu^K` in the formal appendix.

Consequently, the frozen sources establish the closed form `1 + d^2` through
the following preconditions. Conditions 1-2 are L1's stated proof domain;
conditions 3-4 are the additional law-level earning conditions.

1. **L1 cubical domain.** `B` contains the dependent `Pi`/`Sigma` core and path
   structure with `coe` and `hcom` over a cofibration lattice for which
   union-staging holds. The extension adds a single constructor `p` of
   dimension `d >= 1`, a generator of `Omega^d(A,a)` with constant boundary,
   where `A` is the type being declared, and no nontrivial `d`-cell of that
   boundary type is derivable in `B`.
2. **Theory-relative independence.** Generator transposition is only
   propositional, not judgmental: `p^T` is not definitionally `p`. Otherwise
   ordered sites can collapse and `1 + d^2` is not the proved count.
3. **Typed marginality.** A complete normalizer proves that the beta,
   monodromy, and variation judgments are distinct normalized natural-family
   classes and that none lies in the image of weakening
   `wk_x : Sch_2(H) -> Sch_2(I(H,x))`.
4. **Individual provenance.** The global EGP injection assigns each such
   marginal class a distinct, witnessed provenance tag. The tag must be either
   a separately paid local kernel-role tag whose clause is the first
   irreducible fresh source of the family, or a particular required output of
   a live demand orbit that existed before `x`.

The first two conditions establish the raw L1 basis. The last two are what
turn raw capacity into Selective-Law novelty.

## Failure rule

The formal law is sitewise and fail-closed; it does not justify a fabricated
binary rule `1 + d^2 otherwise 0`.

The failure value of the **certification precondition** is therefore exact:
without a complete typed audit and EGP certificate, `nu` is **undefined** and
the candidate is not ranked. Zero is assigned only where the audit positively
proves that a purported site is not marginal; it is not a substitute for a
missing proof.

- A basis site that normalizes into the weakening image, is an instance of an
  already counted natural family, or is definitionally staged/composite adds
  **zero** novelty. The numeric contribution is the number of remaining
  marginal basis classes. If every L1 site is old, the path contribution is
  `0`.
- If an asserted new site is marginal but the audit cannot give it a valid,
  distinct EGP anchor, or if the required typed normalizer is absent, the
  candidate's novelty is **undefined**, not zero, and the candidate cannot be
  ranked by the Selective Law.
- If an L1 semantic hypothesis itself fails (nonconstant boundary, a different
  cubical equality theory, missing union-staging proof, multiple constructors
  without a cross-constructor disjointness proof, and so on), L1 supplies no
  replacement cardinality. The law must use a fresh typed audit; it may return
  a partial count, zero, or fail closed as just described.

This distinction is required by the frozen formal axioms: generative novelty
is `|Marg_2(H;x)|`, and EGP is required for every member of that set. The law
does not permit silently dropping an unanchored marginal family while still
ranking the candidate.

## Dependence on the sealed attachment

Mere attachment to an already-sealed object earns no credit, and the number of
sealed objects mentioned is not a multiplier.

What the sealed library must contain for the L1 construction is the **generic
cubical filling apparatus**: dependent `Pi`/`Sigma`, path structure, `coe`, and
`hcom` with union-staging. It need not contain a pre-existing nontrivial
`d`-cell of the same boundary type. Indeed, L1's stated freshness hypothesis is
the opposite: no such nontrivial cell is derivable in `B`. The directions and
probes in `d^2` are the ordered directions of `p` itself, not `d` previously
sealed strata.

Already-sealed structure matters only through two audited mechanisms.

1. **Weakening:** if the purported schema is an old family transported,
   renamed, specialized, or univalently re-presented through the attachment,
   it remains in `im(wk_x)` and earns zero.
2. **Pre-existing demand provenance:** a genuinely marginal family may be
   anchored to a specific `(o,q)` only when `o` is a live semantic demand orbit
   in `LiveOrb(H)` and `q` is one of its independently required outputs. A
   demand first created by `x` cannot serve as inherited provenance. A focus
   label, a reference name, or a cardinal slot is not such an orbit witness.

Alternatively, the family may be paid locally by an irreducible clause/role
tag; that route requires no matching old `d`-cell. Thus the exact dependence is
on **semantic weakening and certified pre-existing demand outputs**, not on
geometric contact or reference count.

This also separates L1 from L2. L2's `r^2` matrix is expressly conditional on
`r` distinct sealed strata forming the strongly connected reference set of a
nontrivial weave. L1's `d^2` matrix is an ordered direction self-pairing of one
cell and has no analogous `r`-reference precondition. Likewise, P6's raw
uniform-action description does not make specializations at library entries
independent novelty: under the strengthened law, those specializations must be
independently exported, inequivalent family classes with distinct provenance.

## Proof status

- **L1:** proof sketch, not a completed cubical-semantic proof. Its finite Rust
  enumeration and Agda cardinality theorem are conditional on the missing
  schema/basis isomorphism. Availability, independence (especially the
  transpose obstruction), exhaustiveness, and union-staging remain semantic
  obligations.
- **Law-level conversion:** conditional theorem of the strengthened Selective
  Law once the typed marginal-family audit and EGP certificate are supplied.
  EGP itself is an explicit additional axiom; it is not derived from the
  Constitutive Law, the earlier PEN rules, free sealing, or univalence.
- **Debt-guard theorem:** does not settle this valuation. Its halt criterion
  explicitly leaves L1's validity domain and dimension-`d` filling capacity as
  a pre-registered valuation obligation.
- **Multi-constructor extension:** the evaluator's per-constructor sum is
  numerically trace-conservative, but L1's formal checker remains a
  single-constructor statement. Cross-constructor disjointness still requires a
  semantic certificate.

## Genesis 1-15 conservativity gate

The permitted replay checks passed:

```text
cargo test -p pen-eval runtime_axioms::tests::genesis_one_through_fifteen_is_value_conservative -- --exact --nocapture
result: 1 passed, 0 failed

cargo test -p pen-eval semantic_provenance::tests::canonical_fifteen_scores_replay_but_revised_audit_stays_unproved -- --exact --nocapture
result: 1 passed, 0 failed
```

This passes the requested numerical regression gate,
`sum(nu_1..nu_15) = 359` and `sum(kappa_1..kappa_15) = 64`. No Step-16 search or
candidate output was run or inspected.

The check is conservativity, not semantic completion. The formal appendix
explicitly distinguishes replaying the fifteen legacy numbers from completing
the typed orbit-level EGP re-audit of all historical candidate cones, orders,
and winner certificates. That stronger re-audit remains open.

## Frozen-source basis

This specification uses only:

- `C:\DEV\book\appendices\app_a_two_laws_formal_axioms.tex`
- `C:\DEV\atomic\docs\EVALUATOR_DERIVATION.md`, only D1-D3 and P6 plus the
  immediately needed definitions
- `C:\DEV\atomic\docs\LEMMA_L1_D_SQUARED.md`
- `C:\DEV\atomic\docs\LEMMA_L2_R_SQUARED.md`
- `C:\DEV\book\debt_guard_theorem.pdf`, extracted directly from the six-page
  PDF and visually checked page by page

No bar value and no Step-16 candidate/survivor material was consulted.

### Protocol disclosure

One locator-only `Select-String` pass over the allowed evaluator file returned
four brief status snippets outside the requested D1-D3/P6 ranges: the package
test status at line 309, the Step-15 replay-ablation status at line 316, the
Step-1-through-15 reference-fixture status at line 318, and a numerical-parity
test-status fragment at line 333. None contained Step-16, bar, candidate, or
survivor information; none was used in this derivation. All substantive
evaluator evidence used above is confined to D1-D3 and P6.
