# Law V2 relative census boundary

Status: Phase-3a finite relative foundation implemented; full Phase-3 exit
gate remains blocked.

`pen-demand` computes and replays a deterministic least fixed point over an
explicitly supplied finite registry. Its result is complete only relative to
that registry, its finite rule relation, its explicit library seeds, the
verified signature, and the implemented definitional kernel. It has no
authority to assert the normative demand domain is complete.

## 1. Trusted and untrusted inputs

The calculation receives:

- a `VerifiedSignature` from `pen-kernel`;
- an `OpaqueWindow` containing exactly two distinct anonymous globals from
  that signature;
- a canonically ordered `FiniteDemandDomain`;
- canonically ordered `LibrarySeeds`; and
- an operation limit.

Before hashing, recursive equality, or cloning a replay payload, the complete
domain and certificate are traversed under one aggregate node budget and the
kernel depth limit. Every motive in the domain is then checked through one
batch kernel call with a shared operation budget. Structural-support
extraction likewise uses one aggregate traversal budget. Exhaustion at any of
these boundaries returns `Unknown(ResourceExhausted)`.

The finite domain contains separate `RegisteredFamily` and
`RegisteredInstance` types plus monotone `FiniteRule` records. Families and
instances carry normalized kernel `TypeFormation` judgments. Their IDs are
recomputed from canonical content; duplicate, out-of-order, forged, or
unknown references fail closed.

Instance-to-family membership and library seed membership are disclosed
relative assumptions. This slice does not infer instances by substituting a
family, and a seed ID is not a kernel derivation. No result from this crate
may hide those assumptions.

## 2. Implemented computation

For each registered instance, the implementation derives structural support
from anonymous globals occurring in the family and instance motives, closed
transitively through verified declaration types and bodies. An instance is
active in this relative calculation exactly when that derived support touches
one of the two window globals.

Derivability is a separate monotone closure:

```text
R0      = explicit library seeds
R(k+1)  = Rk union {
             conclusion(rule)
             | every premise(rule) is in Rk
          }
```

Each nonempty delta is stored as a deterministic sorted layer. Saturation
stops only at an empty delta or when every carrier member has been reached.
`unreached` is recomputed as `active - reached`.

The result type exposes only:

- `CompleteRelative`, meaning the exact finite computation above terminated;
  or
- `Unknown`, for resource exhaustion or unsupported/invalid relative input.

It does not expose law-level `Live`, `Blocked`, `Advanced`, or `Halted`.

## 3. Replay certificate

`UncheckedRelativeCensusCertificate` binds:

- certificate version;
- trusted kernel-source digest;
- trusted normalizer-source digest;
- finite-domain digest;
- verified-signature digest;
- width-two window digest;
- library-seed digest;
- derived support;
- active, reached, and unreached sets; and
- every fixed-point layer.

The verifier recomputes the entire calculation and compares all projections
exactly through borrowed slices, without cloning an attacker-controlled claim.
Success returns a private, non-deserializable
`VerifiedRelativeCensus`. Wire DTOs reject unknown fields. Digests identify
the supplied finite inputs and replay result; they do not prove that the
finite inputs cover the normative semantic domain.

## 4. Explicit non-claims

This implementation does not establish:

- generation or exhaustiveness of the registered scheme calculus;
- total specialization of dependent future-hole schemes;
- that pre-registered instances are all substitutions of their families;
- a sound-and-complete natural-family normalizer;
- univalent transport or the required orbit quotient;
- completeness of the library seeds or finite rule relation;
- generic nonmembership in the ambient derivation system;
- locality, weakening preservation, discharge, or expiration across
  histories;
- the normative one-live-orbit ladder;
- the historical final instance surface as an authoritative result;
- an empty final obligation profile; or
- any acceptance, branch, block, or halt decision.

Accordingly, `pen-engine` continues to report `Unknown` and lists the
authoritative demand census as missing.

## 5. Remaining Phase-3 blockers

The repository does not yet contain an independently defined inductive
depth-two semantic domain together with a coverage induction for its
generator. Defining the domain as whatever an implementation emits would be
circular.

The ambient law also needs a total sound-and-complete family/equivalence
normalizer, a declared decidable specialization fragment, a finite-closure
theorem, and a complete decision procedure for membership in the current
derivation basis. Existing historical artifacts explicitly record gaps in
those obligations.

Frozen demand artifacts may therefore be used only by `pen-oracle` as
post-run regression comparators. Expected labels, counts, event numbers,
historical winners, and expected next families must not enter `pen-demand`,
its certificates, or the production dependency closure.
