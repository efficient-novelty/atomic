> Under Law V2A, start from a disclosed registered three-act bootstrap, an anonymous syntactic basis, the two laws, window width \(2\), and a versioned finite Genesis Fragment of depth two (GF2). Enumerate four normalized Stage-4 representative slots, certify and fully quotient them, retain the resulting \(2\) to \(4\) classes without preference, and halt—without knowing the number fifteen—only when every surviving branch carries a GF2-complete certificate that its next obligation profile is empty. Law V2B's derivation of the bootstrap from the empty public context is a later theorem.

However, the algorithm should not be built as a stronger version of the current bar-optimizing search. The mature two-law formulation in the attached appendix changes the executable target:

* At guarded stages, a candidate is accepted because it **totally discharges the live typed obligation**, not because it clears a numerical threshold.
* If several genuinely distinct candidates discharge the obligation, the result is a **cone**, not a deterministic winner.
* The Step-4 search must preserve four normalized representative slots through
  certification. A decided full quotient yields between two and four classes;
  until the Pi/Sigma equivalence is constructed or obstructed, the engine
  returns `Unknown(UnknownQuotient)`.
* The halt must be triggered by a certified empty obligation profile (O(16)=\varnothing), not by reaching a configured step count and not primarily by failing to clear a bar. 
* The windowed bar can remain as a downstream diagnostic, but the adopted formalization explicitly says that it “gates nothing.” 


That would be genuinely significant. It would also be substantially more convincing than the current “recover the expected hash under a guarded search profile” result.

---

# 1. What “genuinely blind” must mean

There are three useful levels of blindness.

| Level                            | Meaning                                                                                                                                                                                                                                                   | Status                                                    |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| **B1: Name-blind**               | The engine manipulates anonymous syntax but may contain stage-specific policies, target-shaped candidate families, expected costs, or a configured endpoint.                                                                                              | Approximately where parts of the current engine sit.      |
| **B2: Trace-blind, fixed-basis** | The engine receives a fixed anonymous grammar, but no Genesis names, target telescopes, step count, expected ledgers, stage-specific family transitions, bar-selected winner, or future-viability rule.                                                   | The right first theorem and software target.              |
| **B3: Basis-generative**         | Even operators corresponding post hoc to truncation, cohesion, connection, temporal structure, and so on are not preinstalled. The engine synthesizes fresh primitive APIs from a generic grammar of possible extensions when typed demand requires them. | Strongest form of your intuition; a later research phase. |

B2 is already a serious result. B3 is what would most literally justify “two laws in, fifteen steps out, nothing else added.”

## Permitted inputs

The production engine may see only:

1. A trusted normalization and type-checking kernel.
2. A finite anonymous grammar—or, for B3, a generic grammar of extension schemas.
3. The Constitutive and Selective Laws.
4. Window width (w=2).
5. Free sealing and equivalence rules.
6. A versioned finite GF2 carrier delimiting completeness claims.
7. Fragment or resource limits that may cause `Unknown/OutsideFragment` or `Unknown/ResourceExhausted`, but never a false halt.

## Forbidden inputs

The production engine must have no access to:

* “Genesis,” “circle,” “Hopf,” “cohesion,” “curvature,” “Hilbert,” “DCT,” or equivalent semantic labels.
* The target number (15).
* `until_step = 15`.
* `Telescope::reference(step)`.
* The expected ((\nu,\kappa)) vectors.
* Named predicates such as `requires_temporal_shell_package`.
* Stage-index branches such as `if step == 12`.
* Bar-clearing as guarded acceptance.
* Deterministic hash or presentation-order tie-breaking.
* “Choose the candidate that allows the expected next step.”
* Baseline parity as part of acceptance.
* Search heuristics trained or modified using the accepted target trace.

Semantic names should be attached by a **separate post-run decoder**, after the run and its certificates have been frozen.

---

# 2. Why the current repository is a strong chassis but not yet the target algorithm

The repository already provides most of the difficult systems infrastructure:

* Anonymous MBTT expressions and telescopes.
* Canonical encodings and exact rational arithmetic.
* Type and connectivity checks.
* Prefix search, memoization, persistence, resumption, telemetry, and stored artifacts.
* A clean Rust hot path with Agda downstream of acceptance.

But several current mechanisms encode the expected trace too directly.

## 2.1 Hard-coded target objects

`Telescope::reference(step)` explicitly contains all fifteen target telescopes, including the final `Next`/`Eventually` telescope, and `all_reference_telescopes()` explicitly iterates from 1 through 15. These are valuable oracle fixtures, but they must be moved out of every production dependency.

## 2.2 Target-shaped debt

`StructuralDebt` currently contains predicates for the expected progression:

* former eliminator,
* initial HIT,
* truncation HIT,
* higher HIT,
* sphere lift,
* axiomatic bundle,
* modal shell,
* connection shell,
* curvature shell,
* operator bundle,
* Hilbert-functional shell,
* temporal shell.

Those predicates also determine exact clause caps and search focus.

That is an effective guided recovery engine, but it is not a blind implementation of

[
O(n+1)=C(S_n,S_{n-1})\setminus D(B_n).
]

The new engine must compute the obligation from typed syntax and derivability, not recognize which named package should come next.

## 2.3 Stage-indexed admissibility

`admissibility.rs` contains explicit handling for Steps 1, 2, and 3, plus named `StructuralFamily` variants and focus policies for the later sequence.

The replacement must accept an opaque history and a typed obligation profile. The stage number may be present in logs, but it must not be an input to admissibility.

## 2.4 Bar-selected acceptance

The current acceptance rank uses:

* bar clearance,
* minimal positive overshoot,
* clause count,
* structural scores,
* novelty,
* canonical presentation order.

The current semantic-minimality checker also asks whether detachable subbundles clear the bar.

All of that must be removed from the law-level lane. A deterministic order may schedule work, but it may not decide which certified act becomes real.

## 2.5 Configured endpoint

