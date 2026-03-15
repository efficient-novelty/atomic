Quantum-Inspired Classical Acceleration
Code Improvement Plan for pen-atomic
Repository: github.com/efficient-novelty/atomic
Prepared: March 2026  •  Status: Implementation-Ready  •  Classification: Internal

1. Motivation and Background
1.1 What the Engine Does Today
The pen-atomic Rust workspace implements the Principle of Efficient Novelty (PEN) as a deterministic synthesis loop over anonymous Minimal Binary Type Theory (MBTT) telescopes. Starting from an empty accepted library over a fixed type-theoretic signature, the engine iteratively extends the library by selecting the admissible candidate that maximizes structural efficiency (ρ = ν/κ, the ratio of marginal derivational capacity to irreducible specification clauses) while clearing a history-dependent integration bar that grows at the Fibonacci rate.
The current bounded live atomic lane recovers a deterministic 15-step trajectory culminating in a temporal-cohesive shell (semantically read as the Dynamical Cohesive Topos) with ν = 103 and ρ = 12.88. The hot path is structural and name-free: admissibility, candidate generation, exact evaluation, semantic minimality, and minimal-positive-overshoot selection are all computed without target-conditioned semantic rewards.
1.2 Why Quantum-Inspired Acceleration
The companion paper "The Algorithmic Big Bang" reinterprets the PEN generative sequence through quantum-information theory. The pre-physical epoch is modeled as a coherent superposition of well-typed HoTT telescopes in a metatheoretic Hilbert space. Algorithmic time τ acts as an adiabatic evolution parameter, ill-typed candidates are eliminated by destructive interference, and the Step-15 temporal shell triggers endogenous decoherence.
This quantum picture is not implementable literally (we cannot simulate 2⁶¹⁰ amplitudes on a desktop). However, it provides three actionable structural metaphors that map cleanly onto known classical acceleration techniques:
•	Superposition → shared prefix/frontier states instead of duplicating whole telescopes
•	Interference → quotienting/canonicalization so equivalent branches cancel early
•	Bounded entanglement (d = 2) → small cached state signatures, because active historical dependency is strictly local
•	Adiabatic evolution / tunneling → deterministic continuation schedules and diversified branch ordering
1.3 The Key Engineering Constraint
The Coherence Window Theorem proves that in fully coupled intensional HoTT (CCHM), maintaining global canonicity imposes a coherence window of depth d = 2. This two-step memory forces integration debt to follow the Fibonacci recurrence Δₙ₊₁ = Δₙ + Δₙ₋₁, raising the survival threshold at the rate of the Golden Ratio φ ≈ 1.618. Only structures with superlinear novelty growth survive; discrete alternatives (natural numbers, classical logic) yield only linear novelty and are permanently rejected.
This d = 2 bound is the most important implementation constraint: the active historical context never exceeds two layers, so prefix signatures, memoization windows, and cached transition tables can all be compact.
 
2. Review of Proposed Quantum-Inspired Ideas
Three quantum-inspired acceleration ideas were proposed. Each is assessed for compatibility with the engine’s strict correctness contract (exact integer/rational comparisons, exact acceptance, deterministic ordering, name-free hot path).
Proposed Idea	Verdict	Rationale
Simulated Quantum Annealing / Tunneling	ADAPT	Stochastic acceptance violates the exactness contract. But reinterpreted as search-order control (deterministic temperature schedules, replica beams, large-neighborhood restarts), it becomes a strong way to diversify branch exploration without corrupting correctness. All profiles still feed the same exact downstream acceptance logic.
Tensor Networks / MPS from d = 2 Window	ADAPT	d = 2 is a statement about active historical coherence debt, not a proof that telescope search has a clean nearest-neighbor tensor structure. Literal MPS contractions are premature. The realistic translation is: small prefix signatures, dynamic programming / memoization over that signature, shared expansion tables by clause position and obligation state, and possible ZDD/automaton compression later.
Equality Saturation / Phase Interference (egg crate)	ADAPT	Full egg over dependent, binder-heavy MBTT search is high risk (extraction, soundness, interaction with exact admissibility are all nontrivial). The good version is: normalize expressions earlier, hash-cons shared subexpressions, quotient equivalent late-family forms before DFS. This makes "interference" mean "equivalent junk dies early." An optional egg sidecar for a tiny audited rewrite set in late families is acceptable as a Phase 3 experiment.

