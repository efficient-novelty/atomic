# Autonomous Genesis: Plan and Status

Date: 2026-07-30
Repository: `pen-atomic`
Working branch: `codex/law-v2-kernel-cost-v2-inventory`
Status basis: committed history through `a3e30d5` plus the canonical-wire and
safe-Agda work present in this branch on 2026-07-30

## Executive summary

The long-term objective is not to replay the known fifteen-stage Genesis
sequence. It is to build a genuinely blind, proof-producing engine in which:

- the registered Law V2A bootstrap, an anonymous basis, the two laws, a
  width-two memory window, and a finite GF2 completeness boundary are the only
  semantic inputs;
- a guarded act is accepted because it totally discharges the live typed
  obligation;
- all inequivalent total dischargers survive as a cone;
- halting is caused only by a complete proof that the next obligation profile
  is empty; and
- names, expected structures, expected costs, the number fifteen, and
  target-derived selection rules remain outside the production dependency
  graph.

That theorem has not been established. The repository contains a strong
legacy search and persistence chassis, but its known fifteen-step result is
historical/oracle testimony rather than a derivation from the repaired laws.

The Law V2 lane has nevertheless made substantial progress. It has an oracle
firewall, a small independent kernel, a registered three-act bootstrap, a
finite demand/GF2 infrastructure, an implemented narrow H3 profile, and a
certified H4 continuation. The narrow frozen profile generated one new
structure at Act 4—a direct unit eliminator with its computation equation—and
then correctly halted debt-free. This is a real autonomous result for that
profile, but it is not the historical fifteen-stage sequence.

The current critical path is more foundational. Before the semantic audit,
kernel cost, rewrite system, quotient, and productivity law can receive
authority, the project must prove that the concrete Rust certificate system
and the safe Agda model describe exactly the same production judgments. The
current work therefore builds one canonical byte bundle, feeds exactly those
bytes to independent Rust and Agda checkers, and requires byte-identical
normalized transcripts before any correspondence capability can be minted.

The canonical Rust wire, exact Rust-to-Agda input transport, slot-derived
bundle construction, fail-closed capability frontier, and safe Agda envelope
decoder are implemented. As of the 2026-07-30 continuation, safe Agda now
generically decodes the complete eleven-section bundle with Rust-equivalent
resource bounds and structurally checks it against the exact `validate.rs`
invariants; one genuine Rust-encoded bundle byte vector is accepted
end-to-end by refl, and pinned mutation vectors are rejected at the same
boundaries as Rust. The Phase D finite context/global correspondence is
also now proven at the theorem level: scoped wire terms, oldest-first
contexts, and the chronological slot table decode faithfully onto the
intrinsic `PTm`/`PCtx` layer with both round trips, the exact
variable-lookup ordinal and shift-distance equations, strict-prior global
lookup, and a first cross-language decoded-surface transcript agreed
byte-for-byte. The Phase E semantic replay core is also implemented:
safe Agda now replays conversion traces as genuine intrinsic reductions
(beta by instantiation, transparent delta by the exact strict-prior
signature body under the policy, congruence with binder-local scope) with
machine-checked soundness into an intrinsic step relation and a
no-outgoing-step normality theorem, and recomputes every synthesis
certificate's subject and type semantically, including
conversion-mediated dependent application results. Structurally valid but
semantically wrong mutant vectors are now rejected exactly at this layer.
The next genuine blockers are the endpoint-typing and supplement replay
(the typing-judgment bridge), independent Rust replay, and exact common
transcript agreement.

## 1. Objective hierarchy

### 1.1 Ultimate scientific objective

Establish a blind Genesis theorem at the strongest defensible level:

1. validate the disclosed Law V2A three-act bootstrap;
2. compute each live obligation from typed history, rather than from a stage
   number or expected structure;
3. enumerate the complete finite set of constitutively admissible total
   dischargers inside the declared GF2 fragment;
4. quotient those dischargers by an adopted, decided equivalence;
5. freely seal every surviving class, preserving genuine branching;
6. repeat independently on every branch; and
7. halt only when every branch carries a complete empty-obligation
   certificate.

An external decoder may later identify the anonymous output with familiar
mathematical structures. That decoder must not influence generation,
acceptance, quotienting, or halt.

### 1.2 First realistic theorem target

The first realistic target is B2, trace-blind discovery over a fixed anonymous
basis. It is already a strong result because the engine may not use:

- semantic names;
- reference telescopes;
- expected hashes or value vectors;
- the target length fifteen;
- stage-indexed family selection;
- bar clearance as acceptance;
- presentation-order tie-breaking;
- future-trace viability; or
- baseline parity.

B3, in which the primitive basis is itself generated from generic extension
schemas, remains a later research program.

### 1.3 Current engineering objective

The current objective is narrower and precedes renewed autonomous generation:

> Construct a production-grade, exact Rust/safe-Agda correspondence for the
> lambda/unit semantic-audit V3 certificate surface without creating V4,
> adopting or freezing V3, accessing live Profile A, or constructing the
> native V3 carrier prematurely.

The bridge is successful only if one canonical bundle satisfies all of the
following:

