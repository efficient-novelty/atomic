# Steps 9–15 registered-signature adjudication (G-1 batch)

**Date:** 2026-07-20. **Status:** **ADJUDICATION REQUIRED — G-1
BATCH; NO GRAMMAR CONSTRUCTOR AUTHORIZED BY THIS DOCUMENT YET.**

This is the single batched decision required by G-1 of
`agent_e_grammar_completion_plan.md`.  It covers all seven sealed rows at
once.  Until the adoption sentence at the end is affirmed, G-2 through G-8
remain gated.

## Firewall and notation

Grounds used here are only:

1. the exact `Telescope::reference(9..=15)` trace and its exact historical
   predecessor signatures;
2. the frozen grammar requirements in `step_15_completion_open_problem.md`;
3. G-3's already characterized J3 list for Step 15; and
4. for the endpoint-dependent treatment that follows adoption, the already
   adopted `endpoint-premise-ledger-api-rule-v1`.

No bar, archived novelty count, pending membership verdict, M1 result, or
desired effect on a score is a ground for any proposed signature.  Historical
candidate and canonical hashes below identify the rows; they are not
constructor grounds.  The M1 sweep is an archival predecessor to be replayed,
but its outcome is expressly a non-ground for the unit-coherence decision.

Compact raw syntax is the serde syntax of `pen_core::expr::Expr`: `L<n>` is
`Lib(n)`, `v<n>` is `Var(n)`, and the other heads have their Rust names.
Every clause uses its frozen primary `ClauseRole`.  “Trace signature” means
the exact raw expression interpreted under the frozen kernel binding
convention and exact predecessor `∂H_(s-1)`; it does not silently assert a
stronger textbook signature.

## Exact row bindings

| Step | Label | Class | predecessor digest | candidate hash | canonical hash | κ / bits |
|---:|---|---|---|---|---|---:|
| 9 | Hopf | Map | `blake3:ac214686168ca9db360782f7b09fa9e89ade5e6839e933a72ae14e282d423e7a` | `blake3:f57e3a5aa44003adb5f8054013e549e4f3065757d88922313478e8e445825491` | `blake3:db6a9c039fc70d0f2595143ed87bd948041943105ba106556b71ae8c09c9f777` | 4 / 78 |
| 10 | Cohesion | Modal | `blake3:78e6444fdb136cda56a23714ebf9464adb3dac1c32fb49e7063bbb8cdc6d7f32` | `blake3:93289041755cf4c4e0396029273822630028999d945359b10bb88e2376b9788e` | `blake3:09bedb4929cb15f0d7b9a5096b33e1df608d8aa6271064c7055d86aa24cc4ef5` | 4 / 45 |
| 11 | Connections | Axiomatic | `blake3:6aecbe0f3688ae5d072497af8b2d008969d6d0d5efe4b5bee9b0480a21a7228d` | `blake3:03cc71839428ceab088e386e31e6c72b7ca4a1e12c9309557efc76826dcb88cd` | `blake3:0b7476332068e12ce833fdfe0b4246603257d6bfdfce728c1be1ea8a6f2fae27` | 5 / 79 |
| 12 | Curvature | Axiomatic | `blake3:a743317dc1b629d988ffa744c95b69ed4dd572ce70979fd742d0d3756a2f2eb0` | `blake3:0d06e3b14bfd7c1bd16d9f66039e016f84a6162ce85584f7c641753b833b1dcb` | `blake3:fd63680d219c2f711599492331596756059a32ea31d2de245c1eeff0b8363cac` | 6 / 121 |
| 13 | Metric | Axiomatic | `blake3:7ce822571cefa6e5c6351044b5c637305867b1af2b0df564e5e9a32ebb5b772b` | `blake3:b09b3f832bef16953747c213e6b8a548f594339dc7db9148e7ae60a6c53a4cf1` | `blake3:4eef295703d8c92f975a36fa0ee082bf1c4928622617d90e8201cb3069a3f5ae` | 7 / 138 |
| 14 | Hilbert | Axiomatic | `blake3:098799481f6344c06c33e505ccac86eaf3a8fd3f5cb0843c37d186fc5538faac` | `blake3:1ff4820f2272c022a5fece1032aa9647e41167a3dd8f62765e84e22f5d90b19c` | `blake3:435e9a3c57940a352db131398d16d0dcbe73c363df3ecf88fc7c9465d5fcc67d` | 9 / 169 |
| 15 | DCT | Synthesis | `blake3:a440d7df1d88fb210d79848975cd776847dcb5bdd930a25ab727e29b279c0459` | `blake3:e919c8419bbafde89e3e99ff25f348f3c8b679ed22685e84d3cdec634ebf90d4` | `blake3:6f4b4a2f40c60aa263446457214a4edda85975d81766067a7dbb4a204642c3a4` | 8 / 229 |