The current runtime configuration explicitly contains `until_step: 15` and `selector: "minimal_positive_overshoot"`.

The blind engine must instead run until one of three proof-relevant outcomes occurs:

[
\mathsf{Advance},\qquad
\mathsf{Blocked},\qquad
\mathsf{Halt}.
]

A fragment or resource limit may produce only

[
\mathsf{Unknown}(\text{outside fragment or resource exhausted}).
]

## 2.6 Obsolete numerical register

The current repository still reports Step 15 as `nu = 103`. The mature re-audit says that the lawful semantic-family vector is

[
[1,0,1,3,6,3,1,1,0,4,1,0,2,3,6],
]

and explicitly says that the old structural totals are testimony rather than semantic-family counts; at Step 15, 103 even exceeds the lawful provenance bound for a family count. 

The new executable types should therefore make register mixing impossible:

```rust
enum ValueRegister {
    SemanticFamilyV2,
    LegacyStructuralV1,
}
```

`LegacyStructuralV1` can remain available for historical replay. It must not flow into `pen-law`.

---

# 3. The target executable theorem

I would formalize the first target as follows.

## Blind Genesis Theorem, fixed anonymous basis

Let:

* (\mathfrak C) be the trusted cubical extension kernel;
* (G) be a fixed anonymous normalized constructor basis;
* (\mathcal R_2) be a stage-generic registered calculus of depth-two open demand schemes;
* (w=2);
* \(H_3\) be the disclosed Law-V2A registered three-act bootstrap; and
* \(\mathfrak F_{\mathrm{GF2}}^{\le2}\) be the versioned finite Genesis
  Fragment delimiting executable completeness.

Then the executable procedure `BlindGenesis`:

1. validates and loads the Law-V2A registered bootstrap without claiming
   derivation or uniqueness;
2. at every guarded history (H_n), computes the GF2-complete obligation profile
   [
   O(n+1)=C(S_n,S_{n-1})\setminus D(B_n);
   ]
3. enumerates every equivalence class of Constitutively admissible total dischargers;
4. freely seals each class, branching when several classes survive;
5. preserves four normalized Stage-4 representatives through certification
   and reports the \(2\) to \(4\) classes that survive the full quotient;
6. produces branch-equivalent fifteen-stage histories;
7. terminates only when every branch has a complete empty-debt certificate;
8. does so equivariantly under renaming and presentation changes of (G).

The familiar mathematical names would be a separate theorem:

> An external decoder proves that the sealed anonymous signatures are equivalent to the fifteen named Genesis structures.

That separation is important. The discovery engine should not know what it has discovered.

---

# 4. The central algorithm

The new engine should be a **typed obligation solver**, not primarily a telescope scorer.

```text
Law-V2A registered three-act bootstrap
        │
        ▼
validated initial history (no uniqueness claim)
        │
        ▼
last-two-stage window
        │
        ▼
GF2-complete typed demand census
        │
        ├── all demands derivable ──► HALT CERTIFICATE
        │
        ▼
live obligation orbits
        │
        ▼
GF2-complete response-cone synthesis
        │
        ▼
CL checks + total-discharge proofs + semantic audit
        │
        ▼
full equivalence quotient
        │
        ├── one class ──────────────► free seal
        └── several classes ───────► lawful cone
                                             │
                                             └── repeat per branch
```

## 4.1 Trusted public state

A history should carry only proof-relevant data:

```rust
struct History {
    stages: Vec<SealedExtension>,
    public_boundary: NormalizedSignature,
    derivation_basis: SchemaBasis,
    last_two: Window,
    history_digest: Digest,
}
```

A candidate should match the formal definition more closely than the present bare telescope:

```rust
struct Candidate {
    boundary: NormalizedSignature,
    realization: RealizationTerm,
    induced_obligations: ObligationComplex,
    boundary_inclusion: BoundaryMap,
}
```

That mirrors the formal candidate

[
x=(\partial x,w_x,O_\bullet(x\mid H),b_x).
]

The source requires witness-bearing realization, equivalence invariance, depth-two local satisfiability, and canonicity under sealing. 

---

# 5. The most important component: a genuinely generic demand compiler

The hardest scientific problem is not candidate enumeration. It is implementing (C), the operator that determines what the active window owes.

The current `StructuralDebt` predicates should be replaced by a generic compiler over open typed judgments.

## 5.1 Demand schemes

Represent a demand scheme as:

```rust
struct DemandScheme {
    parameters: DependentContext,
    motive: NormalType,
    open_body: OpenJudgment,
    support_rule: SupportRule,
    specialization_theorem: TotalSpecializationCert,
}
```

The attached formalization requires motives to be declared before the filler exists, and requires every registration to carry a total-specialization theorem. Outcome-dependent filtering is forbidden. 

## 5.2 Generic extraction procedure

For a history (H_n):

1. Normalize the public signatures of (S_n) and (S_{n-1}).
2. Enumerate all registered depth-two open schema families whose support touches the active window.
3. Instantiate their support contexts using canonical dependent comprehension.
4. Normalize every schema using the certified family normalizer.
5. Quotient by:

   * judgmental equality,
   * substitution and naturality,
   * transport along univalent equivalences,
   * family-versus-instance identification.
6. Saturate the current library (B_n) under the same depth-two closure.
7. For each extracted family or instance, decide:

   * `Derived(derivation_certificate)`, or
   * `Live(nonmembership_certificate)`.
8. Collapse equivalent live instances into obligation orbits.
9. Prove that older-window demands remain discharged under weakening.

Conceptually:

[
\begin{aligned}
\mathsf{Extracted}*{n+1}
&= \operatorname{NF}!\left(
\mathsf{OpenSch}*2(S_n,S*{n-1};B_n)
\right)\big/{\approx_H},\
\mathsf{Derived}*n
&= \operatorname{NF}!\left(
\mathrm{Cl}*{\mathfrak C}^{\le2}(B_n)
\right)\big/{\approx_H},\
O(n+1)
&= \mathsf{Extracted}*{n+1}\setminus\mathsf{Derived}_n.
\end{aligned}
]

