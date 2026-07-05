//! Runtime calculus, Stage-1 addendum (`docs/RUNTIME_CALCULUS.md` §9,
//! implementation pins in §9.1): the instantiation-field generator and the
//! cadence step loop.
//!
//! **The schedule is the Selective Law at token scale (§9).** Within a
//! cadence step, applications are chosen by the standard selection loop over
//! the instantiation field: admissibility first, bar-clearing, smallest
//! sufficient overshoot, constructive primeness. The step ends at local
//! exhaustion — when nothing in the local field clears, the step closes
//! (Rule 2: the widening move is the only remaining motion; it is *not*
//! implemented here — detecting when it activates is the excluded crossing).
//!
//! Implementation pins (disclosed in §9.1, summarized):
//!
//! 1. **Field refresh at step boundaries (P-window).** The bindable domain
//!    of cadence step k is the set of entries sealed *before* step k; R1
//!    dedup runs against all accepted history including within-step seals.
//! 2. **Bar convention (engine precedent).** The n-th acceptance of the
//!    stratum faces Bar_n = Φ_n·Ω_{n−1} on the stratum's own fresh ledger
//!    (plain cumulative Ω; Bar_1 = 0 on the empty ledger); clearing is
//!    ρ ≥ Bar_n; the winner minimizes the overshoot ρ − Bar_n, ties broken
//!    by the deterministic enumeration order.
//! 3. **σ in sealing order.** With tuple bindings, σ maps each stratum entry
//!    to its successor in sealing order. A clause is R1-discounted iff its
//!    canonical presentation is already accepted, or its σ-preimage's is.
//!    Restricted to single-chain instantiation this is exactly
//!    `runtime_dedup`'s Stage-1 rule.
//! 4. **Bounded search (disclosed lane policy, NOT claimed forced).**
//!    Candidates are enumerated deterministically: binding vectors in which
//!    every slot either stays internal (its frozen Var form) or binds a
//!    sealed entry, lexicographic with keep-internal first and then the
//!    domain newest-first (adjacency priority) — so the all-internal vector,
//!    the frozen schema itself, is candidate one. Scoring stops at a
//!    per-round budget (`FieldPolicy::search_budget`). The mechanics
//!    artifact must include the budget sensitivity check before the §9
//!    freeze.
//!
//! Reviewer finding applied (2026-07-05, post-artifact): the first artifact
//! run showed a connectivity gate in the loop excluded the *entire* tuple
//! field (binding any slot severs the schemas' de Bruijn chains), including
//! the σ-chain continuations that RC-1's own path scores — contradicting
//! §9's "dedup and the bar do the limiting". The gate is removed; partial
//! bindings are included (excluding them would be an arity cap). See §9.1.
//!
//! **Stage-1 exclusion (enforced by absence), unchanged:** nothing here
//! computes, searches for, or logs a crossing index; the loop never consults
//! the ledger demand Δ_k or any engagement fraction; no type derives
//! `Serialize`.

use crate::nu::structural_nu;
use crate::runtime_bar::phi;
use crate::runtime_dedup::map_expr;
use pen_core::canonical::{canonical_key_expr, CanonKey};
use pen_core::clause::ClauseRec;
use pen_core::expr::Expr;
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use std::cell::Cell;
use std::collections::BTreeSet;

/// Engine guard only (not a semantic cap): Φ_n needs F_n, and the engine's
/// Fibonacci is exact u64/i64 arithmetic. A stratum sealing this many
/// applications would need a wider integer before longer runs.
const MAX_SEALINGS_GUARD: u32 = 90;

/// Disclosed lane policy for the bounded field search (§9.1 pin 4).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldPolicy {
    /// Maximum number of candidates *scored* per selection round. Policy,
    /// not principle: must be disclosed with the run and sensitivity-checked
    /// across budgets in the mechanics artifact.
    pub search_budget: u64,
}

