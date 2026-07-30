# Law V2 Semantic Audit Core V1 Prototype Result

**Status:** `PROPOSED_PROTOTYPE_RESULT_NOT_ADOPTED`

**Date:** 2026-07-29

**Authority:** none; generic isolated prototype evidence only

## Result

The repository now contains an isolated generic prototype of the proposed
`gf2-semantic-audit-core-v1` and `gf2-kernel-cost-core-v1` calculi under
`crates/pen-semantic-audit`.

This is not an adopted semantic measurement theory, a frozen verifier, or an
Acts 1--4 audit. Both proposed manifests remain unfrozen and explicitly deny
live Profile A access. No \(\kappa\), \(\nu\), ratio, benchmark, or
Selective-Law jurisdiction value is issued by this result.

The three governing choices for a future audit remain recorded only in the
proposed-not-adopted adjudications:

- [semantic audit calculus](LAW_V2_SEMANTIC_AUDIT_CALCULUS_ADJUDICATION_V1.md);
- [kernel cost](LAW_V2_KERNEL_COST_ADJUDICATION_V1.md); and
- [bootstrap window jurisdiction](LAW_V2_BOOTSTRAP_WINDOW_JURISDICTION_ADJUDICATION_V1.md).

## Prototype surface

The isolated crate exercises generic finite-carrier, restricted
normalization, Q1/Q2 quotient, kernel-cost saturation, marginal-set, and SR2
reference paths. It also contains an abstract safe-Agda reference module and
an independently pinned live-Agda readiness gate.

These components are implementation research. Passing their current unit
vectors does not prove carrier completeness, the exact fresh free-completion
theorem, typed weakening, or Rust/safe-Agda equivalence.

## Fail-closed fresh completion

The required exact theorem must show that the complete public rewrite API is
uniquely reconstructed from a verified predecessor public boundary, a
pre-existing typed computation-demand contract, the frozen cost rules, and
the bodyless fresh head. It must establish typing and substitution stability,
termination and confluence, conservativity, exact deletion/regeneration,
absence of extra equations, a unique-extension universal property, and
Q0/Q2/Q3 transport.

The current structural theorem tags and shape checks do not discharge those
obligations. They therefore have no adoption or live-audit authority. The
repaired generic cost path now returns
`Unknown(MissingFreeCompletionTheorem)` for the certified-fresh-equation
vector. It must continue to do so until an opaque, input-bound theorem
capability is implemented and independently reviewed. Failure to prove
completion is not proof that the equation is first-irreducible.

## Open theorem and input gates

The prototype cannot be frozen or exposed to Acts 1--4 until at least the
following are complete:

1. A verified, canonically ordered public inventory bound to exact predecessor
   and successor boundaries, with total public group/equation coverage.
2. A predecessor-visible typed demand contract and public-availability proof,
   rather than caller-supplied availability labels or post-hoc descriptors.
3. The exact fresh free-completion theorem described above.
4. A complete finite carrier theorem and replayable carrier/quotient
   certificates.
5. Structural typed family weakening, normalization and quotient transport,
   and a verified restriction retraction.
6. An exact marginal-family calculation bound to verified pre/post carriers.
7. SR2 support and injection bound to the derived carrier, quotient, and exact
   first-irreducible cost certificate.
8. A digest-bound verified-empty origin-cutoff Q3 registry.
9. Executable Rust/safe-Agda agreement on the complete prescribed generic
   vector suite, followed by independent review and manifest freeze.

## Validation snapshot

The following generic checks were run for this documentation-only closeout:

```text
cargo test --manifest-path crates/pen-semantic-audit/Cargo.toml --locked
```

Result: 31 tests passed and one live Agda 2.8.0 test was ignored because it
requires the independently pinned local runtime.

The ignored-by-default test was then exercised through the explicit pinned
example:

```text
cargo run --manifest-path crates/pen-semantic-audit/Cargo.toml \
  --example verify_agda_reference --locked
```

That separate run passed and returned:

```text
blake3:33f78fa03eac42a22950a209fb8669ccdeec325c799473b01606115bab3f4c32
```

The fixed safe-Agda module also passed a separate direct Agda typecheck.

```text
cargo clippy --manifest-path crates/pen-semantic-audit/Cargo.toml \
  --all-targets --locked -- -D warnings
```

Result: passed.

```text
python scripts/check_semantic_audit_isolation.py --json
python -m unittest scripts.test_check_semantic_audit_isolation
```

Result: the nested lockfile and root-workspace isolation checks passed; the
two isolation unit tests passed.

This snapshot establishes ordinary prototype regression, fixed-reference
typechecking, pinning, and isolation status only. The default Rust suite still
reports the environment-dependent live test as ignored; the explicit example
is the separate successful invocation of that gate. Neither invocation
compares Rust semantic outputs with Agda outputs, so no Rust/safe-Agda
agreement theorem is claimed.

## Parallel contextual-prototype snapshot

The separate `pen-contextual-completion` lane remains generic,
proposed-not-adopted, and unable to access the registered prefix. For this
closeout its 12 Rust tests passed; `cargo fmt --check` and clippy with warnings
denied passed; and its isolation checker and two isolation unit tests passed.

Those results validate only its bounded finite generic model. They do not
close its context-grammar, specialization, quotient-completeness,
exact-free-sealing, or independent Agda-agreement gates, and they grant no
Profile A execution authority.

## Preserved authority boundary

This work does not modify or supersede:

- the issued Profile A H3 result;
- the issued Profile A H4 continuation and debt-free halt result; or
- [the window-register audit](LAW_V2_WINDOW_REGISTER_AUDIT_V1.md).

Their bound artifacts and digests remain unchanged. The window-register
outcome remains `UndefinedAudit`, with no provisional \((\kappa,\nu)\) values
inserted.

## Stop condition

The prototype lane stops at a definition-and-theorem blocker. The next lawful
critical-path object is a proof-carrying verified public inventory and an
exact, input-bound free-completion theorem. More generic examples, assertion
tags, Boolean certificate fields, or digest labels cannot replace those proof
objects.

No live Profile A execution is authorized from this result.
