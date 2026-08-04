# Preservation-Theory Corrigendum (TC0 Manifest)

Date: 2026-08-04
Status: DRAFT-NOT-FROZEN (formulation lane, plan binding decision 7). This
document mints no authority, proves nothing, and may change freely until its
registered freeze gates. Freeze gates: the TC-R/TC-A statement freeze closes
no later than A4-R2e; the TC-W wire-versus-schema decision freezes before R4
fixture regeneration; TC-V closes only after R4 (plan §7.2.2).

Frozen inputs (immutable; plan §6.3, §13):

- the S1c3 truth audit: the proposition
  `Full(ProjectSeed) = Full(ReindexedSeed)` is uninhabited for every nonempty
  action, because each normalized primitive contributes a fresh formal-action
  key absent from the candidate endpoint;
- the three typed preservation descriptors as proposition syntax with no
  inhabitant, proof value, or theorem authority; and
- the shared nineteen-node manifest, graph ABI, comparator
  `Current < Older < Structural < FormalAction`, and candidate embedding.

Justification discipline (plan §7.2.2): every adjudication below must be
justified from renaming/action semantics and support accounting. No choice
may be justified by which theorem makes later `gamma` positive, recovers a
held-out sequence, or simplifies a desired downstream result. Adversarial
review is required before any freeze.

## 1. Decision register

Each item is marked PROPOSED (with the argued default), or OPEN (a genuine
underdetermination registered for adversarial review). PROPOSED items are
not decided until the freeze.

### D1. Exact semantic objects related by renaming (TC-R) — PROPOSED

The renaming theorem relates transported candidate projections to
projections of the renamed base:

```text
Transport_h(Projection_Q(B)) = Projection_Q(Rename_Q(B,h))
```

over the shared graph/projection type of S1c3, covering declarations and
global identities; binders, referents, owners, and keys; normalized
contexts, terms, paths, and support; classes, coverage, and projection
positions; lineage and footprints; and every retained theorem endpoint.
Byte equality is required only after both endpoints are rendered in the same
transported naming frame.

### D2. Admissible renaming carrier — PROPOSED

`h` ranges over finitely supported bijections of public global identifiers
(declaration/global identity sorts only). Rationale: the public world's
naming freedom is exactly permutation of opaque public identifiers;
injective-non-surjective maps confuse renaming with restriction, and
infinite-support maps have no finite evidence. Freshness discipline: `h`
never acts on bound local ordinals; binder-crossing is handled by the
locator structure, which is ordinal-based and h-invariant.

### D3. Action of `h` on `FormalAction`/`FormalId` keys and loci — PROPOSED

Formal-action identifiers are derived from primitive loci, not primitive
names, so `h` does not act on them directly. Instead they transport
structurally: `Transport_h` rebuilds each `FormalAction(FormalId(p_i), ...)`
key from the transported locus. The induced obligation is the named lemma:

```text
TC-R-L1 (locus commutation):
  locus derivation after Rename_Q(-,h)
    = Transport_h after locus derivation
```

If TC-R-L1 fails for the chosen derivation, D2/D3 reopen; the fallback
(declaring formal keys h-fixed) is admissible only with a proof that
renaming commutes with primitive-locus derivation on the nose.

### D4. Transport-derivation strategy — OPEN

Generic structurally derived transport (one transport functor over the
shared symbolic enum, specialized per sort) versus per-sort hand
definitions. Generic transport minimizes the Agda proof burden over the
nineteen-node manifest and its descendants but requires a once-and-for-all
well-formedness theorem; per-sort definitions localize failures but
multiply endpoint-coverage obligations. To be adjudicated on proof-burden
evidence during drafting; both candidates must satisfy the same laws:

```text
Transport_id      = identity
Transport_(g o h) = Transport_g o Transport_h
Transport_h preserves typing, support, quotient, and coverage
```

### D5. Exact pre-quotient action transform (TC-A) — PROPOSED

As plan §7.2.4:

```text
RawAction(B,q) = ActionRebuild(q, RawCandidate(B), DeltaRaw(B,q))
QAction(B,q)   = Quotient_Q(RawAction(B,q))
```

`ActionRebuild` is not assumed to be disjoint union; the freeze must state
exactly how inherited material is transported, merged, removed, rewritten,
or extended, consistent with the S1c2/S1c3 native-first edge-27 pairing.

### D6. Action-generated formal-event delta — PROPOSED

`DeltaRaw(B,q)` is generated independently from the formal action events and
contains every mandatory formal-action occurrence and support edge (one
leading `ActionArgument=28` incidence per complete normalized primitive,
with the paired `ActionIntroduced=27` companions per the frozen S1c2
census). The delta is never filtered against, or selected by, the stored
footprints; stored footprints are final equality targets only (S1c4 rule).

### D7. Quotient reconstruction function — PROPOSED

`Quotient_Q` is the existing frozen quotient compiler applied to the rebuilt
raw structure — reconstruction, not filtering or inclusion of the old
candidate quotient. The rebuild may change, merge, remove, and add quotient
classes; the theorem package derives the exact disposition of removed or
merged classes rather than assuming preservation.

