//! T1 driver — the Genesis squeeze verification, run through the engine
//! itself (`note_canonical_course.md` §7, T1; ch_genesis_mathematics "Why
//! the Sequence Halts").
//!
//! **How the lane is probed.** The strict guarded lane is driven one step
//! past Genesis: the fifteen reference telescopes are replayed into the
//! library/history and the engine's own per-step machinery
//! (`engine::probe_next_step_unclamped`; Guarded admissibility =
//! `strict_canon_guarded`'s mode, canonical grammar, window depth 2)
//! searches step 16 with its full discovery pipeline — catalog DFS,
//! branch-and-bound pruning against the bar, canonical dedup,
//! admissibility, minimality. Nothing is altered for the probe. Two earlier
//! drafts are superseded and their lessons recorded: the flat
//! `enumerate_telescopes` materializer exhausts memory on the step-16
//! surface (the reason the DFS path exists), and the bootstrap driver
//! (`search_bootstrap_from_prefix_*`) clamps at
//! `LIVE_BOOTSTRAP_MAX_STEP` = 15 and silently skips step 16 — probing the
//! halt through it would be circular.
//!
//! **The verdict shape.** The engine's acceptance stage fails with "no
//! candidate cleared the bar at step 16" when — after bounds-based pruning
//! that discards only states *provably unable to clear* — no admissible
//! candidate reaches Bar₁₆. That error, from the lane's own machinery, is
//! the external half of the squeeze. An `Ok` return with an accepted
//! sixteenth step is falsifier (a) of the canonical-course note and is
//! reported verbatim. Any *other* error is classified as a probe failure,
//! not as the squeeze holding — the test fails rather than declaring a
//! false positive.
//!
//! The internal half (univalent identification leaves the internal class at
//! ν = 0) is verified by `pen_eval::halting`'s exemplars, independent of
//! the search.

use crate::config::GrammarProfile;
use crate::diversify::FrontierRuntimeLimits;
use crate::engine::probe_next_step_unclamped;
use crate::enumerate::{
    EnumerationContext, assess_raw_surface_membership, raw_clause_catalog_widths,
};
use pen_core::canonical::canonical_key_telescope;
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::telescope::Telescope;
use pen_eval::bar::{clears_bar, compute_bar, compute_rho};
use pen_eval::halting::{
    InternalExemplar, accepted_canonical_keys, genesis_bar_16, genesis_history,
    internal_case_exemplars,
};
use pen_eval::lambda_trigger::rational_string;
use pen_eval::minimality::analyze_semantic_minimality;
use pen_eval::nu::structural_nu;
use pen_eval::p5_record::{ImportDag, P5ImportAudit};
use pen_type::admissibility::{
    AdmissibilityMode, StrictAdmissibility, assess_strict_admissibility,
    strict_admissibility_for_mode,
};
use pen_type::check::{CheckResult, check_telescope};
use pen_type::connectivity::passes_connectivity;
use serde::Serialize;

pub const T1_DATE: &str = "2026-07-05";
/// Date of the raw-surface and semantic-minimality correction to the
/// adversarial report. The original full-pipeline T1 run keeps `T1_DATE`.
pub const T1_SURFACE_AUDIT_DATE: &str = "2026-07-18";

/// The lane's step-16 outcome, classified.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum LaneStep16Outcome {
    /// The engine sealed a sixteenth extension: falsifier (a). Reported
    /// verbatim for the full-pipeline re-examination the falsifier demands.
    Accepted {
        candidate_hash: String,
        canonical_hash: String,
        clause_kappa: u16,
        nu: u16,
        rho: String,
        objective_bar: String,
        telescope: Telescope,
    },
    /// The lane's own acceptance stage found nothing clearing the bar.
    SqueezeHeld { engine_report: String },
    /// The probe did not produce a verdict — investigate; this is NOT
    /// evidence for the squeeze.
    ProbeFailure { error: String },
}

/// Full T1 report.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct T1Report {
    pub date: String,
    pub claim: String,
    pub lane: String,
    pub bar_frozen_constant: String,
    pub outcome: LaneStep16Outcome,
    pub internal_exemplars: Vec<InternalExemplar>,
    pub internal_all_zero: bool,
    pub squeeze_verified: bool,
}