/// One accepted application in the step loop.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceptedApplication {
    /// Cadence step during which this application sealed.
    pub cadence_step: u32,
    /// 1-based position n in the stratum's own sealing order.
    pub sealing_index: u32,
    /// The full instantiation (seals as a whole; Rule 1).
    pub telescope: Telescope,
    /// κ-weight of the genuinely-new residual actually scored.
    pub residual_kappa: usize,
    /// R1-discounted ν charged for this application.
    pub nu: u32,
    /// Specification κ of the full instantiation (R-F1: R1 is ν-only).
    pub kappa: u32,
    /// ρ = ν/κ faced against the bar.
    pub rho: Rational,
    /// Bar_n = Φ_n·Ω_{n−1} this acceptance had to clear.
    pub bar: Rational,
}

/// One cadence step: the applications accepted before local exhaustion
/// closed the step. Mechanics only — no demand, no engagement, no crossing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CadenceStepReport {
    /// 1-based cadence step index k.
    pub cadence_step: u32,
    /// Library size at step opening = the bindable domain (entries sealed
    /// before this step; §9.1 pin 1).
    pub domain_size: u32,
    /// Candidates scored across all selection rounds of this step.
    pub candidates_scored: u64,
    /// Applications accepted this step, in sealing order.
    pub accepted: Vec<AcceptedApplication>,
}

/// The Genesis ambient context (library and ν history through Step 15),
/// exposed for the step loop's consumers and the mechanics artifact.
pub fn genesis_reference_context() -> (Library, Vec<(u32, u32)>) {
    let mut library = Vec::new();
    let mut history = Vec::new();
    for step in 1..=15 {
        let telescope = Telescope::reference(step);
        let result = structural_nu(&telescope, &library, &history);
        library.push(LibraryEntry::from_telescope(&telescope, &library));
        history.push((step, result.total));
    }
    (library, history)
}

/// Number of binding slots of a schema: its stratum-internal Var
/// *occurrences* (`Var(d)` in clause `j` with `d <= j` — the engine's
/// clause-connectivity convention, as in R-F2), counted in the deterministic
/// pre-order traversal.
pub fn schema_slot_count(schema: &Telescope) -> usize {
    schema
        .clauses
        .iter()
        .enumerate()
        .map(|(clause_index, clause)| {
            let counter = Cell::new(0_usize);
            map_expr(&clause.expr, &|leaf| {
                if let Expr::Var(depth) = leaf {
                    if *depth <= clause_index as u32 {
                        counter.set(counter.get() + 1);
                    }
                }
                None
            });
            counter.get()
        })
        .sum()
}

/// Instantiates the schema over a partial binding vector: the i-th
/// stratum-internal Var occurrence (pre-order) becomes `Lib(index)` when
/// `binding[i]` is `Some(index)`, and keeps its frozen internal Var form
/// when `binding[i]` is `None`. The grammar binds; dedup and the bar do the
/// limiting (§9: no arity cap, no domain whitelist — and no all-slots
/// requirement, which would be an arity cap by another name).
pub fn instantiate_partial_binding(schema: &Telescope, binding: &[Option<u32>]) -> Telescope {
    assert_eq!(
        binding.len(),
        schema_slot_count(schema),
        "binding vector must cover every slot"
    );
    let cursor = Cell::new(0_usize);
    Telescope::new(
        schema
            .clauses
            .iter()
            .enumerate()
            .map(|(clause_index, clause)| {
                ClauseRec::new(
                    clause.role,
                    map_expr(&clause.expr, &|leaf| {
                        if let Expr::Var(depth) = leaf {
                            if *depth <= clause_index as u32 {
                                let slot = cursor.get();
                                cursor.set(slot + 1);
                                return binding[slot].map(Expr::Lib);
                            }
                        }
                        None
                    }),
                )
            })
            .collect(),
    )
}

/// Full-binding convenience: every slot bound (`instantiate_partial_binding`
/// with all `Some`).
pub fn instantiate_binding(schema: &Telescope, binding: &[u32]) -> Telescope {
    let full: Vec<Option<u32>> = binding.iter().copied().map(Some).collect();
    instantiate_partial_binding(schema, &full)
}

/// σ-preimage in sealing order (§9.1 pin 3): stratum-entry references
/// (indices strictly above the ambient library) shift down one sealing
/// position; the first stratum entry has no predecessor, so an expression
/// referencing it has no σ-preimage.
fn sigma_preimage_expr(expr: &Expr, base_library_len: u32) -> Option<Expr> {
    if expr.lib_refs().contains(&(base_library_len + 1)) {
        return None;
    }
    Some(map_expr(expr, &|leaf| match leaf {
        Expr::Lib(index) if *index > base_library_len => Some(Expr::Lib(index - 1)),
        _ => None,
    }))
}