```text
canonical production bytes
        │
        ├── exact generated Agda input bytes
        ├── independent Rust decode and kernel replay
        └── safe Agda generic decode and semantic checking
                         │
                         ▼
        byte-identical canonical normalized transcripts
                         │
                         ▼
             four opaque correspondence capabilities
                         │
                         ▼
       lambda/unit production refinement and typing metatheory
```

Digests record successful comparisons; they are not substitutes for comparing
the actual bytes.

### 1.4 Explicit non-objectives at the current stage

The current work must not:

- mutate the frozen H3/H4 Profile A results;
- add a new semantic V4 merely to evade a V3 theorem obligation;
- infer correspondence from matching Rust and Agda type names;
- mint authority from a digest, Boolean, tag, or caller assertion;
- create a Profile A adapter or issue live `kappa`/`nu` values;
- claim native-carrier completeness from caller-selected roots;
- use the archived fifteen-step corpus as production evidence; or
- start the downstream rewrite/quotient/productivity chain before production
  refinement is actually available.

## 2. What has been done

### 2.1 Legacy chassis retained as evidence and infrastructure

The legacy lane provides:

- anonymous MBTT syntax and canonical encodings;
- type, scope, connectivity, and structural evaluation;
- deterministic bounded enumeration and deduplication;
- persistent step/frontier artifacts and deterministic resume;
- CLI run, resume, inspect, and Agda export paths; and
- a reproducible guarded and comparison-backed fifteen-step corpus.

These components remain useful systems infrastructure and negative-control
material. Their target-shaped search policies, bars, reference telescopes,
configured endpoint, and historical values are not Law V2 authority.

### 2.2 Oracle firewall and fail-closed Law V2 shell

The Law V2 production graph separates `pen-oracle` from production crates.
The lawful executable has no success constructor for incomplete proof layers
and returns `Unknown` when an authority gate is missing. The root workspace
contains:

- `pen-kernel`: bounded dependent lambda/unit checking, normalization,
  equality, and exact signature extension;
- `pen-demand`: finite manifest-relative demand census and the GSC vertical
  slice;
- `pen-law`: register discipline, the registered bootstrap, history and
  profile contracts;
- `pen-gf2`: finite-fragment contracts and adapters; and
- `pen-engine`: fail-closed orchestration plus the H3/H4 experiment.

### 2.3 Registered bootstrap and H3/H4 result

The exact Law V2A three-act bootstrap is replayed and digest-bound without a
claim of Law V2B derivation or uniqueness.

For the separately frozen `gsc-inductive-completion-core-v1` profile:

- verified events, anchors, origins, cutoffs, and demand inventory were
  constructed;
- the restricted `G-Use`/`G-Compute` grammar and exact substitution-preserving
  fresh equation were implemented;
- the response carrier and Q0/Q2/Q3 disposition were exhausted; and
- Rust/safe-Agda reference agreement was checked.

The result was one quotient class: a direct eliminator with constructor
computation. After it was freely sealed as Act 4, the next width-two demand
inventory replayed completely as already derived. The resulting live-orbit
count was zero, so the frozen profile halted after four acts.

This taught us that autonomous generation is profile-relative. The laws plus
the narrow owner-specific scheme do not force the expected historical
continuation.

### 2.4 Window/register audit

The audit correctly returned `UndefinedAudit`. The existing evidence did not
contain the distinct proof objects required to issue semantic-family
`(kappa, nu)` values:

- first-irreducible kernel bases;
- complete pre/post GF2 semantic-family carriers;
- typed family weakening and restriction;
- provenance injections; and
- the family-versus-instance quotient.

This prevented historical structural totals from being silently reinterpreted
as semantic-family values.

### 2.5 Generic semantic-audit and kernel-cost prototypes

The isolated `pen-semantic-audit` workspace developed several additive,
unfrozen profiles:

- V1 established the first manifest-indexed audit and cost surfaces;
- kernel-cost V2 corrected the false idea that a fresh equation is free merely
  because its shape is determined;
- a replay-minted public inventory bound exact predecessor/successor
  boundaries, declarations, equations, demand contracts, Q3, dependency DAG,
  and normalizer identity;
- the lambda/unit finite prototype exposed why a finite generator list is not
  closed under arbitrary substitution composition;
- V2 moved arbitrary substitution algebra and stability into generic theorem
  packages;
- V3 made rank-zero seed/family identity demand-neutral and separated
  equation-port metadata from demand provenance; and
- synthesis-backed typed occurrences were implemented for fixed supplied
  roots without claiming a complete root census.

These steps progressively narrowed the problem from vague “semantic audit
missing” language to exact theorem and authority obligations.

### 2.6 Proof-carrying kernel synthesis

The isolated `pen-kernel-synthesis` workspace implements syntax-directed
synthesis for the exact fragment:

```text
Sort, UnitType, Unit, Var, Global, Pi, Lambda, Apply
```

The mirror is not trusted. It constructs a candidate derivation, then replays
formation and typing through the unchanged kernel before returning opaque
capabilities.

Protocol V2 adds:

- exactly eight synthesis-code constructors;
- explicit beta, transparent-delta, and congruence conversion traces;
- common-normal-form and no-redex evidence;
- exact outer kernel replay;
- public/checker universe separation; and
- fail-closed exposure of the still-missing binder-local nested-congruence
  replay theorem.

