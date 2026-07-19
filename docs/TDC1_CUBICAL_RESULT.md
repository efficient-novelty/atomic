# TDC cubical fragment v3: conditional Step 5--8 basis replay

This is a create-new, versioned follow-up to
`tdc1-typed-d4-certificate-v1`. It also supersedes the rejected
`tdc1-cubical-step5-8-regression-v2` draft schema: that draft's boundary
witness and constant-family transport shapes were not sufficient to justify
the claimed keys, so its artifact was not frozen or retained.

## Theory-relative scope

The experiment is conditional on the versioned rule
`tdc1-pathcon-attachment-implicit-base-constant-boundary-theory-axiom-v1`.
It fixes an implicit `a : A`, a free `p : I^d -> A`, and the definitional
constant boundary `p|partial I^d = a` for a formed `PathCon(d)` attachment.
This rule is not derived from the shallow kernel AST and has not been adopted
by the user as an axiom. It is distinct from the previously adopted A5.
Consequently the output is a conditional presentation-layer experiment, not
an unconditional theorem of the current Two-Law kernel.

## What v3 machine checks

`pen-type::cubical` now provides:

- interval and constructive cofibration contexts;
- fresh-binder typed `coe` and homogeneous `hcom`, with cap-start and
  pairwise-overlap checks performed by actual restriction;
- capture-safe binder instantiation and level lowering when a dimension
  abstraction or composition dimension is removed;
- a motive-typed `PathCon` eliminator: base scrutinees compute to `b`,
  constructor scrutinees compute to `q`, and a typed neutral `x : A` gives a
  stuck term of type `P(x)`;
- dependent `coe` realizers for diagonal entries and dimension-lambda families
  for off-diagonal entries; both endpoints of an accepted off-diagonal family
  type-check and normalize to the same motive base;
- opaque source-, signature-, term-, normal-form-, and derivation-bound tokens.
  Keys are classified only after type checking and normalization. The old
  degenerate point-`hcom` shape cannot issue a key.

The conditional registered-basis replay is:

| subject | dimension | presented basis count | recorded path count | certified path nu |
|---|---:|---:|---:|---:|
| Step 5 / S1 | 1 | 2 | 2 | undefined |
| Step 6 / Trunc | 1 | 2 | 2 | undefined |
| Step 7 / S2 | 2 | 5 | 5 | undefined |
| Step 8 / S3 | 3 | 10 | 10 | undefined |
| registered candidate | 4 | 17 | 17 | undefined |

The presented set is the registered `1 + d^2` basis: one constructor beta
term and an ordered `d x d` matrix of typed terms. V3 proves a bijection
between this registered index set and its opaque replayable tokens. It does
not prove that every intended semantic path schema belongs to this set, so it
does not certify any path novelty value.

The `hcom` staging regression is also deliberately narrow. It checks one
typed nested singleton instance with a top face and compares it fail-closed
against the direct union. A general CCHM union-staging theorem is absent;
`complete_union_staging` is false and staging is not used to infer `1 + d^2`
exhaustiveness.

## Provenance diagnostic

The capacity inputs are now derived rather than hard-coded. Replay builds the
typed predecessor closure, runs the kernel demand-orbit extractor, and reads
live required outputs from the relevant stage inventory. It then applies the
conditional blind local-role bound (at most one independent local coherence
anchor per clause) to the derived telescope `kappa`. The artifact records the
closure digest, orbit derivation hash, and kernel J2/J3 evidence.

This yields the same arithmetic observations:

- Step 8: `d^2 = 9`, local `kappa = 5`, one extracted live output;
- registered `d=4`: `d^2 = 16`, local `kappa = 2`, zero extracted live
  outputs.

These are conditional diagnostics, not decisive obstructions. Kernel J2/J3
is complete for the frozen package extractor, but no theorem yet identifies
that extractor with all intended depth-two semantic demand schemas. The
artifact therefore sets intended semantic J2/J3 completeness and certified
provenance obstruction to false.

## Honest status

Semantic weakening/univalent equality, intended-schema classification,
general union staging, non-path historical totals, and complete provenance
remain open. The recorded totals `7, 8, 10, 18` are not certified, so `F-T1`
remains active. The result stays in **Z4**, no numeric Step-16 novelty verdict
is valid, and `d=5` was not attempted.

## Reproduction

```powershell
cargo test -p pen-type cubical --no-fail-fast
cargo test -p pen-eval tdc1_cubical --no-fail-fast
cargo run -p pen-search --example tdc1_cubical_regression -- replay docs/tdc1_cubical_regression_v3.json
```

To regenerate, pass `run` a new, non-existing output path. The runner uses
create-new semantics and refuses to overwrite an artifact.