3. Inviolable Correctness Rules
Every change proposed in this plan must respect the following hard rules. Any acceleration that violates these is rejected regardless of measured speedup.
1.	Exact integer/rational arithmetic on the hot path. No floats for ρ, κ, ν, or bar comparisons.
2.	Deterministic, reproducible selection. Given the same config and step, the same candidate wins.
3.	Name-free hot path. No semantic labels ("metric", "Hilbert", "DCT") in the search/eval loop.
4.	Exact acceptance ordering. Admissibility → bar clearance → semantic minimality → minimal positive overshoot. No stochastic relaxation of any gate.
5.	Strict mode parity. The guarded strict canon must remain unchanged: same 15-step sequence, same ν values, same accepted telescopes.
6.	Acceleration is subordinate. Any new acceleration layer (prefix cache, e-graph sidecar, replica beams) operates outside the authoritative acceptance contract. It may reduce what gets evaluated, but never changes what gets accepted.
 
4. Implementation Phases
Phase 1: Exact Online Prefix Search (Highest Priority)
Metaphor: Superposition — many futures share one prefix state instead of being re-enumerated.
Impact: This is the single highest-return change. It converts the engine from enumerate-all-then-group to best-first prefix expansion with sound pruning.
Current State
The engine already has PrefixState, frontier records, worker scheduling, bar pruning, and deterministic priority keys. In search_next_step, the code enumerates telescopes, then in realistic shadow groups complete telescopes by terminal prefixes, builds a retained prefix set, evaluates retained groups exactly, and then runs exact dedupe, semantic minimality, and acceptance over the full pool. The missing piece is expanding prefixes online as first-class search states.
What to Build
•	A new bounds.rs module with a sound PrefixBound API: nu_lower(prefix), nu_upper(prefix), rho_upper(prefix) = nu_upper / clause_kappa.
•	A new prefix_cache.rs keyed by a compact PrefixSignature: clause position, obligation set hash, support shape hash, active-window signature, relevant family flags.
•	Refactor realistic_frontier_shadow so it does best-first prefix expansion, not enumerate-all-then-group.
•	Keep final exact evaluation, semantic minimality, and acceptance completely unchanged.
Pseudocode (Rust-Flavored)
while let Some(prefix) = frontier.pop_best() {
    if dedupe.seen(prefix.signature()) { continue; }
    let bound = bounds::compute(&prefix, &ctx);
    if bound.rho_upper < bar { continue; } // exact prune
    if prefix.is_complete() {
        let candidate = exact_evaluate(prefix, &ctx)?;
        exact_pool.push(candidate);
        continue;
    }
    for clause in catalog.next_clauses(&prefix, &ctx) {
        if let Some(child) = prefix.try_extend(clause, &ctx.checks)? {
            frontier.push(child.with_bound(bounds::compute(&child, &ctx)));
        }
    }
}
Files to Touch
crates/pen-search/src/engine.rs    ← main loop refactor
crates/pen-search/src/state.rs     ← PrefixState upgrade
crates/pen-search/src/frontier.rs  ← best-first queue
crates/pen-search/src/bounds.rs    ← NEW: sound bounding API
crates/pen-search/src/prefix_cache.rs ← NEW: signature-keyed cache
crates/pen-search/src/priority.rs  ← extend priority key
crates/pen-search/src/scheduler.rs ← integrate prefix expansion
crates/pen-search/src/worker.rs    ← adapt worker batches
Success Criteria
•	Same accepted 15-step sequence in guarded mode.
•	Same accepted sequence in realistic shadow mode.
•	Materially fewer full telescope evaluations at late steps (measure via counter).
•	No introduction of semantic labels into the hot path.
 