### 2.7 Production-refinement V1 foundations

Before the current wire implementation, the repository already had:

- a finite Agda `PTm`/`PCtx` mirror with intrinsic variable/global scope;
- context-extension, renaming, weakening, substitution, and binder-lifting
  theorems;
- synthesis-shape and inventory theorems;
- Rust global-slot verification;
- V3 Q0/family inventory checks;
- exact synthesis-protocol identity;
- exact predecessor-public delta-policy binding; and
- opaque types for the four required cross-language correspondences.

The aggregate verifier deliberately stopped at
`MissingProductionCorrespondences`.

### 2.8 Canonical production wire implemented on 2026-07-30

The new isolated `pen-production-wire` workspace is authority-free and has no
dependencies. It implements a manual binary protocol rather than Serde/JSON:

- 16-byte magic `PEN-PROD-WIRE-V1`;
- little-endian schema version and section count;
- exactly eleven ordered, length-delimited sections;
- one-byte tags;
- little-endian `u16`, `u32`, and `u64`;
- `u64` sequence and byte-string lengths;
- option tags `0`/`1`;
- bounded recursion, bundle size, section size, and sequence length;
- full-consumption decoding; and
- byte-identical canonical re-encoding.

The eleven sections bind:

1. V3 manifest surface;
2. production signature;
3. global-slot table;
4. contexts;
5. conversions;
6. nested conversion-typing supplements;
7. synthesis certificates;
8. Q0 inventory;
9. fresh-rule schemas;
10. family inventory; and
11. family payloads.

The Rust validator already checks structural and inventory invariants,
including exact V3 identity, slot ordering and prior references, context
scope, delta ordering and bodyfulness, trace chaining and no-redex census,
synthesis shapes and dependent application result, exact Q0/family
inventories, and fresh-rule shape.

### 2.9 Exact Rust-side bridge surfaces implemented

The semantic-audit workspace now contains:

- `production_wire_slots.rs`: derives the wire slot table only from a
  `VerifiedSignature` and opaque verified slot table, and translates supported
  kernel terms to finite slots;
- `production_wire_builder.rs`: assembles the canonical bundle from verified
  manifest/signature/slot/inventory/delta/synthesis inputs plus
  authority-free payloads;
- `production_wire_input.rs`: renders the one fixed generated Agda input
  module, extracts the byte list back, rerenders it, and requires exact byte
  equality; and
- `production_refinement_wire_authority.rs`: defines the opaque canonical
  bundle, Agda acceptance, Rust replay, and transcript-agreement capabilities,
  while keeping unavailable capabilities unconstructible.

The current authority frontier explicitly remains:

- generic safe-Agda bundle acceptance;
- independent Rust kernel replay;
- exact normalized transcript byte agreement; and
- the single private correspondence factory.

### 2.10 Safe Agda wire foundation implemented

The new safe Agda modules provide:

- `Bytes.agda`: intrinsic bytes and endian decoding;
- `Decoder.agda`: total decoders for bytes, fixed integers, lists, byte
  strings, options, Booleans, and full consumption;
- `Envelope.agda`: exact magic/version/eleven-section order, encoding, parsing,
  and round-trip theorems;
- `ProductionBundleV1.agda`: wire terms plus semantic parsing of the manifest,
  signature, global slots, and contexts; and
- `ContextChecker.agda`: fail-closed structural checks for manifest identity,
  universe bounds, synthesis inventory identity, unique chronological slots,
  strict-prior globals, oldest-first contexts, and predecessor-public delta
  entries.

The envelope retains all remaining sections exactly. The first four sections
received semantic parsing and structural checking in the initial wire
foundation.

### 2.11 Safe Agda sections 5–11 decode/check completed on 2026-07-30

The continuation extended the safe Agda surface to the complete bundle:

- `ProductionBundleV1.agda` now decodes all eleven sections: conversion
  certificates with reduction traces, steps, endpoint judgments, and
  no-redex censuses; binder-local conversion-typing supplements; all eight
  synthesis-code payloads; the Q0 inventory; fresh-rule schemas; the family
  inventory; and family payloads. Decoding enforces the exact Rust resource
  bounds: one million sequence items, 64 MiB bundle/byte-string limits, and
  the single shared 256-level recursion budget threaded through terms,
  reduction steps, and synthesis codes exactly as the Rust reader threads
  its one depth counter.
- `BundleChecker.agda` mirrors `validate.rs` over the decoded bundle:
  role-aware universe bounds, 32-entry context caps on every context
  surface, conversion-id uniqueness, trace chaining and endpoint equalities,
  complete no-redex census recomputation against enabled transparent-delta
  slots, synthesis shape/metadata/code checks, supplement binding, exact
  Q0 and family inventories, the exact fresh-rule pattern, and
  strictly-prior family references.
- `BundleDecodeTestV1.agda` embeds the exact `encode_bundle_v1` bytes of
  the canonical Rust fixture and proves by refl that safe Agda decodes and
  accepts them, and that seven pinned mutation vectors (magic, truncation,
  unknown tag, frozen authority, universe level, variable scope, Q0 order)
  are rejected at the same composed boundary as Rust. These are regression
  vectors, not correspondence authority.