The runtime artifact must contain three independent certificates required by the formal law:

* extraction completeness,
* derivability completeness,
* locality and expiration. 

## 5.3 Family-level representation

Do not enumerate every substitution as a separate demand. The semantic register explicitly distinguishes a natural family from its many instances. 

Use higher-order patterns or a similarly decidable parametric representation:

```rust
struct SchemaFamily {
    abstract_context: DependentContext,
    normal_body: PatternJudgment,
    naturality_certificate: NaturalityCert,
}
```

Concrete instance enumeration is used only where a complete finite census is required—for example, the 89 Step-16 instances.

## 5.4 Primitive obligation reduction

The raw depth-two closure may contain many consequences of one missing interface. Build a dependency graph among unresolved schema families and retain the irreducible live orbit basis:

```text
raw unresolved schemas
        ↓ derivability graph
strongly connected components
        ↓ quotient
primitive live obligation orbits
```

This reduction must be generic. It must never ask whether the resulting orbit “looks like curvature” or “looks temporal.”

---

# 6. Candidate synthesis should fill typed holes

The existing engine largely enumerates telescopes and then asks whether they look admissible. The new engine should invert that process:

> Start from the exact typed outputs the world owes, and synthesize every
> GF2-admitted demand-connected API that provides them.

## 6.1 Response skeletons

For every live orbit (o), extract its required output interface:

```rust
struct RequiredOutput {
    motive: NormalType,
    parameters: DependentContext,
    support: SupportSet,
}
```

Build candidate skeletons from:

* required output heads,
* irreducible support clauses needed to type them,
* computation clauses,
* the four stage-generic semantic roles:

  * `KernelHead`,
  * `AdjointMate`,
  * `SupportAction`,
  * `Coherence`.

Those four roles are already the formal provenance code; string labels and package membership are explicitly not valid anchors. 

## 6.2 Demand-connectedness

A critical theorem is needed to prevent arbitrary “payment plus unrelated extra structure” candidates.

### Candidate-factorization theorem

Every admissible total discharger (x) factors, up to presentation equivalence, into:

[
x \simeq_H x_{\mathrm{core}}\otimes x_{\mathrm{unowed}},
]

where:

* every irreducible clause of (x_{\mathrm{core}}) lies in the dependency closure of a live discharge proof;
* (x_{\mathrm{unowed}}) contributes no output to any live obligation.

“Nothing without demand” must exclude (x_{\mathrm{unowed}}) as a simultaneous continuation. This turns terminal-SCC pruning into a sound **factorization operation**, not a value-based minimality preference.

After the one-time Law-V2A initialization, every positive-cost clause must
lie in the typed dependency closure of a pre-existing live obligation;
candidate-induced obligations cannot bootstrap their own authority. If
\(O=\varnothing\), no positive-cost candidate is admissible. Zero-cost theorem
readouts append no event.

Until this theorem is proved, “exactly one total discharger” is vulnerable to arbitrary supersets.

## 6.3 Finite GF2 response bound

A blind engine cannot declare uniqueness merely because it has not yet found another candidate. It needs a stage-generic completeness bound.

Define a finite response graph inside the versioned Genesis Fragment:

[
\mathsf{Resp}_{\mathrm{GF2}}(H,O)
]

containing every canonical clause role reachable from:

* the live required outputs,
* their support motives,
* the four local provenance roles,
* depth-two coherence completion.

Then prove:

### GF2 closure and response-bound theorem

Every positive-cost, Constitutively admissible, demand-connected total
discharger admitted by GF2 over \((H,O)\) has a canonical representative whose
irreducible public clauses occur in
\(\mathsf{Resp}_{\mathrm{GF2}}(H,O)\).

Consequently,

[
\kappa_H(x)\le K_{\mathrm{GF2}}(H,O)
:=\left|\mathsf{Resp}_{\mathrm{GF2}}(H,O)\right|.
]

The bound is computed from the obligation profile. It is not taken from the expected winner’s (\kappa).

This theorem is the point at which an empirical enumerator becomes complete
relative to GF2. It makes no unrestricted ambient completeness claim.
Unrestricted recursion, higher-order unification, and arbitrary equivalence
search are outside GF2 and return `Unknown(OutsideFragment)`. The theorem and
its verifier are targets, not current repository results.

## 6.4 Exact synthesis

Within the finite GF2 response graph:

1. Enumerate typed clause candidates by role.
2. Canonically deduplicate individual clauses.
3. Construct dependency-compatible API skeletons.
4. Track which required outputs each partial skeleton can still discharge.
5. Prune only when a proof shows:

   * a typing contradiction,
   * disconnected support,
   * an impossible remaining obligation,
   * a duplicate normal form,
   * a removable unowed component.
6. Materialize complete telescopes.
7. Kernel-check them.
8. Produce discharge proofs.

This can reuse the current prefix infrastructure, but the partial-state summary changes from “can this prefix still clear the bar?” to:

```rust
struct PartialDischargeState {
    discharged_outputs: BitSet,
    outstanding_outputs: BitSet,
    available_support: SupportSummary,
    unresolved_type_constraints: ConstraintSet,
    removable_components: SccSummary,
}
```

The current exact-bar pruning machinery should not be repaired for the new lane. The present Step-15 bottleneck is dominated by `partial_prefix_bar_failure`; under the adopted law, that is the wrong proof obligation.

---

# 7. Constitutive certification

Every synthesized candidate should carry a machine-checkable certificate:

```rust
struct CLCertificate {
    typing: TypingCert,
    realization: RealizationCert,
    equivalence_invariance: EquivarianceCert,
    local_satisfiability: DepthTwoDischargeCert,
    sealing_canonicity: NormalizationCert,
    demand_connectedness: DemandCoreCert,
}
```