## Complete raw vocabulary and proposed registered names

The role in parentheses is frozen.  The name after `⇒` is the proposed stable
Schema2 identifier.  Except at Step 15, a semantic nickname is documentation,
not an additional primitive law.

- **Step 9 / Hopf / Map:**
  `Pi(L8,L7)` (formation) ⇒ `hopf_map_shell`;
  `App(L5,v1)` (introduction) ⇒ `hopf_fiber_instance`;
  `Lam(App(L8,L7))` (introduction) ⇒ `hopf_total_witness`;
  `Pi(L7,L8)` (formation) ⇒ `hopf_reverse_or_classifying_shell`.
  The last raw arrow has the opposite orientation from a conventional
  base-to-classifier map.  The trace therefore does **not** warrant silently
  replacing it by `S2 → BAut(S1)` or an equivalence.  The recommended
  registration preserves `Pi(L7,L8)` exactly and treats “classifying” as a
  non-normative nickname.

- **Step 10 / Cohesion / Modal:**
  `Flat(v1)` (formation) ⇒ `flat_former`;
  `Sharp(v1)` (formation) ⇒ `sharp_former`;
  `Disc(v1)` (formation) ⇒ `disc_former`;
  `Shape(v1)` (formation) ⇒ `shape_former`.
  These are the only four trace exports.  The trace does not specify an
  adjunction ordering and contains no separate unit, counit, triangle, or
  composition clause.  Such data may be *generated* later by an adopted
  generic adjoint/mate rule, but must not be registered as additional Step-10
  primitive exports.

- **Step 11 / Connections / Axiomatic:**
  `Pi(L10,Pi(v1,v1))` (formation) ⇒ `connection_shell`;
  `Lam(Pi(v1,v2))` (introduction) ⇒ `connection_intro`;
  `Pi(Flat(v1),v1)` (formation) ⇒ `flat_connection_action`;
  `App(L10,v1)` (introduction) ⇒ `modal_connection_instance`;
  `Lam(v1)` (introduction) ⇒ `connection_unit_action`.

- **Step 12 / Curvature / Axiomatic:**
  `Pi(L11,Pi(v1,v1))` (formation) ⇒ `curvature_shell`;
  `Lam(App(L11,v1))` (introduction) ⇒ `curvature_intro`;
  `Pi(v1,L11)` (formation) ⇒ `connection_targeting_map`;
  `App(L11,App(v1,v2))` (introduction) ⇒ `connection_composite_instance`;
  `Lam(Pi(v1,v2))` (introduction) ⇒ `curvature_law_action`;
  `Pi(L11,L11)` (formation) ⇒ `connection_endomap`.

- **Step 13 / Metric / Axiomatic:**
  `Sigma(Pi(v1,v1),Pi(v1,v1))` (formation) ⇒ `operator_pair_shell`;
  `Pi(Sigma(v1,v2),L11)` (formation) ⇒ `metric_connection_map`;
  `Pi(v1,Pi(v1,v1))` (formation) ⇒ `binary_metric_operation`;
  `Lam(App(v1,v2))` (introduction) ⇒ `metric_evaluation`;
  `Pi(L12,L12)` (formation) ⇒ `curvature_endomap`;
  `Lam(Pi(v1,v1))` (introduction) ⇒ `metric_operator_action`;
  `Pi(L12,v1)` (formation) ⇒ `curvature_supported_family`.