- `BundleEncode.agda` begins the canonicality program: canonical
  structures carrying exact little-endian words, fuel-generalized
  parse/encode round trips for all three recursive payloads (terms,
  reduction steps, and synthesis codes, including embedded conversion
  identifiers and the dependent-result term), generic counted-list round
  trips under the exact resource-bound hypotheses, and complete
  section-payload round trips for contexts, the Q0 inventory, and the
  family inventory.

The work also found and repaired a genuine latent defect: the original
Agda manifest parser read fields in the Rust struct-declaration order,
while the codec encodes the delta-policy digest, synthesis protocol, and
schema version before the universe lists. The committed parser would have
rejected every genuine Rust manifest. The cross-language byte vector now
pins the corrected order.

### 2.12 Phase D context/global correspondence implemented on 2026-07-30

`LawV2/Wire/ContextCorrespondenceV1.agda` proves the finite
context/global correspondence between the decoded wire surface and the
intrinsic production syntax, organized around the four theorem families
named by the reserved Rust capability:

- structural round trip: scoped wire terms decode totally onto
  `PTm` (`wire-to-ptm`) and erase back exactly, in both directions
  (`wire-round-trip`, `ptm-round-trip`), with proof irrelevance in the
  scope evidence; oldest-first contexts build `PCtx` and the
  chronological slot table builds a strict-prior signature mirror `PSig`
  with the erasure-direction round trip plus injectivity of erasure
  (`pctx-to-wire-injective`, `psig-to-wire-injective`), which pins the
  order universally and determines every intrinsic context/signature
  uniquely from its wire bytes — a faithful embedding, with the
  intrinsic-direction identity recoverable entry-wise;
- variable-lookup correspondence: for every context and variable, the
  intrinsic in-context type `lookup-pctx` erases to exactly the raw wire
  entry at oldest-first position `locals - suc index`, shifted
  `suc index` times — the intrinsic content of the synthesis checker's
  `context_ordinal` and `shift_distance` equations — and every global
  slot's strict-prior declaration (type and body) is the chronological
  list entry at its ordinal, with global weakening erasing to the
  identity;
- extension correspondence: appending one scoped entry to a wire context
  builds exactly a `psnoc`, and decodes to `extend-context` in the
  abstract calculus; and
- shift/substitution correspondence: intrinsic weakening erases to the
  exact de Bruijn shift on wire terms (`erasure-pweaken` via a
  cutoff-general renaming spec), and intrinsic single substitution
  decodes to `instantiate` (`decode-pinstantiate`).

`LawV2/Wire/ContextTranscriptTestV1.agda` plus the extended
`cross_language_vectors.rs` pin the first cross-language decoded-surface
transcript: Rust renders strict-prior global declarations and
shift-computed variable lookups from the fixture; Agda independently
renders the same surface from `lookup-psig-type`/`lookup-pctx` erasure;
the 123 transcript bytes agree by refl. An adversarial audit found the
original fixture degenerate for this purpose (its only non-empty context
had one closed entry, so wrong ordinal, selection, or shift conventions
would have produced identical bytes); the fixture now includes a
discriminating context whose variable and under-binder Pi entries make
the ordinal formula, the oldest-first selection, the shift iteration
count, and the under-binder shift cutoff all byte-visible. Capability
minting remains deferred to the Phase H factory; nothing here is
authority.

### 2.13 Phase E semantic replay core implemented on 2026-07-30

`LawV2/Wire/SemanticReplayV1.agda` closes the conversion-side semantic
gap the structural checker cannot see:

- an intrinsic base-Q0 step relation `PStepV1` on `PTm` (beta targeting
  the exact intrinsic instantiation, transparent delta targeting the
  exact strict-prior signature body restricted to the enabled policy
  slots, and the six congruence frames with binder-local scope growth);
- decoded wire step trees replayed by a Boolean checker whose soundness
  theorem produces genuine `PStepV1` steps between the recorded
  endpoints;
- accepted traces yield reflexive-transitive `PStepsV1` chains from both
  decoded conversion endpoints to the decoded common form; and
- the accepted common form provably admits no outgoing intrinsic step,
  with a bridge theorem from the structural census recomputation to
  semantic normality.

`LawV2/Wire/SynthesisReplayV1.agda` recomputes every synthesis
certificate semantically: variables synthesize their exact intrinsic
context lookup (with the ordinal and shift metadata reverified), globals
their strict-prior declared types, pi formation combines component sort
levels with the kernel `max`, lambda introduces the dependent pi type,
and application elimination resolves its function and argument
conversions by identifier under the exact protocol V2 side conditions —
identical derived contexts, `TypeFormation` endpoint judgments, full
semantic conversion replay, left endpoint equal to the synthesized
type, and right endpoint equal to the census-normal common form, from
which the pi is destructured — then recomputes the dependent result by
intrinsic instantiation and compares it against both the embedded
result term and the certificate's inferred type. One deliberate delta
from the kernel remains: the recorded dependent result must equal the
raw intrinsic instantiation, while the kernel records the
kernel-normalized instantiation; reconciling the two requires the
bounded-normalization bridge.