The checker, not the generator, is trusted. Search heuristics may propose candidates; only the small certificate checker may certify them.

A failed or missing certificate means rejection. The formal audit is explicitly fail-closed. 

---

# 8. Semantic-family audit

At guarded stages, (\nu) is not a selector, but the formal law still requires a certified audit.

For each candidate (x):

1. Compute the normalized family basis before integration:
   [
   \mathsf{Sch}_2(H).
   ]
2. Compute it after free sealing:
   [
   \mathsf{Sch}_2(I(H,x)).
   ]
3. Apply typed weakening from the old basis.
4. Compute the marginal family set:
   [
   \operatorname{Marg}_2(H;x)
   ==========================

   \mathsf{Sch}_2(I(H,x))
   \setminus
   \operatorname{im}(\mathrm{wk}_x).
   ]
5. Produce an injective provenance assignment into:
   [
   \bigl(\mathsf K_H(x)\times\mathcal R_2\bigr)
   \sqcup
   \coprod_{o\in\mathsf{LiveOrb}(H)}\mathsf{Req}_H(o).
   ]
6. Verify blindness:

   * no stage index,
   * no bar,
   * no target winner,
   * no archived total,
   * no accepted future,
   * equivariance under substitution, weakening, and equivalence.

The semantic audit should be its own crate, independent of diagnostics:

```text
pen-audit ───► pen-kernel
    ▲
pen-law
```

The historical structural evaluator can be retained separately:

```text
pen-diagnostics ───► sealed run artifacts
```

There must be no dependency in the opposite direction.

---

# 9. Guarded selection and the cone

This section runs only when the verified live obligation profile is nonempty.
An authoritative empty profile goes directly to the debt-free halt rule.
After GF2-complete enumeration:

```rust
let classes = full_equivalence_quotient(certified_total_dischargers);
```

Then:

* `classes.len() == 0`:

  * if enumeration is GF2-complete, emit `Blocked` and a fragment-relative
    theory-or-basis refutation;
  * if enumeration is incomplete, outside GF2, or resource exhausted, emit
    `Unknown`, never halt.
* `classes.len() == 1`:

  * freely seal the unique class.
* `classes.len() > 1`:

  * create one branch per genuinely distinct class.

No ranking occurs.

The quotient ladder must be:

1. presentation quotient;
2. natural-family transport quotient;
3. univalent equivalence quotient;
4. confluence test;
5. only then record genuine plurality.

The Stage-4 milestone has four normalized representative slots before the
full quotient. The current repository has not certified those representatives.
Once they and the order-axis obstruction replay, the quotient may contain only
\(2\) to \(4\) classes until the Pi/Sigma former-axis equivalence is
constructed or obstructed. Archived healing comparisons concern the four
representative continuations, not four proved worlds.

A single displayed Genesis sequence is then an external branch index:

```text
lawful output: full quotient with 2–4 Stage-4 classes
book display:  one representative-indexed presentation
```

---

# 10. Free sealing

For each accepted equivalence class:

1. Construct the least realization of its public API.
2. Quotient only by:

   * previous judgmental equalities,
   * equations forced by the new API.
3. Add no unrequested generator or equation.
4. Normalize and seal the public signature.
5. Prove the boundary inclusion.
6. Update the derivation basis.
7. Advance the active width-two window.

The free-sealing universal property governs realization after acceptance; it must not be repurposed as a preference rule. 

---

# 11. Autonomous halt

The engine should have no semantic target step.

At the beginning of each prospective extension:

```rust
let census = extract_and_decide_demands(history.last_two(), &history.library)?;
let census = verify_complete_gf2_census(census)?;

if census.live_orbits.is_empty() {
    let exclusion = certify_no_positive_cost_continuation(census)?;
    return StageOutcome::Halt(build_halt_certificate(census, exclusion));
}
```

`verify_complete_gf2_census` returns `Unknown(OutsideFragment)` or
`Unknown(ResourceExhausted)` rather than an empty census whenever its
completeness premises cannot be established.

A `HaltCertificate` should contain:

```rust
struct HaltCertificate {
    history_digest: Digest,
    window_digest: Digest,

    extraction_complete: ExtractionCompletenessCert,
    derivability_complete: DerivabilityCompletenessCert,
    expiration_complete: ExpirationCert,

    instance_census: Vec<DecidedDemandInstance>,
    live_orbit_count: usize,       // must be zero
    f1_exclusion: FalsifierCert,

    gf2_fragment_digest: Digest,
    no_positive_cost_continuation: DemandConnectednessCert,
}
```

For the expected run, the independent oracle should find:

* fifteen sealed acts;
* all 89 prospective Step-16 demand instances derivable;
* zero live obligation orbits;
* no demanded-but-underdetermined F1 instance;
* the same result on every class that survives a certified Stage-4 quotient.

The engine must distinguish four outcomes:

```rust
enum RunOutcome {
    Advanced(Cone),
    Halted(HaltCertificate),
    Blocked(UnpaidDemandCertificate),
    Unknown(UnknownReason),
}

enum UnknownReason {
    OutsideFragment,
    ResourceExhausted,
    UnknownQuotient,
}
```

This distinction is essential. “No candidate found before the memory limit” must never be reported as “the world is debt-free.”

---

# 12. Core loop pseudocode