- **Step 14 / Hilbert / Axiomatic:**
  `Sigma(Pi(v1,Pi(v1,Univ)),v1)` (formation) ⇒ `hilbert_functional_shell`;
  `Pi(v1,v1)` (formation) ⇒ `hilbert_endomap`;
  `Pi(v1,Sigma(v1,v1))` (formation) ⇒ `decomposition_shell`;
  `Pi(Lam(v1),Sigma(v1,v2))` (formation) ⇒ `spectral_shell`;
  `Sigma(Pi(v1,v1),Pi(v1,v1))` (formation) ⇒ `operator_algebra_pair`;
  `Pi(L13,v1)` (formation) ⇒ `metric_supported_family`;
  `Pi(L12,v1)` (formation) ⇒ `curvature_supported_family`;
  `Pi(L11,v1)` (formation) ⇒ `connection_supported_family`;
  `Lam(Pi(v1,Univ))` (introduction) ⇒ `functional_derivative_action`.

- **Step 15 / DCT / Synthesis:**
  `Next(v1)` (formation) ⇒ `next_former`;
  `Eventually(v1)` (formation) ⇒ `eventually_former`;
  `Pi(Next(v1),Eventually(v1))` (formation) ⇒ `next_eventually_comparison`;
  `Lam(App(L10,Next(v1)))` (introduction) ⇒ `cohesive_temporal_interaction`;
  `Pi(Flat(Next(v1)),Next(Flat(v1)))` (formation) ⇒ `flat_next_exchange`;
  `Pi(Sharp(Eventually(v1)),Eventually(Sharp(v1)))` (formation) ⇒
  `sharp_eventually_exchange`;
  `Lam(App(Eventually(v1),v2))` (introduction) ⇒
  `eventually_eliminator`;
  `Pi(Next(Next(v1)),Next(v1))` (formation) ⇒ `next_composition`.
  This is exactly J3's two formers, comparison, interaction, two exchanges,
  eliminator, and composition in sealed clause order.  **No J3/trace mismatch
  was found.**

## Recommended batch decision

The trace fixes raw heads, references, order, and frozen roles.  It does not
fix the stronger textbook meanings suggested by several historical labels.
The most conservative interpretation compatible with the Two-Law firewall is
therefore:

1. register all 43 trace signatures under the stable identifiers above,
   against their exact predecessor digests;
2. make trace signatures, not semantic nicknames, normative for typing;
3. give Step 15 the exact J3 role mapping above;
4. expose only the four Step-10 formers as primitive Step-10 declarations;
   units/counits/mates/triangle and composition laws must be derived by a
   separately replayable generic rule or remain absent;
5. do not strengthen the Hopf reverse shell into a classifier/equivalence;
6. implement endpoint-dependent naturality through the already-adopted public
   12-form endpoint-premise API; and
7. leave G-6 undecided.  This G-1 batch authorizes no Step-8 unit-coherence
   generator.  After adoption, G-6 must assess the sealed Step-8 trace and
   frozen grammar spec on their own terms; the M1 sweep outcome must be
   recorded as a non-ground.  Either a separately trace/spec-grounded rule or
   an explicit no-generator finding remains possible at that later gate.

This choice deliberately leaves richer intended mathematics as generated
structure or named future refinements.  It supplies a finite, exact interface
that can be typed and falsified without changing the sealed checker.

## Consequences and falsifiers

- Adoption authorizes implementation of G-2 through G-7 only under the exact
  predecessor binding and the no-extra-export rule above; G-8 remains last.
- A registered signature that fails term typing triggers F-G1 and a versioned
  successor proposal.  The checker is not loosened.
- A proposed generator whose provenance reaches a score, count, pending
  verdict, or sweep result triggers F-G2 and is invalid.
- A trace item not expressible after adoption triggers F-G3 and is reported
  verbatim.
- No membership verdict, E-2b count, F-Q2 score, halt claim, or continuation
  claim is authorized by this adjudication.

## Adoption block

To adopt the complete batch in one act, affirm exactly:

> I adopt `steps-9-15-trace-faithful-signature-batch-v1`, including the
> no-extra-primitive-export rule, the exact J3 mapping for Step 15, the
> endpoint-premise API dependency, and the requirement that the separate G-6
> unit-coherence decision remain verdict-blind.

Absent that affirmation, the status remains **ADJUDICATION REQUIRED** and G-2
through G-8 stay closed.