/// The R1 discount test in field form: a clause scores 0 iff its canonical
/// presentation is already accepted (history-derivable repeat, P-ν), or its
/// σ-preimage's is (transport image, P-internal). Restricted to single-chain
/// instantiations this coincides with `runtime_dedup`'s rule.
pub fn is_transport_image(
    expr: &Expr,
    base_library_len: u32,
    accepted_clause_keys: &BTreeSet<CanonKey>,
) -> bool {
    if accepted_clause_keys.contains(&canonical_key_expr(expr)) {
        return true;
    }
    match sigma_preimage_expr(expr, base_library_len) {
        Some(preimage) => accepted_clause_keys.contains(&canonical_key_expr(&preimage)),
        None => false,
    }
}

#[derive(Clone, Debug)]
struct ScoredCandidate {
    telescope: Telescope,
    residual_kappa: usize,
    nu: u32,
    kappa: u32,
    rho: Rational,
}

/// Admissibility + R1 scoring of one candidate. Gate (§9.1): ρ > 0 (the v1
/// lesson: a ν = 0 candidate is not a live producer). Dedup and the bar do
/// the limiting — the structural connectivity gate was removed after the
/// first mechanics artifact showed it excluded the entire tuple field
/// (reviewer finding, 2026-07-05; see §9.1). The pen-type structural-family
/// packages are not applied to stratum instantiations — no pinned mapping
/// exists (flagged in §9.1 for the reviewer).
fn score_candidate(
    candidate: &Telescope,
    base_library_len: u32,
    accepted_clause_keys: &BTreeSet<CanonKey>,
    library: &Library,
    history: &[(u32, u32)],
) -> Option<ScoredCandidate> {
    if candidate.clauses.is_empty() {
        return None;
    }

    let residual = Telescope::new(
        candidate
            .clauses
            .iter()
            .filter(|clause| {
                !is_transport_image(&clause.expr, base_library_len, accepted_clause_keys)
            })
            .cloned()
            .collect(),
    );
    let nu = structural_nu(&residual, library, history);
    if nu.total == 0 {
        return None;
    }

    let kappa = u32::try_from(candidate.kappa()).ok()?;
    Some(ScoredCandidate {
        telescope: candidate.clone(),
        residual_kappa: residual.kappa(),
        nu: nu.total,
        kappa,
        rho: Rational::new(i64::from(nu.total), i64::from(kappa)),
    })
}

/// Deterministic candidate stream (§9.1 pin 4): partial-binding vectors in
/// lexicographic order. Per slot the option order is keep-internal first,
/// then the step's domain newest-first; the all-internal vector — the frozen
/// schema itself — is therefore candidate one. Last slot fastest.
struct CandidateStream<'a> {
    schema: &'a Telescope,
    domain: &'a [u32],
    odometer: Vec<usize>,
    exhausted: bool,
}

impl<'a> CandidateStream<'a> {
    fn new(schema: &'a Telescope, slots: usize, domain: &'a [u32]) -> Self {
        Self {
            schema,
            domain,
            odometer: vec![0; slots],
            exhausted: false,
        }
    }

    fn next_candidate(&mut self) -> Option<Telescope> {
        if self.exhausted {
            return None;
        }

        // Odometer position 0 = keep the internal Var; position p >= 1 =
        // bind the p-th domain entry (newest first).
        let binding: Vec<Option<u32>> = self
            .odometer
            .iter()
            .map(|position| {
                if *position == 0 {
                    None
                } else {
                    Some(self.domain[*position - 1])
                }
            })
            .collect();
        let candidate = instantiate_partial_binding(self.schema, &binding);

        // Advance the odometer (last slot fastest); radix = keep + domain.
        let radix = self.domain.len() + 1;
        let mut advanced = false;
        for slot in (0..self.odometer.len()).rev() {
            self.odometer[slot] += 1;
            if self.odometer[slot] < radix {
                advanced = true;
                break;
            }
            self.odometer[slot] = 0;
        }
        if !advanced {
            self.exhausted = true;
        }

        Some(candidate)
    }
}

