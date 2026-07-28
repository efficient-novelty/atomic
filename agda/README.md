`agda/` holds the sidecar verification surface only.

The generated modules are deliberately conservative: each accepted step is exported as
an Agda witness module with structural MBTT comments, candidate fingerprints, and a
machine-readable manifest. The hot search loop never depends on Agda.

The hand-written baseline support modules are:

- `BridgePayload.agda`
- `AbstractionBarrier.agda`
- `CountingLemmas.agda` (conditional finite-cardinality proofs for L1/L2)
- `ProvenanceBound.agda` (injection-based debt-credit and local-role bounds)
- `P5RecordBoundary.agda` (conditional internality plus the failed P5 import-DAG premise)
- `CertifiedHalt.agda` (proof-carrying amplification boundary and support-local Step-16 bound)
- `StepWitness.agda`

The separate `LawV2/H3Compatibility.agda` module is an
`ORACLE_ONLY_PROVISIONAL` compatibility experiment. It hand-encodes the
anonymous H3 boundary and, under explicit probe-local assumptions, defines a
dependent-core family with the proposed unit-use shape. Constructor
completeness and computation mode are encoded inputs, not derived history
evidence; GSC birth, fixed-type, and parameter-projection support remain
unassigned. The module does not execute an exact or adopted GSC compiler.

Its direct eliminator and beta proof use Agda's separate host datatype
`One`/`star`; the module proves no bridge from that datatype to registered
`g2`/`g3`. These definitions are therefore an analogous shape model, not a
typed registered-H3 response or GF2 discharge certificate. The module is not
copied by `pen-cli export-agda`, is not part of the production GF2 backend, and
confers no GSC or Law-V2 authority.

`CountingLemmas.agda` proves the equations `1 + d²` and `2κ + r²` from
explicit schema/basis isomorphisms.  The isomorphisms are hypotheses: the
current MBTT AST has no cubical `coe`/`hcom` semantics or fibration witness
with which to discharge availability, independence, and exhaustiveness.
It also defines the weaker `AtMost A n` boundary: an injection from `A` into
an explicit finite code of size at most `n`. Upper bounds use `AtMost`; they do
not require every finite tag to be realized.

`ProvenanceBound.agda` formalizes extraction-guarded debt-credit provenance.
An actual marginal schema must inject either into a charged `(local role,
kernel clause)` tag or into a code supplied by previously live demand orbits.
The certificate also carries a `ValidAnchor` proof for every schema/tag pair,
so an arbitrary injective numbering is insufficient. When debt is empty, only
the local product remains. A semantic certificate covering the four displayed
local-role slots gives `AtMost Marginal (4*κ)`; weakening that same certificate
gives the bar-blind envelope `AtMost Marginal (9*κ)`. The latter is not an
independent nine-role fallback. Both results are conditional on the faithful,
valid classifier, which Agda does not infer from a class label or the current
shallow syntax.

`P5RecordBoundary.agda` separates two statements that cannot be conflated:
transparent elaboration implies marginal novelty zero, while P5-record credit
requires a reachability-dominant direct import.  It proves that Steps 13 and 14
have the required dominant imports and that the Step-16 `{L14,L15}` candidate
does not.  It does **not** invent the missing transparent-elaboration witness;
the current AST has no typed definitions or reduction semantics from which to
construct one. Given weakening and erasure with both inverse laws, however,
it now proves directly that the type of non-weakened marginal schemas embeds
in the empty finite code, hence is `AtMost 0`.

`CertifiedHalt.agda` parameterizes the three amplification records by external
evidence propositions (HIT formation, P5 irreducibility/dominance, and typed
temporal naturality); it exports no freely inhabited evidence token. It gives the
support-local depth-2 code, and checks its nine finite Step-16 cap cases. The
widest code bounds are `14`, `24`, and `36` for clause costs `2`, `3`, and `4`.
The semantic boundary is now a faithful injection into that code, not an exact
isomorphism. It separately mirrors all 27 cases of the tighter proof-carrying
class calculus, obtaining exact maxima `8`, `11`, and `14`, and checks their
cross-multiplied strict inequalities against `Bar16 = 354333/39040`. It also
connects the conditional four-role provenance bound to the independent
`9*κ < Bar16*κ` arithmetic. Constructing the operational classifier remains
an explicit proof obligation rather than an AST-shape axiom.

`pen-cli export-agda` copies those support modules into each output directory so the
generated `PayloadNN.agda` and `StepNN.agda` files form a self-contained verification
bundle.

Typical entry points:

- `cargo run -p pen-cli -- export-agda --until-step 15`
- `cargo run -p xtask -- export-reference-agda 15`

If the `agda` executable is available, `pen-cli export-agda --verify` will also write
per-step verification logs next to the generated modules. If it is not available, the
export manifest records verification as skipped instead of pretending it passed.