### D8. Theorem form for nonempty actions — OPEN (bounded)

Candidates, in descending strength: exact reconstruction equality; cospan
relation; extension relation; restriction theorem; support-delta theorem; or
a stated combination. Registered bound: full-projection equality is
excluded (S1c3 impossibility), and the adopted form must imply the
support-delta statement (the action projection extends the transported
inherited projection by exactly the derived delta support). The strongest
actually provable form is adopted and truthfully labeled; a support-only
statement must not be mislabeled full preservation (plan §7.2.4).

### D9. Composition typing and re-basing — PROPOSED

Primary frozen statement: flattening coherence over the original base,

```text
QAction(B, q1 ++ q2) = QAction(B, flatten(q1, q2))
```

with `flatten` the frozen total-action concatenation of S1c1's
right-associated suffix algebra. Rationale: this keeps every theorem
endpoint on the single audited base `B` and avoids making `RawAction(B,q)` a
lawful base — with re-established invariants — a prerequisite of the
theorem package. The re-basing lemma (`RawAction(B,q)` inhabits the
candidate-base invariants, and deltas computed at the shifted base transport
along the rebuild) is registered as a separate optional strengthening,
OPEN, and nothing in JG2b2b3 may depend on it unless it is separately
frozen and proved.

### D10. Composed-delta equation — OPEN

How `DeltaRaw` at a shifted base is transported or merged when the rebuild
has removed or merged inherited classes. Under D9's primary form this
reduces to a coherence law for `flatten`:

```text
DeltaRaw(B, flatten(q1,q2))
  ~ delta-merge(DeltaRaw(B,q1), transported DeltaRaw along q1 of q2)
```

where `delta-merge` and the transport are the objects to be decided. This
item cannot fully freeze before the S1c4 lineage representation closes.

### D11. Identity-action specialization — PROPOSED

Both identity laws are frozen clauses of TC-A:

```text
DeltaRaw(B,[]) = empty          QAction(B,[])      = QCandidate(B)
                                QAction(B, A ++ []) = QAction(B, A)
```

The nullary law makes the empty total action a literal-equality positive
control; the append law (plan §13) prevents an appended identity from
resetting a nonempty image.

### D12. Renaming equivariance of transform and delta — PROPOSED

Both the inherited transform and `DeltaRaw` are equivariant under D2's
carrier via D3's transport:

```text
DeltaRaw(Rename_Q(B,h), q) = Transport_h(DeltaRaw(B,q))
RawAction(Rename_Q(B,h), q) = Transport_h(RawAction(B,q))
```

conditional on TC-R-L1.

### D13. Proof-relevant versus representation-only fields — OPEN

To be enumerated at freeze over the shared graph ABI: which fields carry
theorem content (keys, referents, owners, classes, coverage, lineage,
support edges) and which are representation-only (display ordinals, vector
packing) and therefore outside the equivariance and reconstruction claims.
The enumeration must be total over the ABI; an unlisted field is
proof-relevant by default (fail-closed).

### D14. Endpoint wire/version strategy (TC-W) — PROPOSED; DEADLINE: before R4

Preferred path (plan §7.2.5): retain the A4 preservation descriptors as
typed historical proposition syntax; define a separate versioned theorem
schema for TC-R/TC-A; bind JG2b2b3 authority to the new schema; do not
reopen A3/R2c/R2d/R2e/R4. The alternative (corrected endpoints inside A4)
requires a new wire version and a rerun of every affected closure and
mutation suite. This decision must be frozen before R4 fixture
regeneration so a wire-version outcome triggers at most one R4 pass.
Current draft assessment: nothing in D1–D13 requires changed A4 endpoint
data; the separate-schema path stands unless adversarial review finds an
endpoint that cannot be expressed over the existing descriptors.

### D15. Complete generic falsifier suite (TC-V) — PROPOSED (outline)

Per plan §7.2.6, closing only after R4:

- nonempty actions with formal keys reject the old full-equality subject;
- key deletion, candidate-only filtering, copied support, and fabricated
  delta mutants fail at the corrigendum layer;
- renaming mutants comparing untransported identifiers fail;
- a transport mutant violating `Transport_(g o h) = Transport_g o
  Transport_h` fails;
- a flatten mutant violating the append-identity fails;
- identity action remains a literal-equality positive control; and
- resource exhaustion returns `Unknown`.

## 2. Closure conditions

TC-R/TC-A statement freeze requires: every D-item decided (no OPEN
remaining), adversarial review recorded, and the S1c4/R2d-roots
representations consumed read-only. TC-V closure requires: safe Agda proves
the generic laws; Rust independently reconstructs every finite endpoint;
canonical transcripts agree byte-for-byte; opaque authority has one private
deterministic constructor; and the full D15 suite discriminates.

JG2b2b3 consumes only the corrected TC-R/TC-A authority, never the old
equality descriptors (plan §7.2.7).