```rust
fn blind_genesis(
    kernel: &Kernel,
    grammar: &AnonymousGrammar,
    scheme_calculus: &DemandSchemeCalculus,
) -> RunOutcome {
    let initial = validate_registered_v2a_bootstrap(kernel)?;
    let mut cone = Cone::singleton(initial);

    loop {
        let mut successors = ConeBuilder::new();
        let mut halts = Vec::new();

        for history in cone.branches() {
            let census = demand_census(
                kernel,
                scheme_calculus,
                history.active_window(2),
                history.derivation_basis(),
            )?;

            let census = verify_complete_gf2_census(&census)?;

            if census.live_orbits().is_empty() {
                let exclusion =
                    certify_no_positive_cost_continuation(history, &census)?;
                halts.push(build_halt_certificate(history, census, exclusion)?);
                continue;
            }

            let response_graph =
                construct_response_graph(kernel, history, census.live_orbits())?;

            let response_bound =
                certify_response_bound(history, census.live_orbits(), &response_graph)?;

            let raw_candidates =
                enumerate_complete_response_cone(
                    kernel,
                    grammar,
                    history,
                    census.live_orbits(),
                    &response_graph,
                    response_bound,
                )?;

            let mut acceptable = Vec::new();

            for candidate in raw_candidates {
                let cl = certify_constitutive_admissibility(
                    kernel,
                    history,
                    &candidate,
                )?;

                let discharge = certify_total_discharge(
                    kernel,
                    history,
                    census.live_orbits(),
                    &candidate,
                )?;

                let audit = certify_semantic_family_audit(
                    kernel,
                    history,
                    &candidate,
                )?;

                acceptable.push(CertifiedCandidate {
                    candidate,
                    cl,
                    discharge,
                    audit,
                });
            }

            let classes =
                quotient_and_test_confluence(kernel, history, acceptable)?;

            if classes.is_empty() {
                return RunOutcome::Blocked(
                    certify_unpaid_demand(history, census, response_bound)?
                );
            }

            for class in classes {
                successors.add(free_seal(kernel, history, class)?);
            }
        }

        if successors.is_empty() {
            return RunOutcome::Halted(
                combine_cone_level_halt_certificates(halts)?
            );
        }

        cone = successors.finish_with_equivalent_branch_merging()?;
    }
}
```

There is deliberately no `step_index` parameter in any proof-relevant function.

---

# 13. The bootstrap problem must be addressed explicitly

The attached formalization says that Steps 1–3 are **registered founding acts**, not selected acts. 

That means the current theory does not yet literally prove “two laws alone derive all fifteen” in the same sense for every step. A genuinely blind implementation needs one of two honest contracts.

## Law V2A: registered three-act bootstrap

Treat a disclosed, versioned three-act founding prefix as input to the
machine's public specification:

1. validate the three registered acts as well-typed sealed input;
2. record their exact digest in the run manifest;
3. make no derivation, leastness, or uniqueness claim; and
4. transfer jurisdiction when the first live demand appears.

This is an honest initial-condition contract, not an empty-context theorem.

## Law V2B: least-arena derivation and uniqueness theorem

Prove that the least fixed point of constitutive arena formation from the empty context has, up to equivalence:

[
\mathcal U_0,\qquad \mathbf 1,\qquad \star:\mathbf 1,
]

and that the first exported obligation appears exactly after the witness.

This is the stronger target and is not a current theorem. Until it is proved,
the precise claim is:

> The registered constitutive bootstrap supplies Steps 1–3; the blind two-law discharge engine autonomously derives the guarded cone from Step 4 onward.

That is still a strong result and avoids hiding the bootstrap issue in code.

---

# 14. The demand-scheme registration problem

A second potential hidden script lies in the phrase “motives are declared at scheme registration.”

If the registry manually contains twelve schemes recognizable as the expected next obligations, the algorithm is not blind even if all names are removed.

The scheme registry must therefore be generated by a small stage-generic calculus, for example:

```rust
enum SchemeConstructor {
    FormationUse,
    IntroductionElimination,
    ComputationClosure,
    NaturalitySquare,
    AdjointMate,
    SupportAction,
    CoherenceCell,
    WindowCompatibility,
}
```

These should be combinators over arbitrary normalized public signatures—not named Genesis packages.

The required theorem is:

### Scheme-generation theorem

Every authoritative registered demand family is generated from the generic scheme calculus and the current public boundary alone; registration commutes with grammar renaming, substitution, weakening, and univalent equivalence.

The scheme generator must not contain a table indexed by stage or telescope class.

---

# 15. Fixed-basis versus grammar-generative discovery

The current syntax globally contains constructors such as `Flat`, `Sharp`, `Next`, and `Eventually`.

That is acceptable for B2 if they are anonymized and the engine does not know which will be used. But it is not yet the strongest “nothing else added” version.

## B2 implementation

Use an anonymous grammar file:

```rust
struct PrimitiveSchema {
    id: OpaqueOpId,
    arity: Arity,
    parameter_context: DependentContext,
    result_motive: NormalType,
    equations: Vec<EquationSchema>,
}
```

The engine sees only typing and equation schemas. Human labels are stored in a decoder file unavailable to the run.

## B3 implementation

Replace concrete primitive variants with a grammar of possible fresh public operators:

```rust
enum MetaHead {
    TypeFormer,
    TermFormer,
    PathConstructor,
    UnaryEndofunctor,
    BinaryFormer,
    NaturalTransformation,
    ComparisonMap,
    CoherenceCell,
}
```

When a live obligation has no discharger in the existing public grammar:

1. certify that the current response cone is exhausted;
2. enumerate fresh operator schemas that could inhabit the outstanding motives;
3. require every fresh operator to occur in a discharge proof;
4. require a conservative free realization;
5. quotient equivalent extensions;
6. branch if several genuinely distinct minimal extensions survive.

This would allow the engine to synthesize anonymous counterparts of cohesive or temporal operators rather than selecting pre-existing named atoms.

B3 should follow B2. Attempting both at once would make it difficult to know whether failures come from the law engine or from grammar synthesis.

---

# 16. Step 15 requires a stronger witness than a structural telescope

The chapter itself distinguishes:

* an inconsistent strict combination of infinitesimal identification and unrestricted Löb;
* a viable filtered completion;
* remaining exchange-law and mechanization obligations. 

Consequently, a law-faithful Step-15 certificate cannot merely say “this telescope has `Next` and `Eventually` in the expected arrangement.”

It must provide either:

