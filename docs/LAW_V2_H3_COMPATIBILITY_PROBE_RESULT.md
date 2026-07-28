# Law V2 H3 compatibility probe

**Date:** 2026-07-28
**Authority:** `ORACLE_ONLY_PROVISIONAL_NON_NORMATIVE`
**Outcome:** `blocked_before_A_B_C`
**Artifact:** [`law_v2_h3_compatibility_probe_v1.json`](law_v2_h3_compatibility_probe_v1.json)
**Result digest:** `blake3:21206c6bc40cb5f123b9d6bc6db6a93d7111a2b13ae4c05230e09ad5445c5a53`

## Result

The proposed GSC semantics cannot yet be classified as Outcome A, B, or C.
GSC V1 is not adopted, its semantic manifest and compiler equations are not
frozen, and the registered bootstrap does not carry the structured capability
needed to construct its descriptor. The literal Law/GSC result is therefore
`Unknown`, not an exact execution of `compile_use`.

> The registered H3 boundary does not carry the structured
> `ClosedFormerFrame` capability required by GSC V1.

`VerifiedRegisteredBootstrap` proves the exact three declarations and final
normalized signature, but it has no verified event export index, declaration
origins, constructor-group token, or computation-mode certificate. The
proposal explicitly forbids inferring a closed former merely from a type and
an inhabitant.

If the current proposal's missing-token rule were adopted unchanged, applying
that conditional rule would return `OutsideFragment`. That conditional
token-rule disposition is not the literal status of an unadopted calculus.

The probe instead runs a separate, explicit oracle experiment:

```text
g1 : Sort 1 := Sort 0
g2 : Sort 0 := UnitType
g3 : UnitType := Unit

probe assumption:
  g2 is a closed former
  g3 is its sole nullary, nonrecursive introduction
  that introduction list is complete
  computation mode is judgmental
```

These completeness and mode facts are inputs to the probe. They are not
derived from the registered history. No GSC birth, fixed-type, or
parameter-projection support is assigned. This matcher is not production code
and does not amend or execute adopted GSC semantics.

## Conditional probe-local dependent-core projection

Under those extra assumptions, a hand-authored dependent-core projection with
the proposed `compile_use` shape is:

```text
P : Π (z : g2). g1
m : P g3

premise refs: none

u : Π (z : g2). P z
    [probe owner-specific use shape; not a complete GSC UsePort code]
```

In the native de Bruijn representation:

```text
parameter_context[0] =
  Pi(Global(g2), Global(g1))

parameter_context[1] =
  Apply(Var(0), Global(g3))

use_output_type =
  Pi(Global(g2), Apply(Var(2), Var(0)))
```

The kernel separately checks formation of the parameter-context entries and
output type against the registered H3 signature. Those checks establish
dependent-core well-formedness of the displayed terms; they do not establish
that an adopted compiler emitted the family or discharged its compiler
verification goals. Birth support, fixed type support, and
parameter-support projections all remain `Unknown`: the required verified
history, opaque normalization rule, and support encoding do not exist.

The proposal text separates use from computation. In the probe-local encoding,
the use projection has one term output and no equation output. Given a use
source `u`, the analogous later computation family would ask for:

```text
u g3  ↦  m : P g3
```

The exact compiler identity, output port ID, role/footprint/action codes,
support derivation, verification goals, and equation grammar remain
manifest-dependent. The projection ID in the artifact is consequently
probe-local, not a GSC semantic identity.

## Direct-eliminator countercandidate

The probe's exact displayed direct head is:

```text
r :
  Π (P : Π (z : g2). g1).
  P g3 →
  Π (z : g2). P z
```

That is the submitted global-referencing spelling. During checking, the native
kernel unfolds the registered bodies, and the verified declaration stores the
corresponding `UnitType`/`Unit`/`Sort 0`-normalized spelling. The formation
check therefore does not certify any GSC support set.

The native kernel establishes only the following dependent-core facts:

- the closed dependent Pi type forms;
- a fresh bodyless declaration `r` of that type is accepted as a syntactic
  opaque signature extension; and
- under `[P,m]`, `r Var(1) Var(0)` has the probe-local output type.

Acceptance of a bodyless head is not admission by a GF2 response grammar, a
constructed eliminator, a computation rule, or a discharge certificate.

It also establishes the expected limitation:

- `λ P m z → m` is rejected with `TypeMismatch`, because `P g3` is not
  definitionally `P z`;
- the core has no unit eliminator or unit eta rule; and
- it has no fresh-head equation-extension syntax with which to install or
  verify `r P m g3 ↦ m`.

The safe Agda sidecar constructs an analogous eliminator for its own host
datatype `One` and constructor `star`, and proves that host equation by `refl`.
It supplies no formal bridge identifying `One` with registered `g2` or `star`
with registered `g3`. It is therefore a shape model in host Agda, not an
independent typed realization of the registered H3 response and not evidence
that current GF2 candidate syntax admits it.

The JSON replay binds and inspects this Agda source but intentionally records
external Agda typechecking as `Unknown`: it does not invoke or attest the
executable, version, flags, imports, or result. The command in the verification
section is a separate validation step.

At Law level, the direct beta fill remains `Unknown`: rejection of the
definitional-equality attempt for an opaque head and absence of an equation
extension verifier do not refute a future well-formed response. Its
demand-connectedness and quotient relation to the archived rows are likewise
`Unknown`.