`LawV2/Wire/SemanticReplayTestV1.agda` pins the discriminating evidence:
the extended fixture (a genuine beta conversion, identity conversions,
and an application-elimination certificate) passes the semantic replay
by refl, while three mutants that pass the complete Rust and Agda
STRUCTURAL checks — a delta step to a term that is not the signature
body, a beta step to a term that is not the instantiation, and a
variable-lookup certificate whose inferred type is not the context
lookup — are each semantically rejected by refl. The Rust side pins that
all three mutants are structurally valid (`encode_bundle_v1` accepts
them) and byte-identical to the committed literals.

Not yet replayed semantically: endpoint typing judgments, the
conversion-typing supplements (binder-local context derivation along
step paths and formation-level recovery), and an inductive-relation
soundness presentation for the synthesis side. These form the immediate
Phase E continuation and require the typing-judgment bridge.

### 2.14 Current validation snapshot

At this checkpoint:

- `pen-production-wire`: 7 unit and 4 cross-language pinning tests passed;
- `pen-semantic-audit --lib`: 166 passed, 6 ignored, 0 failed;
- the safe Agda `ContextChecker`, `BundleChecker`, `BundleEncode`,
  `BundleDecodeTestV1`, `ContextCorrespondenceV1`,
  `ContextTranscriptTestV1`, `SemanticReplayV1`, `SynthesisReplayV1`,
  and `SemanticReplayTestV1` entry points and all transitive wire
  modules type-check with `--safe --without-K --ignore-interfaces`,
  which forces the embedded 1727-byte Rust vector through the full
  decoder, structural checker, and semantic replay — and the three
  semantic mutant vectors through structural acceptance and semantic
  rejection — at type-check time;
- the new wire/bridge source contains no `postulate`, unsafe pragma,
  termination bypass, or unsolved-hole allowance; and
- the protected root workspace manifests and issued H3/H4 artifacts remain
  unchanged.

Ignored tests include separately pinned external/runtime gates; an ignored
test is not counted as authority.

## 3. Detailed forward plan

### Phase A — Freeze and document the bridge contract

Status: substantially complete.

Deliverables:

- production-refinement adjudication V2;
- explicit authority exclusions;
- exact wire grammar and limits;
- exact universe/context/global conventions;
- exact transcript contents; and
- single-factory minting rule.

Exit gate: implementation cannot silently change the accepted semantic
surface after observing a concrete bundle.

### Phase B — Complete the authority-free canonical Rust wire

Status: implemented; final review and hardening remain.

Remaining work:

- add property/fuzz-style mutation coverage where useful;
- ensure every enum tag and boundary length has a direct regression;
- confirm error classification remains stable enough for audit; and
- preserve dependency-free, manual canonical encoding.

Exit gate: every accepted byte string decodes fully and re-encodes to the
identical byte string; all malformed variants fail closed.

### Phase C — Complete safe Agda structural decoding

Status: decoding and structural checking complete for all eleven sections;
canonicality theorems partially complete.

Completed on 2026-07-30:

1. conversions and every reduction-step payload decode;
2. binder-local conversion-typing supplements decode;
3. all eight synthesis-code payloads decode;
4. Q0, fresh schemas, family inventory, and family payloads decode;
5. the Rust resource and full-consumption bounds are enforced (sequence
   caps, byte caps, and the shared 256-level recursion budget); and
6. round-trip/canonicality theorems exist for identifiers, terms,
   reduction steps, synthesis codes, counted lists, and the context, Q0,
   and family-inventory section payloads, plus a genuine Rust byte vector
   accepted end-to-end and seven rejected mutation vectors.

Remaining work:

1. round-trip theorems for the record-shaped payloads (conversions,
   synthesis certificates, supplements, fresh rules, family payloads); and
2. the whole-envelope canonical-encode composition.

Exit gate: reached for parsing and structural rejection; the remaining
canonicality theorems are additive.

### Phase D — Prove finite context/global correspondence

Status: theorem layer complete as of 2026-07-30; capability minting
remains deferred to the Phase H factory.

Completed:

- wire terms map totally onto `PTm global-count local-count` under the
  minimal scope predicate, with erasure round trips in both directions
  and proof irrelevance;
- oldest-first wire contexts build `PCtx` with an erasure round trip,
  injectivity of erasure, and a `psnoc` extension correspondence
  (including the abstract `extend-context` transport);
- the variable index, oldest-first ordinal (`locals - suc index`), and
  shift-distance (`suc index`, realized as iterated intrinsic weakening)
  equations are proven as `variable-lookup-correspondence`;
- the chronological slot table builds a strict-prior signature mirror
  `PSig` with erasure round trips for declaration types and bodies,
  injectivity of the joint erasure, and strict-prior lookup
  correspondences at the exact chronological list ordinal, with global
  weakening erasing to the identity;
- weakening transports through the decoder as an exact wire-level
  de Bruijn shift, and single substitution decodes to `instantiate`
  (renaming/simultaneous substitution transport is inherited from the
  existing `ProductionDecodingV1` layer through the faithful `PTm`
  representation); and
- a first Rust/Agda decoded context/global transcript agrees
  byte-for-byte on the fixture (development pin, not the Phase G
  versioned transcript).