/// Runs the cadence step loop for `cadence_steps` steps (§9).
///
/// Per step: selection rounds over the step's field — admissible candidates
/// clearing Bar_n = Φ_n·Ω_{n−1} compete, the smallest sufficient overshoot
/// wins, the winner seals wholly (library entry + discounted ν into the
/// history + clauses into the accepted set + stratum ledger update) — until
/// a round finds nothing clearing: local exhaustion, the step closes.
///
/// Constructive primeness holds by construction: every candidate is a single
/// schema instantiation, never a package of applications.
pub fn run_step_loop(
    schema: &Telescope,
    ambient_library: &Library,
    ambient_history: &[(u32, u32)],
    cadence_steps: u32,
    policy: &FieldPolicy,
) -> Vec<CadenceStepReport> {
    let base = u32::try_from(ambient_library.len()).expect("library size should fit u32");
    let slots = schema_slot_count(schema);

    let mut library = ambient_library.clone();
    let mut history = ambient_history.to_vec();
    let mut accepted_clause_keys: BTreeSet<CanonKey> = BTreeSet::new();
    let mut sum_nu: u64 = 0;
    let mut sum_kappa: u64 = 0;
    let mut sealing_index: u32 = 0;

    let mut reports = Vec::with_capacity(cadence_steps as usize);
    for cadence_step in 1..=cadence_steps {
        // §9.1 pin 1: the bindable domain is fixed at step opening — entries
        // sealed before this step, newest first (adjacency priority).
        let domain_top = u32::try_from(library.len()).expect("library size should fit u32");
        let domain: Vec<u32> = (1..=domain_top).rev().collect();

        let mut report = CadenceStepReport {
            cadence_step,
            domain_size: domain_top,
            candidates_scored: 0,
            accepted: Vec::new(),
        };

        loop {
            let bar = if sealing_index == 0 {
                Rational::zero()
            } else {
                phi(sealing_index as usize + 1)
                    * Rational::new(sum_nu as i64, sum_kappa as i64)
            };

            // One selection round: budgeted deterministic scan for the
            // smallest sufficient overshoot among clearing candidates.
            let mut stream = CandidateStream::new(schema, slots, &domain);
            let mut best: Option<ScoredCandidate> = None;
            let mut scored: u64 = 0;
            while scored < policy.search_budget {
                let Some(candidate) = stream.next_candidate() else {
                    break;
                };
                scored += 1;
                let Some(scored_candidate) = score_candidate(
                    &candidate,
                    base,
                    &accepted_clause_keys,
                    &library,
                    &history,
                ) else {
                    continue;
                };
                if scored_candidate.rho < bar {
                    continue;
                }
                let better = match &best {
                    None => true,
                    // Smaller ρ = smaller overshoot (bar fixed within the
                    // round); strict `<` keeps the earlier candidate on ties
                    // (deterministic enumeration order is the tie-break).
                    Some(current) => scored_candidate.rho < current.rho,
                };
                if better {
                    best = Some(scored_candidate);
                }
            }
            report.candidates_scored += scored;

            let Some(winner) = best else {
                // Local exhaustion: nothing in the local field clears; the
                // step closes (the widening move is outside this loop).
                break;
            };

            sealing_index += 1;
            assert!(
                sealing_index <= MAX_SEALINGS_GUARD,
                "engine guard: {MAX_SEALINGS_GUARD} sealings reached — extend \
                 Fibonacci width before longer runs (not a semantic cap)"
            );

            library.push(LibraryEntry::from_telescope(&winner.telescope, &library));
            history.push((base + sealing_index, winner.nu));
            for clause in &winner.telescope.clauses {
                accepted_clause_keys.insert(canonical_key_expr(&clause.expr));
            }
            sum_nu += u64::from(winner.nu);
            sum_kappa += u64::from(winner.kappa);

            report.accepted.push(AcceptedApplication {
                cadence_step,
                sealing_index,
                telescope: winner.telescope,
                residual_kappa: winner.residual_kappa,
                nu: winner.nu,
                kappa: winner.kappa,
                rho: winner.rho,
                bar,
            });
        }

        reports.push(report);
    }

    reports
}