/// One clause-κ slice of the step-16 raw discovery surface, counted without
/// materialization.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct KappaSurface {
    pub clause_kappa: u16,
    /// Raw clause-catalog width per telescope position.
    pub position_widths: Vec<usize>,
    /// Product of the widths (saturating): the raw telescope surface this κ
    /// would ask the materializing paths to hold.
    pub raw_telescope_product: String,
}

/// The step-16 admissibility context and raw surface geometry — computed by
/// exact counting only (`raw_clause_catalog_widths`), safe on any machine.
///
/// Added after BOTH materializing probe designs (flat enumeration, then the
/// engine's own per-step catalog path) failed with a 3 GiB allocation on a
/// desktop: the strict lane has never faced its own step-16 surface, and
/// with the post-15 debt servicing every package the generic surface is
/// evidently enormous. This diagnostic measures it before any further
/// execution strategy is chosen.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Step16SurfaceDiagnostics {
    pub admissibility: StrictAdmissibility,
    pub per_kappa: Vec<KappaSurface>,
}

/// Counts the step-16 surface without materializing it.
pub fn step16_surface_diagnostics() -> Step16SurfaceDiagnostics {
    let (library, _, _) = genesis_history();
    let admissibility = strict_admissibility_for_mode(16, 2, &library, AdmissibilityMode::Guarded);
    let context = EnumerationContext::from_admissibility(&library, admissibility);

    let per_kappa = (admissibility.min_clause_kappa..=admissibility.max_clause_kappa)
        .map(|clause_kappa| {
            let position_widths = raw_clause_catalog_widths(context, clause_kappa);
            let product = position_widths
                .iter()
                .fold(1_u128, |acc, width| acc.saturating_mul(*width as u128));
            KappaSurface {
                clause_kappa,
                position_widths,
                raw_telescope_product: product.to_string(),
            }
        })
        .collect();

    Step16SurfaceDiagnostics {
        admissibility,
        per_kappa,
    }
}

/// Drives the strict lane at step 16 from the replayed Genesis history.
///
/// The bootstrap driver (`search_bootstrap_from_prefix_*`) clamps at
/// `LIVE_BOOTSTRAP_MAX_STEP` = 15 — an engineering honesty bound on the
/// supported range, discovered when the first probe returned instantly with
/// no step-16 record. Verifying the halt claim through a driver that
/// refuses to look at step 16 would be circular, so the probe enters the
/// per-step machinery directly (`engine::probe_next_step_unclamped`):
/// the same discovery/screening/acceptance pipeline, no clamp, nothing else
/// altered (Guarded admissibility = the strict_canon_guarded lane's mode;
/// canonical grammar; unlimited frontier runtime).
pub fn run_lane_step16() -> LaneStep16Outcome {
    let (library, _, records) = genesis_history();
    let result = probe_next_step_unclamped(
        16,
        2,
        &library,
        &records,
        AdmissibilityMode::Guarded,
        GrammarProfile::CanonicalMbttV1,
        FrontierRuntimeLimits::unlimited(),
    );

    match result {
        Ok(step) => LaneStep16Outcome::Accepted {
            candidate_hash: step.accepted.candidate_hash.clone(),
            canonical_hash: step.accepted.canonical_hash.clone(),
            clause_kappa: step.accepted.clause_kappa,
            nu: step.accepted.nu,
            rho: rational_string(step.accepted.rho),
            objective_bar: rational_string(step.objective_bar),
            telescope: step.accepted.telescope.clone(),
        },
        Err(error) => {
            let message = format!("{error:#}");
            let squeeze_shapes = [
                "no candidate cleared the bar at step 16",
                "no semantically minimal candidates survived for step 16",
            ];
            if squeeze_shapes.iter().any(|shape| message.contains(shape)) {
                LaneStep16Outcome::SqueezeHeld {
                    engine_report: message,
                }
            } else {
                LaneStep16Outcome::ProbeFailure { error: message }
            }
        }
    }
}