Phase 2: Memoized Local State from d = 2 (Medium Priority)
Metaphor: Bounded entanglement — the active dependency window is always exactly 2, so state signatures and transition tables can be compact.
Impact: Incremental legality checks and cached clause catalogs eliminate redundant work during prefix expansion.
Current State
Enumeration calls check_telescope at every partial prefix and records terminal prefixes with two clauses remaining. Late families already use bounded generative clause families with positional structural filters. This is exactly the pattern where incremental legality summaries and memoized prefix transitions pay off.
What to Build
•	Introduce a PartialCheckSummary struct so that check_telescope becomes incremental rather than re-checking from scratch.
•	Cache connectivity/admissibility summaries by PrefixSignature (reuse from Phase 1).
•	Precompute per-position clause catalogs under each admissibility mode.
•	Cache support-filter results for operator/Hilbert/temporal families.
•	If merge rates are high, add a decision-diagram or automaton layer for clause families by position.
Files to Touch
crates/pen-eval/src/check.rs       ← incremental check_telescope
crates/pen-search/src/prefix_cache.rs ← extend with check summaries
crates/pen-search/src/catalog.rs    ← NEW or refactored: clause catalogs
crates/pen-eval/src/families.rs     ← cache family filter results
Success Criteria
•	Incremental-check cache hit rate > 60% at steps 10+.
•	Per-position clause catalog lookup is O(1) amortized.
•	No change to accepted canon.
Phase 3: Restricted Interference Layer (Lower Priority)
Metaphor: Phase interference — equivalent junk cancels early, just as ill-typed ASTs interfere destructively in the quantum picture.
Impact: Reduces redundant exploration in late-family generation. This is an optional acceleration sidecar, explicitly outside the authoritative acceptance contract.
What to Build (Three Incremental Steps)
7.	Expression normalization before clause insertion. Canonicalize equivalent clause forms before they enter the search tree.
8.	Hash-consing / structural DAG sharing. Generated expressions share storage; identical subexpressions are never duplicated.
9.	Optional egg sidecar for a tiny rewrite set in late families. Use the egg crate with a small, audited set of equivalences for operator/Hilbert/temporal subfamilies only. Gate behind a shadow profile.
Critical Constraint
The e-graph must never become the source of truth. Anything extracted still passes through: exact structural check → exact evaluation → exact minimality → exact acceptance. Place this in pen-accel or a new optional crate.
Files to Touch
crates/pen-accel/src/normalize.rs   ← NEW: expression normalization
crates/pen-accel/src/hashcons.rs    ← NEW: hash-consed DAG
crates/pen-accel/src/egraph.rs      ← NEW: optional egg sidecar
crates/pen-search/src/engine.rs     ← integrate sidecar hook
Phase 4: Deterministic Continuation Profiles (Lower Priority)
Metaphor: Adiabatic evolution / tunneling — diversified search order lets the engine escape local-optimum traps without stochastic acceptance.
Impact: Multiple deterministic beam profiles explore different structural corners of the search space, all feeding the same exact acceptance pool.
What to Build
Add new shadow profiles that differ only in branch ordering, not in acceptance logic:
•	realistic_frontier_shadow_tau0: aggressively follows focus-family structure.
•	realistic_frontier_shadow_tau1: favors bridge/support states.
•	realistic_frontier_shadow_tau2: favors high rho_upper slack.
At increasing τ, gradually vary: priority weight on focus-family alignment, diversity bonus for rare bridge heads, penalty for structural debt, historical-anchor rigidity, and tie-breaking among equal-bound states.
Replica Beam Pattern
•	Each beam runs independently with its own priority function.
•	Beams exchange retained prefix states by signature (deduplicated).
•	The final candidate pool is still exact-evaluated.
•	This is a much better classical analog of tunneling than simulated annealing.
Files to Touch
crates/pen-search/src/priority.rs     ← parameterized priority functions
crates/pen-search/src/scheduler.rs    ← multi-beam scheduling
configs/                              ← new shadow profile configs
Phase 5: Declarative Late-Family Grammar (Future)
Metaphor: Moving from hand-tuned constructors to declarative grammar schemas, letting the prefix engine decide which branches survive.
What to Build
•	Encode late families (operator, Hilbert, temporal) as declarative clause-shape grammars.
•	Derive admissible clause families from grammar + active-window obligations.
•	Keep reference survival as an inclusion test, not a constructor shortcut.
•	Let the prefix search engine (Phase 1) drive the enumeration.
 