Remaining for this phase's exit gate: `VerifiedFiniteContextCorrespondenceV1`
is minted only by the private factory once Phases E–G supply the other
correspondence legs; the 32-byte `GlobalId`-to-slot binding stays in the
verified Rust slot table and is bound at the capability, not in `PSig`.

### Phase E — Implement the combined conversion/synthesis checker

Status: semantic replay core complete as of 2026-07-30; the
typing-judgment bridge remains.

Conversion work completed:

- beta, authorized transparent delta, and congruence replay
  semantically at the intrinsic level, with soundness into `PStepV1`
  (beta must target the exact instantiation; delta must target the
  exact strict-prior signature body and only for policy-enabled slots;
  congruence frames must fix the unchanged component);
- every trace link is checked semantically and accepted traces yield
  intrinsic `PStepsV1` chains reducing both sides to the one common
  normal form;
- the complete no-redex census recomputation is bridged to semantic
  normality: the accepted common form provably admits no outgoing
  intrinsic step; and
- delta-policy agreement is semantic: the intrinsic delta relation is
  defined by the policy-restricted strict-prior body lookup itself.

Conversion work remaining:

- endpoint typing judgments (`HasType`/`TypeFormation`) are not yet
  replayed against the typing judgment;
- binder-local conversion-typing supplements are not yet consumed: the
  derived nested contexts along step paths and the existential
  formation-level recovery need the typing-judgment bridge.

Synthesis work completed:

- all eight constructors recompute the subject and type semantically;
- variable ordinal and shift metadata are reverified and the type is
  the exact intrinsic context lookup;
- globals resolve by finite slot to their strict-prior declared types;
- binder contexts are reconstructed during recursion, never trusted;
- application function/argument conversions are resolved by identifier
  and consumed under the exact protocol V2 side conditions: identical
  derived context, `TypeFormation` endpoint judgment, full semantic
  replay, left endpoint equal to the synthesized type, and right
  endpoint equal to the census-normal common form (the pi is
  destructured from the normal form, as in the kernel); and
- the dependent result substitution is computed by intrinsic
  instantiation and compared against both the embedded result term and
  the inferred type.

Synthesis work remaining:

- the dependent-result normalization delta: the kernel records the
  kernel-normalized instantiation while this layer requires the raw
  intrinsic instantiation; reconciling them requires the
  bounded-normalization bridge; and