/// Runs the full T1 verification: lane verdict + internal-class check.
pub fn verify_genesis_squeeze() -> T1Report {
    let (library, pairs, _) = genesis_history();
    let outcome = run_lane_step16();
    let internal_exemplars = internal_case_exemplars(&library, &pairs);
    let internal_all_zero = internal_exemplars
        .iter()
        .all(|exemplar| exemplar.nu_total == 0);
    let squeeze_verified =
        matches!(outcome, LaneStep16Outcome::SqueezeHeld { .. }) && internal_all_zero;

    T1Report {
        date: T1_DATE.to_owned(),
        claim: "no Step-16 candidate clears the bar in the current lane \
                (ch_genesis_mathematics, the Step-16 squeeze; T1 of \
                note_canonical_course.md)"
            .to_owned(),
        lane: "strict_canon_guarded, window depth 2, Genesis reference prefix \
               replayed through step 15, engine step machinery unaltered"
            .to_owned(),
        bar_frozen_constant: rational_string(genesis_bar_16()),
        outcome,
        internal_exemplars,
        internal_all_zero,
        squeeze_verified,
    }
}

// ---------------------------------------------------------------------------
// Adversarial probe (chosen strategy after the surface measurement):
// exhaustive step-16 enumeration is infeasible (raw surface 10^11..10^24),
// so the probe attacks the squeeze with the *formula-maximizing* candidate
// shapes and runs each through the real gate stack. Non-exhaustive by
// construction — the scope disclosure travels with the report.
// ---------------------------------------------------------------------------

/// One adversarial candidate's passage through the lane's gates.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AdversarialGateTrace {
    pub name: String,
    pub rationale: String,
    pub clause_kappa: u32,
    /// Canonical identification with a sealed entry (univalence's engine
    /// face): if true, ν ≡ 0 by identification and no formula is consulted.
    pub identified_with_sealed_structure: bool,
    pub admissibility_class: String,
    pub admissibility_reason: String,
    pub admitted: bool,
    /// True iff the raw expression/catalog generator can actually emit this
    /// hand-built telescope under the step's size, feature, path-dimension,
    /// scope, and library-reference bounds.
    pub raw_surface_member: bool,
    pub raw_surface_rejections: Vec<String>,
    pub type_checks: bool,
    pub type_error: Option<String>,
    pub connectivity_passes: bool,
    pub semantically_minimal: bool,
    pub bar_clearing_detachable_subbundles: usize,
    /// The graph premise of the paper's P5-record theorem.  `Some` means the
    /// structural evaluator classified this telescope as Axiomatic.  A true
    /// value is necessary but not sufficient for P5: constructive
    /// irreducibility and Minimal Complete API evidence remain semantic
    /// obligations outside the current AST.
    pub p5_import_audit: Option<P5ImportAudit>,
    pub nu_g: u32,
    pub nu_c: u32,
    pub nu_h: u32,
    pub nu_total: u32,
    pub rho: Option<String>,
    pub clears_bar: bool,
    /// The falsifier condition: not identified, raw-generable, admitted,
    /// type-correct, connected, semantically minimal, and clearing Bar₁₆.
    pub survives_all_gates_and_clears: bool,
    pub telescope: Telescope,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AdversarialProbeReport {
    pub date: String,
    pub bar: String,
    pub scope_disclosure: String,
    pub candidates: Vec<AdversarialGateTrace>,
    pub clearing_survivors: usize,
    /// Engine survivors whose positive score uses the Axiomatic closed form
    /// even though the unique-dominant-import premise of P5 is false.
    pub p5_domain_failures_among_engine_survivors: usize,
    /// True iff no adversarial candidate survives all gates while clearing.
    pub squeeze_supported: bool,
}

fn nested_pi_over(refs: &[u32]) -> Expr {
    let mut iter = refs.iter().rev();
    let innermost = *iter.next().expect("weave clause needs at least one ref");
    iter.fold(Expr::Lib(innermost), |acc, lib| {
        Expr::Pi(Box::new(Expr::Lib(*lib)), Box::new(acc))
    })
}