5. Instrumentation and Metrics
The repo already has comparison/report machinery, run artifacts, and preserved search profiles. Use that infrastructure and add the following counters:
Counter	Purpose
prefixes_created	Total prefix states generated during search
prefixes_merged_by_signature	Prefix states deduplicated before expansion (Phase 1 payoff)
incremental_check_cache_hits	Saved re-checks from Phase 2 memoization
exact_prefix_prunes_by_bar	Prefixes killed early by rho_upper < bar (Phase 1 payoff)
full_telescopes_evaluated	The primary metric: should drop materially at late steps
canonical_dedupe_prunes	Redundant complete telescopes removed before scoring
semantic_minimality_prunes	Candidates rejected because a proper sub-bundle clears the bar
wall_clock_seconds	Total elapsed time per step
peak_rss_bytes	Memory high-water mark
frontier_hot_cold_sizes	Frontier size over time (to detect blowup)

6. Anti-Patterns to Avoid
The following are explicitly prohibited. If a code review reveals any of these, the change must be reverted before merging.
Anti-Pattern	Why It Is Prohibited
Stochastic acceptance in strict_canon_guarded	Violates deterministic selection contract. Destroys reproducibility.
Replacing exact rationals with floats on the hot path	Floating-point comparison is not exact. Bar clearance and rho ordering require exact arithmetic.
Letting egg/accelerator bypass exact check/eval/minimality/acceptance	Acceleration reduces what gets evaluated; it never changes what gets accepted.
Introducing semantic labels into the hot path	The engine searches over anonymous MBTT ASTs. Labels are post-hoc readings only.
Widening the grammar before prefix-first pruning is real	Without prefix bounds, wider grammar means combinatorial explosion with no pruning benefit.
Marketing MPS/e-graphs as literal quantum computation	These are classical data structures with quantum-inspired motivation. The code comments and docs should be honest.

7. Recommended Implementation Order
If you can only do one thing, do Phase 1. It is the most principled classical analog of the quantum picture and the most likely real speedup.
Phase	Description	Priority	Risk	Prerequisite
1	Exact online prefix search	CRITICAL	Medium	None
2	Memoized local state (d = 2)	HIGH	Low	Phase 1 signatures
3	Restricted interference layer	MEDIUM	High	Phase 1 complete
4	Deterministic continuation profiles	MEDIUM	Low	Phase 1 complete
5	Declarative late-family grammar	LOW	Medium	Phase 1 + 2

8. One-Paragraph Summary
The best path is not "simulate the universal mind as a quantum computer." It is: keep the current exact selector; move from whole-telescope enumeration to exact online prefix search (Phase 1); exploit d = 2 as memoized local state compression (Phase 2); use restricted quotienting as interference (Phase 3); use deterministic continuation profiles as adiabatic/tunneling-inspired search order (Phase 4); and eventually move late families to declarative grammars (Phase 5). Every acceleration operates outside the authoritative acceptance contract. The strict 15-step canon is invariant.
