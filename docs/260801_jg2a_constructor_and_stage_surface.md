# Phase JG2a — Constructor Grammar and Exact Stage Surface

**Status:** DISCHARGED on 2026-08-01  
**Authorities:** `VerifiedGenerativeCapabilityConstructorGrammarV1` and
`VerifiedGenerativeCapabilityStageSurfaceV1`  
**Implementation:** `crates/pen-generative-audit`  
**Next phase:** JG2b2b — executable occurrence/indexed-interface calculus and
exact generic substitution/naturality metatheory

## 1. Why JG2 required an internal repair

The first attempt to enter JG2 exposed a missing premise rather than a raw-
carrier implementation problem. JG1 V1 freezes the seven roles, eight
admission obligations, receipt-source sum, canonical transcript, and digest.
It does not freeze an executable finite constructor/interface calculus.

That distinction matters. A verified `pen-kernel::VerifiedSignature` proves
typing and normalization, but its declarations do not intrinsically say
whether an operation is a `Compiler`, `Transport`, or
`DischargeTransformer`. Unrestricted derivability is also not a finite set.
Consequently, assigning a caller role tag or accepting a caller-selected
capability list would have made the proposed JG2 carrier self-attested rather
than complete.

Two further authorities are absent from the current kernel surface:

- `VerifiedClosedSpecialization` replays one particular substitution; it is
  not a proof of naturality under every admissible substitution; and
- the isolated generative crate has no opaque complete inventory of newly paid
  clauses or prior live generative-requirement outputs.

JG2 is therefore decomposed without weakening its exit criterion:

1. **JG2a — constructor grammar and exact stage surface:** freeze the finite
   interface-shape vocabulary and bind an exact kernel-replayed public stage
   transition;
2. **JG2b — complete birth-stage, naturality, and support surface:** freeze the
   executable occurrence and indexed-interface grammars, prove one generic
   theorem over arbitrary admissible typed substitutions, and derive the finite
   history-bound occurrence/theorem-application and support censuses from
   opaque public authorities at each capability's own birth transition; and
3. **JG2c — exhaustive raw carrier:** enumerate every surface/constructor
   tuple and record either an admitted raw derivation or a typed certified
   negative.

Full JG2 remains active until JG2c mints the complete raw carrier. JG3 remains
blocked.

## 2. Frozen constructor/interface grammar

`VerifiedGenerativeCapabilityConstructorGrammarV1` is an opaque proof of the
exact JG2a proposal. The proposal contains one closed constructor schema for
each frozen JG1 role. Each schema has a finite ordered input-sort sequence and
one output sort. The seven shapes are pairwise distinct, so a later checker
must derive the role from the verified constructor shape; it may not accept a
separate role choice.

The exact target-neutral interface shapes are:

| Constructor | JG1 role | Public interface shape |
| --- | --- | --- |
| `Formation` | `Formation` | `PublicContext -> PublicInterface` |
| `Abstraction` | `Abstraction` | `(PublicContext, PublicInterface) -> InterfaceFamily` |
| `Aggregation` | `Aggregation` | `InterfaceFamily -> PublicInterface` |
| `Transport` | `Transport` | `(Substitution, PublicInterface) -> PublicInterface` |
| `Comparison` | `Comparison` | `(PublicInterface, PublicInterface) -> ComparisonWitness` |
| `DemandCompiler` | `Compiler` | `(SealedPublicGrammar, DemandScheme) -> LiveDemand` |
| `DischargeTransformer` | `DischargeTransformer` | `(Substitution, Discharge) -> Discharge` |

These are kinds of reusable public operation, not claims that any stage
realizes them. In particular, transport acts on an interface rather than a
selected term, the demand compiler turns a registered scheme over sealed
grammar into a live demand, and the discharge transformer reindexes an
already established discharge.

Verification requires:

- the exact JG1 V1 token and digest;
- the exact constructor and interface-sort vocabulary and order;
- the exact one-to-one constructor-to-role map;
- pairwise-distinct finite interface shapes;
- the pinned canonical transcript; and
- the pinned domain-separated transcript digest.

The authority also exposes a combined scope-grammar digest over JG1 and the
constructor grammar. That digest is an identity used to bind certificate
scope; it is not evidence by itself.

The pinned constructor transcript is 296 bytes. Its digest is
`blake3:894ea782df934accc9d2efc6b469d144ab73fe2ad2b6b4de096d961cefd7341a`;
the combined JG1/JG2a scope-grammar digest is
`blake3:2fcb8a3d72c8719c25b1910a7972beb61dc906709007a320c07bb358402d49b7`.

This grammar closes the finite constructor universe. It does not certify that
any public declaration realizes a schema or satisfies the eight JG1
obligations.

## 3. Exact kernel-replayed stage surface

`verify_generative_capability_stage_surface_v1` consumes:

- the opaque JG1 and constructor-grammar authorities;
- a `pen-kernel::Kernel`;
- a locally trusted `TrustedScope` for `FreeSealingFragment`;
- the verified predecessor signature;
- the exact nonempty candidate extension; and
- an opaque `VerifiedFreeSealing` capability.