/// Formula-maximizing shapes plus in-surface boundary controls.
///
/// Some maximizers are intentionally beyond the raw generator's leaf domain
/// (for example d > `max_path_dimension`, or more distinct Lib references
/// than the latest-two window).  `run_adversarial_probe` records that fact
/// explicitly; such a shape can diagnose formula behavior but cannot count as
/// a lane survivor.
fn adversarial_candidates(admissibility: &StrictAdmissibility) -> Vec<(String, String, Telescope)> {
    let mut candidates = Vec::new();
    let band = admissibility.min_clause_kappa..=admissibility.max_clause_kappa;
    // A right-nested Pi chain of k Lib refs costs 2k−1 expression nodes.
    let refs_per_clause = usize::from((admissibility.max_expr_nodes + 1) / 2).max(1);

    // (1) r²-maximizing weaves (Map class: ν_C = 2κ + r²). The exact shape
    // the runtime-field ladder climbed; r drawn newest-first, distinct
    // across the telescope.
    for kappa in band.clone() {
        let mut pool: Vec<u32> = (1..=15).rev().collect();
        let clauses: Vec<ClauseRec> = (0..kappa)
            .map(|_| {
                let take = refs_per_clause.min(pool.len().max(1));
                let refs: Vec<u32> = if pool.len() >= take {
                    pool.drain(..take).collect()
                } else {
                    vec![15] // pool exhausted: reuse the window anchor
                };
                ClauseRec::new(ClauseRole::Formation, nested_pi_over(&refs))
            })
            .collect();
        candidates.push((
            format!("weave_max_r_kappa{kappa}"),
            "Map-class r² maximizer: 2κ + r² with r = distinct sealed refs; \
             arithmetically clears the bar from r ≥ 5 at κ = 2"
                .to_owned(),
            Telescope::new(clauses),
        ));
    }

    // A unique-import variant of the Axiomatic witness. Unlike the
    // two-import candidate above, this one satisfies P5's graph-theoretic
    // unique-dominant-import premise: its only direct import is L15.
    if band.contains(&3) {
        candidates.push((
            "axiomatic_single_l15_kappa3".to_owned(),
            "single-import Axiomatic boundary: inherits nu(L15) while satisfying the \
             unique-dominant-import graph premise"
                .to_owned(),
            Telescope::new(vec![
                ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(Box::new(Expr::Lib(15)), Box::new(Expr::Var(1))),
                ),
                ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
                ),
                ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::App(Box::new(Expr::Lib(15)), Box::new(Expr::Var(1))),
                ),
            ]),
        ));
    }

    // Two raw size-five temporal clauses each match the evaluator's
    // polymorphic-temporal-eliminator predicate. Their two library-sized
    // bonuses clear the bar without any P5 inheritance or library import.
    if band.contains(&2) {
        let temporal_polymorphic_clause = || {
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
                ),
            )
        };
        candidates.push((
            "temporal_polymorphic_kappa2".to_owned(),
            "Synthesis boundary: two polymorphic temporal clauses each receive a \
             library-sized universe-polymorphism bonus"
                .to_owned(),
            Telescope::new(vec![
                temporal_polymorphic_clause(),
                temporal_polymorphic_clause(),
            ]),
        ));
    }

    // The strongest L2-shaped Map surface the raw open-band generator can
    // actually express without a historical anchor: only Lib(14)/Lib(15)
    // are leaf choices, hence r <= 2.  At kappa=2 its engine score is
    // 2*kappa + r^2 = 8, rho=4, below Bar_16.
    if band.contains(&2) {
        candidates.push((
            "weave_surface_max_r2_kappa2".to_owned(),
            "raw-surface L2 boundary control: latest-two reference window gives r=2".to_owned(),
            Telescope::new(vec![
                ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(Box::new(Expr::Lib(15)), Box::new(Expr::Lib(14))),
                ),
                ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(Box::new(Expr::Lib(14)), Box::new(Expr::Lib(15))),
                ),
            ]),
        ));
    }

    // (2) Axiomatic inheritance: ν_C = ν(L_max) + κ + (r−1) inherits the
    // temporal shell's ν = 103 — the leverage the book's external case says
    // a disconnected jump cannot have. The gates must say why not.
    for kappa in [3_u16, 4] {
        if !band.contains(&kappa) {
            continue;
        }
        let mut clauses = vec![
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(Box::new(Expr::Lib(15)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(
                // Raw generation deterministically assigns App(Lib, _)
                // the Introduction role.  Using Computation here would make
                // the hand-built witness expression-valid but absent from
                // the actual clause catalog.
                ClauseRole::Introduction,
                Expr::App(Box::new(Expr::Lib(14)), Box::new(Expr::Var(1))),
            ),
        ];
        if kappa == 4 {
            clauses.push(ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(Box::new(Expr::Lib(13)), Box::new(Expr::Var(1))),
            ));
        }
        candidates.push((
            format!("axiomatic_inheritance_kappa{kappa}"),
            "Axiomatic-class ν(L_max) inheritance: references the temporal \
             shell, formula grants max_ref_nu = 103"
                .to_owned(),
            Telescope::new(clauses),
        ));
    }

    // (3) High-dimensional path packages (Hit: ν_H = 1 + d²): d = 4 clears
    // arithmetically at κ = 2 if the grammar admits the dimension.
    if band.contains(&2) {
        for dimension in [1_u32, 3, 4, 5] {
            candidates.push((
                format!("hit_path_d{dimension}"),
                "Hit-class d² maximizer over the sealed window".to_owned(),
                Telescope::new(vec![
                    ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::App(Box::new(Expr::Univ), Box::new(Expr::Lib(15))),
                    ),
                    ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(dimension)),
                ]),
            ));
        }
    }

    // The raw d=1 surface can still clear without a formation clause: the
    // Hit closed form then charges kappa + |library| in nu_C. The later
    // point constructor connects the leading path attachment.
    if band.contains(&2) {
        candidates.push((
            "hit_no_formation_d1".to_owned(),
            "Hit boundary: a d=1 path package without formation receives the \
             kappa-plus-library fallback"
                .to_owned(),
            Telescope::new(vec![
                ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(1)),
                ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
            ]),
        ));
    }

    // (4) Controls: a verbatim re-proposal (identification must catch it),
    // the modal shell rebuild (likewise), a pure re-reference (trivially
    // derivable), and a bare temporal pair (include_temporal gate).
    candidates.push((
        "control_reproposal_step13".to_owned(),
        "verbatim re-proposal of the metric shell — internal case".to_owned(),
        Telescope::reference(13),
    ));
    if band.contains(&4) {
        candidates.push((
            "control_modal_rebuild_step10".to_owned(),
            "verbatim re-proposal of the modal shell — internal case".to_owned(),
            Telescope::reference(10),
        ));
    }
    candidates.push((
        "control_window_rereference".to_owned(),
        "pure pointer re-reference — trivially derivable, ν = 0".to_owned(),
        Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Lib(15)),
            ClauseRec::new(ClauseRole::Formation, Expr::Lib(14)),
        ]),
    ));
    candidates.push((
        "control_bare_temporal_pair".to_owned(),
        "Next/Eventually pair — include_temporal legality control".to_owned(),
        Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Next(Box::new(Expr::Var(1)))),
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Var(1))),
            ),
        ]),
    ));

    candidates
}