/// The per-step ν lists of a field-schedule run — the mechanics-artifact
/// payload (§9 checklist item 3: ν numbers only; no engagement fractions, no
/// demand, no crossings).
pub fn field_nu_profile(
    schema: &Telescope,
    ambient_library: &Library,
    ambient_history: &[(u32, u32)],
    cadence_steps: u32,
    policy: &FieldPolicy,
) -> Vec<Vec<u32>> {
    run_step_loop(schema, ambient_library, ambient_history, cadence_steps, policy)
        .into_iter()
        .map(|report| report.accepted.iter().map(|app| app.nu).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        genesis_reference_context, instantiate_binding, instantiate_partial_binding,
        is_transport_image, run_step_loop, schema_slot_count, CandidateStream, FieldPolicy,
    };
    use crate::lambda_trigger_v2::frozen_structure_schema_variants_v2;
    use pen_core::canonical::{canonical_key_expr, canonical_key_telescope, CanonKey};
    use pen_core::expr::Expr;
    use std::collections::BTreeSet;

    fn sigma_pair(left: u32, right: u32) -> Expr {
        Expr::Sigma(Box::new(Expr::Lib(left)), Box::new(Expr::Lib(right)))
    }

    #[test]
    fn slot_counts_match_the_frozen_schemas() {
        let variants = frozen_structure_schema_variants_v2();
        let expected = [1_usize, 4, 5]; // minimal, weighted, persistent
        for ((name, _, telescope), expected_slots) in variants.iter().zip(expected) {
            assert_eq!(
                schema_slot_count(telescope),
                expected_slots,
                "{name}: stratum-internal Var occurrences"
            );
        }
    }

    #[test]
    fn binding_replaces_slots_in_preorder() {
        let variants = frozen_structure_schema_variants_v2();
        let (_, _, weighted) = &variants[1];
        let bound = instantiate_binding(weighted, &[16, 17, 16, 18]);
        // transport: Pi(Lib(11), Var(1)) -> Pi(Lib(11), Lib(16))
        assert_eq!(
            bound.clauses[1].expr,
            Expr::Pi(Box::new(Expr::Lib(11)), Box::new(Expr::Lib(16)))
        );
        // merger: Sigma(Var(1), Var(1)) -> Sigma(Lib(17), Lib(16))
        assert_eq!(bound.clauses[3].expr, sigma_pair(17, 16));
        // measure: App(Lib(14), Var(1)) -> App(Lib(14), Lib(18))
        assert_eq!(
            bound.clauses[4].expr,
            Expr::App(Box::new(Expr::Lib(14)), Box::new(Expr::Lib(18)))
        );
    }

    #[test]
    fn partial_bindings_keep_internal_slots() {
        let variants = frozen_structure_schema_variants_v2();
        let (_, _, weighted) = &variants[1];
        let bound = instantiate_partial_binding(weighted, &[None, Some(16), Some(17), None]);
        // Transport keeps its frozen internal Var form.
        assert_eq!(bound.clauses[1].expr, weighted.clauses[1].expr);
        // Merger binds both slots: Sigma(Lib(16), Lib(17)).
        assert_eq!(bound.clauses[3].expr, sigma_pair(16, 17));
        // Measure keeps its frozen internal Var form.
        assert_eq!(bound.clauses[4].expr, weighted.clauses[4].expr);
    }

    #[test]
    fn transport_image_rule_matches_the_sealing_order_sigma() {
        let base = 15;
        let mut accepted: BTreeSet<CanonKey> = BTreeSet::new();
        accepted.insert(canonical_key_expr(&sigma_pair(16, 16)));

        // σ-image: the same pattern one sealing position up.
        assert!(is_transport_image(&sigma_pair(17, 17), base, &accepted));
        // Mixed levels: a genuinely new combination, not an image.
        assert!(!is_transport_image(&sigma_pair(16, 17), base, &accepted));
        // History-derivable repeat.
        assert!(is_transport_image(&sigma_pair(16, 16), base, &accepted));
        // References the first stratum entry: no σ-preimage exists, and it
        // is not itself accepted → not discounted.
        assert!(!is_transport_image(&sigma_pair(16, 18), base, &accepted));
        // Ambient-only expressions discount only as exact repeats.
        let ambient = Expr::App(Box::new(Expr::Univ), Box::new(Expr::Lib(13)));
        assert!(!is_transport_image(&ambient, base, &accepted));
        accepted.insert(canonical_key_expr(&ambient));
        assert!(is_transport_image(&ambient, base, &accepted));
    }

    #[test]
    fn candidate_stream_is_frozen_schema_first_then_newest_first_lexicographic() {
        let variants = frozen_structure_schema_variants_v2();
        let (_, _, minimal) = &variants[0];
        let domain = [17_u32, 16, 15];
        let mut stream = CandidateStream::new(minimal, 1, &domain);

        assert_eq!(stream.next_candidate().as_ref(), Some(minimal));
        let first = stream.next_candidate().expect("first binding");
        assert_eq!(
            first.clauses[1].expr,
            Expr::Pi(Box::new(Expr::Lib(11)), Box::new(Expr::Lib(17)))
        );
        let second = stream.next_candidate().expect("second binding");
        assert_eq!(
            second.clauses[1].expr,
            Expr::Pi(Box::new(Expr::Lib(11)), Box::new(Expr::Lib(16)))
        );
    }

    #[test]
    fn step_loop_is_deterministic() {
        let (library, history) = genesis_reference_context();
        let variants = frozen_structure_schema_variants_v2();
        let (_, _, weighted) = &variants[1];
        let policy = FieldPolicy { search_budget: 40 };
        let first = run_step_loop(weighted, &library, &history, 2, &policy);
        let second = run_step_loop(weighted, &library, &history, 2, &policy);
        assert_eq!(first, second);
    }

    #[test]
    fn step_one_field_is_live_and_windows_are_respected() {
        let (library, history) = genesis_reference_context();
        let variants = frozen_structure_schema_variants_v2();
        let (_, _, minimal) = &variants[0];
        let policy = FieldPolicy { search_budget: 64 };
        let reports = run_step_loop(minimal, &library, &history, 3, &policy);

        assert_eq!(reports.len(), 3);
        assert!(
            !reports[0].accepted.is_empty(),
            "step 1 opens on a zero bar; a live schema must seal something"
        );
        // §9.1 pin 1: the domain grows only at step boundaries, by exactly
        // the previous steps' seals.
        assert_eq!(reports[0].domain_size, 15);
        let mut sealed_before = 15_u64;
        for report in &reports {
            assert_eq!(u64::from(report.domain_size), sealed_before);
            for application in &report.accepted {
                let max_ref = application
                    .telescope
                    .lib_refs()
                    .into_iter()
                    .next_back()
                    .unwrap_or(0);
                assert!(
                    u64::from(max_ref) <= sealed_before,
                    "an application may bind only entries sealed before its step"
                );
            }
            sealed_before += report.accepted.len() as u64;
        }
    }

    #[test]
    fn r1_prevents_any_application_from_resealing() {
        let (library, history) = genesis_reference_context();
        let variants = frozen_structure_schema_variants_v2();
        let (_, _, weighted) = &variants[1];
        let policy = FieldPolicy { search_budget: 40 };
        let reports = run_step_loop(weighted, &library, &history, 2, &policy);

        let mut seen: BTreeSet<CanonKey> = BTreeSet::new();
        for report in &reports {
            for application in &report.accepted {
                assert!(
                    seen.insert(canonical_key_telescope(&application.telescope)),
                    "an identical application scores ν = 0 and cannot reseal"
                );
                assert!(application.nu > 0, "accepted applications are live");
                assert!(
                    application.rho >= application.bar,
                    "accepted applications cleared their bar"
                );
            }
        }

        // Field-generator inertness check (the mechanics analog of Gate 3's
        // "else R1 is inert"): with the connectivity gate removed, the tuple
        // field must produce at least one acceptance beyond the frozen
        // schema. No profile shape is asserted — only non-inertness.
        assert!(
            seen.len() >= 2,
            "the tuple field must not be inert for a hierarchical variant"
        );
        let frozen_key = canonical_key_telescope(weighted);
        assert!(
            seen.iter().any(|key| *key != frozen_key),
            "some accepted application must be a genuine tuple instantiation"
        );
    }
}