1. a kernel-checked realization of the filtered DCT specification; or
2. a syntactic shell certificate explicitly marked as awaiting semantic realization.

The post-run decoder should label the anonymous final shell “DCT” only after the filtered model certificate passes. Otherwise the honest claim is “the engine rediscovered the eight-clause temporal-cohesive shell,” not yet the fully realized mathematical object.

This should be treated as an explicit proof workstream, not patched in the evaluator.

---

# 17. Proposed crate architecture

I would add a new lane rather than gradually mutate the legacy engine.

```text
                         pen-oracle
                    dev/test/decoder only
                             │
                             ▼
sealed run artifacts ◄── pen-diagnostics
                             ▲
                             │  no reverse dependency
                             │

pen-kernel ◄── pen-demand ◄── pen-synth
    ▲              ▲             ▲
    │              │             │
    └────────── pen-audit ───► pen-law
                                  ▲
                                  │
                              pen-engine
                                  ▲
                                  │
                              pen-cli
                                  │
                              pen-store
```

## Recommended roles

| Component         | Responsibility                                                                                  |
| ----------------- | ----------------------------------------------------------------------------------------------- |
| `pen-kernel`      | Normalization, type checking, derivation checking, equivalence certificates.                    |
| `pen-demand`      | Open schemes, family normalization, orbit quotient, derivability saturation, demand census.     |
| `pen-synth`       | Complete demand-directed candidate synthesis and proof-producing pruning.                       |
| `pen-audit`       | Semantic-family audit and provenance injection.                                                 |
| `pen-law`         | CL checker, total-discharge acceptance, cone logic, open-stage halt.                            |
| `pen-engine`      | Branch iteration, free sealing, confluence, autonomous termination.                             |
| `pen-diagnostics` | Legacy bars, windowed bar, old structural scores, reports only.                                 |
| `pen-oracle`      | Fifteen target telescopes, human labels, expected trace decoder; never a production dependency. |

---

# 18. File-by-file migration from `atomic`

| Existing file or area                   | Action                                                                                                                                                              |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `crates/pen-core/src/telescope.rs`      | Move `reference()` and `all_reference_telescopes()` into `pen-oracle`. Keep only generic telescope operations in production.                                        |
| `crates/pen-core/src/expr.rs`           | B2: retain syntax but expose anonymous operator IDs to the engine. B3: migrate toward data-driven `PrimitiveSchema`.                                                |
| `crates/pen-core/src/atom.rs`           | Move semantic string names to decoder metadata; production uses opaque IDs and typing rules.                                                                        |
| `crates/pen-type/src/obligations.rs`    | Replace `StructuralDebt` and every `requires_*_package()` predicate with open judgments, demand families, instances, orbit ledgers, and certificates.               |
| `crates/pen-type/src/admissibility.rs`  | Remove `StructuralFamily`, named package policies, step-index cases, and exact target-derived clause bands in the new lane. Implement `CLCertificate` verification. |
| `crates/pen-search/src/enumerate.rs`    | Generate clauses from live motives and support contexts, not from a named late-family surface.                                                                      |
| `crates/pen-search/src/prefix_memo.rs`  | Cache outstanding obligations and type feasibility rather than bar-clearance potential.                                                                             |
| `crates/pen-search/src/branch_bound.rs` | Replace `AcceptRank` bounds with proof-producing discharge-feasibility bounds.                                                                                      |
| `crates/pen-search/src/accept.rs`       | Return the complete quotient set of total dischargers; delete overshoot and deterministic tie-breaking from the new lane.                                           |
| `crates/pen-eval/src/minimality.rs`     | Replace “bar-clearing subbundle” with candidate factorization and demand-connected core proofs.                                                                     |
| `crates/pen-eval/src/nu.rs`             | Archive as legacy structural evaluator. Implement a separate semantic-family audit.                                                                                 |
| `crates/pen-eval/src/bar.rs`            | Move behind a diagnostics-only dependency boundary.                                                                                                                 |
| `crates/pen-search/src/engine.rs`       | Remove target-stop semantics and step-specific viability overrides. Loop until certified halt, block, or unknown.                                                   |
| `crates/pen-search/src/config.rs`       | Remove `until_step` from the lawful run contract. Keep only resource budgets and kernel/grammar digests.                                                            |
| `pen-store`                             | Add demand, discharge, equivalence, semantic-audit, branch, and halt certificate schemas.                                                                           |
| `pen-agda`                              | Independently verify frozen certificates; remain downstream and unable to influence acceptance.                                                                     |

---

# 19. Implementation phases and exit gates

## Phase 0 — Freeze the legacy engine

Create a permanent `legacy_bar_v1` profile and freeze its current artifacts.

Deliver:

* `docs/LAW_V2_EXECUTABLE_SPEC.md`
* `docs/BLINDNESS_CONTRACT.md`
* `docs/CHEATING_THREAT_MODEL.md`
* a versioned register distinction.

**Exit gate:** Current runs remain reproducible, but no new-law code depends on their outcomes.

## Phase 1 — Build the oracle firewall

Move:

* target telescopes,
* target labels,
* expected hashes,
* expected ((\nu,\kappa)),
* step-count expectations,

into a new `pen-oracle` crate available only as a dev-dependency or post-run decoder.

Add a CI rule that production crates cannot depend on it.

**Exit gate:** The lawful production binary builds and runs after the entire oracle crate and target reference files are physically removed.

## Phase 2 — Introduce proof-carrying kernel objects

Implement:

* normalized signatures,
* dependent contexts,
* open judgments,
* derivation certificates,
* equivalence certificates,
* free-sealing certificates,
* CL certificates.

**Exit gate:** Every candidate accepted by the new lane can be rechecked from its stored certificate by a small deterministic verifier.

## Phase 3 — Implement the demand census

Replace the named debt summary with:

* depth-two scheme enumeration,
* family normalization,
* active-window support,
* derivability saturation,
* orbit quotient,
* completeness and expiration certificates.