/// Runs every adversarial candidate through the lane's gate stack in order:
/// canonical identification → raw-catalog membership → strict admissibility
/// → type check → connectivity → evaluator → semantic minimality →
/// Bar₁₆. All gates are recorded even after a failure, so the report shows
/// *which* wall stops each shape.
pub fn run_adversarial_probe() -> AdversarialProbeReport {
    let (library, pairs, records) = genesis_history();
    let admissibility = strict_admissibility_for_mode(16, 2, &library, AdmissibilityMode::Guarded);
    let enumeration_context = EnumerationContext::from_admissibility(&library, admissibility);
    let bar = compute_bar(2, 16, &records).bar;
    assert_eq!(
        bar,
        genesis_bar_16(),
        "replayed bar must match the frozen constant"
    );
    let accepted_keys = accepted_canonical_keys();
    let import_dag = ImportDag::genesis_prefix(15);

    let candidates = adversarial_candidates(&admissibility)
        .into_iter()
        .map(|(name, rationale, telescope)| {
            let identified = accepted_keys.contains(&canonical_key_telescope(&telescope));
            let decision = assess_strict_admissibility(16, &library, &telescope, admissibility);
            let raw_surface = assess_raw_surface_membership(enumeration_context, &telescope);
            let (type_checks, type_error) = match check_telescope(&library, &telescope) {
                CheckResult::Ok => (true, None),
                CheckResult::Err(error) => (false, Some(format!("{error}"))),
            };
            let connectivity = passes_connectivity(&library, &telescope);
            let result = structural_nu(&telescope, &library, &pairs);
            let kappa = u32::try_from(telescope.kappa()).expect("kappa fits u32");
            let (nu_total, rho) = if identified {
                (0, None) // ν ≡ 0 by identification; the formula is not consulted
            } else {
                (result.total, compute_rho(result.total, kappa))
            };
            let clears = rho.map(|rho| clears_bar(rho, bar)).unwrap_or(false);
            let minimality =
                analyze_semantic_minimality(16, bar, admissibility, &telescope, &library, &pairs);
            let semantically_minimal = minimality.is_minimal();
            let bar_clearing_detachable_subbundles =
                minimality.admissible_bar_clear_subbundles.len();
            let p5_import_audit = (telescope.classify(&library)
                == pen_core::telescope::TelescopeClass::Axiomatic)
                .then(|| P5ImportAudit::check(&telescope, &import_dag));

            AdversarialGateTrace {
                name,
                rationale,
                clause_kappa: kappa,
                identified_with_sealed_structure: identified,
                admissibility_class: decision.class.as_str().to_owned(),
                admissibility_reason: decision.reason.clone(),
                admitted: decision.is_admitted(),
                raw_surface_member: raw_surface.is_member,
                raw_surface_rejections: raw_surface.rejection_reasons,
                type_checks,
                type_error,
                connectivity_passes: connectivity,
                semantically_minimal,
                bar_clearing_detachable_subbundles,
                p5_import_audit,
                nu_g: result.nu_g,
                nu_c: result.nu_c,
                nu_h: result.nu_h,
                nu_total,
                rho: rho.map(rational_string),
                clears_bar: clears,
                survives_all_gates_and_clears: !identified
                    && decision.is_admitted()
                    && raw_surface.is_member
                    && type_checks
                    && connectivity
                    && semantically_minimal
                    && clears,
                telescope,
            }
        })
        .collect::<Vec<_>>();

    let clearing_survivors = candidates
        .iter()
        .filter(|trace| trace.survives_all_gates_and_clears)
        .count();
    let p5_domain_failures_among_engine_survivors = candidates
        .iter()
        .filter(|trace| trace.survives_all_gates_and_clears)
        .filter(|trace| {
            trace
                .p5_import_audit
                .as_ref()
                .is_some_and(|audit| !audit.unique_dominant_import_holds)
        })
        .count();

    AdversarialProbeReport {
        date: T1_SURFACE_AUDIT_DATE.to_owned(),
        bar: rational_string(bar),
        scope_disclosure: "adversarial, NOT exhaustive: formula-maximizing shapes \
                           (Map r², Axiomatic ν(L_max) inheritance, Hit d²) plus \
                           identification/legality controls; every hand-built \
                           candidate is checked for exact raw-catalog membership \
                           and semantic minimality before it can count as a \
                           survivor; exhaustive verification remains blocked on \
                           a streaming step-16 search"
            .to_owned(),
        candidates,
        clearing_survivors,
        p5_domain_failures_among_engine_survivors,
        squeeze_supported: clearing_survivors == 0,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        LaneStep16Outcome, run_adversarial_probe, step16_surface_diagnostics,
        verify_genesis_squeeze,
    };

    /// T1 (adversarial form): preserve the currently known, raw-generable,
    /// semantically minimal Step-16 clearer as falsifier evidence.  The test
    /// also prevents out-of-surface L1/L2 maximizers from being mislabeled as
    /// lane survivors. Prints the full gate table with --nocapture.
    #[test]
    fn t1_adversarial_probe_records_only_raw_minimal_clearing_survivors() {
        let report = run_adversarial_probe();
        for trace in &report.candidates {
            println!(
                "{}: kappa {}, identified {}, generated {}, admissibility {} ({}), \
                 type_ok {}, connected {}, minimal {}, nu {} (G{}/C{}/H{}), \
                 rho {:?}, clears {}, SURVIVES {}",
                trace.name,
                trace.clause_kappa,
                trace.identified_with_sealed_structure,
                trace.raw_surface_member,
                trace.admissibility_class,
                trace.admissibility_reason,
                trace.type_checks,
                trace.connectivity_passes,
                trace.semantically_minimal,
                trace.nu_total,
                trace.nu_g,
                trace.nu_c,
                trace.nu_h,
                trace.rho,
                trace.clears_bar,
                trace.survives_all_gates_and_clears,
            );
        }

        // Controls must behave: verbatim re-proposals are identified.
        for control in ["control_reproposal_step13", "control_modal_rebuild_step10"] {
            if let Some(trace) = report.candidates.iter().find(|t| t.name == control) {
                assert!(
                    trace.identified_with_sealed_structure,
                    "{control}: canonical identification failed to catch a verbatim re-proposal"
                );
            }
        }

        let survivors = report
            .candidates
            .iter()
            .filter(|trace| trace.survives_all_gates_and_clears)
            .map(|trace| trace.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            survivors,
            vec![
                "axiomatic_single_l15_kappa3",
                "temporal_polymorphic_kappa2",
                "axiomatic_inheritance_kappa3",
                "hit_no_formation_d1",
            ]
        );
        assert_eq!(report.clearing_survivors, 4);
        assert_eq!(report.p5_domain_failures_among_engine_survivors, 1);
        assert!(!report.squeeze_supported);

        let survivor = report
            .candidates
            .iter()
            .find(|trace| trace.name == "axiomatic_inheritance_kappa3")
            .expect("survivor trace should exist");
        let p5 = survivor
            .p5_import_audit
            .as_ref()
            .expect("survivor is structurally Axiomatic");
        assert_eq!(p5.direct_imports, vec![14, 15]);
        assert!(p5.dominant_imports.is_empty());
        assert!(!p5.unique_dominant_import_holds);

        let assert_live_exact =
            |name: &str, expected_nu: (u32, u32, u32, u32), expected_rho: &str| {
                let trace = report
                    .candidates
                    .iter()
                    .find(|trace| trace.name == name)
                    .unwrap_or_else(|| panic!("{name}: adversarial trace should exist"));
                assert!(
                    !trace.identified_with_sealed_structure,
                    "{name}: identified"
                );
                assert!(trace.raw_surface_member, "{name}: absent from raw surface");
                assert!(trace.admitted, "{name}: not admitted");
                assert!(trace.type_checks, "{name}: failed shallow type check");
                assert!(trace.connectivity_passes, "{name}: disconnected");
                assert!(
                    trace.semantically_minimal,
                    "{name}: not semantically minimal"
                );
                assert!(trace.clears_bar, "{name}: did not clear Bar16");
                assert!(
                    trace.survives_all_gates_and_clears,
                    "{name}: not a survivor"
                );
                assert_eq!(
                    (trace.nu_g, trace.nu_c, trace.nu_h, trace.nu_total),
                    expected_nu,
                    "{name}: unexpected structural score"
                );
                assert_eq!(trace.rho.as_deref(), Some(expected_rho), "{name}: rho");
            };

        assert_live_exact("axiomatic_single_l15_kappa3", (1, 106, 0, 107), "107/3");
        let single_l15 = report
            .candidates
            .iter()
            .find(|trace| trace.name == "axiomatic_single_l15_kappa3")
            .expect("single-L15 trace should exist");
        let single_l15_p5 = single_l15
            .p5_import_audit
            .as_ref()
            .expect("single-L15 witness is structurally Axiomatic");
        assert_eq!(single_l15_p5.direct_imports, vec![15]);
        assert_eq!(single_l15_p5.dominant_imports, vec![15]);
        assert_eq!(single_l15_p5.unique_dominant_import, Some(15));
        assert!(single_l15_p5.unique_dominant_import_holds);

        assert_live_exact("temporal_polymorphic_kappa2", (0, 32, 0, 32), "16/1");
        let temporal = report
            .candidates
            .iter()
            .find(|trace| trace.name == "temporal_polymorphic_kappa2")
            .expect("temporal trace should exist");
        assert!(temporal.p5_import_audit.is_none());

        assert_live_exact("hit_no_formation_d1", (0, 17, 2, 19), "19/2");
        let hit = report
            .candidates
            .iter()
            .find(|trace| trace.name == "hit_no_formation_d1")
            .expect("no-formation HIT trace should exist");
        assert!(hit.p5_import_audit.is_none());

        let l1_outside = report
            .candidates
            .iter()
            .find(|trace| trace.name == "hit_path_d4")
            .expect("d=4 boundary probe");
        assert!(!l1_outside.raw_surface_member);
        assert!(!l1_outside.survives_all_gates_and_clears);

        let l2_boundary = report
            .candidates
            .iter()
            .find(|trace| trace.name == "weave_surface_max_r2_kappa2")
            .expect("r=2 boundary probe");
        assert!(l2_boundary.raw_surface_member);
        assert_eq!(l2_boundary.nu_total, 8);
        assert!(!l2_boundary.clears_bar);
    }

    /// Fast and allocation-safe: measures the step-16 surface by exact
    /// counting. Run with `--nocapture` to see the geometry.
    #[test]
    fn step16_surface_geometry_is_measurable_without_materialization() {
        let diagnostics = step16_surface_diagnostics();
        assert!(
            !diagnostics.per_kappa.is_empty(),
            "step-16 admissibility yielded an empty κ band"
        );
        println!(
            "step-16 admissibility: kappa band {}..={}, max_expr_nodes {}, \
             focus {:?}, quota/bucket {}",
            diagnostics.admissibility.min_clause_kappa,
            diagnostics.admissibility.max_clause_kappa,
            diagnostics.admissibility.max_expr_nodes,
            diagnostics.admissibility.focus_family,
            diagnostics.admissibility.quota_per_bucket,
        );
        for surface in &diagnostics.per_kappa {
            println!(
                "  kappa {}: widths {:?} → raw product {}",
                surface.clause_kappa, surface.position_widths, surface.raw_telescope_product
            );
        }
    }

    /// T1: the engine verification of the book's halt claim — the anchor
    /// test named by note_canonical_course.md §7. A clearing sixteenth
    /// extension is falsifier (a) of that note and a book-level crisis; a
    /// probe failure is an engineering problem. Neither is a flaky test.
    ///
    /// Ignored by default: both materializing execution paths exhaust
    /// desktop memory on the step-16 surface (see the surface-geometry
    /// test). Run explicitly once an execution strategy fitting the
    /// measured surface is chosen: `cargo test -p pen-search t1_ -- --ignored`.
    #[test]
    #[ignore = "step-16 surface exceeds desktop memory on the materializing paths; run after surface diagnostics"]
    fn t1_the_genesis_squeeze_holds_in_the_current_lane() {
        let report = verify_genesis_squeeze();

        match &report.outcome {
            LaneStep16Outcome::SqueezeHeld { .. } => {}
            LaneStep16Outcome::Accepted {
                rho,
                objective_bar,
                telescope,
                ..
            } => panic!(
                "falsifier (a) of note_canonical_course.md: the lane sealed a \
                 sixteenth extension (rho {rho} vs bar {objective_bar}): {telescope:?}"
            ),
            LaneStep16Outcome::ProbeFailure { error } => {
                panic!("T1 probe failure (NOT evidence for the squeeze): {error}")
            }
        }

        assert!(
            report.internal_all_zero,
            "internal-class exemplar scored ν > 0: {:?}",
            report.internal_exemplars
        );
        assert!(report.squeeze_verified);
    }
}