The hypothetical direct response has the raw shape “one fresh head plus one
beta clause.”
It is not lawful to compare that count with the archived legacy
three-clause `kappa`: no adopted common irreducibility measure exists.

## The four archived representatives

The archived Stage-4 rows are exactly the two-by-two product:

```text
former axis:
  Lam(Pi(Var(1), Var(2)))
  Lam(Sigma(Var(1), Var(2)))

argument-order axis:
  App(App(Var(1), Var(2)), Var(3))
  App(App(Var(1), Var(3)), Var(2))

shared beta skeleton:
  App(Lam(Var(1)), Var(2))
```

| Candidate | Canonical key | Historical grammar | GF2 typed API | H3 use | H3 beta | Equivalence to direct |
| --- | --- | --- | --- | --- | --- | --- |
| `2016726758f3…a3407` | `0e9aa132` | Proven | Unknown | Unknown | Unknown | Unknown |
| `43a0ed707770…493308` | `4ede818c` | Proven | Unknown | Unknown | Unknown | Unknown |
| `4b2211ecae25…1265b` | `d860242f` | Proven | Unknown | Unknown | Unknown | Unknown |
| `b4f821d9bb28…0edd4` | `b516ff09` | Proven | Unknown | Unknown | Unknown | Unknown |

All four hashes are recomputed from their exact archived telescopes, and all
four replay under the legacy `pen-type` elaborator. That success is coarse:
each row has clause classifiers `[Fun(Neutral, Type), Neutral, Type]`; clauses
0 and 2 each use one coarse assumption; clause 1 remains a pair of stuck
applications; and no row has a formation-role clause. Consequently none
passes the typed-eliminator formation gate.

This legacy elaboration is not an H3/GF2 discharge proof. The rows are shallow
`pen_core::Telescope` values against `Telescope::reference(1..=3)`, whereas
registered H3 is a `pen_kernel::VerifiedSignature`. There is no translation
certificate between those syntax, identity, typing, or equation systems.

## Complete current Law/capability matrix

| Response | Historical admission | Prospective GF2 admission | Typed head | Use fill | Beta fill | Full DAG discharge | Demand-connected | Equivalence to direct |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Sealed H3 library | Outside fragment | Unknown | Unknown | Unknown | Unknown | Unknown | Outside fragment | Unknown |
| Four archived rows | Proven | Unknown | Unknown | Unknown | Unknown | Unknown | Unknown | Unknown |
| Direct unit eliminator | Outside fragment | Unknown | Proven (core formation only) | Proven (conditional opaque-head typing only) | Unknown | Unknown | Unknown | Unknown |

`Unknown` is intentional. Candidate-grammar absence, unsupported equation
verification, missing typed adapters, and missing quotient saturation are not
negative theorems. In particular, trivial self-identity of the displayed
direct syntax does not decide whether it contributes an additional quotient
class.

## Interpretation

The conditional probe-local projection has the shape of ordinary
owner-specific dependent unit elimination. It is not itself a Pi/Sigma or
contextual-internalization demand. Because no exact adopted compiler has run,
this does not prove that every lawful GSC realization of H3 must have that
shape.

Accordingly:

1. the historical four-way Stage-4 claim is not presently compatible or
   incompatible with GSC—it is untyped at the comparison boundary;
2. the direct head is a useful conditional pressure test: core formation and
   opaque-head application check, but grammar admission, beta extension,
   demand-connectedness, complete-cone membership, and quotient class remain
   undecidable with current capabilities; and
3. an explicitly adopted public context-projection/internalization principle
   asking for adjoints to reindexing is one candidate repair if the intended
   Stage-4 demand is Pi/Sigma. The probe proves neither that this principle is
   necessary nor that it is sufficient; other adopted semantics could produce
   a different demand.

No GSC text, production dependency, candidate selector, or acceptance rule was
changed based on this result.

## New blocker set

Authoritative continuation now requires, in order:

1. an adopted semantic descriptor/compiler manifest;
2. `VerifiedHistory`, a verified anchor, declaration origins, and event-time
   public export indices;
3. derived constructor completeness, computation mode, birth support, type
   support, and parameter-support projections;
4. exact compiler outputs and separately checked compiler verification goals;
5. either a typed legacy-telescope-to-GF2 adapter or newly authored typed
   versions of the four representatives;
6. the restricted fresh-head equation-extension verifier;
7. the finite operational derivability/discharge DAG; and
8. the origin-cutoff theorem registry and quotient checker.

The earliest blocker is item 1 together with the descriptor-bearing history
capability in item 2. Until those exist, Law V2 must continue to return
`Unknown`.

## Verification

The artifact hashes every bound text input after canonicalizing CRLF to LF, so
its provenance is portable across Windows and fresh LF checkouts. The bindings
include the workspace lock/toolchain manifests and the exact kernel, legacy
elaborator, hashing, reference-prefix, bootstrap, Agda, archive, proposal, and
probe sources used by the reported checks.

```powershell
agda --no-libraries --ignore-interfaces --safe --without-K `
  -i C:\DEV\atomic\agda `
  C:\DEV\atomic\agda\LawV2\H3Compatibility.agda

cargo run -q -p pen-oracle `
  --example law_v2_h3_compatibility_probe -- `
  replay docs\law_v2_h3_compatibility_probe_v1.json
```

After the corrected executable artifact is regenerated and its digest is
inserted above, both commands must pass. The replay must report `valid: true`,
`outcome: blocked_before_A_B_C`, and no errors.