The verifier checks that the scope uses the combined grammar digest, binds the
predecessor as its public boundary, and binds the exact canonical candidate
digest. It then checks the sealing scope and subject, independently replays the
extension through the kernel, and requires byte-for-byte equality with the
sealed normalized successor.

The resulting opaque stage-surface token retains the verified predecessor and
successor boundaries and the exact normalized candidate declarations. Its
canonical identity binds the JG1 digest, constructor digest, combined grammar
digest, kernel and scope identities, predecessor/candidate/successor
identities, sealing subject, and declaration counts.

The token proves an exact public boundary transition. It deliberately does
not call the caller-supplied `history_digest` inside `TrustedScope` a verified
history. A complete history/birth inventory is a JG2b obligation.

## 4. Birth-stage semantics

The predecessor-absence and successor-presence obligations apply at a raw
capability's own birth transition. They must not be re-evaluated against only
the immediate predecessor of a later candidate. Otherwise inherited
capabilities would disappear from `RawGCap(H,A)`, making the JG4 weakening map
impossible by construction and collapsing the distinction between the raw
carrier and the later marginal.

JG2b must therefore retain a complete ordered inventory of verified birth
surfaces. JG2c may admit a raw member only with a typed birth-level
absence/presence witness bound to one of those surfaces.

## 5. Exhaustive-carrier requirement retained

JG2c must derive all enumeration seeds internally. For every canonical public
surface row and every frozen constructor tuple, it must record exactly one of:

- `Admitted`, with a raw capability identity; or
- a typed, independently checkable negative disposition.

Unsupported syntax, an incomplete occurrence-by-constructor disposition
matrix, an unverified theorem application, missing support, ambiguous
constructor shape, or resource exhaustion aborts the whole mint. Nothing may
be silently dropped.

A complete raw carrier may be empty. Completeness is witnessed by the
exhaustive disposition ledger, not by nonemptiness. This is required for the
registered `(nu,gamma) = (0,0)` and `(nu>0,gamma=0)` generic quadrants.
Distinct raw derivations remain distinct in JG2; only JG3 may quotient
presentation equivalence or interderivability.

## 6. Explicit non-authorities

JG2a issues no authority for:

- a realized constructor, generative-capability member, or complete carrier;
- a generic arbitrary-substitution theorem, universal naturality, or a complete
  occurrence/theorem-application census;
- verified history, birth closure, paid support, or prior live-output support;
- quotienting, weakening, marginality, strict enlargement, or provenance;
- `gamma`, any inequality involving `nu`, or a bootstrap value;
- debt, candidate acceptance, profile selection, or a live Genesis run; or
- Rust/safe-Agda correspondence.

The crate still has only `pen-kernel` as a production dependency and accepts
no registered Genesis prefix, historical label, held-out trace, oracle result,
expected count, or caller capability inventory.

## 7. JG2b entry condition

JG2b may begin from the two opaque JG2a authorities. It must introduce a
closed, finite, independently enumerated public surface that binds:

1. every verified birth transition retained in the current history;
2. a frozen executable structural occurrence/path grammar;
3. executable kernel-checked realizations, equality, and reindexing actions for
   every abstract interface sort;
4. a generic typed-substitution package proving identity, composition, binder
   lifting, typing and typed-equality preservation, normalization/reindexing
   compatibility, and all seven constructor naturality equations, rather than
   a finite sample advertised as universal;
5. a complete finite occurrence-by-constructor disposition matrix and full
   concrete theorem-application census;
6. predecessor and successor capability-closure evidence at each birth; and
7. the complete typed support census consisting only of newly paid clauses or
   prior live generative-requirement outputs.

JG2a fixes `constructor -> role`; it does not yet provide
`public occurrence -> constructor`. JG2b2b must freeze the occurrence grammar,
theorem subjects, and classifiers; JG2b2c must derive one typed disposition for
every occurrence/constructor pair, prove exact matrix coverage, aggregate each
row to a unique/zero/ambiguous result, and never accept a caller constructor
tag.

JG2b1b now provides the required target-neutral history authority without
importing registered-prefix `pen-law::VerifiedHistory`. Its consuming replay
derives the exact birth index through a producer-finalized head. The subsequent
JG2b2 audit exposed a different missing premise: these abstract interface sorts
are not kernel terms, the grammar defines no executable reindexing action or
occurrence classifier, and one replayed closed specialization is not a theorem
over every admissible substitution.

That verifier must reconstruct a canonical history identity from replayed
events. A JG2a stage identity includes the complete locally trusted scope, so
the same typed boundary can have different JG2a identities when unrelated
non-authoritative scope fields differ. A set of stage-surface digests is
therefore not by itself a unique birth-event census.

JG2b2a now freezes that repaired protocol boundary. JG2b2b must implement the
structural occurrence and indexed-interface calculi, exact theorem subjects,
and generic substitution/naturality metatheory; JG2b2c must then derive the
finite complete-through-head disposition matrix and concrete theorem-
application census. Every surface must fail closed rather than accept
`natural`, `paid`, `live`, or `supported` Booleans. Only after JG2b is complete
may JG2c mint a carrier.