Initially run it against frozen known prefixes, but do not give it expected next labels.

**Exit gate:** On the frozen prefixes, the external oracle observes:

* one live obligation orbit at each guarded stage;
* complete derivability decisions;
* no use of stage number or expected family in extraction;
* an empty live set after the final sealed shell.

## Phase 4 — Prove and implement the finite GF2 response cone

Implement:

* demand-connected response graphs,
* candidate-factorization checking,
* \(K_{\mathrm{GF2}}(H,O)\) response bounds,
* GF2-complete clause enumeration,
* proof-producing prefix pruning.

**Exit gate:** Enlarging operational search caps cannot introduce a previously
omitted in-fragment legal response. Outside-fragment work and resource
exhaustion return `Unknown`, and the GF2 closure theorem replays independently.

## Phase 5 — Replace acceptance with total discharge

Implement:

* CL filtering,
* discharge matrices,
* complete acceptable-class enumeration,
* equivalence quotient,
* cone branching,
* free sealing.

Delete all bar imports from `pen-law`.

**Exit gate:** Removing the entire diagnostics crate leaves the accepted cone byte-identical.

## Phase 6 — Reproduce the cone

Run from the registered bootstrap.

**Exit gate:**

* four individually certified normalized representatives at the first guarded
  stage;
* a certified full quotient yielding between two and four classes, or
  `Unknown(UnknownQuotient)` if the quotient cannot be decided;
* no deterministic branch selection;
* one acceptable class at each later guarded stage on each branch;
* confluence/healing certificates matching the formal claims;
* all branches reach a debt-free state.

## Phase 7 — Implement the semantic-family register

Implement the act-local marginal-family audit and provenance injection.

**Exit gate:** The post-run audit reproduces the semantic vector, with the
appropriate representative-indexed Stage-4 distinction, without consulting
archived scores. The current structural `103` remains available only under a
legacy register tag.

## Phase 8 — Autonomous halt certification

Remove semantic target length from the CLI and engine.

**Exit gate:** The engine integrates fifteen acts as an observed result, invokes a prospective sixteenth demand census, decides every instance, and halts solely because the certified live orbit set is empty.

## Phase 9 — Derive the bootstrap

Replace the registered bootstrap telescopes with the least arena-founding closure and prove bootstrap uniqueness.

**Exit gate:** The engine begins from the genuinely empty public context and produces the first three seals without target fixtures or explicit “run three founding steps” logic.

## Phase 10 — Grammar-generative blind search

Replace the fixed concrete operator basis with fresh anonymous schema synthesis.

**Exit gate:** Removing concrete cohesive and temporal atoms does not prevent their structural APIs from being synthesized when demanded.

## Phase 11 — Presentation invariance

Run:

* renaming tests,
* conservative signature extensions,
* alternative syntax presentations,
* grammar permutations,
* cap and enumeration-order perturbations.

**Exit gate:** The output cone is invariant up to the certified equivalence relation.

---

# 20. Anti-cheating certification suite

| Threat                     | Test                                                                                         | Required result                                                                            |
| -------------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| Hidden target dependency   | Delete `pen-oracle` and all reference telescope files before building the production binary. | Build and run still succeed.                                                               |
| Stage-index guidance       | Replace stage numbers with random opaque event IDs.                                          | Same cone up to event-ID renaming.                                                         |
| Semantic-name leakage      | Randomly permute all anonymous operator IDs and withhold the inverse map.                    | Equivariantly permuted output.                                                             |
| Presentation-order bias    | Shuffle grammar declarations, clause catalogs, worker schedules, and enumeration order.      | Same equivalence classes.                                                                  |
| Hash tie-breaking          | Change canonical hash algorithm while preserving equality.                                   | No change to lawful branches.                                                              |
| Bar dependence             | Remove `pen-diagnostics`, including all bar code.                                            | No change to accepted histories.                                                           |
| Expected-future bias       | Disable every next-step-viability routine.                                                   | No change to lawful branches.                                                              |
| Incomplete frontier        | Vary memory limits and worker counts.                                                        | Either same certified result or `Unknown`; never a different “winner.”                     |
| Conservative syntax sugar  | Add aliases and definable primitives.                                                        | Same post-quotient cone.                                                                   |
| Irrelevant distractors     | Add coherent but demand-disconnected primitives.                                             | They do not become simultaneous continuations.                                             |
| Missing grammar capability | Remove a primitive needed to discharge a later obligation.                                   | Engine reports a live unpaid demand or enters grammar synthesis; it does not falsely halt. |
| Step-4 freedom             | Randomize discovery order of the four representative slots and replay every quotient rung.  | All four representatives appear; the certified quotient yields 2–4 classes or `Unknown(UnknownQuotient)`; none is privileged. |
| Fragment escape            | Inject an input outside GF2 and separately exhaust a resource budget.                         | `Unknown(OutsideFragment)` or `Unknown(ResourceExhausted)`; never uniqueness, blockage, or halt. |
| Halt falsifier             | Inject one synthetic demanded-but-underdetermined scheme.                                    | Halt fails and F1 is reported.                                                             |
| Step-16 completeness       | Independently replay the census.                                                             | All expected instances have valid derivations and zero live orbits.                        |

A particularly strong invariant is equivariance:

[
\mathsf{BlindGenesis}(\pi G)
\simeq
\pi\bigl(\mathsf{BlindGenesis}(G)\bigr)
]

for every admissible renaming or presentation automorphism (\pi).

---

# 21. Stored evidence format

Each stage should write a proof bundle such as:

```text
checkpoints/stages/stage-XX/
    history.json
    window.json
    extraction-certificate.json
    derivability-certificate.json
    expiration-certificate.json
    live-orbits.json
    response-bound.json
    enumeration-manifest.json
    candidate-classes.json
    cl-certificates/
    discharge-certificates/
    equivalence-certificates/
    semantic-audit.json
    seal-certificate.json
```

The final run should additionally contain:

