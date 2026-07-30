# PEN contextual completion (generic prototype)

This standalone nested workspace implements a finite, generic test bed for the
proposed `gsc-contextual-internalization-v1` profile. It has no adopted semantic
authority, cannot access the live registered prefix, does not generate
candidates, and cannot count as production payment.

The prototype has two deliberately separate layers:

1. `pen-kernel` checks a one-entry dependent-context extension, its projection
   images, and a genuinely nonidentity weakening of sealed generic public
   content.
2. A bounded finite-set interpretation exhaustively checks anonymous operations
   on powerset preorders. For a supplied finite projection, inverse image is the
   reindexing operation. The verifier checks explicit left- and right-universal
   certificate data: operation tables, units, counits, hom equivalences, and
   both triangle identities.

The finite powerset model is an executable falsification fixture, not a
universal context grammar or a proof for the repository's full dependent
calculus. In particular, it does not establish a natural-family/univalent
presentation quotient, total specialization to the live local ports, or exact
free sealing.

## Exact theorem boundary

The crate verifies only the following bounded statement: for a kernel-checked
single-entry projection and a separately bound finite map of at most six atoms
per side, the supplied tables are exactly the inverse-image functor's
powerset-preorder left and right adjoints, with exhaustive unit, counit,
hom-equivalence, and triangle data. Naturality in this thin finite category is
forced by monotonicity and uniqueness of each hom, but that fact does not lift
itself to the dependent syntax.

The following adoption blockers remain:

- a common general context-map/substitution calculus rather than this
  single-extension weakening fragment;
- a universal context grammar and canonicity theorem connecting syntax to all
  finite interpretations;
- natural-family and presentation-quotient completeness (atom order is still
  presentation data in the model binding);
- substitution stability in the full dependent calculus;
- total specialization of both universal directions to the existing local use
  and computation ports;
- a universal exact-free-seal certificate and public demand-census replay; and
- an independent Agda implementation agreeing on the generic suite.

Consequently, a successful value from this crate is not a production theorem,
an adopted semantic digest, or authority to run on the registered prefix.

Run the isolated checks with:

```text
cargo test --manifest-path crates/pen-contextual-completion/Cargo.toml --locked
cargo clippy --manifest-path crates/pen-contextual-completion/Cargo.toml --all-targets -- -D warnings
```