- an inductive-relation soundness presentation (mirroring the
  conversion side's `PStepV1` package) and the bridge to the abstract
  conversion-typing judgment.

Exit gates (unchanged, deferred to the Phase H factory):

- `VerifiedKernelBaseConversionCorrespondenceV1`; and
- `VerifiedSynthesisCodeCorrespondenceV1`.

### Phase F — Complete Q0, fresh-rule, and family correspondence

Status: inventories exist independently; payload relation remains open.

Work:

- decode the exact seven-rule Q0 inventory;
- prove the representation/base-semantic/runtime-public classification;
- check fresh owner/constructor slots, typing, left-linearity, non-recursion,
  scrutinee occurrence, coverage, and substitution stability;
- decode exactly `Seed`, `GenericPublicApplication`, and
  `GenericEquationAction`; and
- transport family naturality through exact payload decoding.

Exit gate: `VerifiedV3InventoryCorrespondenceV1`.

### Phase G — Independent replay and canonical transcript agreement

Status: not implemented.

Work:

- define one versioned transcript codec;
- have Rust replay the canonical bundle through the unchanged kernel and
  synthesis checker;
- have Agda compute the corresponding transcript from the same bytes;
- include decoded contexts, resolutions, inferred types, formation levels,
  conversions, normal forms, synthesis trees, Q0 mappings, fresh-rule
  dispositions, and family mappings;
- compare actual canonical bundle bytes;
- compare actual transcript bytes; and
- only then record their digests.

Exit gates:

- `VerifiedRustProductionReplayV1`;
- `VerifiedAgdaProductionAcceptanceV1`; and
- `VerifiedProductionTranscriptAgreementV1`.

### Phase H — Single private minting factory

Status: capability shapes exist; factory cannot yet succeed.

The only successful factory must consume:

- the verified canonical bundle;
- verified Agda acceptance;
- verified Rust replay; and
- verified exact transcript agreement.

It must check exact V3 manifest, signature, slot table, kernel protocol,
synthesis protocol, delta policy, source tree, accepted sections, inventories,
and actual byte equality before minting the four correspondence capabilities.

Exit gates:

- `VerifiedLambdaUnitProductionRefinementV1`; and
- `VerifiedLambdaUnitTypingMetatheoryV1`.

### Phase I — Native V3 carrier and subject completeness

Status: deliberately blocked behind Phase H.

Work:

1. construct the native rank-0/1/2 V3 carrier;
2. derive required roots and reducts from that carrier;
3. build one canonical subject bundle;
4. prove exact set equality between carrier-required and bundle roots; and
5. invoke the generic accepted-bundle checker.

Exit gate: carrier-derived typed-occurrence completeness. A caller-supplied,
selected, or empty root list cannot satisfy this phase.

### Phase J — Rewrite, quotient, audit, and Law V2 authority

Status: downstream.

Required sequence:

1. typed Q0 stability;
2. complete edge-local reduction graphs;
3. same/nested/disjoint overlap joins;
4. termination/confluence and predecessor conservativity;
5. structural family weakening and restriction;
6. complete family quotient;
7. semantic cost basis and marginals;
8. demand provenance and typed realizations;
9. SR2 provenance injection;
10. full transcript agreement for the resulting vector suite;
11. independent review and freeze; and
12. only then a separately locked Profile A adapter.

### Phase K0 — Formulate and freeze the contextual-internalization profile

Status: registered as a research direction on 2026-07-30
(`docs/260730_contextual_internalization_frontier.md`); formulation
open; not frozen; execution blocked behind Phases E–H.

The H3 direct-eliminator halt is rediagnosed there: the frozen profile
read the unit type as an ordinary finite inductive datatype, and under
that reading the direct eliminator was exactly the correct discharge.
The candidate successor principle — Constitutive Self-Containment
("no operation essential to the formation, substitution, or comparison
of public possibilities may remain permanently external to the world
once it acts nontrivially on sealed public structure") — reads the
first acts as arena-founding and compiles a larger principal debt: the
internalization of reindexing along sealed extensions by its least
universal public interface, stated by universal properties only.

The expected identification of the dischargers with dependent sum and
product is REGISTERED AS A HELD-OUT PREDICTION in that document and
must not inform demand compilation, generation, acceptance,
quotienting, or halting. Work items before any live run: the formal
target-neutral demand schema, the trigger condition, the
least-universal-interface statement in the GF2 fragment, the kernel
Sigma-fragment decision, adversarial review, and the pre-exposure
freeze — all per §5.7 and §12.3. Formulation may proceed in parallel
with the bridge; execution may not precede it.

### Phase K — Resume autonomous Genesis research

Once the semantic and theorem stack is authoritative:

- rerun productive profiles only if they were motivated and frozen before
  observing their results;
- retain all quotient classes rather than selecting a preferred history;
- derive halting from complete empty debt;
- test presentation/renaming invariance;
- compare anonymous output with the historical corpus only after artifacts are
  sealed; and
- treat divergence as a mathematical result, not automatically as a software
  defect.

## 4. Lessons learned

### 4.1 `Unknown` is a successful safety outcome

Several apparent blockers were the checker refusing to manufacture authority
from incomplete evidence. Returning `Unknown` preserved the theorem boundary
and made the next missing proof object explicit.

### 4.2 The semantic profile determines the generated sequence

The narrow H3 profile did not reveal four archived Stage-4 alternatives. It
produced a unique direct eliminator, and the H4 continuation then halted. A
longer sequence requires additional, independently motivated constitutive
structure; it cannot be added retrospectively because the observed halt was
unwelcome.

As of 2026-07-30 a candidate for exactly such independently motivated
structure is registered (Phase K0 and
`docs/260730_contextual_internalization_frontier.md`): the H3 profile
operated at the datatype level of abstraction, and the
contextual-internalization reading assigns the arena-founding acts a
larger principal debt. The registration keeps the anticipated
identification of its dischargers strictly held out.

### 4.3 Similar implementations are not a correspondence theorem

Rust and Agda can each have internally correct datatypes, parsers, inventories,
and tests while still disagreeing about:

- variable direction;
- context order;
- global identity;
- binder-local scope;
- formation universes;
- reduction endpoints; or
- payload meaning.

One canonical input and one exact transcript are the smallest credible bridge.

### 4.4 Hash equality is not byte equality

A digest is useful after an exact comparison has succeeded. A digest supplied
by a caller, or two independently computed digests without retained compared
bytes, cannot serve as semantic authority.

### 4.5 Generic soundness is not completeness of the subject domain

“Every accepted certificate is sound” does not imply “the bundle contains
every root needed by the native carrier.” These are different theorems and
must produce different capabilities.

### 4.6 Finite generator closure was the wrong theorem

A finite set of substitution generators is not closed under arbitrary
composition. The repaired architecture enumerates only finite,
derivation-local witnesses and assigns the unbounded algebraic laws to a
generic theorem package.

### 4.7 Global identities and global slots have different roles

Rust uses stable 32-byte `GlobalId` values for identity and provenance. Agda
uses finite chronological slots for intrinsic scope. The bridge must prove the
mapping from the verified signature; callers may not supply their own order.

### 4.8 Universe values need role-specific bounds

The production boundary distinguishes:

- public syntax levels `{0,1}`;
- checker-produced term/type levels `{0,1,2}`; and
- numeric formation witnesses `{0,1,2,3}`.

A level-3 witness is metadata, not permission for public `Sort(3)`.

### 4.9 Nested congruence requires local contexts

Replaying an outer conversion trace does not prove the typing of a premise
under a binder. The checker must derive each binder-local context and compare
the supplied supplement to that derived context.

### 4.10 Formal executability affects proof design

The Agda crash was caused by a large unary-natural normalization path in a
little-endian `u64` conversion. Rewriting it in Horner form with multiplication
by the small constant on the terminating side restored predictable checking.
Large nested Boolean expressions also obscured reduction; named helper checks
made the proof both clearer and more tractable.

The general lesson is that proof terms and parsers must be designed for
bounded normalization, not merely mathematical correctness on paper.

### 4.11 Evidence-bound workspaces must remain isolated

The root manifests and lockfile are inputs to issued H3/H4 evidence.
Experimental theorem crates therefore use nested workspaces. This creates
some operational friction, but prevents unrelated dependency changes from
silently changing the identity of earlier results.

### 4.12 Field order must be proven against the codec, not the struct

The safe Agda manifest parser was originally written against the Rust
struct-declaration order. The codec writes three of those fields in a
different position, so both sides were internally consistent, fully tested,
and mutually incompatible: every genuine Rust manifest would have been
rejected by Agda. The defect was invisible to per-language test suites and
became obvious the moment one canonical byte vector was fed to both
implementations. This is a concrete instance of lesson 4.3 and the reason
the bridge design insists on shared bytes rather than parallel fixtures;
cross-language byte vectors are now part of the checked Agda surface.

## 5. Big problems and challenges

### 5.1 Exact semantic correspondence

The largest immediate challenge is not serialization. It is proving that
concrete conversion and synthesis payloads have the exact intrinsic Agda
meaning, especially under dependent binders and substitution.

### 5.2 Independent replay without duplicating trust

Rust replay must use the unchanged kernel and must not accept fields merely
because the producer claims they are correct. Agda must consume only the
canonical bytes, not a generated expected answer. Both sides must remain
independent enough that agreement is meaningful.

### 5.3 Complete, bounded transcript design

The transcript must be rich enough to expose every semantically important
choice, deterministic across languages, versioned, resource-bounded, and free
of implementation-specific diagnostic strings.

### 5.4 Carrier completeness and circularity

The native carrier must determine the authoritative root set, but the carrier
itself depends on production typing authority. The ordering must avoid both
directions of circular proof: no carrier before production refinement, and no
completeness claim from roots selected before the carrier.

### 5.5 Rewrite admissibility

Local rule-shape checks are far short of a verified rewrite system. The project
still needs typed stability, complete overlap classification, joins,
termination/confluence, and predecessor conservativity over the exact native
carrier.

### 5.6 Semantic-family completeness and cost

Issuing `kappa` and `nu` requires complete pre/post family carriers, a decided
family quotient, weakening/restriction, marginals, and provenance. Historical
structural values cannot fill these gaps.

### 5.7 Law/profile adoption

Even a correct generic theorem package does not choose the constitutive
profile that should govern Genesis. Any productive extension beyond the
four-act halt needs mathematical motivation, adversarial review, and a
pre-result freeze. Otherwise the experiment becomes target fitting.

### 5.8 Kernel fragment limits

The current trusted kernel is intentionally small. It does not provide the
ambient cubical calculus, path types, univalence, Kan operations, finite sums,
or the full free-sealing universal property. Some future structures may be
outside the fragment rather than refuted.

### 5.9 Scaling formal checks safely

Naive decoding of large `u64` lengths into unary naturals can be expensive.
Proof recursion, decoder fuel, section limits, and transcript size must remain
explicitly bounded. Generated Agda sources and `.agdai` files also require
disciplined cleanup so build products never become protocol inputs.

### 5.10 Maintaining the two-lane honesty boundary

The repository contains excellent legacy fixtures, human names, known target
structures, and extensive target-specific tests. Keeping them available for
regression while proving that production Law V2 crates cannot observe them is
an ongoing architectural and CI obligation.

## 6. Current stop condition

The project has not reached a mathematical impossibility result. It has
reached a precise ordered implementation/theorem frontier.

The next work may lawfully continue with:

1. the remaining record-payload and whole-envelope canonicality theorems;
2. the Phase E typing-judgment bridge (endpoint judgments, binder-local
   supplements, formation levels, dependent-result normalization
   reconciliation, synthesis inductive soundness);
3. exact Q0/fresh/family correspondence;
4. independent Rust replay;
5. canonical transcript agreement; and
6. the single private minting factory.

Item 1 is additive; items 2–6 remain the ordered theorem frontier.
Complete safe Agda decoding and structural checking for sections 5–11,
the Phase D finite context/global correspondence theorem layer, and the
Phase E semantic replay core were discharged on 2026-07-30.

In parallel with the bridge (and lawful at any time, since it exposes
nothing to the registered prefix), the Phase K0
contextual-internalization formulation may proceed: the target-neutral
demand schema, trigger condition, least-universal-interface statement,
kernel-fragment decision, adversarial review, and pre-exposure freeze.
Its execution remains blocked behind Phases E–H without exception.

The work must stop before native carrier construction or live-profile
authority if any of those gates remains unavailable.

## 7. Definition of success for the current plan

The current production-refinement plan is complete only when:

- one canonical bundle is derived from verified Rust inputs;
- the exact same bytes are embedded in and extracted from the fixed Agda input
  module;
- safe Agda generically decodes and accepts all semantic sections;
- Rust independently replays all judgments through the unchanged kernel and
  synthesis protocol;
- Rust and Agda emit byte-identical canonical transcripts;
- the private factory mints all four correspondence capabilities;
- production refinement and the stronger typing metatheory are minted; and
- no authority has been borrowed from Profile A, the native carrier, a
  reference trace, or a caller-provided digest.

That result would not yet prove the full autonomous Genesis theorem, but it
would remove the most important cross-language trust gap currently blocking
the semantic and rewrite layers.