```text
cone.json
confluence-certificate.json
halt-certificate.json
blindness-manifest.json
build-provenance.json
```

`blindness-manifest.json` should record:

* production dependency graph,
* kernel digest,
* grammar digest,
* scheme-calculus digest,
* law digest,
* absence of oracle dependencies,
* absence of target-length configuration,
* source commit and binary digest.

---

# 22. Concrete first pull-request sequence

The most efficient sequence of actual repository changes is:

### PR 1 — `law-v2-oracle-firewall`

* Add `pen-oracle`.
* Move all reference telescopes and human labels.
* Add dependency-denial CI.
* Add `ValueRegister`.
* Freeze legacy behavior.

### PR 2 — `law-v2-certificate-types`

* Add normalized open judgments.
* Add derivation, demand, CL, discharge, equivalence, and halt certificate schemas.
* Add a small certificate verifier.

### PR 3 — `law-v2-demand-census`

* Replace `StructuralDebt` in the new lane.
* Implement depth-two saturation and orbit extraction.
* Replay all frozen prefixes.
* Produce the 89-instance final census artifact.

### PR 4 — `law-v2-response-cone`

* Add demand-directed clause synthesis.
* Add demand-connectedness.
* Add response-bound certificates.
* Replace bar-based prefix bounds with discharge-feasibility bounds.

### PR 5 — `law-v2-cone-engine`

* Accept all total dischargers.
* Implement full quotient and branch handling.
* Remove `until_step`.
* Emit `Advanced`, `Blocked`, `Halted`, or `Unknown`.

### PR 6 — `law-v2-semantic-audit`

* Implement act-local semantic families.
* Move all historical scoring to diagnostics.
* Add blindness and provenance certificates.

### PR 7 — `law-v2-bootstrap`

* Replace explicit Steps 1–3 with least constitutive bootstrap closure.
* Add bootstrap uniqueness tests and proof artifacts.

### PR 8 — `law-v3-generative-basis`

* Replace concrete answer-bearing operators with fresh schema synthesis.
* Run conservative-extension and grammar-ablation campaigns.

---

# 23. The right success claim at each stage

Before bootstrap derivation and grammar generation, the honest strong claim would be:

> Under Law V2A, from a disclosed registered three-act bootstrap, anonymous
> fixed basis, registered stage-generic depth-two scheme calculus, the two
> laws, window width two, and GF2, the engine certifies four normalized
> Stage-4 representatives and computes the full quotient with between two and
> four classes (or returns `Unknown(UnknownQuotient)`). It halts after fifteen
> sealed acts only if the next GF2-complete obligation profile is certified
> empty and demand-connectedness excludes every positive-cost continuation.
> No target step, semantic name, expected score, numerical selector, novelty
> criterion, or future-viability criterion participates in acceptance.

After the bootstrap uniqueness theorem:

> The same result begins at the genuinely empty public context.

After grammar-generative synthesis and presentation-invariance:

> The fifteen-stage cone is an invariant of the two-law dynamics across the certified moduli space of admissible bases.

That final statement would come very close to the strongest form of your intuition.

---

# Bottom line

The current repository should be treated as:

* an excellent search-and-artifact infrastructure,
* an oracle corpus,
* a performance laboratory,
* and a source of negative controls,

but **not** as the specification of the blind algorithm.

The decisive conceptual changes are:

1. replace target-shaped structural debt with complete typed demand extraction;
2. replace arbitrary telescope scoring with proof-directed hole filling;
3. prove the finite GF2 closure and response bound so in-fragment completeness
   is certified;
4. accept by total discharge, not by bar;
5. certify four Stage-4 representatives, then return the certified quotient
   with two to four classes or `Unknown(UnknownQuotient)`;
6. halt from an empty certified obligation profile, not from `until_step`;
7. isolate every target fixture and semantic decoder outside the production dependency graph;
8. disclose the Law-V2A registered bootstrap, and keep Law-V2B least-arena
   derivation and uniqueness as a separate later theorem;
9. treat the final temporal-cohesive shell as fully realized only when the filtered DCT witness is mechanized.

## Execution checkpoint — 2026-07-28

The oracle firewall and executable-law split are now implemented. The current
lawful closure also contains:

* a versioned GF2 feature/limit manifest and four-way decision boundary;
* a native adapter that proves only supported dependent-core judgments and
  fails closed on unsupported fragment features;
* an Agda 2.8.0 / `cubical-0.9` fixed-source readiness probe with
  independently supplied trusted Agda and Git executable-digest pins,
  a repository-reviewed canonical Cubical tree and an independently reviewed
  primitive-runtime pin supplied by trusted configuration for the exact Agda
  distribution, sanitized Git/process execution, and private canonical checker
  snapshots. Text sources are UTF-8 with CRLF canonicalized to LF and bare
  carriage returns rejected; primitive `.agdai` members remain raw. The local
  primitive reference digest is a reproducibility aid, not an authenticity
  anchor;
* a strict anonymous, kernel-replayed Law-V2A three-act registration; and
* native intrinsic registrations for type formation, typed terms, and
  definitional computation, with resource-bounded replay of particular
  complete assignments.

None of these is the GF2 closure theorem. The checker probe has no
caller-source theorem API; the scheme slice has no full generator or
universal total-specialization proof; and the registered prefix has no
Law-V2B leastness claim. The next implementation move is therefore to freeze
the actual finite GF2 carrier and scheme calculus, implement the restricted
GF2-to-Cubical-Agda translation, and prove generator/response closure before
attempting the absolute first guarded cone.

Execution stopped at the
[H3-to-Stage-4 census blocker](LAW_V2_H3_CENSUS_BLOCKER.md): the adopted
records bound the intended schema carrier but do not define the complete
stage-generic scheme-generation relation \(C(S_n,S_{n-1})\). Implementing
that relation now would choose new formal semantics. A versioned adoption of
the scheme calculus and its induction principle is required before the next
law-level code step.
